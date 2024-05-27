// Code generated by the multiversx-sc build system. DO NOT EDIT.

////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

// Init:                                 1
// Endpoints:                           27
// Async Callback (empty):               1
// Total number of exported functions:  29

#![no_std]
#![allow(internal_features)]
#![feature(lang_items)]

multiversx_sc_wasm_adapter::allocator!();
multiversx_sc_wasm_adapter::panic_handler!();

multiversx_sc_wasm_adapter::endpoints! {
    land_plot_staking
    (
        init => init
        stake => stake_land_plots
        unstake => unstake_land_plots
        claimRewards => claim_rewards
        distributeRewards => distribute_rewards
        getOuroTokenId => ouro_token_id
        getUsddTokenId => usdd_token_id
        getUsdcTokenId => usdc_token_id
        getWegldTokenId => wegld_token_id
        getKosonTokenId => koson_token_id
        getLandPlotSftTokenId => land_plot_sft_token_id
        getRewardTokenId => reward_token_id
        getStakedLandPlots => staked_land_plots
        getAggregatedLandPlotScores => aggregated_land_plot_scores
        getUserAggregatedLandPlotScores => user_aggregated_land_plot_scores
        getUserUnclaimedRewards => user_unclaimed_rewards
        getUnclaimedRewardRate => get_unclaimed_reward_rate
        getRewardRate => current_reward_rate
        getLastClaimedRewardRate => last_claimed_reward_rate
        setPairInfo => set_pair_info
        removePairInfo => remove_pair_info
        getSwapPairAddress => swap_pair_address
        setUmbrellaRegistryAddress => set_oracle_registry_address
        setUmbrellaPriceFeed => set_price_feed
        getUmbrellaPrice => get_oracle_price
        getUmbrellaFeedsAddress => get_feeds_address
        getUmbrellaRegistryAddress => umbrella_oracle_registry_sc_address
        getTokenIdentifierFeedMapping => token_identifier_feed_mapping
    )
}

multiversx_sc_wasm_adapter::async_callback_empty! {}