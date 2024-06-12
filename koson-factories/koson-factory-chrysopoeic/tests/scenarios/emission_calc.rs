use koson_factory_chrysopoeic::constants::errors::ERR_ALREADY_DISTRIBUTED;
use multiversx_sc::types::BigUint;
use multiversx_sc_scenario::managed_biguint;

use crate::test_state::{
    helpers::get_direct_send_distribution_list, KosonFactoryState, OWNER_ADDRESS_EXPR,
};

/// The tests in this file are based on the initial issuance logic.
/// They might become obsolete if the MAX_SUPPLY is changed or the daily emission logic is updated.
#[test]
fn nothing_to_issue_on_deploy_epoch() {
    let mut state = KosonFactoryState::new();
    state
        .deploy()
        .init()
        .check_undistributed_emission(managed_biguint!(0));
}

#[test]
fn initial_issuance_amount() {
    let initial_expected_issuance = BigUint::from(2311_477126785564068863u128);

    let mut state = KosonFactoryState::new();
    state
        .deploy()
        .init()
        .set_block_epoch(1)
        .check_undistributed_emission(initial_expected_issuance);
}

#[test]
fn ten_epochs_of_undistributed_amount() {
    let expected_issuance = BigUint::from(23099_917431385953626673u128);

    let mut state = KosonFactoryState::new();
    state
        .deploy()
        .init()
        .set_block_epoch(10)
        .check_undistributed_emission(expected_issuance);
}

#[test]
fn double_distribution_fails() {
    let distribution_list = get_direct_send_distribution_list();
    KosonFactoryState::new()
        .deploy()
        .init()
        .set_distribution_list(
            OWNER_ADDRESS_EXPR,
            distribution_list.0,
            distribution_list.1,
            distribution_list.2,
        )
        .set_block_epoch(1)
        .distribute_reward(OWNER_ADDRESS_EXPR)
        .distribute_reward_and_expect_err(OWNER_ADDRESS_EXPR, ERR_ALREADY_DISTRIBUTED);
}
