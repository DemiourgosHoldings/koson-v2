multiversx_sc::imports!();

#[multiversx_sc::module]
pub trait StorageModule {
    #[view(getKosonTokenId)]
    #[storage_mapper("koson_token_ids")]
    fn koson_token_ids(&self) -> SetMapper<TokenIdentifier>;

    #[view(getStakedKosonTokenId)]
    #[storage_mapper("staked_koson_token_id")]
    fn staked_koson_token_id(&self) -> SingleValueMapper<TokenIdentifier>;

    #[view(getUnbondingKosonTokenId)]
    #[storage_mapper("unbonding_koson_token_id")]
    fn unbonding_koson_token_id(&self) -> SingleValueMapper<TokenIdentifier>;

    #[view(getKosonSupply)]
    #[storage_mapper("koson_supply")]
    fn koson_supply(&self, token_id: &TokenIdentifier) -> SingleValueMapper<BigUint>;

    #[view(getStakedKosonSupply)]
    #[storage_mapper("staked_koson_supply")]
    fn staked_koson_supply(&self, token_id: &TokenIdentifier) -> SingleValueMapper<BigUint>;
}
