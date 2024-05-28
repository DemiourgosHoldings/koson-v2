use super::{
    world, DexPairContract, KosonV2NftStakingContract, KosonV2NftStakingContractState,
    OracleFeedsContract, DEATH_SOUL_TOKEN_ID, DEX_OURO_KOSON_PAIR_ADDRESS_EXPR,
    DEX_OURO_USDC_PAIR_ADDRESS_EXPR, DEX_OURO_USDD_PAIR_ADDRESS_EXPR,
    DEX_OURO_WEGLD_PAIR_ADDRESS_EXPR, EGLD_PRICE_FEED_NAME, INITIAL_ESDT_BALANCE,
    INVALID_ESDT_TOKEN_ID, INVALID_NFT_TOKEN_ID, KOSON_TOKEN_ID, NFT_STAKING_SC_ADDRESS_EXPR,
    ORACLE_SC_ADDRESS_EXPR, ORIGIN_SOULS_TOKEN_IDS, OURO_TOKEN_ID, OWNER_ADDRESS_EXPR,
    SUMMONED_ORIGIN_SOULS_TOKEN_IDS, USDC_TOKEN_ID, USDD_TOKEN_ID, USER_1_ADDRESS_EXPR,
    WEGLD_TOKEN_ID,
};

use multiversx_sc::types::{
    Address, BigUint, EsdtTokenPayment, MultiValueManagedVec, MultiValueManagedVecCounted,
};
use multiversx_sc_scenario::{
    api::StaticApi,
    managed_address, managed_biguint, managed_buffer, managed_token_id,
    scenario_model::{
        Account, AddressValue, BigUintValue, BytesValue, CheckAccount, CheckStateStep, ScCallStep,
        ScDeployStep, ScQueryStep, SetStateStep, TxESDT, TxExpect, U64Value,
    },
};

use dex_pair_sc::ProxyTrait as _;
use soul_nft_staking::{
    logic::ProxyTrait as _,
    reward_rate::ProxyTrait as _,
    unstake_fee_calculator::{
        calculator::ProxyTrait as _, dex_pair_interactor::ProxyTrait as _,
        umbrella_interactor::ProxyTrait as _,
    },
    ProxyTrait as _,
};
use umbrella_oracle_mock::ProxyTrait as _;

