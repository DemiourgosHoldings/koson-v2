use test_state::KosonStakingPoolState;

mod scenarios;
mod test_state;

#[test]
fn test_deploy() {
    let mut state = KosonStakingPoolState::new();
    state.deploy();
}

#[test]
fn test_deploy_and_config_full() {
    KosonStakingPoolState::new().deploy().init();
}
