use crate::constants::{
    errors::{ERR_NOT_A_LAND_PLOT, ERR_NOT_A_REWARD, ERR_NOT_ENOUGH_STAKED},
    score::LAND_PLOT_SCORES,
};

multiversx_sc::imports!();
multiversx_sc::derive_imports!();

#[multiversx_sc::module]
pub trait LogicModule:
    crate::storage::StorageModule + crate::reward_rate::RewardRateModule
{
    fn store_unclaimed_reward(&self, caller: &ManagedAddress) {
        let unclaimed_reward_rate = self.update_user_reward_rate(caller);
        let user_score = self.user_aggregated_land_plot_scores(caller).get();

        self.user_unclaimed_rewards(caller)
            .set(&unclaimed_reward_rate * &user_score);
    }

    fn process_land_plot_stake_payment(
        &self,
        caller: &ManagedAddress,
        payments: &ManagedVec<EsdtTokenPayment>,
    ) -> BigUint {
        let land_plot_sft_token_id = self.land_plot_sft_token_id().get();
        let mut payments_score = BigUint::zero();

        for payment in payments.iter() {
            self.require_payment_is_token_id(
                &payment,
                &land_plot_sft_token_id,
                ERR_NOT_A_LAND_PLOT,
            );

            let payment_nonce_score = LAND_PLOT_SCORES[payment.token_nonce as usize - 1];
            let payment_score = &BigUint::from(payment_nonce_score) * &payment.amount;

            self.staked_land_plots(caller, payment.token_nonce)
                .update(|old_amount| *old_amount += &payment.amount);

            payments_score += payment_score;
        }

        self.user_aggregated_land_plot_scores(caller)
            .update(|old_score| *old_score += &payments_score);
        self.aggregated_land_plot_scores()
            .update(|old_score| *old_score += &payments_score);

        payments_score
    }

    fn process_land_plot_unstake_request(
        &self,
        caller: &ManagedAddress,
        unstake_request: &ManagedVec<UnstakeRequest<Self::Api>>,
    ) -> (ManagedVec<EsdtTokenPayment>, BigUint) {
        let land_plot_sft_token_id = self.land_plot_sft_token_id().get();
        let mut unstaked_payments = ManagedVec::new();
        let mut total_unstake_amount = BigUint::zero();

        for request in unstake_request.iter() {
            let payment = request.amount.clone();
            let payment_nonce = request.nonce;

            let staked_amount = self.staked_land_plots(caller, payment_nonce).get();
            require!(staked_amount >= payment, ERR_NOT_ENOUGH_STAKED);

            self.staked_land_plots(caller, payment_nonce)
                .update(|old_amount| *old_amount -= &payment);

            total_unstake_amount += &payment;

            unstaked_payments.push(EsdtTokenPayment::new(
                land_plot_sft_token_id.clone(),
                payment_nonce,
                payment,
            ));
        }

        self.user_aggregated_land_plot_scores(caller)
            .update(|old_score| *old_score -= &total_unstake_amount);
        self.aggregated_land_plot_scores()
            .update(|old_score| *old_score -= &total_unstake_amount);

        (unstaked_payments, total_unstake_amount)
    }

    fn handle_distribute_rewards(&self, payment: &EsdtTokenPayment) {
        let reward_token_id = self.reward_token_id().get();
        self.require_payment_is_token_id(payment, &reward_token_id, ERR_NOT_A_REWARD);

        let distribution_rate = &payment.amount / &self.aggregated_land_plot_scores().get();
        self.increment_reward_rate(distribution_rate);
    }

    fn require_payment_is_token_id(
        &self,
        payment: &EsdtTokenPayment,
        target_token_id: &TokenIdentifier,
        err_msg: &str,
    ) {
        require!(&payment.token_identifier == target_token_id, err_msg);
    }
}

#[derive(TopEncode, TopDecode, TypeAbi, ManagedVecItem)]
pub struct UnstakeRequest<M: ManagedTypeApi> {
    pub nonce: u64,
    pub amount: BigUint<M>,
}