impl KosonV2NftStakingContractState {
    pub fn new() -> Self {
        let mut world = world();
        world.set_state_step(
            SetStateStep::new()
                .new_token_identifier(format!("str:{}", INVALID_ESDT_TOKEN_ID))
                .new_token_identifier(format!("str:{}", INVALID_NFT_TOKEN_ID))
                .new_token_identifier(format!("str:{}", OURO_TOKEN_ID))
                .new_token_identifier(format!("str:{}", USDD_TOKEN_ID))
                .new_token_identifier(format!("str:{}", USDC_TOKEN_ID))
                .new_token_identifier(format!("str:{}", WEGLD_TOKEN_ID))
                .new_token_identifier(format!("str:{}", KOSON_TOKEN_ID))
                .new_token_identifier(format!("str:{}", ORIGIN_SOULS_TOKEN_IDS[0]))
                .new_token_identifier(format!("str:{}", ORIGIN_SOULS_TOKEN_IDS[1]))
                .new_token_identifier(format!("str:{}", ORIGIN_SOULS_TOKEN_IDS[2]))
                .new_token_identifier(format!("str:{}", ORIGIN_SOULS_TOKEN_IDS[3]))
                .new_token_identifier(format!("str:{}", ORIGIN_SOULS_TOKEN_IDS[4]))
                .new_token_identifier(format!("str:{}", ORIGIN_SOULS_TOKEN_IDS[5]))
                .new_token_identifier(format!("str:{}", SUMMONED_ORIGIN_SOULS_TOKEN_IDS[0]))
                .new_token_identifier(format!("str:{}", SUMMONED_ORIGIN_SOULS_TOKEN_IDS[1]))
                .new_token_identifier(format!("str:{}", SUMMONED_ORIGIN_SOULS_TOKEN_IDS[2]))
                .new_token_identifier(format!("str:{}", SUMMONED_ORIGIN_SOULS_TOKEN_IDS[3]))
                .new_token_identifier(format!("str:{}", SUMMONED_ORIGIN_SOULS_TOKEN_IDS[4]))
                .new_token_identifier(format!("str:{}", SUMMONED_ORIGIN_SOULS_TOKEN_IDS[5]))
                .new_token_identifier(format!("str:{}", DEATH_SOUL_TOKEN_ID))
                .put_account(OWNER_ADDRESS_EXPR, Account::new().nonce(1))
                .new_address(OWNER_ADDRESS_EXPR, 1, NFT_STAKING_SC_ADDRESS_EXPR)
                .put_account(USER_1_ADDRESS_EXPR, Account::new().nonce(1)),
        );

        let owner_address = AddressValue::from(OWNER_ADDRESS_EXPR).to_address();

        let contract = KosonV2NftStakingContract::new(NFT_STAKING_SC_ADDRESS_EXPR);
        let oracle_contract = OracleFeedsContract::new(ORACLE_SC_ADDRESS_EXPR);
        let dex_pair_ouro_koson_contract = DexPairContract::new(DEX_OURO_KOSON_PAIR_ADDRESS_EXPR);
        let dex_pair_ouro_usdd_contract = DexPairContract::new(DEX_OURO_USDD_PAIR_ADDRESS_EXPR);
        let dex_pair_ouro_usdc_contract = DexPairContract::new(DEX_OURO_USDC_PAIR_ADDRESS_EXPR);
        let dex_pair_ouro_wegld_contract = DexPairContract::new(DEX_OURO_WEGLD_PAIR_ADDRESS_EXPR);

        Self {
            world,
            contract,
            oracle_contract,
            dex_pair_ouro_koson_contract,
            dex_pair_ouro_usdd_contract,
            dex_pair_ouro_usdc_contract,
            dex_pair_ouro_wegld_contract,
            owner_address,
        }
    }

    pub fn deploy(&mut self) -> &mut Self {
        let code = self.world.code_expression(NFT_STAKING_SC_ADDRESS_EXPR);
        self.world.sc_deploy(
            ScDeployStep::new()
                .from(OWNER_ADDRESS_EXPR)
                .code(code.clone())
                .call(self.contract.init()),
        );

        let acc = Account::new().owner(OWNER_ADDRESS_EXPR).code(code);

        self.world.set_state_step(
            SetStateStep::new()
                .new_address(OWNER_ADDRESS_EXPR, 2, ORACLE_SC_ADDRESS_EXPR)
                .put_account(NFT_STAKING_SC_ADDRESS_EXPR, acc),
        );

        self.world.sc_deploy(
            ScDeployStep::new()
                .from(OWNER_ADDRESS_EXPR)
                .code(self.world.code_expression(ORACLE_SC_ADDRESS_EXPR))
                .call(self.oracle_contract.init()),
        );

        self.world.set_state_step(SetStateStep::new().new_address(
            OWNER_ADDRESS_EXPR,
            3,
            DEX_OURO_KOSON_PAIR_ADDRESS_EXPR,
        ));
        self.world.set_state_step(SetStateStep::new().new_address(
            OWNER_ADDRESS_EXPR,
            4,
            DEX_OURO_USDD_PAIR_ADDRESS_EXPR,
        ));
        self.world.set_state_step(SetStateStep::new().new_address(
            OWNER_ADDRESS_EXPR,
            5,
            DEX_OURO_USDC_PAIR_ADDRESS_EXPR,
        ));
        self.world.set_state_step(SetStateStep::new().new_address(
            OWNER_ADDRESS_EXPR,
            6,
            DEX_OURO_WEGLD_PAIR_ADDRESS_EXPR,
        ));

        self.world.sc_deploy(
            ScDeployStep::new()
                .from(OWNER_ADDRESS_EXPR)
                .code(self.world.code_expression(DEX_OURO_KOSON_PAIR_ADDRESS_EXPR))
                .call(self.dex_pair_ouro_koson_contract.init()),
        );
        self.world.sc_deploy(
            ScDeployStep::new()
                .from(OWNER_ADDRESS_EXPR)
                .code(self.world.code_expression(DEX_OURO_USDD_PAIR_ADDRESS_EXPR))
                .call(self.dex_pair_ouro_usdd_contract.init()),
        );
        self.world.sc_deploy(
            ScDeployStep::new()
                .from(OWNER_ADDRESS_EXPR)
                .code(self.world.code_expression(DEX_OURO_USDC_PAIR_ADDRESS_EXPR))
                .call(self.dex_pair_ouro_usdc_contract.init()),
        );
        self.world.sc_deploy(
            ScDeployStep::new()
                .from(OWNER_ADDRESS_EXPR)
                .code(self.world.code_expression(DEX_OURO_WEGLD_PAIR_ADDRESS_EXPR))
                .call(self.dex_pair_ouro_wegld_contract.init()),
        );

        self
    }

