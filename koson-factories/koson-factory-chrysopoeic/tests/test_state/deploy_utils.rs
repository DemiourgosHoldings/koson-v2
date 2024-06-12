use super::{
    KosonFactoryState, FACTORY_TKN_ID, IGNORED_TKN_ID, KOSON_STAKING_POOL_1_ADDRESS_EXPR,
    KOSON_STAKING_POOL_2_ADDRESS_EXPR, KOSON_STAKING_POOL_3_ADDRESS_EXPR,
    KOSON_STAKING_POOL_4_ADDRESS_EXPR, KOSON_STAKING_POOL_5_ADDRESS_EXPR,
    KOSON_STAKING_POOL_6_ADDRESS_EXPR, LAND_PLOT_STAKING_POOL_ADDRESS_EXPR, OWNER_ADDRESS_EXPR,
    SOUL_STAKING_POOL_ADDRESS_EXPR,
};

use multiversx_sc::types::{Address, MultiValueManagedVec, MultiValueManagedVecCounted};
use multiversx_sc_scenario::{
    managed_address, managed_token_id,
    scenario_model::{Account, ScCallStep, ScDeployStep, SetStateStep},
};

use koson_staking_pool::ProxyTrait as _;
use land_plot_staking::ProxyTrait as _;
use soul_nft_staking::ProxyTrait as _;

pub const NFT_TOKEN_ID: &str = "NFT-abcdef";
pub const SFT_TOKEN_ID: &str = "SFT-abcdef";

impl KosonFactoryState {
    pub fn deploy_all(&mut self) -> &mut Self {
        // IMPORTANT!
        // If you want to change the order of the contracts, add or remove interactions, consider the below information first:
        // each init_<> fn call increases account nonce by 2. First deploy nonce is 2, the setup increases the nonce to 3, then the next deploy nonce is 4 and so on
        // this means that the nonce for the first pool is 2, for the second pool is 4, for the third pool is 6, for the fourth pool is 8, for the fifth pool is 10, for the sixth pool is 12
        // and for land plot staking we get deploy nonce 14, and for soul staking we get deploy nonce 16
        self.deploy()
            .init_koson_staking_pool(1)
            .init_koson_staking_pool(2)
            .init_koson_staking_pool(3)
            .init_koson_staking_pool(4)
            .init_koson_staking_pool(5)
            .init_koson_staking_pool(6)
            .init_land_plot_staking()
            .init_soul_staking()
            .add_balances()
            .stake_balances();

        self
    }

