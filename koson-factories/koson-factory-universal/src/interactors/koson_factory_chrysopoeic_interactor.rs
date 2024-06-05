multiversx_sc::imports!();

#[multiversx_sc::module]
pub trait KosonFactoryChrysopoeicInteractor {
    fn get_undistributed_chrysopoeic_forge_emission_amount(
        &self,
        address: ManagedAddress,
    ) -> BigUint {
        self.koson_factory_chrysopoeic_interactor_proxy(address)
            .get_total_undistributed_amount_view()
            .execute_on_dest_context()
    }

    #[proxy]
    fn koson_factory_chrysopoeic_interactor_proxy(
        &self,
        address: ManagedAddress,
    ) -> koson_factory_chrysopoeic::Proxy<Self::Api>;
}
