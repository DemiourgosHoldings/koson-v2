use super::{
    world, KosonIndexStakingPoolContract, KosonStakingPoolState, INITIAL_ESDT_BALANCE,
    INVALID_ESDT_TOKEN_ID, KOSON_ANCIENT_TOKEN_ID, KOSON_ESOTERIC_TOKEN_ID,
    KOSON_PRIMORDIAL_TOKEN_ID, KOSON_REWARD_BEARING_TOKEN, KOSON_STAKING_SC_ADDRESS,
    KOSON_UNBONDING_META_TOKEN, OWNER_ADDRESS_EXPR, UNBONDING_TIME_PENALTY, USER_1_ADDRESS_EXPR,
};

use multiversx_sc::types::{EsdtTokenPayment, ManagedVec, MultiValueManagedVec};
use multiversx_sc_scenario::{
    api::StaticApi,
    managed_address, managed_biguint, managed_token_id,
    scenario_model::{
        Account, AddressValue, BigUintValue, BytesValue, CheckAccount, CheckStateStep, ScCallStep,
        ScDeployStep, ScQueryStep, SetStateStep, TxESDT, TxExpect, U64Value,
    },
};

use koson_staking_pool::{
    constants::config::{POOL_INDEX_DENOMINATOR, STAKED_KOSON_KEY, UNBONDING_KOSON_KEY},
    esdt::ProxyTrait as _,
    ProxyTrait as _,
};

impl KosonStakingPoolState {
    pub fn new() -> Self {
        let mut world = world();
        world.set_state_step(
            SetStateStep::new()
                .new_token_identifier(format!("str:{}", KOSON_ESOTERIC_TOKEN_ID))
                .new_token_identifier(format!("str:{}", KOSON_PRIMORDIAL_TOKEN_ID))
                .new_token_identifier(format!("str:{}", KOSON_ANCIENT_TOKEN_ID))
                .new_token_identifier(format!("str:{}", KOSON_REWARD_BEARING_TOKEN))
                .put_account(
                    OWNER_ADDRESS_EXPR,
                    Account::new()
                        .nonce(1)
                        .esdt_balance(
                            format!("str:{}", KOSON_ESOTERIC_TOKEN_ID).as_str(),
                            INITIAL_ESDT_BALANCE,
                        )
                        .esdt_balance(
                            format!("str:{}", KOSON_PRIMORDIAL_TOKEN_ID).as_str(),
                            INITIAL_ESDT_BALANCE,
                        )
                        .esdt_balance(
                            format!("str:{}", KOSON_ANCIENT_TOKEN_ID).as_str(),
                            INITIAL_ESDT_BALANCE,
                        )
                        .esdt_balance(
                            format!("str:{}", INVALID_ESDT_TOKEN_ID).as_str(),
                            INITIAL_ESDT_BALANCE,
                        ),
                )
                .new_address(OWNER_ADDRESS_EXPR, 1, KOSON_STAKING_SC_ADDRESS)
                .put_account(
                    USER_1_ADDRESS_EXPR,
                    Account::new()
                        .nonce(1)
                        .esdt_balance(
                            format!("str:{}", KOSON_ESOTERIC_TOKEN_ID).as_str(),
                            INITIAL_ESDT_BALANCE,
                        )
                        .esdt_balance(
                            format!("str:{}", KOSON_PRIMORDIAL_TOKEN_ID).as_str(),
                            INITIAL_ESDT_BALANCE,
                        )
                        .esdt_balance(
                            format!("str:{}", KOSON_ANCIENT_TOKEN_ID).as_str(),
                            INITIAL_ESDT_BALANCE,
                        )
                        .esdt_balance(
                            format!("str:{}", INVALID_ESDT_TOKEN_ID).as_str(),
                            INITIAL_ESDT_BALANCE,
                        ),
                ),
        );

        let contract = KosonIndexStakingPoolContract::new(KOSON_STAKING_SC_ADDRESS);

        Self { world, contract }
    }

    pub fn deploy(&mut self) -> &mut Self {
        let code = self.world.code_expression(KOSON_STAKING_SC_ADDRESS);

        let esdt_roles = vec![
            "ESDTRoleLocalBurn".to_string(),
            "ESDTRoleLocalMint".to_string(),
            "ESDTRoleNFTCreate".to_string(),
            "ESDTRoleNFTBurn".to_string(),
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
                format!("str:{}", KOSON_REWARD_BEARING_TOKEN).as_str(),
                esdt_roles.clone(),
            )
            .esdt_roles(
                format!("str:{}", KOSON_UNBONDING_META_TOKEN).as_str(),
                esdt_roles.clone(),
            );

        self.world.set_state_step(
            SetStateStep::new()
                .new_address(OWNER_ADDRESS_EXPR, 2, KOSON_STAKING_SC_ADDRESS)
                .put_account(KOSON_STAKING_SC_ADDRESS, acc),
        );

        self
    }

