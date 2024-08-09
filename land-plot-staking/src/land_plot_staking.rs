#![no_std]

use constants::errors::ERR_NOTHING_TO_CLAIM;
use logic::UnstakeRequest;

multiversx_sc::imports!();
multiversx_sc::derive_imports!();

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
    fn init(&self) {}

    #[upgrade]
    fn upgrade(&self) {}

    #[endpoint(initConfig)]
    fn init_config(
        &self,
        land_plots_token_id: TokenIdentifier,
        ouro_token_id: TokenIdentifier,
        usdd_token_id: TokenIdentifier,
        usdc_token_id: TokenIdentifier,
        wegld_token_id: TokenIdentifier,
        koson_token_id: TokenIdentifier,
        reward_token_id: TokenIdentifier,
        oracle_registry_address: ManagedAddress,
    ) {
        self.land_plot_sft_token_id().set(&land_plots_token_id);
        self.ouro_token_id().set(&ouro_token_id);
        self.usdd_token_id().set(&usdd_token_id);
        self.usdc_token_id().set(&usdc_token_id);
        self.wegld_token_id().set(&wegld_token_id);
        self.koson_token_id().set(&koson_token_id);
        self.reward_token_id().set(&reward_token_id);
        self.set_oracle_registry_address(oracle_registry_address);
    }

    #[payable("*")]
    #[endpoint(stake)]
    fn stake_land_plots(&self) -> BigUint {
        let caller = self.blockchain().get_caller();

        self.stake_land_plots_for_user(caller)
    }

    #[payable("*")]
    #[endpoint(stakeForUser)]
    fn stake_land_plots_for_user(&self, user: ManagedAddress) -> BigUint {
        let payments = self.call_value().all_esdt_transfers();

        self.store_unclaimed_reward(&user);
        self.process_land_plot_stake_payment(&user, &payments)
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
        let (mut payments, unstaked_score, fee_in_usd) =
            self.process_land_plot_unstake_request(&caller, &unstake_request.into_vec());

        let expected_unstake_fee = self.convert_unstake_fee(fee_in_usd);
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
    fn claim_rewards(&self) -> EsdtTokenPayment {
        let caller = self.blockchain().get_caller();

        self.store_unclaimed_reward(&caller);

        let unclaimed_rewards = self.user_unclaimed_rewards(&caller).get();
        require!(unclaimed_rewards > 0, ERR_NOTHING_TO_CLAIM);

        self.user_unclaimed_rewards(&caller).set(BigUint::zero());

        let reward_token_id = self.reward_token_id().get();
        self.send()
            .direct_esdt(&caller, &reward_token_id, 0u64, &unclaimed_rewards);

        EsdtTokenPayment::new(reward_token_id, 0u64, unclaimed_rewards)
    }

    #[payable("*")]
    #[endpoint(distributeRewards)]
    fn distribute_rewards(&self) {
        let payment = self.call_value().single_esdt();

        self.handle_distribute_rewards(&payment);
    }

    #[view(getUserScore)]
    fn get_user_score(&self, address: ManagedAddress) -> BigUint {
        self.user_aggregated_land_plot_scores(&address).get()
    }

    #[view(getAggregatedScore)]
    fn get_aggregated_score(&self) -> BigUint {
        self.aggregated_land_plot_scores().get()
    }

    #[view(getStakeEpoch)]
    fn get_stake_epoch(&self, user: ManagedAddress, nonce: u64) -> u64 {
        self.stake_epoch(&user, nonce).get()
    }

    #[view(getStakingContext)]
    fn get_staking_context(&self, user: ManagedAddress) -> StakingContext<Self::Api> {
        let user_score = self.user_aggregated_land_plot_scores(&user).get();
        let aggregated_score = self.get_aggregated_score();
        let land_plot_token_id = self.land_plot_sft_token_id().get();

        let mut staked_assets = ManagedVec::new();
        for nonce in 1..=5u64 {
            let stake_storage = self.staked_land_plots(&user, nonce);
            if stake_storage.is_empty() {
                continue;
            }
            let stake_epoch = self.stake_epoch(&user, nonce).get();

            // TODO: implement this on mainnet
            let unstake_fee = BigUint::from(1_000_000_000_000_000_000u64);
            let staked_nft =
                EsdtTokenPayment::new(land_plot_token_id.clone(), nonce, stake_storage.get());

            staked_assets.push(StakedNftViewType {
                nft: staked_nft,
                stake_epoch,
                unstake_fee,
            });
        }

        let pending_rewards = self.get_total_unclaimed_reward(user.clone());

        StakingContext {
            user_score,
            aggregated_score,
            staked_assets,
            pending_rewards: ManagedVec::from_single_item(EsdtTokenPayment::new(
                self.reward_token_id().get(),
                0u64,
                pending_rewards,
            )),
        }
    }
}

#[derive(TopEncode, TopDecode, NestedEncode, NestedDecode, TypeAbi)]
pub struct StakingContext<M: ManagedTypeApi> {
    pub user_score: BigUint<M>,
    pub aggregated_score: BigUint<M>,
    pub staked_assets: ManagedVec<M, StakedNftViewType<M>>,
    pub pending_rewards: ManagedVec<M, EsdtTokenPayment<M>>,
}

#[derive(TopEncode, TopDecode, NestedEncode, NestedDecode, TypeAbi, ManagedVecItem)]
pub struct StakedNftViewType<M: ManagedTypeApi> {
    nft: EsdtTokenPayment<M>,
    stake_epoch: u64,
    unstake_fee: BigUint<M>,
}
