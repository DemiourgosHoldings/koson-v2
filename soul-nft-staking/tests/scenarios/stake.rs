use soul_nft_staking::constants::{
    errors::ERR_NOT_A_SOUL,
    score::{DEATH_SOUL_SCORE, ORIGIN_SOULS_SCORE, SUMMONED_SOUL_SCORE},
};

use crate::test_state::{
    KosonV2NftStakingContractState, DEATH_SOUL_TOKEN_ID, INVALID_NFT_TOKEN_ID,
    ORIGIN_SOULS_TOKEN_IDS, OWNER_ADDRESS_EXPR, SUMMONED_ORIGIN_SOULS_TOKEN_IDS,
    USER_1_ADDRESS_EXPR,
};

#[test]
fn simple_single_stake() {
    for token_id in ORIGIN_SOULS_TOKEN_IDS.iter() {
        let stake_transfer = vec![(*token_id, 1u64)];

        let mut state = KosonV2NftStakingContractState::new();
        state
            .deploy()
            .init()
            .stake_many(USER_1_ADDRESS_EXPR, stake_transfer, ORIGIN_SOULS_SCORE);
    }
}

#[test]
fn simple_multiple_stake() {
    let stake_transfer = vec![
        (ORIGIN_SOULS_TOKEN_IDS[0], 1),
        (ORIGIN_SOULS_TOKEN_IDS[1], 1),
        (ORIGIN_SOULS_TOKEN_IDS[2], 2),
        (ORIGIN_SOULS_TOKEN_IDS[3], 3),
        (ORIGIN_SOULS_TOKEN_IDS[4], 4),
        (ORIGIN_SOULS_TOKEN_IDS[5], 5),
    ];

    let mut state = KosonV2NftStakingContractState::new();
    state.deploy().init().stake_many(
        USER_1_ADDRESS_EXPR,
        stake_transfer.clone(),
        ORIGIN_SOULS_SCORE * stake_transfer.len() as u64,
    );
}

#[test]
fn stake_one_of_each() {
    let mut token_ids = [ORIGIN_SOULS_TOKEN_IDS, SUMMONED_ORIGIN_SOULS_TOKEN_IDS].concat();
    token_ids.push(DEATH_SOUL_TOKEN_ID);

    let stake_transfer = token_ids
        .iter()
        .map(|token_id| (*token_id, 1u64))
        .collect::<Vec<_>>();

    let expected_score = ORIGIN_SOULS_SCORE * ORIGIN_SOULS_TOKEN_IDS.len() as u64
        + SUMMONED_SOUL_SCORE * SUMMONED_ORIGIN_SOULS_TOKEN_IDS.len() as u64
        + DEATH_SOUL_SCORE;

    let mut state = KosonV2NftStakingContractState::new();
    state
        .deploy()
        .init()
        .stake_many(USER_1_ADDRESS_EXPR, stake_transfer.clone(), expected_score);
}

#[test]
fn stake_updates_user_score() {
    let stake_transfer = vec![(ORIGIN_SOULS_TOKEN_IDS[0], 1)];

    let mut state = KosonV2NftStakingContractState::new();
    state
        .deploy()
        .init()
        .stake_many(USER_1_ADDRESS_EXPR, stake_transfer, ORIGIN_SOULS_SCORE)
        .check_user_score(USER_1_ADDRESS_EXPR, ORIGIN_SOULS_SCORE);
}

#[test]
fn stake_updates_aggregated_score() {
    let stake_transfer = vec![(ORIGIN_SOULS_TOKEN_IDS[0], 1)];

    let mut state = KosonV2NftStakingContractState::new();
    state
        .deploy()
        .init()
        .stake_many(USER_1_ADDRESS_EXPR, stake_transfer, ORIGIN_SOULS_SCORE)
        .check_total_aggregated_score(ORIGIN_SOULS_SCORE);
}

#[test]
fn stake_multiple_times_increases_user_score() {
    let stake_transfer_1 = vec![(ORIGIN_SOULS_TOKEN_IDS[0], 1)];
    let stake_transfer_2 = vec![(ORIGIN_SOULS_TOKEN_IDS[0], 2)];

    let mut state = KosonV2NftStakingContractState::new();
    state
        .deploy()
        .init()
        .stake_many(USER_1_ADDRESS_EXPR, stake_transfer_1, ORIGIN_SOULS_SCORE)
        .stake_many(USER_1_ADDRESS_EXPR, stake_transfer_2, ORIGIN_SOULS_SCORE)
        .check_user_score(USER_1_ADDRESS_EXPR, 2 * ORIGIN_SOULS_SCORE);
}

