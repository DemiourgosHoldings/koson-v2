use land_plot_staking::constants::{errors::ERR_NOT_A_REWARD, score::LAND_PLOT_SCORES};

use crate::test_state::{
    KosonV2NftStakingContractState, INVALID_ESDT_TOKEN_ID, KOSON_TOKEN_ID, NFT_STAKING_TOKEN_ID,
    OWNER_ADDRESS_EXPR, USER_1_ADDRESS_EXPR,
};

#[test]
fn simple_distribute_rewards() {
    let stake_transfer = vec![(NFT_STAKING_TOKEN_ID, 1, 1)];
    let total_stake_score = LAND_PLOT_SCORES[0] * 2;

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
        .distribute_rewards(OWNER_ADDRESS_EXPR, KOSON_TOKEN_ID, total_stake_score)
        .check_current_reward_rate(1);
}

#[test]
fn continuous_distribute_rewards() {
    let stake_transfer = vec![(NFT_STAKING_TOKEN_ID, 1, 1)];
    let total_stake_score = LAND_PLOT_SCORES[0] * 2;

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
        .distribute_rewards(OWNER_ADDRESS_EXPR, KOSON_TOKEN_ID, total_stake_score)
        .distribute_rewards(OWNER_ADDRESS_EXPR, KOSON_TOKEN_ID, total_stake_score)
        .distribute_rewards(OWNER_ADDRESS_EXPR, KOSON_TOKEN_ID, total_stake_score)
        .check_current_reward_rate(3);
}

#[test]
fn distribute_invalid_reward_token() {
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
        .distribute_rewards_expect_err(
            OWNER_ADDRESS_EXPR,
            INVALID_ESDT_TOKEN_ID,
            1000,
            ERR_NOT_A_REWARD,
        );
}

#[test]
fn users_can_distribute_rewards() {
    let stake_transfer = vec![(NFT_STAKING_TOKEN_ID, 1, 1)];
    let total_stake_score = LAND_PLOT_SCORES[0] * 2;

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
        .distribute_rewards(USER_1_ADDRESS_EXPR, KOSON_TOKEN_ID, total_stake_score)
        .check_current_reward_rate(1);
}
