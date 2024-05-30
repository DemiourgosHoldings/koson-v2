#![no_std]

#[allow(unused_imports)]
use multiversx_sc::imports::*;

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
    fn init_config(&self, koson_token_identifiers: MultiValueManagedVec<TokenIdentifier>) {
        for token_id in koson_token_identifiers.iter() {
            self.koson_token_ids().insert(token_id.clone_value());
        }
    }

    #[payable("*")]
    #[endpoint(stake)]
    fn stake_koson(&self) -> EsdtTokenPayment {
        let payments = self.call_value().all_esdt_transfers();

        let staked_koson_payment = self.process_stake(&payments);
        self.send_payment_non_zero(staked_koson_payment.clone());

        staked_koson_payment
    }

    #[payable("*")]
    #[endpoint(startUnstake)]
    fn unstake_koson(&self) -> EsdtTokenPayment {
        let payment = self.call_value().single_esdt();
        let unbonding_koson_payment = self.process_unstake(&payment);

        self.send_payment_non_zero(unbonding_koson_payment.clone());

        unbonding_koson_payment
    }

    #[payable("*")]
    #[endpoint(claimUnstaked)]
    fn claim_unstaked(&self) -> ManagedVec<EsdtTokenPayment> {
        let payments = self.call_value().all_esdt_transfers();

        let outgoing_payments = self.process_claim_unstaked(&payments);
        self.send_multi_payments_non_zero(&outgoing_payments);

        outgoing_payments
    }

    #[payable("*")]
    #[endpoint(distributeReward)]
    fn distribute_reward(&self) -> BigUint {
        let payments = self.call_value().all_esdt_transfers();

        self.handle_distribute_rewards(&payments);

        self.get_pool_index()
    }
}
