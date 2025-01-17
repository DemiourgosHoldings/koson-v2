#![no_std]

#[allow(unused_imports)]
use multiversx_sc::imports::*;

pub mod constants;
pub mod esdt;
pub mod interactors;
pub mod logic;
pub mod storage;
pub mod types;

#[multiversx_sc::contract]
pub trait KosonFactory:
    storage::StorageModule
    + esdt::EsdtModule
    + logic::DistributionLogicModule
    + interactors::koson_factory_chrysopoeic_interactor::KosonFactoryChrysopoeicInteractor
{
    #[init]
    fn init(&self) {}

    #[upgrade]
    fn upgrade(&self) {}

    #[only_owner]
    #[endpoint(initConfig)]
    fn init_config(&self, chrysopoeic_forge_address: ManagedAddress) {
        self.chrysopoeic_forge_address()
            .set(&chrysopoeic_forge_address);
    }

    #[only_owner]
    #[allow_multiple_var_args]
    #[endpoint(setDistributionList)]
    fn set_distribution_list(
        &self,
        addresses: MultiValueManagedVecCounted<ManagedAddress>,
        percentages: MultiValueManagedVecCounted<u64>,
        distribution_types: MultiValueManagedVecCounted<u8>,
    ) -> usize {
        let addresses = addresses.into_vec();
        let percentages = percentages.into_vec();
        let distribution_types = distribution_types.into_vec();

        self.handle_set_distribution_list(addresses, percentages, distribution_types);

        self.distribution_list().len()
    }

    #[endpoint(distribute)]
    fn distribute(&self) {
        self.handle_distribution();
    }

    #[view(getUndistributedAmount)]
    fn get_total_undistributed_amount_view(&self) -> BigUint {
        self.get_total_undistributed_amount()
    }

    #[only_owner]
    #[endpoint(mint)]
    fn admin_mint(&self, amount: BigUint) {
        let token_id = self.factory_token_id().get();
        self.mint_esdt(&token_id, &amount);

        self.current_supply().update(|current_supply| {
            *current_supply += amount;
        });

        let payment = EsdtTokenPayment::new(token_id, 0u64, amount);
        self.send().direct_multi(
            &self.blockchain().get_caller(),
            &ManagedVec::from_single_item(payment),
        );
    }
}
