use crate::constants::{
    config::{STAKED_KOSON_KEY, UNBONDING_KOSON_KEY, UNBONDING_KOSON_NONCE_NAME},
    errors::ERR_ALREADY_ISSUED,
};

multiversx_sc::imports!();

#[multiversx_sc::module]
pub trait EsdtModule: crate::storage::StorageModule {
    #[only_owner]
    #[payable("EGLD")]
    #[endpoint(issue)]
    fn issue_token(&self, token_display_name: ManagedBuffer, token_ticker: ManagedBuffer, key: u8) {
        require!(self.staked_koson_token_id().is_empty(), ERR_ALREADY_ISSUED);

        let issue_cost = self.call_value().egld_value();

        self.send()
            .esdt_system_sc_proxy()
            .issue_and_set_all_roles(
                issue_cost.clone_value(),
                token_display_name,
                token_ticker,
                match key {
                    STAKED_KOSON_KEY => EsdtTokenType::Fungible,
                    UNBONDING_KOSON_KEY => EsdtTokenType::Meta,
                    _ => sc_panic!("Invalid issuance key"),
                },
                18,
            )
            .async_call()
            .with_callback(self.callbacks().issue_token_callback(key))
            .call_and_exit();
    }

    #[only_owner]
    #[endpoint(setTokenId)]
    fn set_token_id(&self, token_id: TokenIdentifier, key: u8) {
        match key {
            STAKED_KOSON_KEY => self.staked_koson_token_id().set(token_id),
            UNBONDING_KOSON_KEY => self.unbonding_koson_token_id().set(token_id),
            _ => sc_panic!("Invalid issuance key"),
        }
    }

    #[callback]
    fn issue_token_callback(
        &self,
        #[call_result] result: ManagedAsyncCallResult<EgldOrEsdtTokenIdentifier>,
        key: u8,
    ) {
        match result {
            ManagedAsyncCallResult::Ok(token_id) => {
                let token_id = token_id.unwrap_esdt();
                self.set_token_id(token_id, key)
            }
            ManagedAsyncCallResult::Err(_) => {
                let caller = self.blockchain().get_owner_address();
                let returned = self.call_value().egld_or_single_esdt();
                if returned.token_identifier.is_egld() && returned.amount > 0 {
                    self.send()
                        .direct(&caller, &returned.token_identifier, 0, &returned.amount);
                }
            }
        }
    }

    fn burn_esdt(&self, token_identifier: &TokenIdentifier, nonce: u64, amount: &BigUint) {
        self.send().esdt_local_burn(token_identifier, nonce, amount);
    }

    fn mint_esdt(&self, token_identifier: &TokenIdentifier, amount: &BigUint) {
        self.send().esdt_local_mint(token_identifier, 0u64, amount);
    }

    fn mint_meta_esdt<T: TopEncode>(
        &self,
        token_identifier: &TokenIdentifier,
        amount: &BigUint,
        metadata: T,
    ) -> EsdtTokenPayment {
        let nonce = self.send().esdt_nft_create_compact_named(
            token_identifier,
            amount,
            &ManagedBuffer::from(UNBONDING_KOSON_NONCE_NAME),
            &metadata,
        );

        EsdtTokenPayment::new(token_identifier.clone(), nonce, amount.clone())
    }
}
