multiversx_sc::derive_imports!();

#[derive(TopEncode, TopDecode, NestedDecode, NestedEncode, PartialEq, Eq, TypeAbi)]
pub enum DistributionType {
    Invalid = 0,
    DirectSend = 1,
    SoulStakingInteraction = 10,
    LandPlotStakingInteraction = 11,
    KosonStakingInteraction = 12,
}
