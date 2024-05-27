use multiversx_sc::types::Address;
use multiversx_sc_scenario::{api::StaticApi, ContractInfo, ScenarioWorld};

pub mod sc_interactions;

pub const NFT_STAKING_SC_ADDRESS_EXPR: &str = "sc:KOSON_V2_NFT_STAKING_SC_ADDRESS";
pub const ORACLE_SC_ADDRESS_EXPR: &str = "sc:ORACLE_FEEDS";

pub const DEX_OURO_KOSON_PAIR_ADDRESS_EXPR: &str = "sc:DEX_OURO_KOSON_PAIR_ADDRESS";
pub const DEX_OURO_USDD_PAIR_ADDRESS_EXPR: &str = "sc:DEX_OURO_USDD_PAIR_ADDRESS";
pub const DEX_OURO_USDC_PAIR_ADDRESS_EXPR: &str = "sc:DEX_OURO_USDC_PAIR_ADDRESS";
pub const DEX_OURO_WEGLD_PAIR_ADDRESS_EXPR: &str = "sc:DEX_OURO_WEGLD_PAIR_ADDRESS";

pub const OWNER_ADDRESS_EXPR: &str = "address:OWNER_ADDRESS";
pub const USER_1_ADDRESS_EXPR: &str = "address:USER_1_ADDRESS";

pub const INVALID_ESDT_TOKEN_ID: &str = "INVALID-000000";
pub const INVALID_NFT_TOKEN_ID: &str = "INVALID-abcdef";
pub const NFT_STAKING_TOKEN_ID: &str = "STAKE-abcdef";
pub const OURO_TOKEN_ID: &str = "OURO-123456";
pub const USDD_TOKEN_ID: &str = "USDD-123456";
pub const USDC_TOKEN_ID: &str = "USDC-123456";
pub const WEGLD_TOKEN_ID: &str = "WEGLD-123456";
pub const KOSON_TOKEN_ID: &str = "KOSON-123456";

pub const EGLD_PRICE_FEED_NAME: &[u8] = b"EGLD_USD_PRICE_FEED";

pub const INITIAL_ESDT_BALANCE: u128 = 1000000000000000000000u128;
pub const INITIAL_SFT_BALANCE: u128 = 10000;

fn world() -> ScenarioWorld {
    let mut blockchain = ScenarioWorld::new();
    blockchain.set_current_dir_from_workspace("../");

    blockchain.register_contract(
        NFT_STAKING_SC_ADDRESS_EXPR,
        land_plot_staking::ContractBuilder,
    );

    blockchain.register_contract(
        ORACLE_SC_ADDRESS_EXPR,
        umbrella_oracle_mock::ContractBuilder,
    );

    blockchain.register_contract(
        DEX_OURO_KOSON_PAIR_ADDRESS_EXPR,
        dex_pair_sc::ContractBuilder,
    );
    blockchain.register_contract(
        DEX_OURO_USDD_PAIR_ADDRESS_EXPR,
        dex_pair_sc::ContractBuilder,
    );
    blockchain.register_contract(
        DEX_OURO_USDC_PAIR_ADDRESS_EXPR,
        dex_pair_sc::ContractBuilder,
    );
    blockchain.register_contract(
        DEX_OURO_WEGLD_PAIR_ADDRESS_EXPR,
        dex_pair_sc::ContractBuilder,
    );

    blockchain
}

type KosonV2NftStakingContract = ContractInfo<land_plot_staking::Proxy<StaticApi>>;
type OracleFeedsContract = ContractInfo<umbrella_oracle_mock::Proxy<StaticApi>>;
type DexPairContract = ContractInfo<dex_pair_sc::Proxy<StaticApi>>;

pub struct KosonV2NftStakingContractState {
    pub world: ScenarioWorld,
    pub contract: KosonV2NftStakingContract,
    pub oracle_contract: OracleFeedsContract,
    pub dex_pair_ouro_koson_contract: DexPairContract,
    pub dex_pair_ouro_usdd_contract: DexPairContract,
    pub dex_pair_ouro_usdc_contract: DexPairContract,
    pub dex_pair_ouro_wegld_contract: DexPairContract,
    pub owner_address: Address,
}
