#![no_std]

use constants::*;
use types::{
    minting_payloads::MintNftSftPayload, token_identifier_details::TokenIdentifierDetails,
};

multiversx_sc::imports!();

pub mod constants;
pub mod esdt;
pub mod storage;
pub mod types;

#[multiversx_sc::contract]
pub trait AssetsRemapper: storage::StorageModule + esdt::EsdtModule {
    #[init]
    fn init(&self) {}

    #[upgrade]
    fn upgrade(&self) {}

    #[only_owner]
    #[endpoint(mint)]
    fn mint_nft_sft_endpoint(
        &self,
        token_id: &TokenIdentifier,
        payloads: MultiValueManagedVec<MintNftSftPayload<Self::Api>>,
    ) {
        for payload in payloads.iter() {
            self.mint_nft_sft(token_id, payload);
        }
    }

    #[payable("*")]
    #[endpoint(migrate)]
    fn migrate_assets(&self) -> ManagedVec<EsdtTokenPayment> {
        let caller = self.blockchain().get_caller();
        let payments_in = self.call_value().all_esdt_transfers();
        let mut payments_out = ManagedVec::new();

        for payment in payments_in.iter() {
            payments_out.push(self.perform_swap(payment));
        }

        self.send_multi_payments_non_zero(&caller, &payments_out);

        payments_out
    }

    fn perform_swap(&self, payment_in: EsdtTokenPayment) -> EsdtTokenPayment {
        let token_id_out = self.token_identifier_map(&payment_in.token_identifier);
        require!(!token_id_out.is_empty(), ERR_TOKEN_NOT_SUPPORTED);
        let token_id_out = token_id_out.get();
        let token_id_out_details = self.token_identifier_details(&token_id_out).get();

        self.swap_from_payment(payment_in, token_id_out_details)
    }

    fn swap_from_payment(
        &self,
        payment: EsdtTokenPayment,
        token_id_out_details: TokenIdentifierDetails<Self::Api>,
    ) -> EsdtTokenPayment {
        let amount_out = token_id_out_details.get_swap_amount(&payment.amount);
        self.burn_payment(&payment);

        match token_id_out_details.token_type {
            EsdtTokenType::Fungible => {
                self.mint_esdt(&token_id_out_details.token_identifier, &amount_out)
            }
            _ => EsdtTokenPayment::new(
                token_id_out_details.token_identifier,
                payment.token_nonce,
                amount_out,
            ),
        }
    }

    fn send_multi_payments_non_zero(
        &self,
        receiver: &ManagedAddress,
        payments: &ManagedVec<EsdtTokenPayment>,
    ) {
        let zero_amount_payments: ManagedVec<EsdtTokenPayment> = payments
            .into_iter()
            .filter(|payment| payment.amount == 0)
            .collect();
        require!(zero_amount_payments.is_empty(), ERR_PAYMENT_AMOUNT_ZERO);

        self.send().direct_multi(receiver, payments);
    }
}
