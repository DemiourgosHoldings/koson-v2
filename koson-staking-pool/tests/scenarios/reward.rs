use soul_nft_staking::constants::{errors::ERR_NOTHING_TO_CLAIM, score::ORIGIN_SOULS_SCORE};

use crate::test_state::{
    KosonV2NftStakingContractState, INITIAL_ESDT_BALANCE, KOSON_TOKEN_ID, ORIGIN_SOULS_TOKEN_IDS,
    OWNER_ADDRESS_EXPR, USER_1_ADDRESS_EXPR,
};

const STAKE_TRANSFER_USER_1: [(&str, u64); 1] = [(ORIGIN_SOULS_TOKEN_IDS[0], 1u64)];
const STAKE_TRANSFER_USER_2: [(&str, u64); 1] = [(ORIGIN_SOULS_TOKEN_IDS[0], 2u64)];
const STAKE_TRANSFER_OWNER: [(&str, u64); 1] = [(ORIGIN_SOULS_TOKEN_IDS[0], 51u64)];

#[test]
fn simple_distribute_rewards() {
    let mut state = KosonV2NftStakingContractState::new();

    apply_scenario_setup(&mut state)
        .check_pending_reward(USER_1_ADDRESS_EXPR, ORIGIN_SOULS_SCORE)
        .check_pending_reward(OWNER_ADDRESS_EXPR, ORIGIN_SOULS_SCORE);
}

#[test]
fn stake_resets_unclaimed_reward_rate() {
    let mut state = KosonV2NftStakingContractState::new();
    apply_scenario_setup(&mut state)
        .stake_many(
            USER_1_ADDRESS_EXPR,
            STAKE_TRANSFER_USER_2.to_vec(),
            ORIGIN_SOULS_SCORE,
        )
        .check_last_claimed_reward_rate(USER_1_ADDRESS_EXPR, 1);
}

#[test]
fn increase_in_stake_does_not_affect_past_rewards() {
    let mut state = KosonV2NftStakingContractState::new();
    apply_scenario_setup(&mut state)
        .stake_many(
            USER_1_ADDRESS_EXPR,
            STAKE_TRANSFER_USER_2.to_vec(),
            ORIGIN_SOULS_SCORE,
        )
        .check_pending_reward(USER_1_ADDRESS_EXPR, ORIGIN_SOULS_SCORE);
}

#[test]
fn stake_keeps_unclaimed_reward() {
    let mut state = KosonV2NftStakingContractState::new();
    apply_scenario_setup(&mut state)
        .stake_many(
            USER_1_ADDRESS_EXPR,
            STAKE_TRANSFER_USER_2.to_vec(),
            ORIGIN_SOULS_SCORE,
        )
        .check_pending_reward(USER_1_ADDRESS_EXPR, ORIGIN_SOULS_SCORE);
}

#[test]
fn double_claim_reward_fails() {
    let mut state = KosonV2NftStakingContractState::new();
    apply_scenario_setup(&mut state)
        .claim_rewards(USER_1_ADDRESS_EXPR, ORIGIN_SOULS_SCORE)
        .claim_rewards_expect_err(USER_1_ADDRESS_EXPR, ERR_NOTHING_TO_CLAIM)
        .check_user_balance(
            USER_1_ADDRESS_EXPR,
            KOSON_TOKEN_ID,
            INITIAL_ESDT_BALANCE + ORIGIN_SOULS_SCORE as u128,
        );
}

#[test]
fn claiming_no_rewards_fails() {
    let mut state = KosonV2NftStakingContractState::new();
    state
        .deploy()
        .init()
        .claim_rewards_expect_err(USER_1_ADDRESS_EXPR, ERR_NOTHING_TO_CLAIM);
}

fn apply_scenario_setup(
    state: &mut KosonV2NftStakingContractState,
) -> &mut KosonV2NftStakingContractState {
    let total_stake_score = ORIGIN_SOULS_SCORE * 2;

    state
        .deploy()
        .init()
        .stake_many(
            USER_1_ADDRESS_EXPR,
            STAKE_TRANSFER_USER_1.to_vec(),
            ORIGIN_SOULS_SCORE,
        )
        .stake_many(
            OWNER_ADDRESS_EXPR,
            STAKE_TRANSFER_OWNER.to_vec(),
            ORIGIN_SOULS_SCORE,
        )
        .distribute_rewards(OWNER_ADDRESS_EXPR, KOSON_TOKEN_ID, total_stake_score)
}
