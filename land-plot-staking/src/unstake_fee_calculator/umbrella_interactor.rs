multiversx_sc::imports!();

pub const UMBRELLA_FEEDS_NAME: &[u8] = b"UmbrellaFeeds";
pub const ORACLE_PRICE_DENOMINATION: u64 = 100_000_000;

#[multiversx_sc::module]
pub trait UmbrellaInteractorModule {
    #[only_owner]
    #[endpoint(setUmbrellaRegistryAddress)]
    fn set_oracle_registry_address(&self, address: ManagedAddress) {
        self.umbrella_oracle_registry_sc_address().set(address);
    }

    #[only_owner]
    #[endpoint(setUmbrellaPriceFeed)]
    fn set_price_feed(&self, token_identifier: TokenIdentifier, feed_name: ManagedBuffer) {
        self.token_identifier_feed_mapping(&token_identifier)
            .set(feed_name);
    }

    #[view(getUmbrellaPrice)]
    fn get_oracle_price(&self, token_identifier: &TokenIdentifier) -> BigUint {
        let feeds_address = self.get_feeds_address();

        let key = self.token_identifier_feed_mapping(token_identifier).get();

        self.umbrella_feeds_proxy(feeds_address)
            .get_price(key)
            .execute_on_dest_context()
    }

    #[view(getUmbrellaFeedsAddress)]
    fn get_feeds_address(&self) -> ManagedAddress {
        self.umbrella_registry_proxy(self.umbrella_oracle_registry_sc_address().get())
            .get_address(ManagedBuffer::from(UMBRELLA_FEEDS_NAME))
            .execute_on_dest_context::<ManagedAddress>()
    }

    #[view(getUmbrellaRegistryAddress)]
    #[storage_mapper("umbrella_oracle_sc_address")]
    fn umbrella_oracle_registry_sc_address(&self) -> SingleValueMapper<ManagedAddress>;

    #[view(getTokenIdentifierFeedMapping)]
    #[storage_mapper("token_identifier_feed_mapping")]
    fn token_identifier_feed_mapping(
        &self,
        token_identifier: &TokenIdentifier,
    ) -> SingleValueMapper<ManagedBuffer>;

    #[proxy]
    fn umbrella_registry_proxy(
        &self,
        address: ManagedAddress,
    ) -> umbrella_registry_proxy::Proxy<Self::Api>;

    #[proxy]
    fn umbrella_feeds_proxy(
        &self,
        address: ManagedAddress,
    ) -> umbrella_feeds_proxy::Proxy<Self::Api>;
}

pub mod umbrella_registry_proxy {
    multiversx_sc::imports!();

    #[multiversx_sc::proxy]
    pub trait RegistryProxy {
        #[view(getAddress)]
        fn get_address(&self, name: &ManagedBuffer) -> ManagedAddress;
    }
}

pub mod umbrella_feeds_proxy {
    multiversx_sc::imports!();
    multiversx_sc::derive_imports!();

    #[derive(TypeAbi, TopEncode, TopDecode)]
    pub struct PriceData<M: ManagedTypeApi> {
        pub heartbeat: u32,
        pub timestamp: u32,
        pub price: BigUint<M>,
    }

    #[multiversx_sc::proxy]
    pub trait FeedsProxy {
        #[view(getPriceData)]
        fn get_price_data(&self, key: ManagedBuffer) -> PriceData<Self::Api>;

        #[view(getPrice)]
        fn get_price(&self, key: ManagedBuffer) -> BigUint;
    }
}
