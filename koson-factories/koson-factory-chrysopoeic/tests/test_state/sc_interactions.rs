use super::{
    world, KosonFactoryContract, KosonFactoryState, KosonStakingContract, LandPlotStakingContract,
    SoulStakingContract, FACTORY_TKN_ID, KOSON_FACTORY_SC_ADDRESS,
    KOSON_STAKING_POOL_1_ADDRESS_EXPR, KOSON_STAKING_POOL_2_ADDRESS_EXPR,
    KOSON_STAKING_POOL_3_ADDRESS_EXPR, KOSON_STAKING_POOL_4_ADDRESS_EXPR,
    KOSON_STAKING_POOL_5_ADDRESS_EXPR, KOSON_STAKING_POOL_6_ADDRESS_EXPR,
    LAND_PLOT_STAKING_POOL_ADDRESS_EXPR, OWNER_ADDRESS_EXPR, SOUL_STAKING_POOL_ADDRESS_EXPR,
    USER_1_ADDRESS_EXPR,
};

use multiversx_sc::types::{BigUint, MultiValueManagedVecCounted};
use multiversx_sc_scenario::{
    api::StaticApi,
    managed_address, managed_token_id,
    scenario_model::{
        Account, AddressValue, CheckAccount, CheckStateStep, ScCallStep, ScDeployStep, ScQueryStep,
        SetStateStep, TxExpect,
    },
};

use koson_factory_chrysopoeic::{esdt::ProxyTrait as _, ProxyTrait as _};

impl KosonFactoryState {
    pub fn new() -> Self {
        let mut world = world();
        world.set_state_step(
            SetStateStep::new()
                .new_token_identifier(format!("str:{}", FACTORY_TKN_ID))
                .put_account(
                    OWNER_ADDRESS_EXPR,
                    Account::new().nonce(1).balance("100000000"),
                )
                .new_address(OWNER_ADDRESS_EXPR, 1, KOSON_FACTORY_SC_ADDRESS)
                .put_account(
                    USER_1_ADDRESS_EXPR,
                    Account::new().nonce(1).balance("100000000"),
                ),
        );

        let contract = KosonFactoryContract::new(KOSON_FACTORY_SC_ADDRESS);
        let koson_staking_1_contract = KosonStakingContract::new(KOSON_STAKING_POOL_1_ADDRESS_EXPR);
        let koson_staking_2_contract = KosonStakingContract::new(KOSON_STAKING_POOL_2_ADDRESS_EXPR);
        let koson_staking_3_contract = KosonStakingContract::new(KOSON_STAKING_POOL_3_ADDRESS_EXPR);
        let koson_staking_4_contract = KosonStakingContract::new(KOSON_STAKING_POOL_4_ADDRESS_EXPR);
        let koson_staking_5_contract = KosonStakingContract::new(KOSON_STAKING_POOL_5_ADDRESS_EXPR);
        let koson_staking_6_contract = KosonStakingContract::new(KOSON_STAKING_POOL_6_ADDRESS_EXPR);
        let soul_staking_contract = SoulStakingContract::new(SOUL_STAKING_POOL_ADDRESS_EXPR);
        let land_plot_staking_contract =
            LandPlotStakingContract::new(LAND_PLOT_STAKING_POOL_ADDRESS_EXPR);

        Self {
            world,
            contract,

            koson_staking_1_contract,
            koson_staking_2_contract,
            koson_staking_3_contract,
            koson_staking_4_contract,
            koson_staking_5_contract,
            koson_staking_6_contract,
            soul_staking_contract,
            land_plot_staking_contract,
        }
    }

    pub fn deploy(&mut self) -> &mut Self {
        let code = self.world.code_expression(KOSON_FACTORY_SC_ADDRESS);

        let esdt_roles = vec![
            "ESDTRoleLocalBurn".to_string(),
            "ESDTRoleLocalMint".to_string(),
        ];

        self.world.sc_deploy(
            ScDeployStep::new()
                .from(OWNER_ADDRESS_EXPR)
                .code(code.clone())
                .call(self.contract.init()),
        );

        let acc = Account::new()
            .owner(OWNER_ADDRESS_EXPR)
            .code(code)
            .esdt_roles(
                format!("str:{}", FACTORY_TKN_ID).as_str(),
                esdt_roles.clone(),
            );

        self.world
            .set_state_step(SetStateStep::new().put_account(KOSON_FACTORY_SC_ADDRESS, acc));

        self
    }

    pub fn init(&mut self) -> &mut Self {
        self.world.sc_call(
            ScCallStep::new().from(OWNER_ADDRESS_EXPR).call(
                self.contract
                    .set_token_id(managed_token_id!(FACTORY_TKN_ID)),
            ),
        );

        self
    }

    pub fn set_distribution_list(
        &mut self,
        address_from: &str,
        addresses: &[&str],
        percentages: &[&u64],
        distribution_types: &[&u8],
    ) -> &mut Self {
        let mut addresses_arg = MultiValueManagedVecCounted::new();
        let mut percentages_arg = MultiValueManagedVecCounted::new();
        let mut distribution_types_arg = MultiValueManagedVecCounted::new();

        for address in addresses {
            addresses_arg.push(managed_address!(&AddressValue::from(*address).to_address()));
        }

        for percentage in percentages {
            percentages_arg.push(**percentage);
        }

        for distribution_type in distribution_types {
            distribution_types_arg.push(**distribution_type);
        }

        self.world
            .sc_call(ScCallStep::new().from(address_from).call(
                self.contract.set_distribution_list(
                    addresses_arg,
                    percentages_arg,
                    distribution_types_arg,
                ),
            ));

        self
    }

    pub fn distribute_reward(&mut self, address_from: &str) -> &mut Self {
        self.world.sc_call(
            ScCallStep::new()
                .from(address_from)
                .call(self.contract.distribute()),
        );

        self
    }

    pub fn distribute_reward_and_expect_err(
        &mut self,
        address_from: &str,
        err_msg: &str,
    ) -> &mut Self {
        self.world.sc_call(
            ScCallStep::new()
                .from(address_from)
                .call(self.contract.distribute())
                .expect(TxExpect::user_error(format!("str:{}", err_msg))),
        );

        self
    }

    pub fn check_undistributed_emission(
        &mut self,
        expected_value: BigUint<StaticApi>,
    ) -> &mut Self {
        self.world.sc_query(
            ScQueryStep::new()
                .call(self.contract.get_total_undistributed_amount_view())
                .expect_value(expected_value),
        );

        self
    }

    pub fn check_user_balance(&mut self, address: &str, token: &str, amount: u128) -> &mut Self {
        self.world
            .check_state_step(CheckStateStep::new().put_account(
                address,
                CheckAccount::new().esdt_balance(format!("str:{}", token).as_str(), amount),
            ));

        self
    }

    pub fn set_block_epoch(&mut self, target_epoch: u64) -> &mut Self {
        self.world
            .set_state_step(SetStateStep::new().block_epoch(target_epoch));

        self
    }
}
