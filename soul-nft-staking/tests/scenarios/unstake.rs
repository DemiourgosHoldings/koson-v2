use multiversx_sc::types::BigUint;
use multiversx_sc_scenario::{api::StaticApi, managed_biguint};
use soul_nft_staking::{
    constants::{
        errors::ERR_NOT_ENOUGH_STAKED,
        score::{DEATH_SOUL_SCORE, ORIGIN_SOULS_SCORE},
    },
    unstake_fee_calculator::{
        calculator::{ONE_TOKEN, ONE_USDC},
        umbrella_interactor::ORACLE_PRICE_DENOMINATION,
    },
};

use crate::test_state::{
    KosonV2NftStakingContractState, DEATH_SOUL_TOKEN_ID, EGLD_PRICE_FEED_NAME,
    INITIAL_ESDT_BALANCE, KOSON_TOKEN_ID, ORIGIN_SOULS_TOKEN_IDS, USDC_TOKEN_ID, USDD_TOKEN_ID,
    USER_1_ADDRESS_EXPR, WEGLD_TOKEN_ID,
};

const DEFAULT_OURO_KOSON_RATE: u64 = ONE_TOKEN / 10;
const DEFAULT_OURO_USDD_RATE: u64 = 2 * ONE_TOKEN;
const DEFAULT_OURO_USDC_RATE: u64 = 3 * ONE_USDC;
const DEFAULT_OURO_WEGLD_RATE: u64 = 4 * ONE_TOKEN;
const WEGLD_ORACLE_RATE: u64 = 5 * ORACLE_PRICE_DENOMINATION;

#[test]
fn simple_stake_unstake_with_expected_fee() {
    let stake_transfer = vec![(ORIGIN_SOULS_TOKEN_IDS[0], 1)];
    let unstake_request = vec![(ORIGIN_SOULS_TOKEN_IDS[0], 1)];
    let unstake_fee = (KOSON_TOKEN_ID, 2499999999999999999); // 8.(3) KOSON per point for full fee * 30 points = 24.(9)

    let mut state = KosonV2NftStakingContractState::new();
    apply_default_scenario_setup(&mut state)
        .stake_many(USER_1_ADDRESS_EXPR, stake_transfer, ORIGIN_SOULS_SCORE)
        .unstake_many(
            USER_1_ADDRESS_EXPR,
            unstake_request,
            unstake_fee,
            ORIGIN_SOULS_SCORE,
        );
}

#[test]
fn simple_stake_unstake_sends_asset_to_caller() {
    let stake_transfer = vec![(ORIGIN_SOULS_TOKEN_IDS[0], 1)];
    let unstake_request = vec![(ORIGIN_SOULS_TOKEN_IDS[0], 1)];
    let unstake_fee = (KOSON_TOKEN_ID, 2499999999999999999); // 8.(3) KOSON per point for full fee * 30 points = 24.(9)

    let mut state = KosonV2NftStakingContractState::new();
    apply_default_scenario_setup(&mut state)
        .stake_many(USER_1_ADDRESS_EXPR, stake_transfer, ORIGIN_SOULS_SCORE)
        .unstake_many(
            USER_1_ADDRESS_EXPR,
            unstake_request,
            unstake_fee,
            ORIGIN_SOULS_SCORE,
        )
        .check_user_score(USER_1_ADDRESS_EXPR, 0)
        .check_user_nft_balance(USER_1_ADDRESS_EXPR, ORIGIN_SOULS_TOKEN_IDS[0], 1, 1);
}

#[test]
fn simple_stake_unstake_updates_user_score() {
    let stake_transfer = vec![(ORIGIN_SOULS_TOKEN_IDS[0], 1)];
    let unstake_request = vec![(ORIGIN_SOULS_TOKEN_IDS[0], 1)];
    let unstake_fee = (KOSON_TOKEN_ID, 2499999999999999999); // 8.(3) KOSON per point for full fee * 30 points = 24.(9)

    let mut state = KosonV2NftStakingContractState::new();
    apply_default_scenario_setup(&mut state)
        .stake_many(USER_1_ADDRESS_EXPR, stake_transfer, ORIGIN_SOULS_SCORE)
        .unstake_many(
            USER_1_ADDRESS_EXPR,
            unstake_request,
            unstake_fee,
            ORIGIN_SOULS_SCORE,
        )
        .check_user_score(USER_1_ADDRESS_EXPR, 0);
}

