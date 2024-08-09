#![no_std]

use multiversx_sc::derive_imports::*;
#[allow(unused_imports)]
use multiversx_sc::imports::*;

#[multiversx_sc::contract]
pub trait SimpleAssetMinter {
    #[init]
    fn init(&self) {}

    #[upgrade]
    fn upgrade(&self) {}

    // Minting logic
    #[only_owner]
    #[endpoint(mintNftSft)]
    fn mint_nft_sft_endpoint(
        &self,
        token_id: TokenIdentifier,
        payloads: MultiValueManagedVec<MintNftSftPayload<Self::Api>>,
    ) {
        require!(
            self.token_identifiers().contains(&token_id),
            "Token not issued"
        );

        let mut minted_assets = ManagedVec::new();

        for payload in payloads.iter() {
            let minted_nonce = self.mint_nft_sft(&token_id, &payload);
            minted_assets.push(EsdtTokenPayment::new(
                token_id.clone(),
                minted_nonce,
                payload.amount,
            ));
        }

        self.send()
            .direct_multi(&self.blockchain().get_caller(), &minted_assets);
    }

    #[only_owner]
    #[endpoint(withdraw)]
    fn withdraw(&self, token: TokenIdentifier, nonces: MultiValueManagedVec<u64>) {
        let mut payments = ManagedVec::new();
        let token_id = EgldOrEsdtTokenIdentifier::esdt(token.clone());
        for nonce in nonces.iter() {
            let balance = self.blockchain().get_sc_balance(&token_id, nonce);
            payments.push(EsdtTokenPayment::new(token.clone(), nonce, balance));
        }

        self.send()
            .direct_multi(&self.blockchain().get_caller(), &payments);
    }

    fn mint_nft_sft(
        &self,
        token_id: &TokenIdentifier,
        payload: &MintNftSftPayload<Self::Api>,
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

    // Collection issuance logic
    #[payable("*")]
    #[only_owner]
    #[endpoint(issueSemiFungible)]
    fn issue_semi_fungible(&self, token_name: ManagedBuffer, token_ticker: ManagedBuffer) {
        let issue_cost = self.call_value().egld_value();
        self.send()
            .esdt_system_sc_proxy()
            .issue_and_set_all_roles(
                issue_cost.clone_value(),
                token_name,
                token_ticker,
                EsdtTokenType::SemiFungible,
                0,
            )
            .with_callback(self.callbacks().issue_token_callback())
            .async_call_and_exit();
    }

    #[callback]
    fn issue_token_callback(
        &self,
        #[call_result] result: ManagedAsyncCallResult<EgldOrEsdtTokenIdentifier>,
    ) {
        match result {
            ManagedAsyncCallResult::Ok(token_id) => {
                let token_id = token_id.unwrap_esdt();
                self.token_identifiers().insert(token_id);
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

    #[view(getIssuedTokenIdentifiers)]
    #[storage_mapper("token_identifiers")]
    fn token_identifiers(&self) -> SetMapper<TokenIdentifier>;
}

#[derive(TopEncode, TopDecode, NestedEncode, NestedDecode, TypeAbi, ManagedVecItem)]
pub struct MintNftSftPayload<A: ManagedTypeApi> {
    pub amount: BigUint<A>,
    pub name: ManagedBuffer<A>,
    pub royalties: BigUint<A>,
    pub hash: ManagedBuffer<A>,
    pub attributes: ManagedBuffer<A>,
    pub uris: ManagedVec<A, ManagedBuffer<A>>,
}
