multiversx_sc::imports!();

#[multiversx_sc::module]
pub trait StorageModule {
    #[view(getOuroTokenId)]
    #[storage_mapper("ouro_token_id")]
    fn ouro_token_id(&self) -> SingleValueMapper<TokenIdentifier>;

    #[view(getUsddTokenId)]
    #[storage_mapper("usdd_token_id")]
    fn usdd_token_id(&self) -> SingleValueMapper<TokenIdentifier>;

    #[view(getUsdcTokenId)]
    #[storage_mapper("usdc_token_id")]
    fn usdc_token_id(&self) -> SingleValueMapper<TokenIdentifier>;

    #[view(getWegldTokenId)]
    #[storage_mapper("wegld_token_id")]
    fn wegld_token_id(&self) -> SingleValueMapper<TokenIdentifier>;

    #[view(getKosonTokenId)]
    #[storage_mapper("koson_token_id")]
    fn koson_token_id(&self) -> SingleValueMapper<TokenIdentifier>;

    #[view(getOriginSoulsNftTokenId)]
    #[storage_mapper("origin_souls_nft_token_id")]
    fn origin_souls_nft_token_id(&self) -> SetMapper<TokenIdentifier>;

    #[view(getDeathSoulsNftTokenId)]
    #[storage_mapper("death_souls_nft_token_id")]
    fn death_souls_nft_token_id(&self) -> SingleValueMapper<TokenIdentifier>;

    #[view(getSummonedSoulsNftTokenId)]
    #[storage_mapper("summoned_souls_nft_token_id")]
    fn summoned_souls_nft_token_id(&self) -> SetMapper<TokenIdentifier>;

    #[view(getTokenIdScore)]
    #[storage_mapper("token_id_score")]
    fn token_id_score(&self, token_id: &TokenIdentifier) -> SingleValueMapper<BigUint>;

    #[view(getRewardTokenId)]
    #[storage_mapper("reward_token_id")]
    fn reward_token_id(&self) -> SingleValueMapper<TokenIdentifier>;

    #[view(getAggregatedSoulStakingScores)]
    #[storage_mapper("aggregated_soul_staking_scores")]
    fn aggregated_soul_staking_scores(&self) -> SingleValueMapper<BigUint>;

    #[view(getUserAggregatedSoulStakingScores)]
    #[storage_mapper("user_aggregated_soul_staking_scores")]
    fn user_aggregated_soul_staking_scores(
        &self,
        user: &ManagedAddress,
    ) -> SingleValueMapper<BigUint>;

    #[view(getUserUnclaimedRewards)]
    #[storage_mapper("user_unclaimed_rewards")]
    fn user_unclaimed_rewards(&self, user: &ManagedAddress) -> SingleValueMapper<BigUint>;

    #[view(getStakedSouls)]
    #[storage_mapper("staked_souls")]
    fn staked_souls(&self, user: &ManagedAddress) -> SetMapper<EsdtTokenPayment>;
}
