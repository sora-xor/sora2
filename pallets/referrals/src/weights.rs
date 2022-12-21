//! Autogenerated weights for referrals
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 3.0.0
//! DATE: 2021-12-01, STEPS: [20, ], REPEAT: 10, LOW RANGE: [], HIGH RANGE: []
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("main-coded"), DB CACHE: 128

// Executed Command:
// target/debug/framenode
// benchmark
// --chain
// main-coded
// --execution
// wasm
// --wasm-execution
// compiled
// --pallet
// referrals
// --extrinsic=*
// --steps
// 20
// --repeat
// 10
// --raw
// --output
// ./

use frame_support::weights::Weight;
use sp_std::marker::PhantomData;

pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> crate::WeightInfo for WeightInfo<T> {
    fn reserve() -> Weight {
        Weight::zero()
    }
    fn unreserve() -> Weight {
        Weight::zero()
    }
    fn set_referrer() -> Weight {
        Weight::zero()
    }
}
