use land_plot_staking::unstake_fee_calculator::{
    calculator::{ONE_TOKEN, ONE_USDC},
    umbrella_interactor::ORACLE_PRICE_DENOMINATION,
};
use multiversx_sc::types::BigUint;
use multiversx_sc_scenario::{api::StaticApi, managed_biguint};

use crate::test_state::{
    KosonV2NftStakingContractState, EGLD_PRICE_FEED_NAME, KOSON_TOKEN_ID, OURO_TOKEN_ID,
    USDC_TOKEN_ID, USDD_TOKEN_ID, WEGLD_TOKEN_ID,
};

const DEFAULT_OURO_KOSON_RATE: u64 = ONE_TOKEN / 10;
const DEFAULT_OURO_USDD_RATE: u64 = 2 * ONE_TOKEN;
const DEFAULT_OURO_USDC_RATE: u64 = 3 * ONE_USDC;
const DEFAULT_OURO_WEGLD_RATE: u64 = 4 * ONE_TOKEN;
const WEGLD_ORACLE_RATE: u64 = 5 * ORACLE_PRICE_DENOMINATION;

#[test]
fn test_dex_setup() {
    let ouro_koson_dex_rate = managed_biguint!(DEFAULT_OURO_KOSON_RATE);
    let ouro_usdd_dex_rate = managed_biguint!(DEFAULT_OURO_USDD_RATE);
    let ouro_usdc_dex_rate = managed_biguint!(DEFAULT_OURO_USDC_RATE);
    let ouro_wegld_dex_rate = managed_biguint!(DEFAULT_OURO_WEGLD_RATE);

    let mut state = KosonV2NftStakingContractState::new();
    apply_default_scenario_setup(&mut state)
        .check_get_equivalent_vesta_dex(
            OURO_TOKEN_ID,
            KOSON_TOKEN_ID,
            managed_biguint!(ONE_TOKEN),
            ouro_koson_dex_rate,
        )
        .check_get_equivalent_vesta_dex(
            OURO_TOKEN_ID,
            USDD_TOKEN_ID,
            managed_biguint!(ONE_TOKEN),
            ouro_usdd_dex_rate,
        )
        .check_get_equivalent_vesta_dex(
            OURO_TOKEN_ID,
            USDC_TOKEN_ID,
            managed_biguint!(ONE_TOKEN),
            ouro_usdc_dex_rate,
        )
        .check_get_equivalent_xexchange(
            OURO_TOKEN_ID,
            WEGLD_TOKEN_ID,
            managed_biguint!(ONE_TOKEN),
            ouro_wegld_dex_rate,
        );
}

#[test]
fn one_point_fee_in_koson() {
    let mut state = KosonV2NftStakingContractState::new();
    apply_default_scenario_setup(&mut state)
        .check_unstake_fee_per_score_in_koson(83333333333333333); // 0.08333333333333333
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

fn apply_default_scenario_setup(
    state: &mut KosonV2NftStakingContractState,
) -> &mut KosonV2NftStakingContractState {
    apply_scenario_setup(
        state,
        managed_biguint!(DEFAULT_OURO_KOSON_RATE),
        managed_biguint!(DEFAULT_OURO_USDD_RATE),
        managed_biguint!(DEFAULT_OURO_USDC_RATE),
        managed_biguint!(DEFAULT_OURO_WEGLD_RATE),
        WEGLD_ORACLE_RATE,
    )
}