    pub fn init(&mut self) -> &mut Self {
        let mut koson_token_identifiers = MultiValueManagedVec::new();
        koson_token_identifiers.push(managed_token_id!(KOSON_ESOTERIC_TOKEN_ID));
        koson_token_identifiers.push(managed_token_id!(KOSON_PRIMORDIAL_TOKEN_ID));
        koson_token_identifiers.push(managed_token_id!(KOSON_ANCIENT_TOKEN_ID));

        self.world.sc_call(
            ScCallStep::new().from(OWNER_ADDRESS_EXPR).call(
                self.contract
                    .init_config(UNBONDING_TIME_PENALTY, koson_token_identifiers),
            ),
        );

        self.world
            .sc_call(
                ScCallStep::new()
                    .from(OWNER_ADDRESS_EXPR)
                    .call(self.contract.set_token_id(
                        managed_token_id!(KOSON_REWARD_BEARING_TOKEN),
                        STAKED_KOSON_KEY,
                    )),
            );

        self.world
            .sc_call(
                ScCallStep::new()
                    .from(OWNER_ADDRESS_EXPR)
                    .call(self.contract.set_token_id(
                        managed_token_id!(KOSON_UNBONDING_META_TOKEN),
                        UNBONDING_KOSON_KEY,
                    )),
            );

        self
    }

    pub fn stake_many(
        &mut self,
        address_from: &str,
        payments: Vec<(&str, u64)>,
        expected_payment: EsdtTokenPayment<StaticApi>,
    ) -> &mut Self {
        self.world.sc_call(
            ScCallStep::new()
                .from(address_from)
                .multi_esdt_transfer(Self::get_txesdt_vec(payments))
                .call(self.contract.stake_koson())
                .expect_value(expected_payment),
        );

        self
    }

    pub fn stake_many_for_user(
        &mut self,
        address_from: &str,
        target_user: &str,
        payments: Vec<(&str, u64)>,
        expected_payment: EsdtTokenPayment<StaticApi>,
    ) -> &mut Self {
        self.world.sc_call(
            ScCallStep::new()
                .from(address_from)
                .multi_esdt_transfer(Self::get_txesdt_vec(payments))
                .call(self.contract.stake_koson_for_user(managed_address!(
                    &AddressValue::from(target_user).to_address()
                )))
                .expect_value(expected_payment),
        );

        self
    }

    pub fn stake_many_expect_err(
        &mut self,
        address_from: &str,
        payments: Vec<(&str, u64)>,
        err_msg: &str,
    ) -> &mut Self {
        self.world.sc_call(
            ScCallStep::new()
                .from(address_from)
                .multi_esdt_transfer(Self::get_txesdt_vec(payments))
                .call(self.contract.stake_koson())
                .expect(TxExpect::user_error(format!("str:{}", err_msg))),
        );

        self
    }

    pub fn stake_many_unchecked(
        &mut self,
        address_from: &str,
        payments: Vec<(&str, u64)>,
    ) -> &mut Self {
        self.world.sc_call(
            ScCallStep::new()
                .from(address_from)
                .multi_esdt_transfer(Self::get_txesdt_vec(payments))
                .call(self.contract.stake_koson()),
        );

        self
    }

    pub fn unstake(
        &mut self,
        address_from: &str,
        unstake_amount: u64,
        expected_payment: EsdtTokenPayment<StaticApi>,
    ) -> &mut Self {
        self.world.sc_call(
            ScCallStep::new()
                .from(address_from)
                .esdt_transfer(
                    format!("str:{}", KOSON_REWARD_BEARING_TOKEN),
                    0u64,
                    unstake_amount,
                )
                .call(self.contract.unstake_koson())
                .expect_value(expected_payment),
        );

        self
    }

    pub fn _unstake_expect_err(
        &mut self,
        address_from: &str,
        unstake_amount: u64,
        err_msg: &str,
    ) -> &mut Self {
        self.world.sc_call(
            ScCallStep::new()
                .from(address_from)
                .esdt_transfer(
                    format!("str:{}", KOSON_REWARD_BEARING_TOKEN),
                    0u64,
                    unstake_amount,
                )
                .call(self.contract.unstake_koson())
                .expect(TxExpect::user_error(format!("str:{}", err_msg))),
        );

        self
    }

    pub fn unstake_unchecked(&mut self, address_from: &str, unstake_amount: u64) -> &mut Self {
        self.world.sc_call(
            ScCallStep::new()
                .from(address_from)
                .esdt_transfer(
                    format!("str:{}", KOSON_REWARD_BEARING_TOKEN),
                    0u64,
                    unstake_amount,
                )
                .call(self.contract.unstake_koson()),
        );

        self
    }