    fn init_balances(&mut self) -> &mut Self {
        let mut owner_acc = Account::new()
            .esdt_balance(
                format!("str:{}", INVALID_ESDT_TOKEN_ID).as_str(),
                INITIAL_ESDT_BALANCE,
            )
            .esdt_balance(
                format!("str:{}", KOSON_TOKEN_ID).as_str(),
                INITIAL_ESDT_BALANCE,
            );
        let mut user_acc = Account::new()
            .esdt_balance(
                format!("str:{}", INVALID_ESDT_TOKEN_ID).as_str(),
                INITIAL_ESDT_BALANCE,
            )
            .esdt_balance(
                format!("str:{}", KOSON_TOKEN_ID).as_str(),
                INITIAL_ESDT_BALANCE,
            );

        let mut token_ids = [ORIGIN_SOULS_TOKEN_IDS, SUMMONED_ORIGIN_SOULS_TOKEN_IDS].concat();
        token_ids.push(DEATH_SOUL_TOKEN_ID);

        for token_id in token_ids.iter() {
            for user_nonce in 1..=50u64 {
                user_acc = user_acc.esdt_nft_balance(
                    format!("str:{}", token_id).as_str(),
                    user_nonce,
                    "1",
                    Some(""),
                );

                owner_acc = owner_acc.esdt_nft_balance(
                    format!("str:{}", token_id).as_str(),
                    user_nonce + 50,
                    "1",
                    Some(""),
                );
            }
        }

        self.world
            .set_state_step(SetStateStep::new().put_account(USER_1_ADDRESS_EXPR, user_acc))
            .set_state_step(SetStateStep::new().put_account(OWNER_ADDRESS_EXPR, owner_acc));

        self
    }

