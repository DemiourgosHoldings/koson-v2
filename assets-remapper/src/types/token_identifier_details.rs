multiversx_sc::imports!();
multiversx_sc::derive_imports!();

#[derive(TopEncode, TopDecode, TypeAbi)]
pub struct TokenIdentifierDetails<M: ManagedTypeApi> {
    pub token_identifier: TokenIdentifier<M>,
    pub token_type: EsdtTokenType,
    pub num_decimals: usize,
    pub ratio_numerator: BigUint<M>,
    pub ratio_denominator: BigUint<M>,
}

impl<M: ManagedTypeApi> TokenIdentifierDetails<M> {
    pub fn new(
        token_identifier: TokenIdentifier<M>,
        token_type: EsdtTokenType,
        num_decimals: usize,
        ratio_numerator: BigUint<M>,
        ratio_denominator: BigUint<M>,
    ) -> Self {
        Self {
            token_identifier,
            token_type,
            num_decimals,
            ratio_numerator,
            ratio_denominator,
        }
    }

    pub fn get_swap_amount(&self, amount: &BigUint<M>) -> BigUint<M> {
        &self.ratio_numerator * amount / &self.ratio_denominator
    }
}
