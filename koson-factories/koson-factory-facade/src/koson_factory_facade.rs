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
    #[endpoint(addFactoryAddresses)]
    fn add_factory_addresses(&self, addresses: MultiValueManagedVec<ManagedAddress>) {
        for address in addresses.iter() {
            self.factory_address_list().insert(address.clone_value());
        }
    }

    #[only_owner]
    #[endpoint(removeFactoryAddresses)]
    fn remove_factory_addresses(&self, addresses: MultiValueManagedVec<ManagedAddress>) {
        for address in addresses.iter() {
            self.factory_address_list().remove(&address);
        }
    }

    #[endpoint(distribute)]
    fn distribute_factory_rewards(&self) {
        // only use 95% gas fee for distribution, keep 5% for facade logic execution
        let total_gas_fee = self.blockchain().get_gas_left() * 95 / 100;
        let gas_per_distribution = total_gas_fee / self.factory_address_list().len() as u64;

        for address in self.factory_address_list().iter() {
            self.trigger_distribute_rewards(address, gas_per_distribution);
        }
    }

    #[view(getFactoryAddressList)]
    #[storage_mapper("factory_address_list")]
    fn factory_address_list(&self) -> SetMapper<ManagedAddress>;
}
