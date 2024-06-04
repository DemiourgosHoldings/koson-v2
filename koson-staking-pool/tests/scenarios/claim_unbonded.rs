use multiversx_sc::types::{EsdtTokenPayment, ManagedVec};
use multiversx_sc_scenario::{managed_biguint, managed_token_id};

use crate::test_state::{
    KosonStakingPoolState, KOSON_ANCIENT_TOKEN_ID, KOSON_ESOTERIC_TOKEN_ID,
    KOSON_PRIMORDIAL_TOKEN_ID, KOSON_UNBONDING_META_TOKEN, UNBONDING_TIME_PENALTY,
    USER_1_ADDRESS_EXPR,
};

/// This test assumes a full fee of 90%.
#[test]
fn full_fee_paid_when_claiming_after_unstake() {
    let stake_transfer = vec![
        (KOSON_ANCIENT_TOKEN_ID, 30000),
        (KOSON_ESOTERIC_TOKEN_ID, 30000),
        (KOSON_PRIMORDIAL_TOKEN_ID, 30000),
    ];

    let claim_unstaked_payments_out = vec![(KOSON_UNBONDING_META_TOKEN, 1u64, 30000)];
    let mut claim_unstaked_payments_in = ManagedVec::new();
    claim_unstaked_payments_in.push(EsdtTokenPayment::new(
        managed_token_id!(KOSON_ESOTERIC_TOKEN_ID),
        0u64,
        managed_biguint!(1_000),
    ));
    claim_unstaked_payments_in.push(EsdtTokenPayment::new(
        managed_token_id!(KOSON_PRIMORDIAL_TOKEN_ID),
        0u64,
        managed_biguint!(1_000),
    ));
    claim_unstaked_payments_in.push(EsdtTokenPayment::new(
        managed_token_id!(KOSON_ANCIENT_TOKEN_ID),
        0u64,
        managed_biguint!(1_000),
    ));

    let mut state = KosonStakingPoolState::new();
    state
        .deploy()
        .init()
        .stake_many_unchecked(USER_1_ADDRESS_EXPR, stake_transfer)
        .unstake_unchecked(USER_1_ADDRESS_EXPR, 30000)
        .claim_unstaked(
            USER_1_ADDRESS_EXPR,
            claim_unstaked_payments_out,
            claim_unstaked_payments_in,
        );
}

#[test]
fn no_fee_paid_when_claiming_after_fee_decay_to_0() {
    let stake_transfer = vec![
        (KOSON_ANCIENT_TOKEN_ID, 30000),
        (KOSON_ESOTERIC_TOKEN_ID, 30000),
        (KOSON_PRIMORDIAL_TOKEN_ID, 30000),
    ];

    let claim_unstaked_payments_out = vec![(KOSON_UNBONDING_META_TOKEN, 1u64, 30000)];
    let mut claim_unstaked_payments_in = ManagedVec::new();
    claim_unstaked_payments_in.push(EsdtTokenPayment::new(
        managed_token_id!(KOSON_ESOTERIC_TOKEN_ID),
        0u64,
        managed_biguint!(10_000),
    ));
    claim_unstaked_payments_in.push(EsdtTokenPayment::new(
        managed_token_id!(KOSON_PRIMORDIAL_TOKEN_ID),
        0u64,
        managed_biguint!(10_000),
    ));
    claim_unstaked_payments_in.push(EsdtTokenPayment::new(
        managed_token_id!(KOSON_ANCIENT_TOKEN_ID),
        0u64,
        managed_biguint!(10_000),
    ));

    let mut state = KosonStakingPoolState::new();
    state
        .deploy()
        .init()
        .stake_many_unchecked(USER_1_ADDRESS_EXPR, stake_transfer)
        .unstake_unchecked(USER_1_ADDRESS_EXPR, 30000)
        .set_block_epoch(UNBONDING_TIME_PENALTY + 1)
        .claim_unstaked(
            USER_1_ADDRESS_EXPR,
            claim_unstaked_payments_out,
            claim_unstaked_payments_in,
        );
}

/// This test assumes a full fee of 90%, thus half fee is 45%.
#[test]
fn half_fee_paid_when_claiming_after_50_perc_fee_decay() {
    let stake_transfer = vec![
        (KOSON_ANCIENT_TOKEN_ID, 30000),
        (KOSON_ESOTERIC_TOKEN_ID, 30000),
        (KOSON_PRIMORDIAL_TOKEN_ID, 30000),
    ];

    let claim_unstaked_payments_out = vec![(KOSON_UNBONDING_META_TOKEN, 1u64, 30000)];
    let mut claim_unstaked_payments_in = ManagedVec::new();
    claim_unstaked_payments_in.push(EsdtTokenPayment::new(
        managed_token_id!(KOSON_ESOTERIC_TOKEN_ID),
        0u64,
        managed_biguint!(5_500),
    ));
    claim_unstaked_payments_in.push(EsdtTokenPayment::new(
        managed_token_id!(KOSON_PRIMORDIAL_TOKEN_ID),
        0u64,
        managed_biguint!(5_500),
    ));
    claim_unstaked_payments_in.push(EsdtTokenPayment::new(
        managed_token_id!(KOSON_ANCIENT_TOKEN_ID),
        0u64,
        managed_biguint!(5_500),
    ));

    let mut state = KosonStakingPoolState::new();
    state
        .deploy()
        .init()
        .stake_many_unchecked(USER_1_ADDRESS_EXPR, stake_transfer)
        .unstake_unchecked(USER_1_ADDRESS_EXPR, 30000)
        .set_block_epoch(UNBONDING_TIME_PENALTY / 2)
        .claim_unstaked(
            USER_1_ADDRESS_EXPR,
            claim_unstaked_payments_out,
            claim_unstaked_payments_in,
        );
}
