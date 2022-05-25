//! Autogenerated weights for ceres_launchpad
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 3.0.0
//! DATE: 2022-02-01, STEPS: [], REPEAT: 10, LOW RANGE: [], HIGH RANGE: []
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
        (172_400_000 as Weight)
            .saturating_add(T::DbWeight::get().reads(6 as Weight))
            .saturating_add(T::DbWeight::get().writes(5 as Weight))
    }
    fn contribute() -> Weight {
        (145_400_000 as Weight)
            .saturating_add(T::DbWeight::get().reads(6 as Weight))
            .saturating_add(T::DbWeight::get().writes(4 as Weight))
    }
    fn emergency_withdraw() -> Weight {
        (205_900_000 as Weight)
            .saturating_add(T::DbWeight::get().reads(6 as Weight))
            .saturating_add(T::DbWeight::get().writes(5 as Weight))
    }
    fn finish_ilo() -> Weight {
        (764_700_000 as Weight)
            .saturating_add(T::DbWeight::get().reads(23 as Weight))
            .saturating_add(T::DbWeight::get().writes(21 as Weight))
    }
    fn claim_lp_tokens() -> Weight {
        (109_300_000 as Weight)
            .saturating_add(T::DbWeight::get().reads(5 as Weight))
            .saturating_add(T::DbWeight::get().writes(4 as Weight))
    }
    fn claim() -> Weight {
        (110_700_000 as Weight)
            .saturating_add(T::DbWeight::get().reads(4 as Weight))
            .saturating_add(T::DbWeight::get().writes(3 as Weight))
    }
    fn change_ceres_burn_fee() -> Weight {
        (27_200_000 as Weight)
            .saturating_add(T::DbWeight::get().reads(1 as Weight))
            .saturating_add(T::DbWeight::get().writes(1 as Weight))
    }
    fn change_ceres_contribution_fee() -> Weight {
        (27_100_000 as Weight)
            .saturating_add(T::DbWeight::get().reads(1 as Weight))
            .saturating_add(T::DbWeight::get().writes(1 as Weight))
    }
    fn claim_pswap_rewards() -> Weight {
        (217_000_000 as Weight)
            .saturating_add(T::DbWeight::get().reads(8 as Weight))
            .saturating_add(T::DbWeight::get().writes(7 as Weight))
    }
    fn add_whitelisted_contributor() -> Weight {
        (26_400_000 as Weight)
            .saturating_add(T::DbWeight::get().reads(2 as Weight))
            .saturating_add(T::DbWeight::get().writes(1 as Weight))
    }
    fn remove_whitelisted_contributor() -> Weight {
        (26_800_000 as Weight)
            .saturating_add(T::DbWeight::get().reads(2 as Weight))
            .saturating_add(T::DbWeight::get().writes(1 as Weight))
    }
    fn add_whitelisted_ilo_organizer() -> Weight {
        (25_900_000 as Weight)
            .saturating_add(T::DbWeight::get().reads(2 as Weight))
            .saturating_add(T::DbWeight::get().writes(1 as Weight))
    }
    fn remove_whitelisted_ilo_organizer() -> Weight {
        (26_600_000 as Weight)
            .saturating_add(T::DbWeight::get().reads(2 as Weight))
            .saturating_add(T::DbWeight::get().writes(1 as Weight))
    }
}
impl crate::WeightInfo for () {
    fn create_ilo() -> Weight {
        2 * EXTRINSIC_FIXED_WEIGHT
    }
    fn contribute() -> Weight {
        EXTRINSIC_FIXED_WEIGHT
    }
    fn emergency_withdraw() -> Weight {
        2 * EXTRINSIC_FIXED_WEIGHT
    }
    fn finish_ilo() -> Weight {
        3 * EXTRINSIC_FIXED_WEIGHT
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
