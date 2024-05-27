use land_plot_staking::constants::score::LAND_PLOT_SCORES;

use crate::test_state::{
    KosonV2NftStakingContractState, KOSON_TOKEN_ID, NFT_STAKING_TOKEN_ID, OWNER_ADDRESS_EXPR,
    USER_1_ADDRESS_EXPR,
};

const STAKE_TRANSFER: [(&str, u64, u64); 1] = [(NFT_STAKING_TOKEN_ID, 1u64, 1u64)];

#[test]
fn simple_distribute_rewards() {
    let mut state = KosonV2NftStakingContractState::new();

    apply_scenario_setup(&mut state)
        .check_pending_reward(USER_1_ADDRESS_EXPR, LAND_PLOT_SCORES[0])
        .check_pending_reward(OWNER_ADDRESS_EXPR, LAND_PLOT_SCORES[0]);
}

#[test]
fn stake_resets_unclaimed_reward_rate() {
    let mut state = KosonV2NftStakingContractState::new();
    apply_scenario_setup(&mut state)
        .stake_many(
            USER_1_ADDRESS_EXPR,
            STAKE_TRANSFER.to_vec(),
            LAND_PLOT_SCORES[0],
        )
        .check_last_claimed_reward_rate(USER_1_ADDRESS_EXPR, 1);
}

#[test]
fn increase_in_stake_does_not_affect_past_rewards() {
    let mut state = KosonV2NftStakingContractState::new();
    apply_scenario_setup(&mut state)
        .stake_many(
            USER_1_ADDRESS_EXPR,
            STAKE_TRANSFER.to_vec(),
            LAND_PLOT_SCORES[0],
        )
        .stake_many(
            USER_1_ADDRESS_EXPR,
            STAKE_TRANSFER.to_vec(),
            LAND_PLOT_SCORES[0],
        )
        .check_pending_reward(USER_1_ADDRESS_EXPR, LAND_PLOT_SCORES[0]);
}

#[test]
fn stake_keeps_unclaimed_reward() {
    let mut state = KosonV2NftStakingContractState::new();
    apply_scenario_setup(&mut state)
        .stake_many(
            USER_1_ADDRESS_EXPR,
            STAKE_TRANSFER.to_vec(),
            LAND_PLOT_SCORES[0],
        )
        .check_pending_reward(USER_1_ADDRESS_EXPR, LAND_PLOT_SCORES[0]);
}

#[test]
fn double_claim_reward_fails() {
    let mut state = KosonV2NftStakingContractState::new();
    apply_scenario_setup(&mut state)
        .stake_many(
            USER_1_ADDRESS_EXPR,
            STAKE_TRANSFER.to_vec(),
            LAND_PLOT_SCORES[0],
        )
        .claim_rewards(USER_1_ADDRESS_EXPR, LAND_PLOT_SCORES[0])
        .check_pending_reward(USER_1_ADDRESS_EXPR, 0);
}

fn apply_scenario_setup(
    state: &mut KosonV2NftStakingContractState,
) -> &mut KosonV2NftStakingContractState {
    let total_stake_score = LAND_PLOT_SCORES[0] * 2;

    state
        .deploy()
        .init()
        .stake_many(
            USER_1_ADDRESS_EXPR,
            STAKE_TRANSFER.to_vec(),
            LAND_PLOT_SCORES[0],
        )
        .stake_many(
            OWNER_ADDRESS_EXPR,
            STAKE_TRANSFER.to_vec(),
            LAND_PLOT_SCORES[0],
        )
        .distribute_rewards(OWNER_ADDRESS_EXPR, KOSON_TOKEN_ID, total_stake_score)
}
