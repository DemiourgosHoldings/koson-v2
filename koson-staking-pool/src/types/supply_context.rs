multiversx_sc::imports!();
multiversx_sc::derive_imports!();

#[derive(TopDecode, TopEncode, NestedDecode, NestedEncode, TypeAbi)]
pub struct SupplyContext<M: ManagedTypeApi> {
    pub staked_koson_supply: BigUint<M>,
    pub unbonding_koson_supply: BigUint<M>,
    pub koson_locked_in_pool_supplies: ManagedVec<M, EsdtTokenPayment<M>>,
}
