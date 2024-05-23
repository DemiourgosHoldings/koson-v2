use multiversx_sc_scenario::*;

fn world() -> ScenarioWorld {
    let mut blockchain = ScenarioWorld::new();
    // blockchain.set_current_dir_from_workspace("relative path to your workspace, if applicable");

    blockchain.register_contract("mxsc:output/land-plot-staking.mxsc.json", land_plot_staking::ContractBuilder);
    blockchain
}

#[test]
fn empty_rs() {
    world().run("scenarios/land_plot_staking.scen.json");
}
