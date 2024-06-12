use crate::constants::{config::MAX_DISTRIBUTION_LEFTOVER_AMOUNT, errors::ERR_NOT_ALL_DISTRIBUTED};

use super::{
    constants::{
        config::{EMISSION_DENOMINATOR, MAX_PERCENTAGE, MAX_SUPPLY},
        errors::{
            ERR_ALREADY_DISTRIBUTED, ERR_INCORRECT_PERCENTAGE, ERR_INVALID_DISTRIBUTION_TYPE,
        },
    },
    types::{distribution_list_item::DistributionListItem, distribution_type::DistributionType},
};

multiversx_sc::imports!();

#[multiversx_sc::module]
pub trait DistributionLogicModule:
    crate::storage::StorageModule
    + crate::esdt::EsdtModule
    + crate::interactors::soul_staking_interactor::SoulStakingInteractor
    + crate::interactors::land_plot_staking_interactor::LandPlotStakingInteractor
    + crate::interactors::koson_staking_pool_interactor::KosonStakingPoolInteractor
{
    fn handle_set_distribution_list(
        &self,
        addresses: ManagedVec<ManagedAddress>,
        percentages: ManagedVec<u64>,
        distribution_types: ManagedVec<u8>,
    ) {
        let mut total_percentage_sum = 0u64;

        for index in 0..addresses.len() {
            let target_address = addresses.get(index).clone_value();
            let percentage = percentages.get(index);
            let distribution_type = distribution_types.get(index);

            total_percentage_sum += percentage;

            let distribution_item = DistributionListItem {
                target_address,
                percentage,
                distribution_type,
            };

            require!(
                distribution_item.get_distribution_type() != DistributionType::Invalid,
                ERR_INVALID_DISTRIBUTION_TYPE
            );

            self.distribution_list().insert(distribution_item);
        }

        require!(
            MAX_PERCENTAGE == total_percentage_sum,
            ERR_INCORRECT_PERCENTAGE
        );
    }

    fn handle_distribution(&self) {
        let block_epoch = self.blockchain().get_block_epoch();
        let last_distribution_epoch = self.last_distribution_epoch().get();

        let undistributed_amount =
            self.get_total_undistributed_amount(last_distribution_epoch, block_epoch);
        require!(
            undistributed_amount > BigUint::zero(),
            ERR_ALREADY_DISTRIBUTED
        );
        let distribution_token = self.factory_token_id().get();

        self.last_distribution_epoch().set(block_epoch);

        self.mint_esdt(&distribution_token, &undistributed_amount);

        self.distribute_to_list(&distribution_token, &undistributed_amount);
    }

    fn distribute_to_list(&self, token: &TokenIdentifier, total_amount: &BigUint) {
        let distribution_list = self
            .distribution_list()
            .iter()
            .collect::<ManagedVec<Self::Api, DistributionListItem<Self::Api>>>();

        let mut total_distributed = BigUint::zero();

        for distribution_item in distribution_list.iter() {
            total_distributed +=
                self.distribute_to_list_item(token, total_amount, distribution_item);
        }

        require!(
            total_amount - &total_distributed < MAX_DISTRIBUTION_LEFTOVER_AMOUNT,
            ERR_NOT_ALL_DISTRIBUTED
        );
    }

    fn distribute_to_list_item(
        &self,
        distribution_token: &TokenIdentifier,
        total_emission: &BigUint,
        distribution_item: DistributionListItem<Self::Api>,
    ) -> BigUint {
        let distribution_amount =
            &BigUint::from(distribution_item.percentage) * total_emission / MAX_PERCENTAGE;

        self.total_distribution_per_address()
            .entry(distribution_item.target_address.clone())
            .and_modify(|old_value| *old_value += &distribution_amount)
            .or_insert(distribution_amount.clone());

        let distribution_payment = EsdtTokenPayment::new(
            distribution_token.clone(),
            0u64,
            distribution_amount.clone(),
        );

        match distribution_item.get_distribution_type() {
            DistributionType::DirectSend => {
                self.handle_direct_send(&distribution_item.target_address, distribution_payment)
            }
            DistributionType::Invalid => sc_panic!(ERR_INVALID_DISTRIBUTION_TYPE),
            DistributionType::SoulStakingInteraction => self.distribute_soul_staking_rewards(
                distribution_item.target_address,
                distribution_payment,
            ),
            DistributionType::LandPlotStakingInteraction => self
                .distribute_land_plot_staking_rewards(
                    distribution_item.target_address,
                    distribution_payment,
                ),
            DistributionType::KosonStakingInteraction => self
                .distribute_koson_staking_pool_rewards(
                    distribution_item.target_address,
                    distribution_payment,
                ),
        };

        distribution_amount
    }

    fn handle_direct_send(&self, target_address: &ManagedAddress, payment: EsdtTokenPayment) {
        self.send()
            .direct_multi(target_address, &ManagedVec::from_single_item(payment));
    }

    fn get_daily_distribution_amount(&self) -> BigUint {
        let current_supply = self.current_supply().get();

        (BigUint::from(MAX_SUPPLY) - current_supply) / EMISSION_DENOMINATOR
    }

    fn get_total_undistributed_amount(
        &self,
        last_distribution_epoch: u64,
        current_epoch: u64,
    ) -> BigUint {
        let total_epochs = current_epoch - last_distribution_epoch;
        if total_epochs == 0 {
            return BigUint::zero();
        }

        let mut total_undistributed = BigUint::zero();

        for _ in 0..total_epochs {
            let epoch_distribution_amount = self.get_daily_distribution_amount();
            self.current_supply().update(|current_supply| {
                *current_supply += &epoch_distribution_amount;
            });
            total_undistributed += epoch_distribution_amount;
        }

        total_undistributed
    }
}
