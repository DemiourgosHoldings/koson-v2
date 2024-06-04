use koson_staking_pool::constants::config::POOL_INDEX_DENOMINATOR;
use multiversx_sc::types::EsdtTokenPayment;
use multiversx_sc_scenario::{managed_biguint, managed_token_id};

use crate::test_state::{
    KosonStakingPoolState, KOSON_ANCIENT_TOKEN_ID, KOSON_ESOTERIC_TOKEN_ID,
    KOSON_PRIMORDIAL_TOKEN_ID, KOSON_REWARD_BEARING_TOKEN, KOSON_UNBONDING_META_TOKEN,
    USER_1_ADDRESS_EXPR,
};

#[test]
fn simple_single_stake_unstake() {
    let stake_transfer = vec![(KOSON_ANCIENT_TOKEN_ID, 1000u64)];

    let mut state = KosonStakingPoolState::new();
    state
        .deploy()
        .init()
        .stake_many_unchecked(USER_1_ADDRESS_EXPR, stake_transfer.clone())
        .unstake(
            USER_1_ADDRESS_EXPR,
            stake_transfer[0].1,
            EsdtTokenPayment::new(
                managed_token_id!(KOSON_UNBONDING_META_TOKEN),
                1u64,
                managed_biguint!(stake_transfer[0].1),
            ),
        );
}

#[test]
fn unstake_updates_staked_koson_storage() {
    let stake_transfer = vec![
        (KOSON_ANCIENT_TOKEN_ID, 1),
        (KOSON_ESOTERIC_TOKEN_ID, 2),
        (KOSON_PRIMORDIAL_TOKEN_ID, 3),
    ];

    let mut state = KosonStakingPoolState::new();
    state
        .deploy()
        .init()
        .stake_many_unchecked(USER_1_ADDRESS_EXPR, stake_transfer)
        .unstake_unchecked(USER_1_ADDRESS_EXPR, 6)
        .check_koson_supply(KOSON_ANCIENT_TOKEN_ID, 1)
        .check_koson_supply(KOSON_ESOTERIC_TOKEN_ID, 2)
        .check_koson_supply(KOSON_PRIMORDIAL_TOKEN_ID, 3)
        .check_staked_koson_supply(KOSON_REWARD_BEARING_TOKEN, 0)
        .check_staked_koson_supply(KOSON_UNBONDING_META_TOKEN, 6);
}

#[test]
fn unstake_does_not_change_pool_index() {
    let stake_transfer = vec![
        (KOSON_ANCIENT_TOKEN_ID, 1),
        (KOSON_ESOTERIC_TOKEN_ID, 2),
        (KOSON_PRIMORDIAL_TOKEN_ID, 3),
    ];

    let mut state = KosonStakingPoolState::new();
    state
        .deploy()
        .init()
        .stake_many_unchecked(USER_1_ADDRESS_EXPR, stake_transfer)
        .unstake_unchecked(USER_1_ADDRESS_EXPR, 6)
        .check_current_index(POOL_INDEX_DENOMINATOR);
}
