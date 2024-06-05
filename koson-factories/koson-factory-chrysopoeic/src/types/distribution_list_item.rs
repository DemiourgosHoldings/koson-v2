use super::distribution_type::DistributionType;

multiversx_sc::imports!();
multiversx_sc::derive_imports!();

#[derive(
    TopEncode, TopDecode, NestedDecode, NestedEncode, PartialEq, Eq, TypeAbi, ManagedVecItem,
)]
pub struct DistributionListItem<M: ManagedTypeApi> {
    pub target_address: ManagedAddress<M>,
    pub percentage: u64,
    pub distribution_type: u8,
}

impl<M: ManagedTypeApi> DistributionListItem<M> {
    pub fn get_distribution_type(&self) -> DistributionType {
        match self.distribution_type {
            1 => DistributionType::DirectSend,
            10 => DistributionType::SoulStakingInteraction,
            11 => DistributionType::LandPlotStakingInteraction,
            12 => DistributionType::KosonStakingInteraction,
            _ => DistributionType::Invalid,
        }
    }
}
