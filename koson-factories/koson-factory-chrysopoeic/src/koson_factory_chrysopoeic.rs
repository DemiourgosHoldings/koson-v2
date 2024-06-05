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
    + interactors::soul_staking_interactor::SoulStakingInteractor
    + interactors::land_plot_staking_interactor::LandPlotStakingInteractor
    + interactors::koson_staking_pool_interactor::KosonStakingPoolInteractor
{
    #[init]
    fn init(&self) {
        self.last_distribution_epoch()
            .set(self.blockchain().get_block_epoch());
    }

    #[upgrade]
    fn upgrade(&self) {}

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
}
