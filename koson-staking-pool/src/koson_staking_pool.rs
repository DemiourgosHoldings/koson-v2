#![no_std]

use constants::config::POOL_INDEX_DENOMINATOR;
#[allow(unused_imports)]
use multiversx_sc::imports::*;
use types::supply_context::SupplyContext;

pub mod constants;
pub mod esdt;
pub mod logic;
pub mod reward_rate;
pub mod storage;
pub mod types;

#[multiversx_sc::contract]
pub trait KosonStakingPool:
    esdt::EsdtModule + storage::StorageModule + logic::LogicModule + reward_rate::RewardRateModule
{
    #[init]
    fn init(&self) {}

    #[upgrade]
    fn upgrade(&self) {}

    #[only_owner]
    #[endpoint(initConfig)]
    fn init_config(
        &self,
        unbonding_time_penalty: u64,
        koson_token_identifiers: MultiValueManagedVec<TokenIdentifier>,
    ) {
        for token_id in koson_token_identifiers.iter() {
            self.koson_token_ids().insert(token_id.clone_value());
            self.unbonding_time_penalty().set(unbonding_time_penalty);
        }
    }

    #[payable("*")]
    #[endpoint(stake)]
    fn stake_koson(&self) -> EsdtTokenPayment {
        let caller = self.blockchain().get_caller();

        self.stake_koson_for_user(caller)
    }

    #[payable("*")]
    #[endpoint(stakeForUser)]
    fn stake_koson_for_user(&self, user: ManagedAddress) -> EsdtTokenPayment {
        let payments = self.call_value().all_esdt_transfers();

        let staked_koson_payment = self.process_stake(&payments);
        self.send_payment_non_zero(&user, staked_koson_payment.clone());

        staked_koson_payment
    }

    #[payable("*")]
    #[endpoint(startUnstake)]
    fn unstake_koson(&self) -> EsdtTokenPayment {
        let caller = self.blockchain().get_caller();
        let payment = self.call_value().single_esdt();

        let unbonding_koson_payment = self.process_unstake(&payment);

        self.send_payment_non_zero(&caller, unbonding_koson_payment.clone());

        unbonding_koson_payment
    }

    #[payable("*")]
    #[endpoint(claimUnstaked)]
    fn claim_unstaked(&self) -> ManagedVec<EsdtTokenPayment> {
        let caller = self.blockchain().get_caller();
        let payments = self.call_value().all_esdt_transfers();

        let outgoing_payments = self.process_claim_unstaked(&payments);
        self.send_multi_payments_non_zero(&caller, &outgoing_payments);

        outgoing_payments
    }

    #[payable("*")]
    #[endpoint(distributeReward)]
    fn distribute_reward(&self) -> BigUint {
        let payments = self.call_value().all_esdt_transfers();

        self.handle_distribute_rewards(&payments);

        self.get_pool_index()
    }

    #[view(getPoolIndex)]
    fn get_pool_index_view(&self) -> (BigUint, BigUint) {
        (self.get_pool_index(), BigUint::from(POOL_INDEX_DENOMINATOR))
    }

    #[view(getSupplyContext)]
    fn supply_context(&self) -> SupplyContext<Self::Api> {
        todo!()
    }

    #[view(getStorageKosonSupply)]
    fn get_storage_koson_supply(&self, koson_token_id: TokenIdentifier) -> BigUint {
        self.koson_supply(&koson_token_id).get()
    }

    #[view(getStorageStakedKosonSupply)]
    fn get_storage_staked_koson_supply(&self, staked_koson_token_id: TokenIdentifier) -> BigUint {
        self.staked_koson_supply(&staked_koson_token_id).get()
    }
}
