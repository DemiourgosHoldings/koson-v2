use multiversx_sc_scenario::{api::StaticApi, ContractInfo, ScenarioWorld};

pub mod deploy_utils;
pub mod helpers;
pub mod sc_interactions;

pub const KOSON_FACTORY_SC_ADDRESS: &str = "sc:KOSON_FACTORY_SC_ADDRESS";

pub const OWNER_ADDRESS_EXPR: &str = "address:OWNER_ADDRESS";
pub const USER_1_ADDRESS_EXPR: &str = "address:USER_1_ADDRESS";
pub const KOSON_STAKING_POOL_1_ADDRESS_EXPR: &str = "sc:KOSON_STAKING_POOL_1_ADDRESS";
pub const KOSON_STAKING_POOL_2_ADDRESS_EXPR: &str = "sc:KOSON_STAKING_POOL_2_ADDRESS";
pub const KOSON_STAKING_POOL_3_ADDRESS_EXPR: &str = "sc:KOSON_STAKING_POOL_3_ADDRESS";
pub const KOSON_STAKING_POOL_4_ADDRESS_EXPR: &str = "sc:KOSON_STAKING_POOL_4_ADDRESS";
pub const KOSON_STAKING_POOL_5_ADDRESS_EXPR: &str = "sc:KOSON_STAKING_POOL_5_ADDRESS";
pub const KOSON_STAKING_POOL_6_ADDRESS_EXPR: &str = "sc:KOSON_STAKING_POOL_6_ADDRESS";
pub const SOUL_STAKING_POOL_ADDRESS_EXPR: &str = "sc:SOUL_STAKING_POOL_ADDRESS";
pub const LAND_PLOT_STAKING_POOL_ADDRESS_EXPR: &str = "sc:LAND_PLOT_STAKING_POOL_ADDRESS";

pub const FACTORY_TKN_ID: &str = "KOSON-123456";
pub const IGNORED_TKN_ID: &str = "IGNORED-123456";

fn world() -> ScenarioWorld {
    let mut blockchain = ScenarioWorld::new();
    blockchain.set_current_dir_from_workspace("../");

    blockchain.register_contract(
        KOSON_FACTORY_SC_ADDRESS,
        koson_factory_chrysopoeic::ContractBuilder,
    );

    for koson_staking_pool_address in [
        KOSON_STAKING_POOL_1_ADDRESS_EXPR,
        KOSON_STAKING_POOL_2_ADDRESS_EXPR,
        KOSON_STAKING_POOL_3_ADDRESS_EXPR,
        KOSON_STAKING_POOL_4_ADDRESS_EXPR,
        KOSON_STAKING_POOL_5_ADDRESS_EXPR,
        KOSON_STAKING_POOL_6_ADDRESS_EXPR,
    ] {
        blockchain.register_contract(
            koson_staking_pool_address,
            koson_staking_pool::ContractBuilder,
        );
    }

    blockchain.register_contract(
        SOUL_STAKING_POOL_ADDRESS_EXPR,
        soul_nft_staking::ContractBuilder,
    );
    blockchain.register_contract(
        LAND_PLOT_STAKING_POOL_ADDRESS_EXPR,
        land_plot_staking::ContractBuilder,
    );

    blockchain
}

type KosonFactoryContract = ContractInfo<koson_factory_chrysopoeic::Proxy<StaticApi>>;
type KosonStakingContract = ContractInfo<koson_staking_pool::Proxy<StaticApi>>;
type SoulStakingContract = ContractInfo<soul_nft_staking::Proxy<StaticApi>>;
type LandPlotStakingContract = ContractInfo<land_plot_staking::Proxy<StaticApi>>;

pub struct KosonFactoryState {
    pub world: ScenarioWorld,
    pub contract: KosonFactoryContract,
    pub koson_staking_1_contract: KosonStakingContract,
    pub koson_staking_2_contract: KosonStakingContract,
    pub koson_staking_3_contract: KosonStakingContract,
    pub koson_staking_4_contract: KosonStakingContract,
    pub koson_staking_5_contract: KosonStakingContract,
    pub koson_staking_6_contract: KosonStakingContract,
    pub soul_staking_contract: SoulStakingContract,
    pub land_plot_staking_contract: LandPlotStakingContract,
}
