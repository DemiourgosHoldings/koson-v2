use crate::constants::{
    config::POOL_INDEX_DENOMINATION,
    errors::{ERR_PAYMENT_AMOUNT_ZERO, ERR_PAYMENT_NOT_ALLOWED},
};

multiversx_sc::imports!();

#[multiversx_sc::module]
pub trait LogicModule: crate::storage::StorageModule + crate::esdt::EsdtModule {
    fn process_stake(&self, payments: &ManagedVec<EsdtTokenPayment>) -> EsdtTokenPayment {
        let mut staked_koson_amount_to_send = BigUint::zero();
        for payment in payments.iter() {
            self.require_payment_token_is_koson(&payment.token_identifier);
            staked_koson_amount_to_send += self.process_single_payment_stake(&payment);
        }

        EsdtTokenPayment::new(
            self.staked_koson_token_id().get(),
            0u64,
            staked_koson_amount_to_send,
        )
    }

    fn mint_and_send_staked_koson(&self, payment: EsdtTokenPayment) {
        require!(payment.amount > 0, ERR_PAYMENT_AMOUNT_ZERO);

        self.mint_esdt(&payment.token_identifier, &payment.amount);
        self.send().direct_multi(
            &self.blockchain().get_caller(),
            &ManagedVec::from_single_item(payment),
        );
    }

    fn process_single_payment_stake(&self, payment: &EsdtTokenPayment) -> BigUint {
        let payment_koson_supply = self.koson_supply(&payment.token_identifier).get();
        let remaining_koson_tokens =
            self.get_all_koson_token_ids_but_one(&payment.token_identifier);
        let mut remaining_koson_supply = BigUint::zero();

        for token_id in remaining_koson_tokens.iter() {
            remaining_koson_supply += self.koson_supply(&token_id).get();
        }

        let pool_index = self.get_pool_index();

        let staked_koson_amount_to_send = self.get_staked_koson_amount_out(
            &payment.amount,
            &payment_koson_supply,
            &remaining_koson_supply,
            &pool_index,
        );

        self.staked_koson_supply(&self.staked_koson_token_id().get())
            .update(|old_supply| *old_supply += &staked_koson_amount_to_send);
        self.koson_supply(&payment.token_identifier)
            .update(|old_supply| *old_supply += &payment.amount);

        staked_koson_amount_to_send
    }

    fn get_pool_index(&self) -> BigUint {
        let mut total_koson_supply = BigUint::zero();
        let mut total_staked_koson_supply = BigUint::zero();

        for token_id in self.koson_token_ids().iter() {
            total_koson_supply += self.koson_supply(&token_id).get();
        }

        for token_id in [
            self.staked_koson_token_id().get(),
            self.unbonding_koson_token_id().get(),
        ]
        .iter()
        {
            total_staked_koson_supply += self.staked_koson_supply(token_id).get();
        }

        total_koson_supply * POOL_INDEX_DENOMINATION / total_staked_koson_supply
    }

    fn get_all_koson_token_ids_but_one(
        &self,
        token_id_to_omit: &TokenIdentifier,
    ) -> ManagedVec<TokenIdentifier> {
        let mut other_tokens = ManagedVec::new();
        for token_id in self.koson_token_ids().iter() {
            if &token_id != token_id_to_omit {
                other_tokens.push(token_id);
            }
        }

        other_tokens
    }

    fn get_staked_koson_amount_out(
        &self,
        payment_in_amount: &BigUint,
        payment_koson_supply: &BigUint,
        remaining_koson_types_supply: &BigUint,
        pool_index: &BigUint,
    ) -> BigUint {
        payment_in_amount * pool_index * payment_koson_supply
            / remaining_koson_types_supply
            / POOL_INDEX_DENOMINATION
    }

    fn require_payment_token_is_koson(&self, token_id: &TokenIdentifier) {
        require!(
            self.koson_token_ids().contains(token_id),
            ERR_PAYMENT_NOT_ALLOWED
        );
    }
}
