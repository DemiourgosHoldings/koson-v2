use land_plot_staking::{
    constants::{errors::ERR_NOTHING_TO_CLAIM, score::LAND_PLOT_SCORES},
    unstake_fee_calculator::{
        calculator::{ONE_TOKEN, ONE_USDC},
        umbrella_interactor::ORACLE_PRICE_DENOMINATION,
    },
};
use multiversx_sc::types::BigUint;
use multiversx_sc_scenario::{api::StaticApi, managed_biguint};

use crate::test_state::{
    KosonV2NftStakingContractState, EGLD_PRICE_FEED_NAME, INITIAL_ESDT_BALANCE, KOSON_TOKEN_ID,
    NFT_STAKING_TOKEN_ID, OURO_TOKEN_ID, OWNER_ADDRESS_EXPR, USDC_TOKEN_ID, USDD_TOKEN_ID,
    USER_1_ADDRESS_EXPR, WEGLD_TOKEN_ID,
};

#[test]
fn test_dex_setup() {
    let ouro_koson_dex_rate = managed_biguint!(ONE_TOKEN);
    let ouro_usdd_dex_rate = managed_biguint!(2 * ONE_TOKEN);
    let ouro_usdc_dex_rate = managed_biguint!(ONE_USDC);
    let ouro_wegld_dex_rate = managed_biguint!(3 * ONE_TOKEN);

    let mut state = KosonV2NftStakingContractState::new();
    apply_scenario_setup(
        &mut state,
        ouro_koson_dex_rate.clone(),
        ouro_usdd_dex_rate.clone(),
        ouro_usdc_dex_rate.clone(),
        ouro_wegld_dex_rate.clone(),
        4 * ORACLE_PRICE_DENOMINATION,
    )
    .check_get_equivalent_vesta_dex(
        OURO_TOKEN_ID,
        KOSON_TOKEN_ID,
        managed_biguint!(1),
        ouro_koson_dex_rate,
    )
    .check_get_equivalent_vesta_dex(
        OURO_TOKEN_ID,
        USDD_TOKEN_ID,
        managed_biguint!(1),
        ouro_usdd_dex_rate,
    )
    .check_get_equivalent_vesta_dex(
        OURO_TOKEN_ID,
        USDC_TOKEN_ID,
        managed_biguint!(1),
        ouro_usdc_dex_rate,
    )
    .check_get_equivalent_xexchange(
        OURO_TOKEN_ID,
        WEGLD_TOKEN_ID,
        managed_biguint!(1),
        ouro_wegld_dex_rate,
    );
}

fn apply_scenario_setup(
    state: &mut KosonV2NftStakingContractState,
    ouro_koson_dex_rate: BigUint<StaticApi>,
    ouro_usdd_dex_rate: BigUint<StaticApi>,
    ouro_usdc_dex_rate: BigUint<StaticApi>,
    ouro_wegld_dex_rate: BigUint<StaticApi>,
    wegld_oracle_rate: u64,
) -> &mut KosonV2NftStakingContractState {
    state
        .deploy()
        .init()
        .set_exchange_rate(KOSON_TOKEN_ID, ouro_koson_dex_rate)
        .set_exchange_rate(USDD_TOKEN_ID, ouro_usdd_dex_rate)
        .set_exchange_rate(USDC_TOKEN_ID, ouro_usdc_dex_rate)
        .set_exchange_rate(WEGLD_TOKEN_ID, ouro_wegld_dex_rate)
        .set_oracle_feed_price(EGLD_PRICE_FEED_NAME, wegld_oracle_rate)
}
