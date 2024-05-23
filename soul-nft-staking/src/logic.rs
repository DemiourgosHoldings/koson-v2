use crate::constants::errors::{ERR_NOT_A_REWARD, ERR_NOT_A_SOUL, ERR_NOT_ENOUGH_STAKED};

multiversx_sc::imports!();
multiversx_sc::derive_imports!();

#[multiversx_sc::module]
pub trait LogicModule:
    crate::storage::StorageModule + crate::reward_rate::RewardRateModule
{
    fn store_unclaimed_reward(&self, caller: &ManagedAddress) {
        let unclaimed_reward_rate = self.update_user_reward_rate(caller);
        let user_score = self.user_aggregated_soul_staking_scores(caller).get();

        self.user_unclaimed_rewards(caller)
            .set(&unclaimed_reward_rate * &user_score);
    }

    fn process_soul_stake_payment(
        &self,
        caller: &ManagedAddress,
        payments: &ManagedVec<EsdtTokenPayment>,
    ) -> BigUint {
        let mut payments_score = BigUint::zero();

        for payment in payments.iter() {
            let soul_payment_score = self.get_payment_score_or_fail(&payment.token_identifier);

            self.staked_souls(caller).insert(payment.clone());

            payments_score += soul_payment_score;
        }

        self.user_aggregated_soul_staking_scores(caller)
            .update(|old_score| *old_score += &payments_score);
        self.aggregated_soul_staking_scores()
            .update(|old_score| *old_score += &payments_score);

        payments_score
    }

    fn process_soul_unstake_request(
        &self,
        caller: &ManagedAddress,
        unstake_request: &ManagedVec<EsdtTokenPayment>,
    ) -> (ManagedVec<EsdtTokenPayment>, BigUint) {
        let mut unstaked_payments = ManagedVec::new();
        let mut total_unstake_amount = BigUint::zero();
        let mut staked_souls = self.staked_souls(caller);

        for request in unstake_request.iter() {
            require!(staked_souls.remove(&request), ERR_NOT_ENOUGH_STAKED);

            let soul_payment_score = self.get_payment_score_or_fail(&request.token_identifier);

            total_unstake_amount += &soul_payment_score;

            unstaked_payments.push(request);
        }

        self.user_aggregated_soul_staking_scores(caller)
            .update(|old_score| *old_score -= &total_unstake_amount);
        self.aggregated_soul_staking_scores()
            .update(|old_score| *old_score -= &total_unstake_amount);

        (unstaked_payments, total_unstake_amount)
    }

    fn handle_distribute_rewards(&self, payment: &EsdtTokenPayment) {
        let reward_token_id = self.reward_token_id().get();
        self.require_payment_is_token_id(payment, &reward_token_id, ERR_NOT_A_REWARD);

        let distribution_rate = &payment.amount / &self.aggregated_soul_staking_scores().get();
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

    fn get_payment_score_or_fail(&self, target_token_id: &TokenIdentifier) -> BigUint {
        require!(
            !self.token_id_score(target_token_id).is_empty(),
            ERR_NOT_A_SOUL
        );

        self.token_id_score(target_token_id).get()
    }
}
