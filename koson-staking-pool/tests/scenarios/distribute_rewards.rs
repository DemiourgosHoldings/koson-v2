use koson_staking_pool::constants::config::POOL_INDEX_DENOMINATOR;

use crate::test_state::{
    KosonStakingPoolState, KOSON_ANCIENT_TOKEN_ID, KOSON_ESOTERIC_TOKEN_ID,
    KOSON_PRIMORDIAL_TOKEN_ID, USER_1_ADDRESS_EXPR,
};

#[test]
fn reward_distribution_increases_pool_index() {
    let stake_transfer = vec![
        (KOSON_ANCIENT_TOKEN_ID, 1),
        (KOSON_ESOTERIC_TOKEN_ID, 2),
        (KOSON_PRIMORDIAL_TOKEN_ID, 3),
    ];

    let mut state = KosonStakingPoolState::new();
    state
        .deploy()
        .init()
        .stake_many_unchecked(USER_1_ADDRESS_EXPR, stake_transfer.clone())
        .distribute_many_rewards(USER_1_ADDRESS_EXPR, stake_transfer)
        .check_current_index(2 * POOL_INDEX_DENOMINATOR);
}

#[test]
fn reward_distribution_updates_koson_supplies() {
    let stake_transfer = vec![
        (KOSON_ANCIENT_TOKEN_ID, 1),
        (KOSON_ESOTERIC_TOKEN_ID, 2),
        (KOSON_PRIMORDIAL_TOKEN_ID, 3),
    ];

    let mut state = KosonStakingPoolState::new();
    state
        .deploy()
        .init()
        .stake_many_unchecked(USER_1_ADDRESS_EXPR, stake_transfer.clone())
        .distribute_many_rewards(USER_1_ADDRESS_EXPR, stake_transfer)
        .check_koson_supply(KOSON_ANCIENT_TOKEN_ID, 2)
        .check_koson_supply(KOSON_ESOTERIC_TOKEN_ID, 4)
        .check_koson_supply(KOSON_PRIMORDIAL_TOKEN_ID, 6);
}

#[test]
fn claim_after_reward_distribution_yields_tokens_at_new_index() {
    let stake_transfer = vec![
        (KOSON_ANCIENT_TOKEN_ID, 1),
        (KOSON_ESOTERIC_TOKEN_ID, 2),
        (KOSON_PRIMORDIAL_TOKEN_ID, 3),
    ];

    let mut state = KosonStakingPoolState::new();
    state
        .deploy()
        .init()
        .stake_many_unchecked(USER_1_ADDRESS_EXPR, stake_transfer.clone())
        .distribute_many_rewards(USER_1_ADDRESS_EXPR, stake_transfer.clone());
}
