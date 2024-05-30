use crate::{
    constants::{
        config::POOL_INDEX_DENOMINATOR,
        errors::{ERR_PAYMENT_AMOUNT_ZERO, ERR_PAYMENT_NOT_ALLOWED},
    },
    types::wrapped_payment::WrappedPayment,
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

        self.mint_esdt(
            &self.staked_koson_token_id().get(),
            &staked_koson_amount_to_send,
        );

        EsdtTokenPayment::new(
            self.staked_koson_token_id().get(),
            0u64,
            staked_koson_amount_to_send,
        )
    }

    fn send_payment_non_zero(&self, payment: EsdtTokenPayment) {
        require!(payment.amount > 0, ERR_PAYMENT_AMOUNT_ZERO);
        self.send().direct_multi(
            &self.blockchain().get_caller(),
            &ManagedVec::from_single_item(payment),
        );
    }

    fn send_multi_payments_non_zero(&self, payments: &ManagedVec<EsdtTokenPayment>) {
        let zero_amount_payments: ManagedVec<EsdtTokenPayment> = payments
            .into_iter()
            .filter(|payment| payment.amount == 0)
            .collect();
        require!(zero_amount_payments.is_empty(), ERR_PAYMENT_AMOUNT_ZERO);

        self.send()
            .direct_multi(&self.blockchain().get_caller(), payments);
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

    fn process_unstake(&self, payment: &EsdtTokenPayment) -> EsdtTokenPayment {
        self.require_token_ids_match(
            &payment.token_identifier,
            &self.staked_koson_token_id().get(),
        );

        let unbonding_koson_token_id = self.unbonding_koson_token_id().get();

        self.burn_esdt(&payment.token_identifier, 0u64, &payment.amount);
        self.staked_koson_supply(&payment.token_identifier)
            .update(|old_supply| *old_supply -= &payment.amount);

        let unbonding_koson_payment = self.mint_meta_esdt(
            &unbonding_koson_token_id,
            &payment.amount,
            WrappedPayment::new(self.blockchain().get_block_epoch()),
        );

        self.staked_koson_supply(&unbonding_koson_token_id)
            .update(|old_supply| *old_supply += &payment.amount);

        unbonding_koson_payment
    }

    fn process_claim_unstaked(
        &self,
        payments_in: &ManagedVec<EsdtTokenPayment>,
    ) -> ManagedVec<EsdtTokenPayment> {
        let unbonding_koson_token_id = self.unbonding_koson_token_id().get();
        let total_staked_koson_supply = self
            .staked_koson_supply(&self.staked_koson_token_id().get())
            .get()
            + self
                .staked_koson_supply(&self.unbonding_koson_token_id().get())
                .get();

        let mut payments_out = ManagedVec::new();

        for payment in payments_in.iter() {
            self.require_token_ids_match(&payment.token_identifier, &unbonding_koson_token_id);
            let unbonded_payments =
                self.get_unbonded_koson_amounts_out(&payment.amount, &total_staked_koson_supply);

            for unbonded_payment in unbonded_payments.iter() {
                payments_out.push(unbonded_payment);
            }

            self.staked_koson_supply(&payment.token_identifier)
                .update(|old_supply| {
                    *old_supply -= &payment.amount;
                });
            self.burn_esdt(&payment.token_identifier, 0u64, &payment.amount);
        }

        payments_out
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

        total_koson_supply * POOL_INDEX_DENOMINATOR / total_staked_koson_supply
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
            / POOL_INDEX_DENOMINATOR
    }

    fn get_unbonded_koson_amounts_out(
        &self,
        token_amount_in: &BigUint,
        total_staked_koson_supply: &BigUint,
    ) -> ManagedVec<EsdtTokenPayment> {
        let mut payments_out = ManagedVec::new();

        for koson_token_id in self.koson_token_ids().iter() {
            let supply = self.koson_supply(&koson_token_id).get();

            let amount_to_send = token_amount_in * &supply / total_staked_koson_supply;
            payments_out.push(EsdtTokenPayment::new(koson_token_id, 0u64, amount_to_send));
        }

        payments_out
    }

    fn handle_distribute_rewards(&self, payments: &ManagedVec<EsdtTokenPayment>) {
        for payment in payments.iter() {
            self.require_payment_token_is_koson(&payment.token_identifier);
            self.koson_supply(&payment.token_identifier)
                .update(|old_supply| *old_supply += &payment.amount);
        }
    }

    fn require_payment_token_is_koson(&self, token_id: &TokenIdentifier) {
        require!(
            self.koson_token_ids().contains(token_id),
            ERR_PAYMENT_NOT_ALLOWED
        );
    }

    fn require_token_ids_match(&self, token_id_1: &TokenIdentifier, token_id_2: &TokenIdentifier) {
        require!(token_id_1 == token_id_2, ERR_PAYMENT_NOT_ALLOWED);
    }
}
