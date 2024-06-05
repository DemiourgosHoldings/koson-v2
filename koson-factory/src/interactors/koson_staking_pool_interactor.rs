multiversx_sc::imports!();

#[multiversx_sc::module]
pub trait KosonStakingPoolInteractor {
    fn distribute_koson_staking_pool_rewards(
        &self,
        address: ManagedAddress,
        payment: EsdtTokenPayment,
    ) {
        let _: IgnoreValue = self
            .staking_pool_interactor_proxy(address)
            .distribute_reward()
            .with_multi_token_transfer(ManagedVec::from_single_item(payment))
            .execute_on_dest_context();
    }

    #[proxy]
    fn staking_pool_interactor_proxy(
        &self,
        address: ManagedAddress,
    ) -> koson_staking_pool::Proxy<Self::Api>;
}