    fn init_koson_staking_pool(&mut self, pool_index: u8) -> &mut Self {
        let nonce = pool_index as u64 * 2;
        match pool_index {
            1 => {
                self.world.set_state_step(SetStateStep::new().new_address(
                    OWNER_ADDRESS_EXPR,
                    nonce,
                    KOSON_STAKING_POOL_1_ADDRESS_EXPR,
                ));
                let code = self
                    .world
                    .code_expression(KOSON_STAKING_POOL_1_ADDRESS_EXPR);
                self.world.sc_deploy(
                    ScDeployStep::new()
                        .from(OWNER_ADDRESS_EXPR)
                        .code(code)
                        .call(self.koson_staking_1_contract.init()),
                );
                self.world
                    .sc_call(ScCallStep::new().from(OWNER_ADDRESS_EXPR).call(
                        self.koson_staking_1_contract.init_config(
                            0u64,
                            MultiValueManagedVec::from_single_item(managed_token_id!(
                                FACTORY_TKN_ID
                            )),
                        ),
                    ));
            }
            2 => {
                self.world.set_state_step(SetStateStep::new().new_address(
                    OWNER_ADDRESS_EXPR,
                    nonce,
                    KOSON_STAKING_POOL_2_ADDRESS_EXPR,
                ));
                let code = self
                    .world
                    .code_expression(KOSON_STAKING_POOL_2_ADDRESS_EXPR);
                self.world.sc_deploy(
                    ScDeployStep::new()
                        .from(OWNER_ADDRESS_EXPR)
                        .code(code)
                        .call(self.koson_staking_2_contract.init()),
                );
                self.world
                    .sc_call(ScCallStep::new().from(OWNER_ADDRESS_EXPR).call(
                        self.koson_staking_2_contract.init_config(
                            0u64,
                            MultiValueManagedVec::from_single_item(managed_token_id!(
                                FACTORY_TKN_ID
                            )),
                        ),
                    ));
            }
            3 => {
                self.world.set_state_step(SetStateStep::new().new_address(
                    OWNER_ADDRESS_EXPR,
                    nonce,
                    KOSON_STAKING_POOL_3_ADDRESS_EXPR,
                ));
                let code = self
                    .world
                    .code_expression(KOSON_STAKING_POOL_3_ADDRESS_EXPR);
                self.world.sc_deploy(
                    ScDeployStep::new()
                        .from(OWNER_ADDRESS_EXPR)
                        .code(code)
                        .call(self.koson_staking_3_contract.init()),
                );
                self.world
                    .sc_call(ScCallStep::new().from(OWNER_ADDRESS_EXPR).call(
                        self.koson_staking_3_contract.init_config(
                            0u64,
                            MultiValueManagedVec::from_single_item(managed_token_id!(
                                FACTORY_TKN_ID
                            )),
                        ),
                    ));
            }
            4 => {
                self.world.set_state_step(SetStateStep::new().new_address(
                    OWNER_ADDRESS_EXPR,
                    nonce,
                    KOSON_STAKING_POOL_4_ADDRESS_EXPR,
                ));
                let code = self
                    .world
                    .code_expression(KOSON_STAKING_POOL_4_ADDRESS_EXPR);
                self.world.sc_deploy(
                    ScDeployStep::new()
                        .from(OWNER_ADDRESS_EXPR)
                        .code(code)
                        .call(self.koson_staking_4_contract.init()),
                );
                self.world
                    .sc_call(ScCallStep::new().from(OWNER_ADDRESS_EXPR).call(
                        self.koson_staking_4_contract.init_config(
                            0u64,
                            MultiValueManagedVec::from_single_item(managed_token_id!(
                                FACTORY_TKN_ID
                            )),
                        ),
                    ));
            }
            5 => {
                self.world.set_state_step(SetStateStep::new().new_address(
                    OWNER_ADDRESS_EXPR,
                    nonce,
                    KOSON_STAKING_POOL_5_ADDRESS_EXPR,
                ));
                let code = self
                    .world
                    .code_expression(KOSON_STAKING_POOL_5_ADDRESS_EXPR);
                self.world.sc_deploy(
                    ScDeployStep::new()
                        .from(OWNER_ADDRESS_EXPR)
                        .code(code)
                        .call(self.koson_staking_5_contract.init()),
                );
                self.world
                    .sc_call(ScCallStep::new().from(OWNER_ADDRESS_EXPR).call(
                        self.koson_staking_5_contract.init_config(
                            0u64,
                            MultiValueManagedVec::from_single_item(managed_token_id!(
                                FACTORY_TKN_ID
                            )),
                        ),
                    ));
            }
            6 => {
                self.world.set_state_step(SetStateStep::new().new_address(
                    OWNER_ADDRESS_EXPR,
                    nonce,
                    KOSON_STAKING_POOL_6_ADDRESS_EXPR,
                ));
                let code = self
                    .world
                    .code_expression(KOSON_STAKING_POOL_6_ADDRESS_EXPR);
                self.world.sc_deploy(
                    ScDeployStep::new()
                        .from(OWNER_ADDRESS_EXPR)
                        .code(code)
                        .call(self.koson_staking_6_contract.init()),
                );
                self.world
                    .sc_call(ScCallStep::new().from(OWNER_ADDRESS_EXPR).call(
                        self.koson_staking_6_contract.init_config(
                            0u64,
                            MultiValueManagedVec::from_single_item(managed_token_id!(
                                FACTORY_TKN_ID
                            )),
                        ),
                    ));
            }
            _ => panic!("Invalid pool index"),
        }

        self
    }