    pub fn init(&mut self) -> &mut Self {
        self.init_balances();

        let mut origin_soul_token_ids = MultiValueManagedVecCounted::new();
        for token_id in ORIGIN_SOULS_TOKEN_IDS.iter() {
            origin_soul_token_ids.push(managed_token_id!(*token_id));
        }

        let mut summoned_soul_token_ids = MultiValueManagedVec::new();
        for token_id in SUMMONED_ORIGIN_SOULS_TOKEN_IDS.iter() {
            summoned_soul_token_ids.push(managed_token_id!(*token_id));
        }

        let feeds = vec![(WEGLD_TOKEN_ID, EGLD_PRICE_FEED_NAME)];

        for (token_id, feed_name) in feeds {
            self.world.sc_call(
                ScCallStep::new().from(OWNER_ADDRESS_EXPR).call(
                    self.contract
                        .set_price_feed(managed_token_id!(token_id), managed_buffer!(feed_name)),
                ),
            );
        }

        self.world
            .sc_call(
                ScCallStep::new()
                    .from(OWNER_ADDRESS_EXPR)
                    .call(self.contract.init_config(
                        managed_token_id!(OURO_TOKEN_ID),
                        managed_token_id!(USDD_TOKEN_ID),
                        managed_token_id!(USDC_TOKEN_ID),
                        managed_token_id!(WEGLD_TOKEN_ID),
                        managed_token_id!(KOSON_TOKEN_ID),
                        managed_address!(&AddressValue::from(ORACLE_SC_ADDRESS_EXPR).to_address()),
                        managed_token_id!(DEATH_SOUL_TOKEN_ID),
                        origin_soul_token_ids,
                        summoned_soul_token_ids,
                    )),
            );

        let dex_pair_config = [
            (
                DEX_OURO_KOSON_PAIR_ADDRESS_EXPR,
                OURO_TOKEN_ID,
                KOSON_TOKEN_ID,
            ),
            (
                DEX_OURO_USDD_PAIR_ADDRESS_EXPR,
                OURO_TOKEN_ID,
                USDD_TOKEN_ID,
            ),
            (
                DEX_OURO_USDC_PAIR_ADDRESS_EXPR,
                OURO_TOKEN_ID,
                USDC_TOKEN_ID,
            ),
            (
                DEX_OURO_WEGLD_PAIR_ADDRESS_EXPR,
                OURO_TOKEN_ID,
                WEGLD_TOKEN_ID,
            ),
        ];

        for pair_config in dex_pair_config.iter() {
            let dex_pair_address = AddressValue::from(pair_config.0).to_address();
            self.world
                .sc_call(ScCallStep::new().from(OWNER_ADDRESS_EXPR).call(
                    self.contract.set_pair_info(
                        managed_token_id!(pair_config.1),
                        managed_token_id!(pair_config.2),
                        managed_address!(&dex_pair_address),
                    ),
                ));
        }

        self
    }

    pub fn stake_many(
        &mut self,
        address_from: &str,
        payments: Vec<(&str, u64, u64)>,
        expected_stake_score: u64,
    ) -> &mut Self {
        self.world.sc_call(
            ScCallStep::new()
                .from(address_from)
                .multi_esdt_transfer(Self::get_txesdt_vec(payments))
                .call(self.contract.stake_souls())
                .expect_value(managed_biguint!(expected_stake_score)),
        );

        self
    }

    pub fn stake_many_expect_err(
        &mut self,
        address_from: &str,
        payments: Vec<(&str, u64, u64)>,
        err_msg: &str,
    ) -> &mut Self {
        self.world.sc_call(
            ScCallStep::new()
                .from(address_from)
                .multi_esdt_transfer(Self::get_txesdt_vec(payments))
                .call(self.contract.stake_souls())
                .expect(TxExpect::user_error(format!("str:{}", err_msg))),
        );

        self
    }

    pub fn unstake_many(
        &mut self,
        address_from: &str,
        unstake_items: Vec<(&str, u64, u64)>,
        fee_payment: (&str, u64),
        expected_unstake_score: u64,
    ) -> &mut Self {
        self.world.sc_call(
            ScCallStep::new()
                .from(address_from)
                .esdt_transfer(format!("str:{}", fee_payment.0), 0u64, fee_payment.1)
                .call(
                    self.contract
                        .unstake_souls(Self::get_unstake_request(unstake_items)),
                )
                .expect_value(managed_biguint!(expected_unstake_score)),
        );

        self
    }

    pub fn unstake_many_expect_err(
        &mut self,
        address_from: &str,
        unstake_items: Vec<(&str, u64, u64)>,
        fee_payment: (&str, u64),
        err_msg: &str,
    ) -> &mut Self {
        self.world.sc_call(
            ScCallStep::new()
                .from(address_from)
                .esdt_transfer(format!("str:{}", fee_payment.0), 0u64, fee_payment.1)
                .call(
                    self.contract
                        .unstake_souls(Self::get_unstake_request(unstake_items)),
                )
                .expect(TxExpect::user_error(format!("str:{}", err_msg))),
        );

        self
    }

