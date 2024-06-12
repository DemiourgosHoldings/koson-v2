use koson_factory_chrysopoeic::{
    constants::config::MAX_PERCENTAGE, types::distribution_type::DistributionType,
};

use super::{
    KOSON_STAKING_POOL_1_ADDRESS_EXPR, KOSON_STAKING_POOL_2_ADDRESS_EXPR,
    KOSON_STAKING_POOL_3_ADDRESS_EXPR, KOSON_STAKING_POOL_4_ADDRESS_EXPR,
    KOSON_STAKING_POOL_5_ADDRESS_EXPR, KOSON_STAKING_POOL_6_ADDRESS_EXPR,
    LAND_PLOT_STAKING_POOL_ADDRESS_EXPR, SOUL_STAKING_POOL_ADDRESS_EXPR, USER_1_ADDRESS_EXPR,
};

pub fn get_direct_send_distribution_list() -> (
    &'static [&'static str],
    &'static [&'static u64],
    &'static [&'static u8],
) {
    (&[USER_1_ADDRESS_EXPR], &[&MAX_PERCENTAGE], &[&1u8])
}

pub fn get_single_koson_staking_pool_distribution_list() -> (
    &'static [&'static str],
    &'static [&'static u64],
    &'static [&'static u8],
) {
    (
        &[KOSON_STAKING_POOL_1_ADDRESS_EXPR],
        &[&MAX_PERCENTAGE],
        &[&(DistributionType::KosonStakingInteraction as u8)],
    )
}

pub fn get_single_nft_staking_pool_distribution_list() -> (
    &'static [&'static str],
    &'static [&'static u64],
    &'static [&'static u8],
) {
    (
        &[SOUL_STAKING_POOL_ADDRESS_EXPR],
        &[&MAX_PERCENTAGE],
        &[&(DistributionType::SoulStakingInteraction as u8)],
    )
}

pub fn get_single_land_plot_staking_pool_distribution_list() -> (
    &'static [&'static str],
    &'static [&'static u64],
    &'static [&'static u8],
) {
    (
        &[LAND_PLOT_STAKING_POOL_ADDRESS_EXPR],
        &[&MAX_PERCENTAGE],
        &[&(DistributionType::LandPlotStakingInteraction as u8)],
    )
}

pub fn get_actual_distribution_list() -> (
    &'static [&'static str],
    &'static [&'static u64],
    &'static [&'static u8],
) {
    (
        &[
            KOSON_STAKING_POOL_1_ADDRESS_EXPR,
            KOSON_STAKING_POOL_2_ADDRESS_EXPR,
            KOSON_STAKING_POOL_3_ADDRESS_EXPR,
            KOSON_STAKING_POOL_4_ADDRESS_EXPR,
            KOSON_STAKING_POOL_5_ADDRESS_EXPR,
            KOSON_STAKING_POOL_6_ADDRESS_EXPR,
            SOUL_STAKING_POOL_ADDRESS_EXPR,
            LAND_PLOT_STAKING_POOL_ADDRESS_EXPR,
            USER_1_ADDRESS_EXPR,
        ],
        &[
            &7142857142857143,  // &(5 * MAX_PERCENTAGE / 70) = 7.142857142857143
            &8571428571428570,  // &(6 * MAX_PERCENTAGE / 70) = 8.571428571428570
            &10000000000000000, // &(7 * MAX_PERCENTAGE / 70) = 10.000000000000000
            &11428571428571430, // &(8 * MAX_PERCENTAGE / 70) = 11.428571428571430
            &12857142857142857, // &(9 * MAX_PERCENTAGE / 70) = 12.857142857142857
            &14285714285714286, // &(10 * MAX_PERCENTAGE / 70) = 14.285714285714286
            &7142857142857143,  // &(5 * MAX_PERCENTAGE / 70) = 7.142857142857143
            &7142857142857143,  // &(5 * MAX_PERCENTAGE / 70) = 7.142857142857143
            &21428571428571428, // &(15 * MAX_PERCENTAGE / 70) = 21.428571428571428
        ],
        &[
            &(DistributionType::KosonStakingInteraction as u8),
            &(DistributionType::KosonStakingInteraction as u8),
            &(DistributionType::KosonStakingInteraction as u8),
            &(DistributionType::KosonStakingInteraction as u8),
            &(DistributionType::KosonStakingInteraction as u8),
            &(DistributionType::KosonStakingInteraction as u8),
            &(DistributionType::SoulStakingInteraction as u8),
            &(DistributionType::LandPlotStakingInteraction as u8),
            &(DistributionType::DirectSend as u8),
        ],
    )
}
