use super::umbrella_interactor::ORACLE_PRICE_DENOMINATION;

multiversx_sc::imports!();

pub const UNSTAKE_FEE_PER_SCORE_USD: u64 = 100_000_000_000_000_000; // 10^17
pub const ONE_TOKEN: u64 = 1_000_000_000_000_000_000; // 10^18
pub const ONE_USDC: u64 = 1_000_000; // 10^6

#[multiversx_sc::module]
pub trait UnstakeFeeCalculator:
    super::dex_pair_interactor::DexPairInteractorModule
    + super::umbrella_interactor::UmbrellaInteractorModule
    + crate::storage::StorageModule
{
    fn calculate_unstake_fee_usdc(&self, unstake_total_score: BigUint) -> BigUint {
        let ouro_token_id = self.ouro_token_id().get();
        let wegld_token_id = self.wegld_token_id().get();
        let usdd_token_id = self.usdd_token_id().get();
        let usdc_token_id = self.usdc_token_id().get();
        let koson_token_id = self.koson_token_id().get();

        let koson_ouro_price = self.get_equivalent_vesta_dex(
            &ouro_token_id,
            &koson_token_id,
            &BigUint::from(ONE_TOKEN),
        );
        let median_ouro_usd_price = self.get_median_ouro_price(
            &ouro_token_id,
            &usdd_token_id,
            &usdc_token_id,
            &wegld_token_id,
        );
        let unstake_fee_per_score = BigUint::from(UNSTAKE_FEE_PER_SCORE_USD);

        let koson_usd_price = koson_ouro_price * median_ouro_usd_price / BigUint::from(ONE_TOKEN);

        unstake_total_score * unstake_fee_per_score / koson_usd_price
    }

    fn get_median_ouro_price(
        &self,
        ouro_token_id: &TokenIdentifier,
        usdd_token_id: &TokenIdentifier,
        usdc_token_id: &TokenIdentifier,
        wegld_token_id: &TokenIdentifier,
    ) -> BigUint {
        let ouro_usdd_price =
            self.get_equivalent_vesta_dex(ouro_token_id, usdd_token_id, &BigUint::from(ONE_TOKEN));
        let ouro_usdc_price =
            self.get_equivalent_vesta_dex(ouro_token_id, usdc_token_id, &BigUint::from(ONE_TOKEN))
                * ONE_TOKEN
                / ONE_USDC; // USDC has only 6 decimals, we need 18
        let ouro_egld_usd_price = self.get_ouro_egld_usd_price(ouro_token_id, wegld_token_id);

        (ouro_usdd_price + ouro_usdc_price + ouro_egld_usd_price) / BigUint::from(3u64)
    }

    fn get_ouro_egld_usd_price(
        &self,
        ouro_token_id: &TokenIdentifier,
        wegld_token_id: &TokenIdentifier,
    ) -> BigUint {
        let ouro_egld_price =
            self.get_equivalent_vesta_dex(ouro_token_id, wegld_token_id, &BigUint::from(ONE_TOKEN));
        let egld_oracle_price =
            self.get_oracle_price(wegld_token_id) * ONE_TOKEN / ORACLE_PRICE_DENOMINATION;

        ouro_egld_price * egld_oracle_price / ONE_TOKEN
    }
}
