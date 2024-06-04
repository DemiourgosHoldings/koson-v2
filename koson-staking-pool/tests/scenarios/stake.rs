use koson_staking_pool::constants::{
    config::POOL_INDEX_DENOMINATOR, errors::ERR_PAYMENT_NOT_ALLOWED,
};
use multiversx_sc::types::EsdtTokenPayment;
use multiversx_sc_scenario::{managed_biguint, managed_token_id};

use crate::test_state::{
    KosonStakingPoolState, INVALID_ESDT_TOKEN_ID, KOSON_ANCIENT_TOKEN_ID, KOSON_ESOTERIC_TOKEN_ID,
    KOSON_PRIMORDIAL_TOKEN_ID, KOSON_REWARD_BEARING_TOKEN, USER_1_ADDRESS_EXPR,
};

#[test]
fn simple_single_stake() {
    for token_id in [
        KOSON_PRIMORDIAL_TOKEN_ID,
        KOSON_ANCIENT_TOKEN_ID,
        KOSON_ESOTERIC_TOKEN_ID,
    ] {
        let stake_transfer = vec![(token_id, 1u64)];

        let mut state = KosonStakingPoolState::new();
        state.deploy().init().stake_many(
            USER_1_ADDRESS_EXPR,
            stake_transfer,
            EsdtTokenPayment::new(
                managed_token_id!(KOSON_REWARD_BEARING_TOKEN),
                0u64,
                managed_biguint!(1),
            ),
        );
    }
}

#[test]
fn simple_multiple_stake() {
    let stake_transfer = vec![
        (KOSON_ANCIENT_TOKEN_ID, 1),
        (KOSON_ESOTERIC_TOKEN_ID, 2),
        (KOSON_PRIMORDIAL_TOKEN_ID, 3),
    ];

    let mut state = KosonStakingPoolState::new();
    state.deploy().init().stake_many(
        USER_1_ADDRESS_EXPR,
        stake_transfer.clone(),
        EsdtTokenPayment::new(
            managed_token_id!(KOSON_REWARD_BEARING_TOKEN),
            0u64,
            managed_biguint!(6),
        ),
    );
}

#[test]
fn stake_updates_staked_koson_storage() {
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
        .check_koson_supply(KOSON_ANCIENT_TOKEN_ID, 1)
        .check_koson_supply(KOSON_ESOTERIC_TOKEN_ID, 2)
        .check_koson_supply(KOSON_PRIMORDIAL_TOKEN_ID, 3)
        .check_staked_koson_supply(KOSON_REWARD_BEARING_TOKEN, 6);
}

#[test]
fn initial_stake_yields_pool_index_1() {
    let stake_transfer = vec![(KOSON_ANCIENT_TOKEN_ID, 1000)];

    let mut state = KosonStakingPoolState::new();
    state
        .deploy()
        .init()
        .stake_many_unchecked(USER_1_ADDRESS_EXPR, stake_transfer)
        .check_current_index(POOL_INDEX_DENOMINATOR);
}

#[test]
fn stake_invalid_token_fails() {
    let stake_transfer = vec![(INVALID_ESDT_TOKEN_ID, 1u64)];

    let mut state = KosonStakingPoolState::new();
    state.deploy().init().stake_many_expect_err(
        USER_1_ADDRESS_EXPR,
        stake_transfer,
        ERR_PAYMENT_NOT_ALLOWED,
    );
}
