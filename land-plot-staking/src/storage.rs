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

    #[view(getLandPlotSftTokenId)]
    #[storage_mapper("land_plot_sft_token_id")]
    fn land_plot_sft_token_id(&self) -> SingleValueMapper<TokenIdentifier>;

    #[view(getRewardTokenId)]
    #[storage_mapper("reward_token_id")]
    fn reward_token_id(&self) -> SingleValueMapper<TokenIdentifier>;

    #[view(getStakedLandPlots)]
    #[storage_mapper("staked_land_plots")]
    fn staked_land_plots(&self, user: &ManagedAddress, nonce: u64) -> SingleValueMapper<BigUint>;

    #[view(getAggregatedLandPlotScores)]
    #[storage_mapper("aggregated_land_plot_scores")]
    fn aggregated_land_plot_scores(&self) -> SingleValueMapper<BigUint>;

    #[view(getUserAggregatedLandPlotScores)]
    #[storage_mapper("user_aggregated_land_plot_scores")]
    fn user_aggregated_land_plot_scores(&self, user: &ManagedAddress)
        -> SingleValueMapper<BigUint>;

    #[view(getUserUnclaimedRewards)]
    #[storage_mapper("user_unclaimed_rewards")]
    fn user_unclaimed_rewards(&self, user: &ManagedAddress) -> SingleValueMapper<BigUint>;

    #[view(getStakeEpochStorage)]
    #[storage_mapper("stake_epoch")]
    fn stake_epoch(&self, user: &ManagedAddress, nonce: u64) -> SingleValueMapper<u64>;
}
