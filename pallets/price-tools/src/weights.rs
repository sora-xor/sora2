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

//! Autogenerated weights for price_tools
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-07-04, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `ef768cf07a8c`, CPU: `Intel(R) Xeon(R) Platinum 8275CL CPU @ 3.00GHz`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("local"), DB CACHE: 1024

// Executed Command:
// /usr/local/bin/framenode
// benchmark
// pallet
// --chain=local
// --steps=50
// --repeat=20
// --pallet=price_tools
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --header=./misc/file_header.txt
// --template=./misc/pallet-weight-template.hbs
// --output=./pallets/price-tools/src/weights.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use core::marker::PhantomData;

/// Weight functions needed for price_tools.
pub trait WeightInfo {
	fn on_initialize(a: u32, b: u32, ) -> Weight;
}

/// Weights for price_tools using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	/// Storage: PriceTools PriceInfos (r:27 w:26)
	/// Proof Skipped: PriceTools PriceInfos (max_values: None, max_size: None, mode: Measured)
	/// Storage: DEXManager DEXInfos (r:1 w:0)
	/// Proof Skipped: DEXManager DEXInfos (max_values: None, max_size: None, mode: Measured)
	/// Storage: XSTPool EnabledSynthetics (r:2 w:0)
	/// Proof Skipped: XSTPool EnabledSynthetics (max_values: None, max_size: None, mode: Measured)
	/// Storage: DEXAPI EnabledSourceTypes (r:1 w:0)
	/// Proof Skipped: DEXAPI EnabledSourceTypes (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: PoolXYK Properties (r:16 w:0)
	/// Proof Skipped: PoolXYK Properties (max_values: None, max_size: None, mode: Measured)
	/// Storage: TradingPair LockedLiquiditySources (r:1 w:0)
	/// Proof Skipped: TradingPair LockedLiquiditySources (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: System Account (r:10 w:0)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// Storage: Tokens Accounts (r:10 w:0)
	/// Proof: Tokens Accounts (max_values: None, max_size: Some(136), added: 2611, mode: MaxEncodedLen)
	/// The range of component `a` is `[0, 10]`.
	/// The range of component `b` is `[0, 10]`.
	fn on_initialize(a: u32, b: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `2190 + a * (1077 ±0) + b * (1560 ±0)`
		//  Estimated: `52716 + a * (8985 ±1) + b * (19614 ±1)`
		// Minimum execution time: 702_895_000 picoseconds.
		Weight::from_parts(352_031_214, 52716)
			// Standard Error: 98_245
			.saturating_add(Weight::from_parts(35_972_634, 0).saturating_mul(a.into()))
			// Standard Error: 98_245
			.saturating_add(Weight::from_parts(103_785_870, 0).saturating_mul(b.into()))
			.saturating_add(T::DbWeight::get().reads(18_u64))
			.saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(a.into())))
			.saturating_add(T::DbWeight::get().reads((4_u64).saturating_mul(b.into())))
			.saturating_add(T::DbWeight::get().writes(6_u64))
			.saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(a.into())))
			.saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(b.into())))
			.saturating_add(Weight::from_parts(0, 8985).saturating_mul(a.into()))
			.saturating_add(Weight::from_parts(0, 19614).saturating_mul(b.into()))
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	/// Storage: PriceTools PriceInfos (r:27 w:26)
	/// Proof Skipped: PriceTools PriceInfos (max_values: None, max_size: None, mode: Measured)
	/// Storage: DEXManager DEXInfos (r:1 w:0)
	/// Proof Skipped: DEXManager DEXInfos (max_values: None, max_size: None, mode: Measured)
	/// Storage: XSTPool EnabledSynthetics (r:2 w:0)
	/// Proof Skipped: XSTPool EnabledSynthetics (max_values: None, max_size: None, mode: Measured)
	/// Storage: DEXAPI EnabledSourceTypes (r:1 w:0)
	/// Proof Skipped: DEXAPI EnabledSourceTypes (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: PoolXYK Properties (r:16 w:0)
	/// Proof Skipped: PoolXYK Properties (max_values: None, max_size: None, mode: Measured)
	/// Storage: TradingPair LockedLiquiditySources (r:1 w:0)
	/// Proof Skipped: TradingPair LockedLiquiditySources (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: System Account (r:10 w:0)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// Storage: Tokens Accounts (r:10 w:0)
	/// Proof: Tokens Accounts (max_values: None, max_size: Some(136), added: 2611, mode: MaxEncodedLen)
	/// The range of component `a` is `[0, 10]`.
	/// The range of component `b` is `[0, 10]`.
	fn on_initialize(a: u32, b: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `2190 + a * (1077 ±0) + b * (1560 ±0)`
		//  Estimated: `52716 + a * (8985 ±1) + b * (19614 ±1)`
		// Minimum execution time: 702_895_000 picoseconds.
		Weight::from_parts(352_031_214, 52716)
			// Standard Error: 98_245
			.saturating_add(Weight::from_parts(35_972_634, 0).saturating_mul(a.into()))
			// Standard Error: 98_245
			.saturating_add(Weight::from_parts(103_785_870, 0).saturating_mul(b.into()))
			.saturating_add(RocksDbWeight::get().reads(18_u64))
			.saturating_add(RocksDbWeight::get().reads((1_u64).saturating_mul(a.into())))
			.saturating_add(RocksDbWeight::get().reads((4_u64).saturating_mul(b.into())))
			.saturating_add(RocksDbWeight::get().writes(6_u64))
			.saturating_add(RocksDbWeight::get().writes((1_u64).saturating_mul(a.into())))
			.saturating_add(RocksDbWeight::get().writes((1_u64).saturating_mul(b.into())))
			.saturating_add(Weight::from_parts(0, 8985).saturating_mul(a.into()))
			.saturating_add(Weight::from_parts(0, 19614).saturating_mul(b.into()))
	}
}