    pub fn claim_unstaked(
        &mut self,
        address_from: &str,
        payments: Vec<(&str, u64, u64)>,
        expected_payments: ManagedVec<StaticApi, EsdtTokenPayment<StaticApi>>,
    ) -> &mut Self {
        self.world.sc_call(
            ScCallStep::new()
                .from(address_from)
                .multi_esdt_transfer(Self::get_meta_txesdt_vec(payments))
                .call(self.contract.claim_unstaked())
                .expect_value(expected_payments),
        );

        self
    }

    pub fn _claim_unstaked_unchecked(
        &mut self,
        address_from: &str,
        payments: Vec<(&str, u64, u64)>,
    ) -> &mut Self {
        self.world.sc_call(
            ScCallStep::new()
                .from(address_from)
                .multi_esdt_transfer(Self::get_meta_txesdt_vec(payments))
                .call(self.contract.claim_unstaked()),
        );

        self
    }

    pub fn _claim_unstaked_expect_err(
        &mut self,
        address_from: &str,
        payments: Vec<(&str, u64, u64)>,
        err_msg: &str,
    ) -> &mut Self {
        self.world.sc_call(
            ScCallStep::new()
                .from(address_from)
                .multi_esdt_transfer(Self::get_meta_txesdt_vec(payments))
                .call(self.contract.claim_unstaked())
                .expect(TxExpect::user_error(format!("str:{}", err_msg))),
        );

        self
    }

    pub fn distribute_many_rewards(
        &mut self,
        address_from: &str,
        payments: Vec<(&str, u64)>,
    ) -> &mut Self {
        self.world.sc_call(
            ScCallStep::new()
                .from(address_from)
                .multi_esdt_transfer(Self::get_txesdt_vec(payments))
                .call(self.contract.distribute_reward()),
        );

        self
    }

    pub fn _distribute_rewards_expect_err(
        &mut self,
        address_from: &str,
        payments: Vec<(&str, u64)>,
        err_msg: &str,
    ) -> &mut Self {
        self.world.sc_call(
            ScCallStep::new()
                .from(address_from)
                .multi_esdt_transfer(Self::get_txesdt_vec(payments))
                .call(self.contract.distribute_reward())
                .expect(TxExpect::user_error(format!("str:{}", err_msg))),
        );

        self
    }

    pub fn check_current_index(&mut self, expected_value: u64) -> &mut Self {
        self.world.sc_query(
            ScQueryStep::new()
                .call(self.contract.get_pool_index_view())
                .expect_value((
                    managed_biguint!(expected_value),
                    managed_biguint!(POOL_INDEX_DENOMINATOR),
                )),
        );

        self
    }

    pub fn check_koson_supply(&mut self, koson_token_id: &str, expected_value: u64) -> &mut Self {
        self.world.sc_query(
            ScQueryStep::new()
                .call(
                    self.contract
                        .get_storage_koson_supply(managed_token_id!(koson_token_id)),
                )
                .expect_value(managed_biguint!(expected_value)),
        );

        self
    }

    pub fn check_staked_koson_supply(
        &mut self,
        staked_koson_token_id: &str,
        expected_value: u64,
    ) -> &mut Self {
        self.world.sc_query(
            ScQueryStep::new()
                .call(
                    self.contract
                        .get_storage_staked_koson_supply(managed_token_id!(staked_koson_token_id)),
                )
                .expect_value(managed_biguint!(expected_value)),
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

    pub fn _check_user_nft_balance(
        &mut self,
        address: &str,
        token: &str,
        nonce: u64,
        amount: u128,
    ) -> &mut Self {
        self.world
            .check_state_step(CheckStateStep::new().put_account(
                address,
                CheckAccount::new().esdt_nft_balance_and_attributes(
                    format!("str:{}", token).as_str(),
                    nonce,
                    amount,
                    Option::Some(""),
                ),
            ));

        self
    }

    pub fn set_block_epoch(&mut self, target_epoch: u64) -> &mut Self {
        self.world
            .set_state_step(SetStateStep::new().block_epoch(target_epoch));

        self
    }

    fn get_txesdt_vec(vec_data: Vec<(&str, u64)>) -> Vec<TxESDT> {
        let mut payments = vec![];
        for (token_id, amount) in vec_data.iter() {
            payments.push(TxESDT {
                esdt_token_identifier: BytesValue::from(format!("str:{}", *token_id)),
                nonce: U64Value::zero(),
                esdt_value: BigUintValue::from(*amount),
            })
        }
        payments
    }

    fn get_meta_txesdt_vec(vec_data: Vec<(&str, u64, u64)>) -> Vec<TxESDT> {
        let mut payments = vec![];
        for (token_id, nonce, amount) in vec_data.iter() {
            payments.push(TxESDT {
                esdt_token_identifier: BytesValue::from(format!("str:{}", *token_id)),
                nonce: U64Value::from(*nonce),
                esdt_value: BigUintValue::from(*amount),
            })
        }
        payments
    }
}
