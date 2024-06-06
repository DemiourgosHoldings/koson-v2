use multiversx_sc::types::BigUint;
use multiversx_sc_scenario::managed_biguint;

use crate::test_state::KosonFactoryState;

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
