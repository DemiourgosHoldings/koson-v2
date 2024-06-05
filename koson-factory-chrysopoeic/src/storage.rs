multiversx_sc::imports!();

use crate::types::distribution_list_item::DistributionListItem;

#[multiversx_sc::module]
pub trait StorageModule {
    #[view(getDistributionList)]
    #[storage_mapper("distribution_list")]
    fn distribution_list(&self) -> SetMapper<DistributionListItem<Self::Api>>;

    #[view(getLastDistributionEpoch)]
    #[storage_mapper("last_distribution_epoch")]
    fn last_distribution_epoch(&self) -> SingleValueMapper<u64>;

    #[view(getCurrentSupply)]
    #[storage_mapper("current_supply")]
    fn current_supply(&self) -> SingleValueMapper<BigUint>;

    #[view(getFactoryTokenIdentifier)]
    #[storage_mapper("factory_token_id")]
    fn factory_token_id(&self) -> SingleValueMapper<TokenIdentifier>;

    #[view(getTotalDistributionPerAddress)]
    #[storage_mapper("total_distribution_per_address")]
    fn total_distribution_per_address(&self) -> MapMapper<ManagedAddress<Self::Api>, BigUint>;
}
