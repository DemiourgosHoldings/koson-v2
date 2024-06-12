use test_state::{KosonFactoryState, OWNER_ADDRESS_EXPR};

mod scenarios;
mod test_state;

#[test]
fn test_deploy() {
    let mut state = KosonFactoryState::new();
    state.deploy();
}

#[test]
fn test_deploy_and_config_full() {
    KosonFactoryState::new().deploy().init();
}

#[test]
fn test_deploy_all_and_config_full() {
    KosonFactoryState::new().deploy_all().init();
}

#[test]
fn test_set_distribution_list() {
    let (addresses, percentages, distribution_types) =
        test_state::helpers::get_direct_send_distribution_list();
    KosonFactoryState::new()
        .deploy()
        .init()
        .set_distribution_list(
            OWNER_ADDRESS_EXPR,
            addresses,
            percentages,
            distribution_types,
        );
}
