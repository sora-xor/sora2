// This file is part of the SORA network and Polkaswap app.

// Copyright (c) 2020, 2021, Polka Biome Ltd. All rights reserved.
// SPDX-License-Identifier: BSD-4-Clause

// Redistribution and use in source and binary forms, with or without modification,
// are permitted provided that the following conditions are met:

// Redistributions of source code must retain the above copyright notice, this list
// of conditions and the following disclaimer.
// Redistributions in binary form must reproduce the above copyright notice, this
// list of conditions and the following disclaimer in the documentation and/or other
// materials provided with the distribution.
//
// All advertising materials mentioning features or use of this software must display
// the following acknowledgement: This product includes software developed by Polka Biome
// Ltd., SORA, and Polkaswap.
//
// Neither the name of the Polka Biome Ltd. nor the names of its contributors may be used
// to endorse or promote products derived from this software without specific prior written permission.

// THIS SOFTWARE IS PROVIDED BY Polka Biome Ltd. AS IS AND ANY EXPRESS OR IMPLIED WARRANTIES,
// INCLUDING, BUT NOT LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR
// A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL Polka Biome Ltd. BE LIABLE FOR ANY
// DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING,
// BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR PROFITS;
// OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT,
// STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE
// USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.

//! Autogenerated weights for liquidity_proxy
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-05-29, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `f18210f8c8d9`, CPU: `Intel(R) Xeon(R) Platinum 8275CL CPU @ 3.00GHz`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("local"), DB CACHE: 1024

// Executed Command:
// /usr/local/bin/framenode
// benchmark
// pallet
// --chain=local
// --steps=50
// --repeat=20
// --pallet=liquidity_proxy
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --header=./misc/file_header.txt
// --template=./misc/pallet-weight-template.hbs
// --output=./pallets/liquidity-proxy/src/weights.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use core::marker::PhantomData;

/// Weight functions needed for liquidity_proxy.
pub trait WeightInfo {
	fn enable_liquidity_source() -> Weight;
	fn disable_liquidity_source() -> Weight;
	fn check_indivisible_assets() -> Weight;
	fn new_trivial() -> Weight;
	fn is_forbidden_filter() -> Weight;
	fn list_liquidity_sources() -> Weight;
}

