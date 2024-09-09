use multiversx_sc_scenario::*;

fn world() -> ScenarioWorld {
    let mut blockchain = ScenarioWorld::new();
    // blockchain.set_current_dir_from_workspace("relative path to your workspace, if applicable");

    blockchain.register_contract("mxsc:output/simple-asset-minter.mxsc.json", simple_asset_minter::ContractBuilder);
    blockchain
}

#[test]
fn empty_rs() {
    world().run("scenarios/simple_asset_minter.scen.json");
}