#[test]
fn simple_stake_unstake_updates_aggregated_score() {
    let stake_transfer = vec![(ORIGIN_SOULS_TOKEN_IDS[0], 1)];
    let unstake_request = vec![(ORIGIN_SOULS_TOKEN_IDS[0], 1)];
    let unstake_fee = (KOSON_TOKEN_ID, 2499999999999999999); // 8.(3) KOSON per point for full fee * 30 points = 24.(9)

    let mut state = KosonV2NftStakingContractState::new();
    apply_default_scenario_setup(&mut state)
        .stake_many(USER_1_ADDRESS_EXPR, stake_transfer, ORIGIN_SOULS_SCORE)
        .unstake_many(
            USER_1_ADDRESS_EXPR,
            unstake_request,
            unstake_fee,
            ORIGIN_SOULS_SCORE,
        )
        .check_total_aggregated_score(0);
}

#[test]
fn simple_stake_unstake_after_10_days() {
    let stake_transfer = vec![(ORIGIN_SOULS_TOKEN_IDS[0], 1)];
    let unstake_request = vec![(ORIGIN_SOULS_TOKEN_IDS[0], 1)];
    let unstake_fee = (KOSON_TOKEN_ID, (2499999999999999999u128 * 15 / 25) as u64); // 8.(3) KOSON per point for full fee * 30 points = 24.(9)

    let mut state = KosonV2NftStakingContractState::new();
    apply_default_scenario_setup(&mut state)
        .stake_many(USER_1_ADDRESS_EXPR, stake_transfer, ORIGIN_SOULS_SCORE)
        .set_block_epoch(10)
        .unstake_many(
            USER_1_ADDRESS_EXPR,
            unstake_request,
            unstake_fee,
            ORIGIN_SOULS_SCORE,
        )
        .check_total_aggregated_score(0);
}

#[test]
fn simple_stake_unstake_after_25_days() {
    let stake_transfer = vec![(ORIGIN_SOULS_TOKEN_IDS[0], 1)];
    let unstake_request = vec![(ORIGIN_SOULS_TOKEN_IDS[0], 1)];
    let unstake_fee = (KOSON_TOKEN_ID, 0u64); // 8.(3) KOSON per point for full fee * 30 points = 24.(9)

    let mut state = KosonV2NftStakingContractState::new();
    apply_default_scenario_setup(&mut state)
        .stake_many(USER_1_ADDRESS_EXPR, stake_transfer, ORIGIN_SOULS_SCORE)
        .set_block_epoch(25)
        .unstake_many(
            USER_1_ADDRESS_EXPR,
            unstake_request,
            unstake_fee,
            ORIGIN_SOULS_SCORE,
        )
        .check_total_aggregated_score(0);
}

#[test]
fn sending_more_fee_token_returns_remainder() {
    let unstake_fee_amount = 2449999999999999999u128;
    let amount_to_pay = unstake_fee_amount * 2;
    let remaining_balance = INITIAL_ESDT_BALANCE - unstake_fee_amount;

    let stake_transfer = vec![(DEATH_SOUL_TOKEN_ID, 1)];
    let unstake_request = vec![(DEATH_SOUL_TOKEN_ID, 1)];

    let unstake_fee = (KOSON_TOKEN_ID, amount_to_pay as u64); // 8.(3) KOSON per point for full fee * 30 points = 24.(9)

    let mut state = KosonV2NftStakingContractState::new();
    apply_default_scenario_setup(&mut state)
        .stake_many(USER_1_ADDRESS_EXPR, stake_transfer, DEATH_SOUL_SCORE)
        .set_block_epoch(10)
        .unstake_many(
            USER_1_ADDRESS_EXPR,
            unstake_request,
            unstake_fee,
            DEATH_SOUL_SCORE,
        )
        .check_total_aggregated_score(0)
        .check_user_balance(USER_1_ADDRESS_EXPR, KOSON_TOKEN_ID, remaining_balance);
}

#[test]
fn cannot_unstake_from_others() {
    let unstake_request = vec![(ORIGIN_SOULS_TOKEN_IDS[0], 1)];
    let unstake_fee = (KOSON_TOKEN_ID, 2499999999999999999); // 8.(3) KOSON per point for full fee * 30 points = 24.(9)

    let mut state = KosonV2NftStakingContractState::new();
    apply_default_scenario_setup(&mut state).unstake_many_expect_err(
        USER_1_ADDRESS_EXPR,
        unstake_request,
        unstake_fee,
        ERR_NOT_ENOUGH_STAKED,
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
