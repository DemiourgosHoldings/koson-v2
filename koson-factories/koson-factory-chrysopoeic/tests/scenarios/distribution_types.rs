use crate::test_state::{
    helpers::get_simple_distribution_list, KosonFactoryState, FACTORY_TKN_ID, OWNER_ADDRESS_EXPR,
    USER_1_ADDRESS_EXPR,
};

#[test]
fn send_direct_integration() {
    let (addresses, percentages, distribution_types) = get_simple_distribution_list();
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
