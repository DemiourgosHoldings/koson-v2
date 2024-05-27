use super::{
    world, DexPairContract, KosonV2NftStakingContract, KosonV2NftStakingContractState,
    OracleFeedsContract, DEX_SWAP_SC_ADDRESS_EXPR, EGLD_PRICE_FEED_NAME, INVALID_ESDT_TOKEN_ID,
    INVALID_NFT_TOKEN_ID, KOSON_TOKEN_ID, NFT_STAKING_SC_ADDRESS_EXPR, NFT_STAKING_TOKEN_ID,
    ORACLE_SC_ADDRESS_EXPR, OURO_TOKEN_ID, OWNER_ADDRESS_EXPR, USDC_TOKEN_ID, USDD_TOKEN_ID,
    USER_1_ADDRESS_EXPR, WEGLD_TOKEN_ID,
};

use multiversx_sc::types::{Address, EsdtTokenPayment, MultiValueManagedVec};
use multiversx_sc_scenario::{
    api::StaticApi,
    managed_address, managed_biguint, managed_buffer, managed_token_id,
    scenario_model::{
        Account, AddressValue, BigUintValue, BytesValue, CheckAccount, CheckStateStep, ScCallStep,
        ScDeployStep, ScQueryStep, SetStateStep, TxESDT, TxExpect, U64Value,
    },
};

use dex_pair_sc::ProxyTrait as _;
use land_plot_staking::{
    logic::ProxyTrait as _, logic::UnstakeRequest, reward_rate::ProxyTrait as _,
    unstake_fee_calculator::umbrella_interactor::ProxyTrait as _, ProxyTrait as _,
};
use umbrella_oracle_mock::ProxyTrait as _;

impl KosonV2NftStakingContractState {
    pub fn new() -> Self {
        let mut world = world();
        world.set_state_step(
            SetStateStep::new()
                .new_token_identifier(format!("str:{}", INVALID_ESDT_TOKEN_ID))
                .new_token_identifier(format!("str:{}", INVALID_NFT_TOKEN_ID))
                .new_token_identifier(format!("str:{}", NFT_STAKING_TOKEN_ID))
                .new_token_identifier(format!("str:{}", OURO_TOKEN_ID))
                .new_token_identifier(format!("str:{}", USDD_TOKEN_ID))
                .new_token_identifier(format!("str:{}", USDC_TOKEN_ID))
                .new_token_identifier(format!("str:{}", WEGLD_TOKEN_ID))
                .new_token_identifier(format!("str:{}", KOSON_TOKEN_ID))
                .put_account(
                    OWNER_ADDRESS_EXPR,
                    Account::new()
                        .nonce(1)
                        .esdt_balance(
                            format!("str:{}", INVALID_ESDT_TOKEN_ID).as_str(),
                            "1000000000000000000000",
                        )
                        .esdt_balance(
                            format!("str:{}", KOSON_TOKEN_ID).as_str(),
                            "1000000000000000000000",
                        )
                        .esdt_nft_balance(
                            format!("str:{}", NFT_STAKING_TOKEN_ID).as_str(),
                            1,
                            "10000",
                            Option::Some(""),
                        )
                        .esdt_nft_balance(
                            format!("str:{}", NFT_STAKING_TOKEN_ID).as_str(),
                            2,
                            "10000",
                            Option::Some(""),
                        )
                        .esdt_nft_balance(
                            format!("str:{}", NFT_STAKING_TOKEN_ID).as_str(),
                            3,
                            "10000",
                            Option::Some(""),
                        )
                        .esdt_nft_balance(
                            format!("str:{}", NFT_STAKING_TOKEN_ID).as_str(),
                            4,
                            "10000",
                            Option::Some(""),
                        )
                        .esdt_nft_balance(
                            format!("str:{}", NFT_STAKING_TOKEN_ID).as_str(),
                            5,
                            "10000",
                            Option::Some(""),
                        )
                        .esdt_nft_balance(
                            format!("str:{}", INVALID_NFT_TOKEN_ID).as_str(),
                            1,
                            "10000",
                            Option::Some(""),
                        ),
                )
                .new_address(OWNER_ADDRESS_EXPR, 1, NFT_STAKING_SC_ADDRESS_EXPR)
                .put_account(
                    USER_1_ADDRESS_EXPR,
                    Account::new()
                        .nonce(1)
                        .esdt_balance(
                            format!("str:{}", INVALID_ESDT_TOKEN_ID).as_str(),
                            "1000000000000000000000",
                        )
                        .esdt_balance(
                            format!("str:{}", KOSON_TOKEN_ID).as_str(),
                            "1000000000000000000000",
                        )
                        .esdt_nft_balance(
                            format!("str:{}", NFT_STAKING_TOKEN_ID).as_str(),
                            1,
                            "10000",
                            Option::Some(""),
                        )
                        .esdt_nft_balance(
                            format!("str:{}", NFT_STAKING_TOKEN_ID).as_str(),
                            2,
                            "10000",
                            Option::Some(""),
                        )
                        .esdt_nft_balance(
                            format!("str:{}", NFT_STAKING_TOKEN_ID).as_str(),
                            3,
                            "10000",
                            Option::Some(""),
                        )
                        .esdt_nft_balance(
                            format!("str:{}", NFT_STAKING_TOKEN_ID).as_str(),
                            4,
                            "10000",
                            Option::Some(""),
                        )
                        .esdt_nft_balance(
                            format!("str:{}", NFT_STAKING_TOKEN_ID).as_str(),
                            5,
                            "10000",
                            Option::Some(""),
                        )
                        .esdt_nft_balance(
                            format!("str:{}", INVALID_NFT_TOKEN_ID).as_str(),
                            1,
                            "10000",
                            Option::Some(""),
                        ),
                ),
        );

        let owner_address = AddressValue::from(OWNER_ADDRESS_EXPR).to_address();

        let contract = KosonV2NftStakingContract::new(NFT_STAKING_SC_ADDRESS_EXPR);
        let oracle_contract = OracleFeedsContract::new(ORACLE_SC_ADDRESS_EXPR);
        let dex_pair_contract = DexPairContract::new(DEX_SWAP_SC_ADDRESS_EXPR);

        Self {
            world,
            contract,
            oracle_contract,
            dex_pair_contract,
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
            DEX_SWAP_SC_ADDRESS_EXPR,
        ));

