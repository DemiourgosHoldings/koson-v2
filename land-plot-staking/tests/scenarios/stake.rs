use land_plot_staking::constants::score::LAND_PLOT_SCORES;

use crate::test_state::{
    KosonV2NftStakingContractState, NFT_STAKING_TOKEN_ID, OWNER_ADDRESS_EXPR, USER_1_ADDRESS_EXPR,
};

#[test]
fn simple_single_stake() {
    for nonce in 1..=LAND_PLOT_SCORES.len() {
        let stake_transfer = vec![(NFT_STAKING_TOKEN_ID, nonce as u64, 1)];

        let mut state = KosonV2NftStakingContractState::new();
        state.deploy().init().stake_many(
            USER_1_ADDRESS_EXPR,
            stake_transfer,
            LAND_PLOT_SCORES[nonce - 1],
        );
    }
}

#[test]
fn stake_one_of_each() {
    let set_score = LAND_PLOT_SCORES.iter().sum::<u64>();
    let stake_transfer = vec![
        (NFT_STAKING_TOKEN_ID, 1, 1),
        (NFT_STAKING_TOKEN_ID, 2, 1),
        (NFT_STAKING_TOKEN_ID, 3, 1),
        (NFT_STAKING_TOKEN_ID, 4, 1),
        (NFT_STAKING_TOKEN_ID, 5, 1),
    ];

    let mut state = KosonV2NftStakingContractState::new();
    state
        .deploy()
        .init()
        .stake_many(USER_1_ADDRESS_EXPR, stake_transfer, set_score);
}

#[test]
fn stake_updates_user_score() {
    let stake_transfer = vec![(NFT_STAKING_TOKEN_ID, 1, 1)];

    let mut state = KosonV2NftStakingContractState::new();
    state
        .deploy()
        .init()
        .stake_many(USER_1_ADDRESS_EXPR, stake_transfer, LAND_PLOT_SCORES[0])
        .check_user_score(USER_1_ADDRESS_EXPR, LAND_PLOT_SCORES[0]);
}

#[test]
fn stake_updates_aggregated_score() {
    let stake_transfer = vec![(NFT_STAKING_TOKEN_ID, 1, 1)];

    let mut state = KosonV2NftStakingContractState::new();
    state
        .deploy()
        .init()
        .stake_many(USER_1_ADDRESS_EXPR, stake_transfer, LAND_PLOT_SCORES[0])
        .check_total_aggregated_score(LAND_PLOT_SCORES[0]);
}

#[test]
fn stake_multiple_times_increases_user_score() {
    let stake_transfer = vec![(NFT_STAKING_TOKEN_ID, 1, 1)];

    let mut state = KosonV2NftStakingContractState::new();
    state
        .deploy()
        .init()
        .stake_many(
            USER_1_ADDRESS_EXPR,
            stake_transfer.clone(),
            LAND_PLOT_SCORES[0],
        )
        .stake_many(USER_1_ADDRESS_EXPR, stake_transfer, LAND_PLOT_SCORES[0])
        .check_user_score(USER_1_ADDRESS_EXPR, 2 * LAND_PLOT_SCORES[0]);
}

#[test]
fn stake_multiple_times_increases_aggregated_score() {
    let stake_transfer = vec![(NFT_STAKING_TOKEN_ID, 1, 1)];

    let mut state = KosonV2NftStakingContractState::new();
    state
        .deploy()
        .init()
        .stake_many(
            USER_1_ADDRESS_EXPR,
            stake_transfer.clone(),
            LAND_PLOT_SCORES[0],
        )
        .stake_many(USER_1_ADDRESS_EXPR, stake_transfer, LAND_PLOT_SCORES[0])
        .check_total_aggregated_score(2 * LAND_PLOT_SCORES[0]);
}

#[test]
fn different_users_stake_increase_aggregated_score() {
    let stake_transfer = vec![(NFT_STAKING_TOKEN_ID, 1, 1)];

    let mut state = KosonV2NftStakingContractState::new();
    state
        .deploy()
        .init()
        .stake_many(
            USER_1_ADDRESS_EXPR,
            stake_transfer.clone(),
            LAND_PLOT_SCORES[0],
        )
        .stake_many(OWNER_ADDRESS_EXPR, stake_transfer, LAND_PLOT_SCORES[0])
        .check_total_aggregated_score(2 * LAND_PLOT_SCORES[0]);
}

#[test]
fn stake_updates_stake_epoch() {
    let stake_transfer = vec![(NFT_STAKING_TOKEN_ID, 1, 1)];

    let mut state = KosonV2NftStakingContractState::new();
    state
        .deploy()
        .init()
        .stake_many(
            USER_1_ADDRESS_EXPR,
            stake_transfer.clone(),
            LAND_PLOT_SCORES[0],
        )
        .check_stake_epoch(USER_1_ADDRESS_EXPR, 1, 0)
        .set_block_epoch(1)
        .check_stake_epoch(USER_1_ADDRESS_EXPR, 1, 0)
        .stake_many(USER_1_ADDRESS_EXPR, stake_transfer, LAND_PLOT_SCORES[0])
        .check_stake_epoch(USER_1_ADDRESS_EXPR, 1, 1);
}
