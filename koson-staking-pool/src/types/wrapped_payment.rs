use crate::constants::config::{UNBONDING_FEE_DENOMINATOR, UNBONDING_MAX_FEE};

multiversx_sc::imports!();
multiversx_sc::derive_imports!();

#[derive(TopEncode, TopDecode, TypeAbi)]
pub struct WrappedPayment {
    pub mint_epoch: u64,
}

impl WrappedPayment {
    pub fn new(mint_epoch: u64) -> Self {
        Self { mint_epoch }
    }

    // returns (remaining amount, fee)
    pub fn compute_fee<M: ManagedTypeApi>(
        &self,
        amount: &BigUint<M>,
        unbonding_time_penalty: u64,
        current_block_epoch: u64,
    ) -> (BigUint<M>, BigUint<M>) {
        let no_fee_epoch = self.mint_epoch + unbonding_time_penalty;
        if current_block_epoch >= no_fee_epoch {
            return (amount.clone(), BigUint::zero());
        }

        let feeable_epochs = no_fee_epoch - current_block_epoch;
        // let fee = amount * feeable_epochs / unbonding_time_penalty * UNBONDING_MAX_FEE
        //     / UNBONDING_FEE_DENOMINATOR;

        let max_fee = amount * UNBONDING_MAX_FEE / UNBONDING_FEE_DENOMINATOR;

        let fee = max_fee * feeable_epochs / unbonding_time_penalty;

        (amount - &fee, fee)
    }
}
