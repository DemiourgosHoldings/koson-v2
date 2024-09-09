#![no_std]

#[allow(unused_imports)]
use multiversx_sc::imports::*;

pub mod factory_interactor;

#[multiversx_sc::contract]
pub trait KosonFactoryFacade: factory_interactor::FactoryInteractor {
    #[init]
    fn init(&self) {}

    #[upgrade]
    fn upgrade(&self) {}

    #[only_owner]
    #[endpoint(initConfig)]
    fn init_config(
        &self,
        chrysopoeic_forge_address: ManagedAddress,
        universal_forge_address: ManagedAddress,
    ) {
        self.chrysopoeic_forge_address()
            .set(&chrysopoeic_forge_address);
        self.universal_forge_address().set(&universal_forge_address);
    }

    #[endpoint(distribute)]
    fn distribute_factory_rewards(&self) {
        // DO NOT CHANGE THE ORDER OF FACTORY ADDRESS LIST
        // Universal forge rewards are based on the other factories daily emissions.
        // If another factory goes first, the universal forge will distribute 0 rewards for that factory.

        let mut factory_address_list: ManagedVec<Self::Api, ManagedAddress> = ManagedVec::new();
        factory_address_list.push(self.universal_forge_address().get());
        factory_address_list.push(self.chrysopoeic_forge_address().get());

        // only use 95% gas fee for distribution, keep 5% for facade logic execution
        let total_gas_fee = self.blockchain().get_gas_left() * 95 / 100;
        let gas_per_distribution = total_gas_fee / factory_address_list.len() as u64;

        for address in factory_address_list.iter() {
            self.trigger_distribute_rewards(address.clone_value(), gas_per_distribution);
        }
    }

    #[view(getChrysopoeicForgeAddress)]
    #[storage_mapper("chrysopoeic_forge_address")]
    fn chrysopoeic_forge_address(&self) -> SingleValueMapper<ManagedAddress>;

    #[view(getUniversalForgeAddress)]
    #[storage_mapper("universal_forge_address")]
    fn universal_forge_address(&self) -> SingleValueMapper<ManagedAddress>;
}