    pub fn claim_rewards(&mut self, address_from: &str, expected_reward: u64) -> &mut Self {
        self.world.sc_call(
            ScCallStep::new()
                .from(address_from)
                .call(self.contract.claim_rewards())
                .expect_value(EsdtTokenPayment::<StaticApi>::new(
                    managed_token_id!(KOSON_TOKEN_ID),
                    0u64,
                    managed_biguint!(expected_reward),
                )),
        );

        self
    }

    pub fn claim_rewards_expect_err(&mut self, address_from: &str, err_msg: &str) -> &mut Self {
        self.world.sc_call(
            ScCallStep::new()
                .from(address_from)
                .call(self.contract.claim_rewards())
                .expect(TxExpect::user_error(format!("str:{}", err_msg))),
        );

        self
    }

    pub fn distribute_rewards(
        &mut self,
        address_from: &str,
        token_id: &str,
        amount: u64,
    ) -> &mut Self {
        self.world.sc_call(
            ScCallStep::new()
                .from(address_from)
                .esdt_transfer(format!("str:{}", token_id), 0, amount)
                .call(self.contract.distribute_rewards()),
        );

        self
    }

    pub fn distribute_rewards_expect_err(
        &mut self,
        address_from: &str,
        token_id: &str,
        amount: u64,
        err_msg: &str,
    ) -> &mut Self {
        self.world.sc_call(
            ScCallStep::new()
                .from(address_from)
                .esdt_transfer(format!("str:{}", token_id), 0, amount)
                .call(self.contract.distribute_rewards())
                .expect(TxExpect::user_error(format!("str:{}", err_msg))),
        );

        self
    }

    pub fn check_pending_reward(&mut self, address: &str, expected_value: u64) -> &mut Self {
        self.world.sc_query(
            ScQueryStep::new()
                .call(self.contract.get_total_unclaimed_reward(managed_address!(
                    &AddressValue::from(address).to_address()
                )))
                .expect_value(managed_biguint!(expected_value)),
        );

        self
    }

    pub fn check_current_reward_rate(&mut self, expected_value: u64) -> &mut Self {
        self.world.sc_query(
            ScQueryStep::new()
                .call(self.contract.get_current_reward_rate())
                .expect_value(managed_biguint!(expected_value)),
        );

        self
    }

    pub fn check_last_claimed_reward_rate(
        &mut self,
        address: &str,
        expected_value: u64,
    ) -> &mut Self {
        self.world.sc_query(
            ScQueryStep::new()
                .call(self.contract.get_last_claimed_reward_rate(managed_address!(
                    &AddressValue::from(address).to_address()
                )))
                .expect_value(managed_biguint!(expected_value)),
        );

        self
    }

    pub fn check_unstake_fee_per_score_in_koson(&mut self, expected_fee: u64) -> &mut Self {
        self.world.sc_query(
            ScQueryStep::new()
                .call(self.contract.get_unstake_fee_per_score_koson())
                .expect_value(managed_biguint!(expected_fee)),
        );

        self
    }

    pub fn set_oracle_feed_price(&mut self, feed: &[u8], price: u64) -> &mut Self {
        self.world.sc_call(
            ScCallStep::new().from(&self.owner_address).call(
                self.oracle_contract
                    .set_price(feed, managed_biguint!(price)),
            ),
        );

        self
    }

    pub fn check_oracle_registry_address(&mut self, expected_value: &Address) -> &mut Self {
        self.world.sc_query(
            ScQueryStep::new()
                .call(self.contract.get_umbrella_registry_addr())
                .expect_value(managed_address!(expected_value)),
        );

        self
    }

    pub fn check_oracle_feed_price(&mut self, token_id: &str, expected_value: u64) -> &mut Self {
        self.world.sc_query(
            ScQueryStep::new()
                .call(self.contract.get_oracle_price(managed_token_id!(token_id)))
                .expect_value(managed_biguint!(expected_value)),
        );

        self
    }

