multiversx_sc::imports!();

#[multiversx_sc::module]
pub trait SoulStakingInteractor {
    fn distribute_soul_staking_rewards(&self, address: ManagedAddress, payment: EsdtTokenPayment) {
        let _: IgnoreValue = self
            .soul_staking_interactor_proxy(address)
            .distribute_rewards()
            .with_multi_token_transfer(ManagedVec::from_single_item(payment))
            .execute_on_dest_context();
    }

    #[proxy]
    fn soul_staking_interactor_proxy(
        &self,
        address: ManagedAddress,
    ) -> land_plot_staking::Proxy<Self::Api>;
}
