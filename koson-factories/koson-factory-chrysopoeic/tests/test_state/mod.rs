use multiversx_sc_scenario::{api::StaticApi, ContractInfo, ScenarioWorld};

pub mod helpers;
pub mod sc_interactions;

pub const KOSON_FACTORY_SC_ADDRESS: &str = "sc:KOSON_FACTORY_SC_ADDRESS";

pub const OWNER_ADDRESS_EXPR: &str = "address:OWNER_ADDRESS";
pub const USER_1_ADDRESS_EXPR: &str = "address:USER_1_ADDRESS";

pub const FACTORY_TKN_ID: &str = "KOSON-123456";

fn world() -> ScenarioWorld {
    let mut blockchain = ScenarioWorld::new();
    blockchain.set_current_dir_from_workspace("../");

    blockchain.register_contract(
        KOSON_FACTORY_SC_ADDRESS,
        koson_factory_chrysopoeic::ContractBuilder,
    );

    blockchain
}

type KosonFactoryContract = ContractInfo<koson_factory_chrysopoeic::Proxy<StaticApi>>;

pub struct KosonFactoryState {
    pub world: ScenarioWorld,
    pub contract: KosonFactoryContract,
}
