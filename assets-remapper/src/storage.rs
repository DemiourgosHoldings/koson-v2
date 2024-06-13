use crate::types::token_identifier_details::TokenIdentifierDetails;

multiversx_sc::imports!();

#[multiversx_sc::module]
pub trait StorageModule {
    #[view(getTokenIdentifierMapValue)]
    #[storage_mapper("token_identifier_map")]
    fn token_identifier_map(
        &self,
        token_id_in: &TokenIdentifier,
    ) -> SingleValueMapper<TokenIdentifier>;

    #[view(getTokenIdentifierDetails)]
    #[storage_mapper("token_identifier_details")]
    fn token_identifier_details(
        &self,
        token_id: &TokenIdentifier,
    ) -> SingleValueMapper<TokenIdentifierDetails<Self::Api>>;
}