        self.world.sc_deploy(
            ScDeployStep::new()
                .from(OWNER_ADDRESS_EXPR)
                .code(self.world.code_expression(DEX_SWAP_SC_ADDRESS_EXPR))
                .call(self.dex_pair_contract.init()),
        );

        self
    }

    pub fn init(&mut self) -> &mut Self {
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
                        managed_token_id!(NFT_STAKING_TOKEN_ID),
                        managed_token_id!(OURO_TOKEN_ID),
                        managed_token_id!(USDD_TOKEN_ID),
                        managed_token_id!(USDC_TOKEN_ID),
                        managed_token_id!(WEGLD_TOKEN_ID),
                        managed_token_id!(KOSON_TOKEN_ID),
                        managed_address!(&AddressValue::from(ORACLE_SC_ADDRESS_EXPR).to_address()),
                    )),
            );

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
                .call(self.contract.stake_land_plots())
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
                .call(self.contract.stake_land_plots())
                .expect(TxExpect::user_error(format!("str:{}", err_msg))),
        );

        self
    }

    pub fn unstake_many(
        &mut self,
        address_from: &str,
        unstake_items: Vec<(u64, u64)>,
        fee_payment: (&str, u64),
        expected_unstake_score: u64,
    ) -> &mut Self {
        self.world.sc_call(
            ScCallStep::new()
                .from(address_from)
                .esdt_transfer(fee_payment.0, 0u64, fee_payment.1)
                .call(
                    self.contract
                        .unstake_land_plots(Self::get_unstake_request(unstake_items)),
                )
                .expect_value(managed_biguint!(expected_unstake_score)),
        );

        self
    }

    pub fn unstake_many_expect_err(
        &mut self,
        address_from: &str,
        unstake_items: Vec<(u64, u64)>,
        fee_payment: (&str, u64),
        err_msg: &str,
    ) -> &mut Self {
        self.world.sc_call(
            ScCallStep::new()
                .from(address_from)
                .esdt_transfer(fee_payment.0, 0u64, fee_payment.1)
                .call(
                    self.contract
                        .unstake_land_plots(Self::get_unstake_request(unstake_items)),
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
                .esdt_transfer(token_id, 0, amount)
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
                .esdt_transfer(token_id, 0, amount)
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
        address: &str,
        nonce: u64,
        expected_epoch: u64,
    ) -> &mut Self {
        self.world.sc_query(
            ScQueryStep::new()
                .call(self.contract.get_stake_epoch(
                    managed_address!(&AddressValue::from(address).to_address()),
                    nonce,
                ))
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

    pub fn set_block_epoch(&mut self, target_epoch: u64) -> &mut Self {
        self.world
            .set_state_step(SetStateStep::new().block_epoch(target_epoch));

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
        vec_data: Vec<(u64, u64)>,
    ) -> MultiValueManagedVec<StaticApi, UnstakeRequest<StaticApi>> {
        let mut requests = MultiValueManagedVec::new();
        for (nonce, amount) in vec_data.iter() {
            requests.push(UnstakeRequest {
                nonce: *nonce,
                amount: managed_biguint!(*amount),
            })
        }
        requests
    }
}