    fn init_soul_staking(&mut self) -> &mut Self {
        self.world.set_state_step(SetStateStep::new().new_address(
            OWNER_ADDRESS_EXPR,
            16,
            SOUL_STAKING_POOL_ADDRESS_EXPR,
        ));
        let code = self.world.code_expression(SOUL_STAKING_POOL_ADDRESS_EXPR);
        self.world.sc_deploy(
            ScDeployStep::new()
                .from(OWNER_ADDRESS_EXPR)
                .code(code)
                .call(self.soul_staking_contract.init()),
        );
        self.world
            .sc_call(ScCallStep::new().from(OWNER_ADDRESS_EXPR).call(
                self.soul_staking_contract.init_config(
                    managed_token_id!(IGNORED_TKN_ID),
                    managed_token_id!(IGNORED_TKN_ID),
                    managed_token_id!(IGNORED_TKN_ID),
                    managed_token_id!(IGNORED_TKN_ID),
                    managed_token_id!(FACTORY_TKN_ID),
                    managed_address!(&Address::zero()),
                    managed_token_id!(NFT_TOKEN_ID),
                    MultiValueManagedVecCounted::new(),
                    MultiValueManagedVec::new(),
                ),
            ));

        let mut token_ids = MultiValueManagedVecCounted::new();
        let mut scores = MultiValueManagedVecCounted::new();

        token_ids.push(managed_token_id!(NFT_TOKEN_ID));
        scores.push(1u64.into());
        self.world.sc_call(
            ScCallStep::new()
                .from(OWNER_ADDRESS_EXPR)
                .call(self.soul_staking_contract.setup_scores(token_ids, scores)),
        );

        self
    }

    fn init_land_plot_staking(&mut self) -> &mut Self {
        self.world.set_state_step(SetStateStep::new().new_address(
            OWNER_ADDRESS_EXPR,
            14,
            LAND_PLOT_STAKING_POOL_ADDRESS_EXPR,
        ));
        let code = self
            .world
            .code_expression(LAND_PLOT_STAKING_POOL_ADDRESS_EXPR);
        self.world.sc_deploy(
            ScDeployStep::new()
                .from(OWNER_ADDRESS_EXPR)
                .code(code)
                .call(self.land_plot_staking_contract.init()),
        );
        self.world
            .sc_call(ScCallStep::new().from(OWNER_ADDRESS_EXPR).call(
                self.land_plot_staking_contract.init_config(
                    managed_token_id!(SFT_TOKEN_ID),
                    managed_token_id!(IGNORED_TKN_ID),
                    managed_token_id!(IGNORED_TKN_ID),
                    managed_token_id!(IGNORED_TKN_ID),
                    managed_token_id!(IGNORED_TKN_ID),
                    managed_token_id!(FACTORY_TKN_ID),
                    managed_address!(&Address::zero()),
                ),
            ));

        self
    }

    fn add_balances(&mut self) -> &mut Self {
        let account = Account::new()
            .esdt_nft_balance(format!("str:{}", NFT_TOKEN_ID).as_str(), 1, "1", Some(""))
            .esdt_nft_balance(format!("str:{}", SFT_TOKEN_ID).as_str(), 1, "100", Some(""));

        self.world.set_state_step(
            SetStateStep::new()
                .new_token_identifier(format!("str:{}", NFT_TOKEN_ID))
                .new_token_identifier(format!("str:{}", SFT_TOKEN_ID))
                .put_account(OWNER_ADDRESS_EXPR, account),
        );

        self
    }

    fn stake_balances(&mut self) -> &mut Self {
        self.world.sc_call(
            ScCallStep::new()
                .from(OWNER_ADDRESS_EXPR)
                .esdt_transfer(format!("str:{}", NFT_TOKEN_ID), 1, "1")
                .call(self.soul_staking_contract.stake_souls()),
        );

        self.world.sc_call(
            ScCallStep::new()
                .from(OWNER_ADDRESS_EXPR)
                .esdt_transfer(format!("str:{}", SFT_TOKEN_ID), 1, "100")
                .call(self.land_plot_staking_contract.stake_land_plots()),
        );

        self
    }
}