    pub fn check_user_score(&mut self, address: &str, expected_score: u64) -> &mut Self {
        self.world.sc_query(
            ScQueryStep::new()
                .call(
                    self.contract.get_user_score(managed_address!(
                        &AddressValue::from(address).to_address()
                    )),
                )
                .expect_value(managed_biguint!(expected_score)),
        );

        self
    }

    pub fn check_total_aggregated_score(&mut self, expected_score: u64) -> &mut Self {
        self.world.sc_query(
            ScQueryStep::new()
                .call(self.contract.get_aggregated_score())
                .expect_value(managed_biguint!(expected_score)),
        );

        self
    }

    pub fn check_stake_epoch(
        &mut self,
        token_id: &str,
        nonce: u64,
        expected_epoch: u64,
    ) -> &mut Self {
        self.world.sc_query(
            ScQueryStep::new()
                .call(
                    self.contract
                        .get_stake_epoch(managed_token_id!(token_id), nonce),
                )
                .expect_value(expected_epoch),
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

    pub fn check_user_nft_balance(
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

    pub fn set_exchange_rate(&mut self, token: &str, rate: BigUint<StaticApi>) -> &mut Self {
        self.world.sc_call(
            ScCallStep::new()
                .from(OWNER_ADDRESS_EXPR)
                .call(match token {
                    KOSON_TOKEN_ID => self
                        .dex_pair_ouro_koson_contract
                        .set_rate(managed_token_id!(OURO_TOKEN_ID), rate),
                    USDD_TOKEN_ID => self
                        .dex_pair_ouro_usdd_contract
                        .set_rate(managed_token_id!(OURO_TOKEN_ID), rate),
                    USDC_TOKEN_ID => self
                        .dex_pair_ouro_usdc_contract
                        .set_rate(managed_token_id!(OURO_TOKEN_ID), rate),
                    WEGLD_TOKEN_ID => self
                        .dex_pair_ouro_wegld_contract
                        .set_rate(managed_token_id!(OURO_TOKEN_ID), rate),
                    _ => panic!("Invalid token id"),
                }),
        );

        self
    }

    pub fn check_get_equivalent_vesta_dex(
        &mut self,
        from_token_id: &str,
        to_token_id: &str,
        amount: BigUint<StaticApi>,
        expected_amount: BigUint<StaticApi>,
    ) -> &mut Self {
        self.world.sc_query(
            ScQueryStep::new()
                .call(self.contract.get_equivalent_vesta_dex(
                    managed_token_id!(from_token_id),
                    managed_token_id!(to_token_id),
                    amount,
                ))
                .expect_value(expected_amount),
        );

        self
    }

    pub fn check_get_equivalent_xexchange(
        &mut self,
        from_token_id: &str,
        to_token_id: &str,
        amount: BigUint<StaticApi>,
        expected_amount: BigUint<StaticApi>,
    ) -> &mut Self {
        self.world.sc_query(
            ScQueryStep::new()
                .call(self.contract.get_equivalent_xexchange(
                    managed_token_id!(from_token_id),
                    managed_token_id!(to_token_id),
                    amount,
                ))
                .expect_value(expected_amount),
        );

        self
    }

    fn get_txesdt_vec(vec_data: Vec<(&str, u64, u64)>) -> Vec<TxESDT> {
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

    fn get_unstake_request(
        vec_data: Vec<(&str, u64, u64)>,
    ) -> MultiValueManagedVec<StaticApi, EsdtTokenPayment<StaticApi>> {
        let mut unstake_request = MultiValueManagedVec::new();
        for (token_id, nonce, amount) in vec_data.iter() {
            unstake_request.push(EsdtTokenPayment::new(
                managed_token_id!(*token_id),
                *nonce,
                managed_biguint!(*amount),
            ));
        }
        unstake_request
    }
}
