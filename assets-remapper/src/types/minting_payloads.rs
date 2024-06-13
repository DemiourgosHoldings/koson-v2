multiversx_sc::imports!();
multiversx_sc::derive_imports!();

#[derive(TopEncode, TopDecode, NestedEncode, NestedDecode, TypeAbi, ManagedVecItem)]
pub struct MintNftSftPayload<A: ManagedTypeApi> {
    pub amount: BigUint<A>,
    pub name: ManagedBuffer<A>,
    pub royalties: BigUint<A>,
    pub hash: ManagedBuffer<A>,
    pub attributes: ManagedBuffer<A>,
    pub uris: ManagedVec<A, ManagedBuffer<A>>,
}
