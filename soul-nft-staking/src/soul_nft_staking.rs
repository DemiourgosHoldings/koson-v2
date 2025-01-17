#![no_std]

use constants::errors::ERR_NOTHING_TO_CLAIM;

multiversx_sc::imports!();
multiversx_sc::derive_imports!();

pub mod constants;
pub mod logic;
pub mod reward_rate;
pub mod storage;
pub mod unstake_fee_calculator;

#[multiversx_sc::contract]
pub trait SoulNftStaking:
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

    #[only_owner]
    #[allow_multiple_var_args]
    #[endpoint(initConfig)]
    fn init_config(
        &self,
        ouro_token_id: TokenIdentifier,
        usdd_token_id: TokenIdentifier,
        usdc_token_id: TokenIdentifier,
        wegld_token_id: TokenIdentifier,
        koson_token_id: TokenIdentifier,
        reward_token_id: TokenIdentifier,
        oracle_registry_address: ManagedAddress,
        death_soul_token_id: TokenIdentifier,
        origin_souls_token_ids: MultiValueManagedVecCounted<TokenIdentifier>,
        summoned_souls_token_ids: MultiValueManagedVec<TokenIdentifier>,
    ) {
        self.ouro_token_id().set(&ouro_token_id);
        self.usdd_token_id().set(&usdd_token_id);
        self.usdc_token_id().set(&usdc_token_id);
        self.wegld_token_id().set(&wegld_token_id);
        self.koson_token_id().set(&koson_token_id);
        self.reward_token_id().set(&reward_token_id);
        self.set_oracle_registry_address(oracle_registry_address);
        self.death_souls_nft_token_id().set(death_soul_token_id);

        for token_id in origin_souls_token_ids.into_vec().iter() {
            self.origin_souls_nft_token_id()
                .insert(token_id.clone_value());
        }
        for token_id in summoned_souls_token_ids.into_vec().iter() {
            self.summoned_souls_nft_token_id()
                .insert(token_id.clone_value());
        }
    }

    #[only_owner]
    #[endpoint(setupScores)]
    #[allow_multiple_var_args]
    fn setup_scores(
        &self,
        token_ids: MultiValueManagedVecCounted<TokenIdentifier>,
        scores: MultiValueManagedVecCounted<BigUint>,
    ) {
        let token_ids = token_ids.into_vec();
        let scores = scores.into_vec();

        for (token_id, score) in token_ids.iter().zip(scores.iter()) {
            self.token_id_score(&token_id).set(score);
        }
    }

    #[payable("*")]
    #[endpoint(stake)]
    fn stake_souls(&self) -> BigUint {
        let caller = self.blockchain().get_caller();

        self.stake_souls_for_user(caller)
    }

    #[payable("*")]
    #[endpoint(stakeForUser)]
    fn stake_souls_for_user(&self, user: ManagedAddress) -> BigUint {
        let payments = self.call_value().all_esdt_transfers();

        self.store_unclaimed_reward(&user);
        self.process_soul_stake_payment(&user, &payments)
    }

    #[payable("*")]
    #[endpoint(unstake)]
    fn unstake_souls(
        &self,
        unstake_request: MultiValueManagedVec<EsdtTokenPayment<Self::Api>>,
    ) -> BigUint {
        let caller = self.blockchain().get_caller();
        let payment = self.call_value().single_esdt();

        self.store_unclaimed_reward(&caller);
        let (mut payments, unstaked_score, fee_usd) =
            self.process_soul_unstake_request(&caller, &unstake_request.into_vec());

        let expected_unstake_fee = self.convert_unstake_fee(fee_usd);
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
        self.user_aggregated_soul_staking_scores(&address).get()
    }

    #[view(getAggregatedScore)]
    fn get_aggregated_score(&self) -> BigUint {
        self.aggregated_soul_staking_scores().get()
    }

    #[view(getStakeEpoch)]
    fn get_stake_epoch(&self, token_id: TokenIdentifier, nonce: u64) -> u64 {
        self.soul_stake_epoch(&token_id, nonce).get()
    }

    #[view(getStakingContext)]
    fn get_staking_context(&self, user: ManagedAddress) -> StakingContext<Self::Api> {
        let user_score = self.user_aggregated_soul_staking_scores(&user).get();
        let aggregated_score = self.get_aggregated_score();

        let mut staked_assets = ManagedVec::new();
        for staked_nft in self.staked_souls(&user).iter() {
            let stake_epoch =
                self.get_stake_epoch(staked_nft.token_identifier.clone(), staked_nft.token_nonce);

            // TODO: implement this on mainnet
            let unstake_fee = BigUint::from(1_000_000_000_000_000_000u64);

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
