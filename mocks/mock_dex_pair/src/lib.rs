#![no_std]

multiversx_sc::imports!();
multiversx_sc::derive_imports!();

#[multiversx_sc::contract]
pub trait DexPairScContract {
    #[init]
    fn init(&self) {}

    #[view(getEquivalent)]
    fn get_equivalent(&self, token_in: TokenIdentifier, amount_in: BigUint) -> BigUint {
        amount_in * self.rates(token_in).get()
    }

    #[payable("*")]
    #[endpoint(swapTokensFixedInput)]
    fn swap_tokens_fixed_input(
        &self,
        token_out: TokenIdentifier,
        _amount_out_min: BigUint,
    ) -> SwapResultType<Self::Api> {
        let payment_in = self.call_value().single_esdt();

        // used just to convert VST into OURO
        let amount_out =
            payment_in.amount.clone() / self.rates(payment_in.token_identifier.clone()).get();

        self.send().direct(
            &self.blockchain().get_caller(),
            &EgldOrEsdtTokenIdentifier::esdt(token_out.clone()),
            0,
            &amount_out,
        );
        SwapResultType {
            total_fee: BigUint::zero(),
            special_fee: BigUint::zero(),
            payment_out: EsdtTokenPayment::new(token_out, 0, amount_out),
            payment_in,
            refund_amount_in: BigUint::zero(),
        }
    }

    #[payable("*")]
    #[endpoint(injectFunds)]
    fn inject_funds(&self) {}

    #[endpoint(setRate)]
    fn set_rate(&self, token_id: TokenIdentifier, rate: BigUint) {
        self.rates(token_id).set(rate);
    }

    #[storage_mapper("rates")]
    fn rates(&self, token_id: TokenIdentifier) -> SingleValueMapper<BigUint>;
}

#[derive(
    TopEncode, TopDecode, NestedEncode, NestedDecode, PartialEq, TypeAbi, Clone, ManagedVecItem,
)]
pub struct SwapResultType<M: ManagedTypeApi> {
    pub total_fee: BigUint<M>,
    pub special_fee: BigUint<M>,
    pub payment_in: EsdtTokenPayment<M>,
    pub payment_out: EsdtTokenPayment<M>,
    pub refund_amount_in: BigUint<M>,
}
