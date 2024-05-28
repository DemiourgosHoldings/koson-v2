multiversx_sc::imports!();

#[multiversx_sc::module]
pub trait DexPairInteractorModule {
    #[only_owner]
    #[endpoint(setPairInfo)]
    fn set_pair_info(
        &self,
        token_id_from: TokenIdentifier,
        token_id_to: TokenIdentifier,
        pair_address: ManagedAddress,
    ) {
        self.swap_pair_address(&token_id_from, &token_id_to)
            .set(&pair_address);
    }

    #[only_owner]
    #[endpoint(removePairInfo)]
    fn remove_pair_info(&self, token_id_from: TokenIdentifier, token_id_to: TokenIdentifier) {
        self.swap_pair_address(&token_id_from, &token_id_to).clear();
    }

    #[view(getEquivalentVestaDex)]
    fn get_equivalent_vesta_dex(
        &self,
        from: &TokenIdentifier,
        to: &TokenIdentifier,
        amount: &BigUint,
    ) -> BigUint {
        let pair_address = self.swap_pair_address(from, to).get();

        self.dex_swap_pool_proxy(pair_address)
            .get_equivalent(from.clone(), amount.clone())
            .execute_on_dest_context()
    }

    #[view(getEquivalentXExchange)]
    fn get_equivalent_xexchange(
        &self,
        from: &TokenIdentifier,
        to: &TokenIdentifier,
        amount: &BigUint,
    ) -> BigUint {
        let pair_address = self.swap_pair_address(from, to).get();

        self.dex_swap_pool_proxy(pair_address)
            .get_equivalent(from.clone(), amount.clone())
            .execute_on_dest_context()
    }

    #[view(getSwapPairAddress)]
    #[storage_mapper("swap_pair_address")]
    fn swap_pair_address(
        &self,
        token_id_from: &TokenIdentifier,
        token_id_to: &TokenIdentifier,
    ) -> SingleValueMapper<ManagedAddress>;

    #[proxy]
    fn dex_swap_pool_proxy(&self, sc_address: ManagedAddress) -> dex_pool_proxy::Proxy<Self::Api>;
}

mod dex_pool_proxy {
    multiversx_sc::imports!();

    #[multiversx_sc::proxy]
    pub trait DexSwapPool {
        #[view(getEquivalent)]
        fn get_equivalent(&self, token_in: TokenIdentifier, amount_in: BigUint) -> BigUint;
    }
}
