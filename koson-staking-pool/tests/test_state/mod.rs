use multiversx_sc_scenario::{api::StaticApi, ContractInfo, ScenarioWorld};

pub mod sc_interactions;

pub const KOSON_STAKING_SC_ADDRESS: &str = "sc:KOSON_V2_NFT_STAKING_SC_ADDRESS";

pub const OWNER_ADDRESS_EXPR: &str = "address:OWNER_ADDRESS";
pub const USER_1_ADDRESS_EXPR: &str = "address:USER_1_ADDRESS";

pub const INVALID_ESDT_TOKEN_ID: &str = "INVALID-000000";
pub const INITIAL_ESDT_BALANCE: u128 = 1000000000000000000000u128;

pub const KOSON_ESOTERIC_TOKEN_ID: &str = "EKOSON-123456";
pub const KOSON_ANCIENT_TOKEN_ID: &str = "AKOSON-123456";
pub const KOSON_PRIMORDIAL_TOKEN_ID: &str = "PKOSON-123456";
pub const KOSON_REWARD_BEARING_TOKEN: &str = "RKOSON-123456";
pub const KOSON_UNBONDING_META_TOKEN: &str = "UKOSON-123456";

pub const UNBONDING_TIME_PENALTY: u64 = 100; // 100 days

fn world() -> ScenarioWorld {
    let mut blockchain = ScenarioWorld::new();
    blockchain.set_current_dir_from_workspace("../");

    blockchain.register_contract(
        KOSON_STAKING_SC_ADDRESS,
        koson_staking_pool::ContractBuilder,
    );

    blockchain
}

type KosonIndexStakingPoolContract = ContractInfo<koson_staking_pool::Proxy<StaticApi>>;

pub struct KosonStakingPoolState {
    pub world: ScenarioWorld,
    pub contract: KosonIndexStakingPoolContract,
}
