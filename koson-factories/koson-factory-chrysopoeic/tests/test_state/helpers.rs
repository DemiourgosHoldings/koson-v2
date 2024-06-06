use koson_factory_chrysopoeic::constants::config::MAX_PERCENTAGE;

use super::USER_1_ADDRESS_EXPR;

pub fn get_simple_distribution_list() -> (
    &'static [&'static str],
    &'static [&'static u64],
    &'static [&'static u8],
) {
    (&[USER_1_ADDRESS_EXPR], &[&MAX_PERCENTAGE], &[&1u8])
}
