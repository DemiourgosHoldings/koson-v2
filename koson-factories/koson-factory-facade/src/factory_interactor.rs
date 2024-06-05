multiversx_sc::imports!();

#[multiversx_sc::module]
pub trait FactoryInteractor {
    fn trigger_distribute_rewards(&self, factory_address: ManagedAddress, gas_limit: u64) {
        let _: IgnoreValue = self
            .factory_interactor_proxy(factory_address)
            .distribute()
            .with_gas_limit(gas_limit)
            .execute_on_dest_context();
    }

    #[proxy]
    fn factory_interactor_proxy(
        &self,
        address: ManagedAddress,
    ) -> koson_factory_chrysopoeic::Proxy<Self::Api>;
}
