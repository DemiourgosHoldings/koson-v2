#![no_std]

multiversx_sc::imports!();
multiversx_sc::derive_imports!();

#[multiversx_sc::contract]
pub trait UmbrellaOracleMock {
    #[init]
    fn init(&self) {}

    #[view(getAddress)]
    fn get_address(&self, _name: &ManagedBuffer) -> ManagedAddress {
        self.blockchain().get_sc_address()
    }

    #[view(getPrice)]
    fn get_price(&self, key: ManagedBuffer) -> BigUint {
        self.mocked_price_data(key).get()
    }

    #[endpoint(setPrice)]
    fn set_price(&self, feed: ManagedBuffer, price: BigUint) {
        self.mocked_price_data(feed).set(price);
    }

    #[storage_mapper("mocked_price_data")]
    fn mocked_price_data(&self, feed: ManagedBuffer) -> SingleValueMapper<BigUint>;
}

#[derive(TypeAbi, TopEncode, TopDecode)]
pub struct PriceData<M: ManagedTypeApi> {
    pub heartbeat: u32,
    pub timestamp: u32,
    pub price: BigUint<M>,
}
