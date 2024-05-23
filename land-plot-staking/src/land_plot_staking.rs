#![no_std]

use logic::UnstakeRequest;

multiversx_sc::imports!();

pub mod constants;
pub mod logic;
pub mod reward_rate;
pub mod storage;
pub mod unstake_fee_calculator;

#[multiversx_sc::contract]
pub trait LandPlotStaking:
    storage::StorageModule
    + logic::LogicModule
    + reward_rate::RewardRateModule
    + unstake_fee_calculator::calculator::UnstakeFeeCalculator
    + unstake_fee_calculator::dex_pair_interactor::DexPairInteractorModule
    + unstake_fee_calculator::umbrella_interactor::UmbrellaInteractorModule
{
    #[init]
    fn init(&self, land_plots_token_id: TokenIdentifier) {
        self.land_plot_sft_token_id().set(&land_plots_token_id);
    }

    #[payable("*")]
    #[endpoint(stake)]
    fn stake_land_plots(&self) -> BigUint {
        let caller = self.blockchain().get_caller();
        let payments = self.call_value().all_esdt_transfers();

        self.store_unclaimed_reward(&caller);
        self.process_land_plot_stake_payment(&caller, &payments)
    }

    #[payable("*")]
    #[endpoint(unstake)]
    fn unstake_land_plots(
        &self,
        unstake_request: MultiValueManagedVec<UnstakeRequest<Self::Api>>,
    ) -> BigUint {
        let caller = self.blockchain().get_caller();
        let payment = self.call_value().single_esdt();

        self.store_unclaimed_reward(&caller);
        let (mut payments, unstaked_score) =
            self.process_land_plot_unstake_request(&caller, &unstake_request.into_vec());

        let expected_unstake_fee = self.calculate_unstake_fee_usdc(unstaked_score.clone());
        self.require_payment_is_token_id(
            &payment,
            &self.koson_token_id().get(),
            "Expected unstake fee payment in KOSON",
        );

        require!(
            payment.amount >= expected_unstake_fee,
            "Insufficient unstake fee payment"
        );

        let fee_diff = &payment.amount - &expected_unstake_fee;

        if fee_diff > 0 {
            payments.push(EsdtTokenPayment::new(
                payment.token_identifier,
                0u64,
                fee_diff,
            ));
        }

        self.send().direct_multi(&caller, &payments);

        unstaked_score
    }

    #[endpoint(claimRewards)]
    fn claim_rewards(&self) {
        let caller = self.blockchain().get_caller();

        self.store_unclaimed_reward(&caller);

        let unclaimed_rewards = self.user_unclaimed_rewards(&caller).get();

        self.user_unclaimed_rewards(&caller).set(BigUint::zero());
        self.send().direct_esdt(
            &caller,
            &self.reward_token_id().get(),
            0u64,
            &unclaimed_rewards,
        );
    }

    #[payable("*")]
    #[endpoint(distributeRewards)]
    fn distribute_rewards(&self) {
        let payment = self.call_value().single_esdt();

        self.handle_distribute_rewards(&payment);
    }
}
