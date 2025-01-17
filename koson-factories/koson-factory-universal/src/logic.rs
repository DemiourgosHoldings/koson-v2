use crate::constants::{config::MAX_DISTRIBUTION_LEFTOVER_AMOUNT, errors::ERR_NOT_ALL_DISTRIBUTED};

use super::{
    constants::{
        config::{MAX_PERCENTAGE, PERCENTAGE_OF_CHRYSOPOEIC_FORGE_SUPPLY},
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
    + crate::interactors::koson_factory_chrysopoeic_interactor::KosonFactoryChrysopoeicInteractor
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
            total_percentage_sum == MAX_PERCENTAGE,
            ERR_INCORRECT_PERCENTAGE
        );
    }

    fn handle_distribution(&self) {
        let block_epoch = self.blockchain().get_block_epoch();

        let undistributed_amount = self.get_total_undistributed_amount();
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
        };

        distribution_amount
    }

    fn handle_direct_send(&self, target_address: &ManagedAddress, payment: EsdtTokenPayment) {
        self.send()
            .direct_multi(target_address, &ManagedVec::from_single_item(payment));
    }

    fn get_total_undistributed_amount(&self) -> BigUint {
        let undistributed_chrysopoiec_forge_amount = self
            .get_undistributed_chrysopoeic_forge_emission_amount(
                self.chrysopoeic_forge_address().get(),
            );

        undistributed_chrysopoiec_forge_amount * PERCENTAGE_OF_CHRYSOPOEIC_FORGE_SUPPLY
            / MAX_PERCENTAGE
    }
}
