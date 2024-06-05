// Code generated by the multiversx-sc build system. DO NOT EDIT.

////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

// Init:                                 1
// Endpoints:                           12
// Async Callback:                       1
// Total number of exported functions:  14

#![no_std]
#![allow(internal_features)]
#![feature(lang_items)]

multiversx_sc_wasm_adapter::allocator!();
multiversx_sc_wasm_adapter::panic_handler!();

multiversx_sc_wasm_adapter::endpoints! {
    koson_factory_universal
    (
        init => init
        upgrade => upgrade
        setDistributionList => set_distribution_list
        distribute => distribute
        getUndistributedAmount => get_total_undistributed_amount_view
        getDistributionList => distribution_list
        getLastDistributionEpoch => last_distribution_epoch
        getCurrentSupply => current_supply
        getFactoryTokenIdentifier => factory_token_id
        getTotalDistributionPerAddress => total_distribution_per_address
        getChrysopoeicForgeAddress => chrysopoeic_forge_address
        issue => issue_token
        setTokenId => set_token_id
    )
}

multiversx_sc_wasm_adapter::async_callback! { koson_factory_universal }