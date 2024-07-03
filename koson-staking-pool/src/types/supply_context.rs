multiversx_sc::imports!();
multiversx_sc::derive_imports!();

#[derive(TopDecode, TopEncode, NestedDecode, NestedEncode, TypeAbi)]
pub struct StakingPoolContext<M: ManagedTypeApi> {
    pub reward_index: BigUint<M>,
    pub token_balances: ManagedVec<M, EsdtTokenPayment<M>>,
    pub max_claim_fee: BigUint<M>,
    pub unbonding_epochs: u64,
    pub staked_koson_token_identifier: TokenIdentifier<M>,
    pub unbonding_koson_token_identifier: TokenIdentifier<M>,
}
