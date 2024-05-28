#![no_std]

multiversx_sc::imports!();

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
        self.reward_token_id().set(&koson_token_id);
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

    #[payable("*")]
    #[endpoint(stake)]
    fn stake_souls(&self) -> BigUint {
        let caller = self.blockchain().get_caller();
        let payments = self.call_value().all_esdt_transfers();

        self.store_unclaimed_reward(&caller);
        self.process_soul_stake_payment(&caller, &payments)
    }

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
