use soul_nft_staking::constants::{errors::ERR_NOT_A_REWARD, score::ORIGIN_SOULS_SCORE};

use crate::test_state::{
    KosonV2NftStakingContractState, INVALID_ESDT_TOKEN_ID, KOSON_TOKEN_ID, ORIGIN_SOULS_TOKEN_IDS,
    OWNER_ADDRESS_EXPR, USER_1_ADDRESS_EXPR,
};

#[test]
fn simple_distribute_rewards() {
    let stake_transfer_1 = vec![(ORIGIN_SOULS_TOKEN_IDS[0], 1)];
    let stake_transfer_2 = vec![(ORIGIN_SOULS_TOKEN_IDS[0], 51)];
    let total_stake_score = ORIGIN_SOULS_SCORE * 2;

    let mut state = KosonV2NftStakingContractState::new();
    state
        .deploy()
        .init()
        .stake_many(USER_1_ADDRESS_EXPR, stake_transfer_1, ORIGIN_SOULS_SCORE)
        .stake_many(OWNER_ADDRESS_EXPR, stake_transfer_2, ORIGIN_SOULS_SCORE)
        .distribute_rewards(OWNER_ADDRESS_EXPR, KOSON_TOKEN_ID, total_stake_score)
        .check_current_reward_rate(1);
}

#[test]
fn continuous_distribute_rewards() {
    let stake_transfer_1 = vec![(ORIGIN_SOULS_TOKEN_IDS[0], 1)];
    let stake_transfer_2 = vec![(ORIGIN_SOULS_TOKEN_IDS[0], 51)];
    let total_stake_score = ORIGIN_SOULS_SCORE * 2;

    let mut state = KosonV2NftStakingContractState::new();
    state
        .deploy()
        .init()
        .stake_many(USER_1_ADDRESS_EXPR, stake_transfer_1, ORIGIN_SOULS_SCORE)
        .stake_many(OWNER_ADDRESS_EXPR, stake_transfer_2, ORIGIN_SOULS_SCORE)
        .distribute_rewards(OWNER_ADDRESS_EXPR, KOSON_TOKEN_ID, total_stake_score)
        .distribute_rewards(OWNER_ADDRESS_EXPR, KOSON_TOKEN_ID, total_stake_score)
        .distribute_rewards(OWNER_ADDRESS_EXPR, KOSON_TOKEN_ID, total_stake_score)
        .check_current_reward_rate(3);
}

#[test]
fn distribute_invalid_reward_token() {
    let stake_transfer_1 = vec![(ORIGIN_SOULS_TOKEN_IDS[0], 1)];
    let stake_transfer_2 = vec![(ORIGIN_SOULS_TOKEN_IDS[0], 51)];

    let mut state = KosonV2NftStakingContractState::new();
    state
        .deploy()
        .init()
        .stake_many(USER_1_ADDRESS_EXPR, stake_transfer_1, ORIGIN_SOULS_SCORE)
        .stake_many(OWNER_ADDRESS_EXPR, stake_transfer_2, ORIGIN_SOULS_SCORE)
        .distribute_rewards_expect_err(
            OWNER_ADDRESS_EXPR,
            INVALID_ESDT_TOKEN_ID,
            1000,
            ERR_NOT_A_REWARD,
        );
}

#[test]
fn users_can_distribute_rewards() {
    let stake_transfer_1 = vec![(ORIGIN_SOULS_TOKEN_IDS[0], 1)];
    let stake_transfer_2 = vec![(ORIGIN_SOULS_TOKEN_IDS[0], 51)];
    let total_stake_score = ORIGIN_SOULS_SCORE * 2;

    let mut state = KosonV2NftStakingContractState::new();
    state
        .deploy()
        .init()
        .stake_many(USER_1_ADDRESS_EXPR, stake_transfer_1, ORIGIN_SOULS_SCORE)
        .stake_many(OWNER_ADDRESS_EXPR, stake_transfer_2, ORIGIN_SOULS_SCORE)
        .distribute_rewards(USER_1_ADDRESS_EXPR, KOSON_TOKEN_ID, total_stake_score)
        .check_current_reward_rate(1);
}
