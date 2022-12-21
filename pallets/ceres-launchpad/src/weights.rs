//! Autogenerated weights for ceres_launchpad
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 3.0.0
//! DATE: 2022-06-07, STEPS: [], REPEAT: 10, LOW RANGE: [], HIGH RANGE: []
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("main-coded"), DB CACHE: 128

// Executed Command:
// target\release\framenode.exe
// benchmark
// --chain
// main-coded
// --execution
// wasm
// --wasm-execution
// compiled
// --pallet
// ceres_launchpad
// --extrinsic
// *
// --repeat
// 10
// --raw
// --output
// ./

#![allow(unused_parens)]
#![allow(unused_imports)]

use common::weights::constants::EXTRINSIC_FIXED_WEIGHT;
use frame_support::traits::Get;
use frame_support::weights::Weight;
use sp_std::marker::PhantomData;

/// Weight functions for ceres_launchpad.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> crate::WeightInfo for WeightInfo<T> {
    fn create_ilo() -> Weight {
        Weight::zero()
    }
    fn contribute() -> Weight {
        Weight::zero()
    }
    fn emergency_withdraw() -> Weight {
        Weight::zero()
    }
    fn finish_ilo() -> Weight {
        Weight::zero()
    }
    fn claim_lp_tokens() -> Weight {
        Weight::zero()
    }
    fn claim() -> Weight {
        Weight::zero()
    }
    fn change_ceres_burn_fee() -> Weight {
        Weight::zero()
    }
    fn change_ceres_contribution_fee() -> Weight {
        Weight::zero()
    }
    fn claim_pswap_rewards() -> Weight {
        Weight::zero()
    }
    fn add_whitelisted_contributor() -> Weight {
        Weight::zero()
    }
    fn remove_whitelisted_contributor() -> Weight {
        Weight::zero()
    }
    fn add_whitelisted_ilo_organizer() -> Weight {
        Weight::zero()
    }
    fn remove_whitelisted_ilo_organizer() -> Weight {
        Weight::zero()
    }
}
impl crate::WeightInfo for () {
    fn create_ilo() -> Weight {
        EXTRINSIC_FIXED_WEIGHT.mul(2)
    }
    fn contribute() -> Weight {
        EXTRINSIC_FIXED_WEIGHT
    }
    fn emergency_withdraw() -> Weight {
        EXTRINSIC_FIXED_WEIGHT.mul(2)
    }
    fn finish_ilo() -> Weight {
        EXTRINSIC_FIXED_WEIGHT.mul(3)
    }
    fn claim_lp_tokens() -> Weight {
        EXTRINSIC_FIXED_WEIGHT
    }
    fn claim() -> Weight {
        EXTRINSIC_FIXED_WEIGHT
    }
    fn change_ceres_burn_fee() -> Weight {
        EXTRINSIC_FIXED_WEIGHT
    }
    fn change_ceres_contribution_fee() -> Weight {
        EXTRINSIC_FIXED_WEIGHT
    }
    fn claim_pswap_rewards() -> Weight {
        EXTRINSIC_FIXED_WEIGHT
    }
    fn add_whitelisted_contributor() -> Weight {
        EXTRINSIC_FIXED_WEIGHT
    }
    fn remove_whitelisted_contributor() -> Weight {
        EXTRINSIC_FIXED_WEIGHT
    }
    fn add_whitelisted_ilo_organizer() -> Weight {
        EXTRINSIC_FIXED_WEIGHT
    }
    fn remove_whitelisted_ilo_organizer() -> Weight {
        EXTRINSIC_FIXED_WEIGHT
    }
}
