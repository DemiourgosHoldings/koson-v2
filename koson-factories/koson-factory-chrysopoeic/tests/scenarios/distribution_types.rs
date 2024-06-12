use koson_factory_chrysopoeic::constants::config::MAX_PERCENTAGE;

use crate::test_state::{
    helpers::*, KosonFactoryState, FACTORY_TKN_ID, KOSON_STAKING_POOL_1_ADDRESS_EXPR,
    LAND_PLOT_STAKING_POOL_ADDRESS_EXPR, OWNER_ADDRESS_EXPR, SOUL_STAKING_POOL_ADDRESS_EXPR,
    USER_1_ADDRESS_EXPR,
};

#[test]
fn send_direct_integration() {
    let (addresses, percentages, distribution_types) = get_direct_send_distribution_list();
    let total_distribution_amount = 2311_477126785564068863u128;

    KosonFactoryState::new()
        .deploy()
        .init()
        .set_distribution_list(
            OWNER_ADDRESS_EXPR,
            addresses,
            percentages,
            distribution_types,
        )
        .set_block_epoch(1)
        .distribute_reward(OWNER_ADDRESS_EXPR)
        .check_user_balance(
            USER_1_ADDRESS_EXPR,
            FACTORY_TKN_ID,
            total_distribution_amount,
        );
}

#[test]
fn soul_nft_staking_integration() {
    let (addresses, percentages, distribution_types) =
        get_single_nft_staking_pool_distribution_list();
    let total_distribution_amount = 2311_477126785564068863u128;

    KosonFactoryState::new()
        .deploy_all()
        .init()
        .set_distribution_list(
            OWNER_ADDRESS_EXPR,
            addresses,
            percentages,
            distribution_types,
        )
        .set_block_epoch(1)
        .distribute_reward(OWNER_ADDRESS_EXPR)
        .check_user_balance(
            SOUL_STAKING_POOL_ADDRESS_EXPR,
            FACTORY_TKN_ID,
            total_distribution_amount,
        );
}

#[test]
fn land_plot_staking_integration() {
    let (addresses, percentages, distribution_types) =
        get_single_land_plot_staking_pool_distribution_list();
    let total_distribution_amount = 2311_477126785564068863u128;

    KosonFactoryState::new()
        .deploy_all()
        .init()
        .set_distribution_list(
            OWNER_ADDRESS_EXPR,
            addresses,
            percentages,
            distribution_types,
        )
        .set_block_epoch(1)
        .distribute_reward(OWNER_ADDRESS_EXPR)
        .check_user_balance(
            LAND_PLOT_STAKING_POOL_ADDRESS_EXPR,
            FACTORY_TKN_ID,
            total_distribution_amount,
        );
}

#[test]
fn koson_staking_integration() {
    let (addresses, percentages, distribution_types) =
        get_single_koson_staking_pool_distribution_list();
    let total_distribution_amount = 2311_477126785564068863u128;

    KosonFactoryState::new()
        .deploy_all()
        .init()
        .set_distribution_list(
            OWNER_ADDRESS_EXPR,
            addresses,
            percentages,
            distribution_types,
        )
        .set_block_epoch(1)
        .distribute_reward(OWNER_ADDRESS_EXPR)
        .check_user_balance(
            KOSON_STAKING_POOL_1_ADDRESS_EXPR,
            FACTORY_TKN_ID,
            total_distribution_amount + 1, // +1 that's already staked
        );
}

#[test]
fn full_staking_distribution_integration() {
    let (addresses, percentages, distribution_types) = get_actual_distribution_list();
    let total_distribution_amount = 2311_477126785564068863u128;

    let mut expected_balances_after_distribution = vec![];

    for percentage in percentages.iter() {
        expected_balances_after_distribution
            .push(total_distribution_amount * (**percentage as u128) / MAX_PERCENTAGE as u128);
    }

    let mut state = KosonFactoryState::new();
    state
        .deploy_all()
        .init()
        .set_distribution_list(
            OWNER_ADDRESS_EXPR,
            addresses,
            percentages,
            distribution_types,
        )
        .set_block_epoch(1)
        .distribute_reward(OWNER_ADDRESS_EXPR);

    for index in 0..addresses.len() {
        let expected_balance = match addresses[index].starts_with("sc:KOSON_STAKING_POOL") {
            true => expected_balances_after_distribution[index] + 1, // +1 that's already staked
            false => expected_balances_after_distribution[index],
        };
        state.check_user_balance(addresses[index], FACTORY_TKN_ID, expected_balance);
    }
}