#[test]
fn stake_multiple_times_increases_aggregated_score() {
    let stake_transfer_1 = vec![(ORIGIN_SOULS_TOKEN_IDS[0], 1)];
    let stake_transfer_2 = vec![(ORIGIN_SOULS_TOKEN_IDS[0], 2)];

    let mut state = KosonV2NftStakingContractState::new();
    state
        .deploy()
        .init()
        .stake_many(USER_1_ADDRESS_EXPR, stake_transfer_1, ORIGIN_SOULS_SCORE)
        .stake_many(USER_1_ADDRESS_EXPR, stake_transfer_2, ORIGIN_SOULS_SCORE)
        .check_total_aggregated_score(2 * ORIGIN_SOULS_SCORE);
}

#[test]
fn different_users_stake_increase_aggregated_score() {
    let user_stake_transfer = vec![(ORIGIN_SOULS_TOKEN_IDS[0], 1)];
    let owner_stake_transfer = vec![(ORIGIN_SOULS_TOKEN_IDS[0], 51)];

    let mut state = KosonV2NftStakingContractState::new();
    state
        .deploy()
        .init()
        .stake_many(USER_1_ADDRESS_EXPR, user_stake_transfer, ORIGIN_SOULS_SCORE)
        .stake_many(OWNER_ADDRESS_EXPR, owner_stake_transfer, ORIGIN_SOULS_SCORE)
        .check_total_aggregated_score(2 * ORIGIN_SOULS_SCORE);
}

#[test]
fn stake_updates_stake_epoch() {
    let stake_transfer_1 = vec![(ORIGIN_SOULS_TOKEN_IDS[0], 1)];
    let stake_transfer_2 = vec![(ORIGIN_SOULS_TOKEN_IDS[0], 2)];

    let mut state = KosonV2NftStakingContractState::new();
    state
        .deploy()
        .init()
        .stake_many(USER_1_ADDRESS_EXPR, stake_transfer_1, ORIGIN_SOULS_SCORE)
        .check_stake_epoch(ORIGIN_SOULS_TOKEN_IDS[0], 1, 0)
        .set_block_epoch(1)
        .check_stake_epoch(ORIGIN_SOULS_TOKEN_IDS[0], 1, 0)
        .stake_many(USER_1_ADDRESS_EXPR, stake_transfer_2, ORIGIN_SOULS_SCORE)
        .check_stake_epoch(ORIGIN_SOULS_TOKEN_IDS[0], 2, 1);
}

#[test]
fn stake_invalid_token_fails() {
    let stake_transfer = vec![(INVALID_NFT_TOKEN_ID, 1)];

    let mut state = KosonV2NftStakingContractState::new();
    state.deploy().init().stake_many_expect_err(
        USER_1_ADDRESS_EXPR,
        stake_transfer,
        ERR_NOT_A_SOUL,
    );
}

#[test]
fn staking_on_behalf_of_an_user() {
    let stake_transfer_1 = vec![(ORIGIN_SOULS_TOKEN_IDS[0], 1)]; // staked for sender's address
    let stake_transfer_2 = vec![(ORIGIN_SOULS_TOKEN_IDS[0], 2)]; // staked for owner's address

    KosonV2NftStakingContractState::new()
        .deploy()
        .init()
        .stake_many(USER_1_ADDRESS_EXPR, stake_transfer_1, ORIGIN_SOULS_SCORE)
        .check_stake_epoch(ORIGIN_SOULS_TOKEN_IDS[0], 1, 0)
        .stake_many_for_user(
            USER_1_ADDRESS_EXPR,
            OWNER_ADDRESS_EXPR,
            stake_transfer_2,
            ORIGIN_SOULS_SCORE,
        )
        .check_user_score(USER_1_ADDRESS_EXPR, ORIGIN_SOULS_SCORE)
        .check_user_score(OWNER_ADDRESS_EXPR, ORIGIN_SOULS_SCORE);
}