/// Weights for liquidity_proxy using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	/// Storage: TradingPair LockedLiquiditySources (r:1 w:1)
	/// Proof Skipped: TradingPair LockedLiquiditySources (max_values: Some(1), max_size: None, mode: Measured)
	fn enable_liquidity_source() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `170`
		//  Estimated: `665`
		// Minimum execution time: 15_362_000 picoseconds.
		Weight::from_parts(15_828_000, 665)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: TradingPair LockedLiquiditySources (r:1 w:1)
	/// Proof Skipped: TradingPair LockedLiquiditySources (max_values: Some(1), max_size: None, mode: Measured)
	fn disable_liquidity_source() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `147`
		//  Estimated: `642`
		// Minimum execution time: 14_103_000 picoseconds.
		Weight::from_parts(14_481_000, 642)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: Assets AssetInfos (r:2 w:0)
	/// Proof Skipped: Assets AssetInfos (max_values: None, max_size: None, mode: Measured)
	fn check_indivisible_assets() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `536`
		//  Estimated: `5486`
		// Minimum execution time: 11_709_000 picoseconds.
		Weight::from_parts(11_903_000, 5486)
			.saturating_add(T::DbWeight::get().reads(2_u64))
	}
	/// Storage: XSTPool EnabledSynthetics (r:2 w:0)
	/// Proof Skipped: XSTPool EnabledSynthetics (max_values: None, max_size: None, mode: Measured)
	fn new_trivial() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `311`
		//  Estimated: `5261`
		// Minimum execution time: 14_930_000 picoseconds.
		Weight::from_parts(15_450_000, 5261)
			.saturating_add(T::DbWeight::get().reads(2_u64))
	}
	/// Storage: MulticollateralBondingCurvePool EnabledTargets (r:1 w:0)
	/// Proof Skipped: MulticollateralBondingCurvePool EnabledTargets (max_values: Some(1), max_size: None, mode: Measured)
	fn is_forbidden_filter() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `358`
		//  Estimated: `853`
		// Minimum execution time: 5_485_000 picoseconds.
		Weight::from_parts(5_646_000, 853)
			.saturating_add(T::DbWeight::get().reads(1_u64))
	}
	/// Storage: DEXAPI EnabledSourceTypes (r:1 w:0)
	/// Proof Skipped: DEXAPI EnabledSourceTypes (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: DEXManager DEXInfos (r:1 w:0)
	/// Proof Skipped: DEXManager DEXInfos (max_values: None, max_size: None, mode: Measured)
	/// Storage: PoolXYK Properties (r:1 w:0)
	/// Proof Skipped: PoolXYK Properties (max_values: None, max_size: None, mode: Measured)
	/// Storage: MulticollateralBondingCurvePool EnabledTargets (r:1 w:0)
	/// Proof Skipped: MulticollateralBondingCurvePool EnabledTargets (max_values: Some(1), max_size: None, mode: Measured)
	fn list_liquidity_sources() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `889`
		//  Estimated: `9496`
		// Minimum execution time: 23_653_000 picoseconds.
		Weight::from_parts(24_592_000, 9496)
			.saturating_add(T::DbWeight::get().reads(4_u64))
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	/// Storage: TradingPair LockedLiquiditySources (r:1 w:1)
	/// Proof Skipped: TradingPair LockedLiquiditySources (max_values: Some(1), max_size: None, mode: Measured)
	fn enable_liquidity_source() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `170`
		//  Estimated: `665`
		// Minimum execution time: 15_362_000 picoseconds.
		Weight::from_parts(15_828_000, 665)
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: TradingPair LockedLiquiditySources (r:1 w:1)
	/// Proof Skipped: TradingPair LockedLiquiditySources (max_values: Some(1), max_size: None, mode: Measured)
	fn disable_liquidity_source() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `147`
		//  Estimated: `642`
		// Minimum execution time: 14_103_000 picoseconds.
		Weight::from_parts(14_481_000, 642)
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: Assets AssetInfos (r:2 w:0)
	/// Proof Skipped: Assets AssetInfos (max_values: None, max_size: None, mode: Measured)
	fn check_indivisible_assets() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `536`
		//  Estimated: `5486`
		// Minimum execution time: 11_709_000 picoseconds.
		Weight::from_parts(11_903_000, 5486)
			.saturating_add(RocksDbWeight::get().reads(2_u64))
	}
	/// Storage: XSTPool EnabledSynthetics (r:2 w:0)
	/// Proof Skipped: XSTPool EnabledSynthetics (max_values: None, max_size: None, mode: Measured)
	fn new_trivial() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `311`
		//  Estimated: `5261`
		// Minimum execution time: 14_930_000 picoseconds.
		Weight::from_parts(15_450_000, 5261)
			.saturating_add(RocksDbWeight::get().reads(2_u64))
	}
	/// Storage: MulticollateralBondingCurvePool EnabledTargets (r:1 w:0)
	/// Proof Skipped: MulticollateralBondingCurvePool EnabledTargets (max_values: Some(1), max_size: None, mode: Measured)
	fn is_forbidden_filter() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `358`
		//  Estimated: `853`
		// Minimum execution time: 5_485_000 picoseconds.
		Weight::from_parts(5_646_000, 853)
			.saturating_add(RocksDbWeight::get().reads(1_u64))
	}
	/// Storage: DEXAPI EnabledSourceTypes (r:1 w:0)
	/// Proof Skipped: DEXAPI EnabledSourceTypes (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: DEXManager DEXInfos (r:1 w:0)
	/// Proof Skipped: DEXManager DEXInfos (max_values: None, max_size: None, mode: Measured)
	/// Storage: PoolXYK Properties (r:1 w:0)
	/// Proof Skipped: PoolXYK Properties (max_values: None, max_size: None, mode: Measured)
	/// Storage: MulticollateralBondingCurvePool EnabledTargets (r:1 w:0)
	/// Proof Skipped: MulticollateralBondingCurvePool EnabledTargets (max_values: Some(1), max_size: None, mode: Measured)
	fn list_liquidity_sources() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `889`
		//  Estimated: `9496`
		// Minimum execution time: 23_653_000 picoseconds.
		Weight::from_parts(24_592_000, 9496)
			.saturating_add(RocksDbWeight::get().reads(4_u64))
	}
}
