use crate::types::{
    minting_payloads::MintNftSftPayload, token_identifier_details::TokenIdentifierDetails,
};

multiversx_sc::imports!();

#[multiversx_sc::module]
pub trait EsdtModule: crate::storage::StorageModule {
    #[only_owner]
    #[payable("EGLD")]
    #[endpoint(issue)]
    #[allow_multiple_var_args]
    fn issue_token(
        &self,
        token_display_name: ManagedBuffer,
        token_ticker: ManagedBuffer,
        token_type: EsdtTokenType,
        num_decimals: usize,
        target_token_id: TokenIdentifier,
        ratio_numerator: OptionalValue<BigUint>,
        ratio_denominator: OptionalValue<BigUint>,
    ) {
        let ratio_numerator = match ratio_numerator {
            OptionalValue::Some(value) => value,
            OptionalValue::None => BigUint::from(1u32),
        };
        let ratio_denominator = match ratio_denominator {
            OptionalValue::Some(value) => value,
            OptionalValue::None => BigUint::from(1u32),
        };

        let issue_cost = self.call_value().egld_value();
        self.send()
            .esdt_system_sc_proxy()
            .issue_and_set_all_roles(
                issue_cost.clone_value(),
                token_display_name,
                token_ticker,
                token_type.clone(),
                num_decimals,
            )
            .with_callback(self.callbacks().issue_token_callback(
                target_token_id,
                token_type,
                num_decimals,
                ratio_numerator,
                ratio_denominator,
            ))
            .async_call_and_exit();
    }

    #[callback]
    fn issue_token_callback(
        &self,
        #[call_result] result: ManagedAsyncCallResult<EgldOrEsdtTokenIdentifier>,
        key: TokenIdentifier,
        token_type: EsdtTokenType,
        num_decimals: usize,
        ratio_numerator: BigUint,
        ratio_denominator: BigUint,
    ) {
        match result {
            ManagedAsyncCallResult::Ok(token_id) => {
                let token_id = token_id.unwrap_esdt();
                self.map_tokens(
                    key,
                    token_id,
                    token_type,
                    num_decimals,
                    ratio_numerator,
                    ratio_denominator,
                );
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

    #[payable("*")]
    #[only_owner]
    #[endpoint(mapTokens)]
    fn map_tokens(
        &self,
        token_id_in: TokenIdentifier,
        token_id_out: TokenIdentifier,
        token_type: EsdtTokenType,
        num_decimals: usize,
        ratio_numerator: BigUint,
        ratio_denominator: BigUint,
    ) {
        let payments = self.call_value().all_esdt_transfers();
        let disable_mint = payments.len() > 0;

        if disable_mint {
            for payment in payments.iter() {
                require!(
                    payment.token_identifier == token_id_out,
                    "Invalid payment token"
                );
            }
        }

        self.token_identifier_map(&token_id_in).set(&token_id_out);
        self.token_identifier_details(&token_id_out)
            .set(TokenIdentifierDetails::new(
                token_id_out,
                token_type,
                num_decimals,
                ratio_numerator,
                ratio_denominator,
                disable_mint,
            ));
    }

    fn burn_esdt(&self, token_identifier: &TokenIdentifier, nonce: u64, amount: &BigUint) {
        self.send().esdt_local_burn(token_identifier, nonce, amount);
    }

    fn burn_payment(&self, payment: &EsdtTokenPayment) {
        self.burn_esdt(
            &payment.token_identifier,
            payment.token_nonce,
            &payment.amount,
        );
    }

    fn mint_esdt(&self, token_identifier: &TokenIdentifier, amount: &BigUint) -> EsdtTokenPayment {
        self.send().esdt_local_mint(token_identifier, 0u64, amount);

        EsdtTokenPayment::new(token_identifier.clone(), 0u64, amount.clone())
    }

    fn mint_nft_sft(
        &self,
        token_id: &TokenIdentifier,
        payload: MintNftSftPayload<Self::Api>,
    ) -> u64 {
        self.send().esdt_nft_create(
            token_id,
            &payload.amount,
            &payload.name,
            &payload.royalties,
            &payload.hash,
            &payload.attributes,
            &payload.uris,
        )
    }
}
