use multiversx_sc_scenario::scenario_model::AddressValue;
use test_state::{
    KosonV2NftStakingContractState, EGLD_PRICE_FEED_NAME, ORACLE_SC_ADDRESS_EXPR, WEGLD_TOKEN_ID,
};

// mod scenarios;
mod test_state;

#[test]
fn test_deploy() {
    let mut state = KosonV2NftStakingContractState::new();
    state.deploy();
}

#[test]
fn test_deploy_and_config_full() {
    KosonV2NftStakingContractState::new().deploy().init();
}

#[test]
fn test_oracle_integration_full() {
    KosonV2NftStakingContractState::new()
        .deploy()
        .init()
        .check_oracle_registry_address(&AddressValue::from(ORACLE_SC_ADDRESS_EXPR).to_address())
        .set_oracle_feed_price(EGLD_PRICE_FEED_NAME, 1_000_000)
        .check_oracle_feed_price(WEGLD_TOKEN_ID, 1_000_000);
}
