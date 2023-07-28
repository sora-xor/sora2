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

//! Autogenerated weights for xst
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-07-28, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `a331765a0ed8`, CPU: `Intel(R) Xeon(R) Platinum 8124M CPU @ 3.00GHz`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("local"), DB CACHE: 1024

// Executed Command:
// /usr/local/bin/framenode
// benchmark
// pallet
// --chain=local
// --steps=50
// --repeat=20
// --pallet=xst
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --header=./misc/file_header.txt
// --template=./misc/pallet-weight-template.hbs
// --output=./pallets/xst/src/weights.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use core::marker::PhantomData;

/// Weight functions needed for xst.
pub trait WeightInfo {
	fn set_reference_asset() -> Weight;
	fn enable_synthetic_asset() -> Weight;
	fn disable_synthetic_asset() -> Weight;
	fn register_synthetic_asset() -> Weight;
	fn set_synthetic_asset_fee() -> Weight;
	fn set_synthetic_base_asset_floor_price() -> Weight;
	fn quote() -> Weight;
	fn exchange() -> Weight;
}

/// Weights for xst using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	/// Storage: Assets AssetOwners (r:1 w:0)
	/// Proof Skipped: Assets AssetOwners (max_values: None, max_size: None, mode: Measured)
	/// Storage: Assets AssetInfos (r:1 w:0)
	/// Proof Skipped: Assets AssetInfos (max_values: None, max_size: None, mode: Measured)
	/// Storage: XSTPool ReferenceAssetId (r:0 w:1)
	/// Proof Skipped: XSTPool ReferenceAssetId (max_values: Some(1), max_size: None, mode: Measured)
	fn set_reference_asset() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `799`
		//  Estimated: `7347`
		// Minimum execution time: 25_370_000 picoseconds.
		Weight::from_parts(26_045_000, 7347)
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: XSTPool EnabledSymbols (r:1 w:1)
	/// Proof Skipped: XSTPool EnabledSymbols (max_values: None, max_size: None, mode: Measured)
	/// Storage: OracleProxy EnabledOracles (r:1 w:0)
	/// Proof Skipped: OracleProxy EnabledOracles (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Band SymbolRates (r:2 w:0)
	/// Proof Skipped: Band SymbolRates (max_values: None, max_size: None, mode: Measured)
	/// Storage: Assets AssetOwners (r:1 w:0)
	/// Proof Skipped: Assets AssetOwners (max_values: None, max_size: None, mode: Measured)
	/// Storage: Assets AssetInfos (r:1 w:0)
	/// Proof Skipped: Assets AssetInfos (max_values: None, max_size: None, mode: Measured)
	/// Storage: DEXManager DEXInfos (r:1 w:0)
	/// Proof Skipped: DEXManager DEXInfos (max_values: None, max_size: None, mode: Measured)
	/// Storage: TradingPair EnabledSources (r:1 w:0)
	/// Proof Skipped: TradingPair EnabledSources (max_values: None, max_size: None, mode: Measured)
	/// Storage: XSTPool EnabledSynthetics (r:0 w:1)
	/// Proof Skipped: XSTPool EnabledSynthetics (max_values: None, max_size: None, mode: Measured)
	fn enable_synthetic_asset() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1837`
		//  Estimated: `32516`
		// Minimum execution time: 82_989_000 picoseconds.
		Weight::from_parts(83_827_000, 32516)
			.saturating_add(T::DbWeight::get().reads(8_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
	/// Storage: XSTPool EnabledSynthetics (r:1 w:1)
	/// Proof Skipped: XSTPool EnabledSynthetics (max_values: None, max_size: None, mode: Measured)
	/// Storage: XSTPool EnabledSymbols (r:0 w:1)
	/// Proof Skipped: XSTPool EnabledSymbols (max_values: None, max_size: None, mode: Measured)
	fn disable_synthetic_asset() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `276`
		//  Estimated: `3027`
		// Minimum execution time: 25_784_000 picoseconds.
		Weight::from_parts(26_466_000, 3027)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
	/// Storage: Assets AssetOwners (r:2 w:1)
	/// Proof Skipped: Assets AssetOwners (max_values: None, max_size: None, mode: Measured)
	/// Storage: System Account (r:1 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// Storage: Permissions Owners (r:2 w:2)
	/// Proof Skipped: Permissions Owners (max_values: None, max_size: None, mode: Measured)
	/// Storage: Permissions Permissions (r:3 w:1)
	/// Proof Skipped: Permissions Permissions (max_values: None, max_size: None, mode: Measured)
	/// Storage: XSTPool EnabledSymbols (r:1 w:1)
	/// Proof Skipped: XSTPool EnabledSymbols (max_values: None, max_size: None, mode: Measured)
	/// Storage: OracleProxy EnabledOracles (r:1 w:0)
	/// Proof Skipped: OracleProxy EnabledOracles (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Band SymbolRates (r:2 w:0)
	/// Proof Skipped: Band SymbolRates (max_values: None, max_size: None, mode: Measured)
	/// Storage: DEXManager DEXInfos (r:1 w:0)
	/// Proof Skipped: DEXManager DEXInfos (max_values: None, max_size: None, mode: Measured)
	/// Storage: TradingPair EnabledSources (r:1 w:1)
	/// Proof Skipped: TradingPair EnabledSources (max_values: None, max_size: None, mode: Measured)
	/// Storage: Assets AssetInfos (r:0 w:1)
	/// Proof Skipped: Assets AssetInfos (max_values: None, max_size: None, mode: Measured)
	/// Storage: XSTPool EnabledSynthetics (r:0 w:1)
	/// Proof Skipped: XSTPool EnabledSynthetics (max_values: None, max_size: None, mode: Measured)
	fn register_synthetic_asset() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `3040`
		//  Estimated: `63198`
		// Minimum execution time: 209_005_000 picoseconds.
		Weight::from_parts(211_092_000, 63198)
			.saturating_add(T::DbWeight::get().reads(14_u64))
			.saturating_add(T::DbWeight::get().writes(9_u64))
	}
	/// Storage: XSTPool EnabledSynthetics (r:1 w:1)
	/// Proof Skipped: XSTPool EnabledSynthetics (max_values: None, max_size: None, mode: Measured)
	fn set_synthetic_asset_fee() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `276`
		//  Estimated: `2751`
		// Minimum execution time: 23_339_000 picoseconds.
		Weight::from_parts(23_760_000, 2751)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: XSTPool SyntheticBaseAssetFloorPrice (r:0 w:1)
	/// Proof Skipped: XSTPool SyntheticBaseAssetFloorPrice (max_values: Some(1), max_size: None, mode: Measured)
	fn set_synthetic_base_asset_floor_price() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 10_837_000 picoseconds.
		Weight::from_parts(11_100_000, 0)
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: XSTPool EnabledSynthetics (r:1 w:0)
	/// Proof Skipped: XSTPool EnabledSynthetics (max_values: None, max_size: None, mode: Measured)
	/// Storage: OracleProxy EnabledOracles (r:1 w:0)
	/// Proof Skipped: OracleProxy EnabledOracles (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: OracleProxy SymbolProviders (r:1 w:0)
	/// Proof Skipped: OracleProxy SymbolProviders (max_values: None, max_size: None, mode: Measured)
	/// Storage: Band SymbolRates (r:1 w:0)
	/// Proof Skipped: Band SymbolRates (max_values: None, max_size: None, mode: Measured)
	/// Storage: XSTPool ReferenceAssetId (r:1 w:0)
	/// Proof Skipped: XSTPool ReferenceAssetId (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: PriceTools PriceInfos (r:2 w:0)
	/// Proof Skipped: PriceTools PriceInfos (max_values: None, max_size: None, mode: Measured)
	/// Storage: XSTPool SyntheticBaseAssetFloorPrice (r:1 w:0)
	/// Proof Skipped: XSTPool SyntheticBaseAssetFloorPrice (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Timestamp Now (r:1 w:0)
	/// Proof: Timestamp Now (max_values: Some(1), max_size: Some(8), added: 503, mode: MaxEncodedLen)
	fn quote() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `3127`
		//  Estimated: `36252`
		// Minimum execution time: 73_811_000 picoseconds.
		Weight::from_parts(74_569_000, 36252)
			.saturating_add(T::DbWeight::get().reads(9_u64))
	}
	/// Storage: XSTPool EnabledSynthetics (r:1 w:0)
	/// Proof Skipped: XSTPool EnabledSynthetics (max_values: None, max_size: None, mode: Measured)
	/// Storage: OracleProxy EnabledOracles (r:1 w:0)
	/// Proof Skipped: OracleProxy EnabledOracles (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: OracleProxy SymbolProviders (r:1 w:0)
	/// Proof Skipped: OracleProxy SymbolProviders (max_values: None, max_size: None, mode: Measured)
	/// Storage: Band SymbolRates (r:1 w:0)
	/// Proof Skipped: Band SymbolRates (max_values: None, max_size: None, mode: Measured)
	/// Storage: XSTPool ReferenceAssetId (r:1 w:0)
	/// Proof Skipped: XSTPool ReferenceAssetId (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: PriceTools PriceInfos (r:2 w:0)
	/// Proof Skipped: PriceTools PriceInfos (max_values: None, max_size: None, mode: Measured)
	/// Storage: XSTPool SyntheticBaseAssetFloorPrice (r:1 w:0)
	/// Proof Skipped: XSTPool SyntheticBaseAssetFloorPrice (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Timestamp Now (r:1 w:0)
	/// Proof: Timestamp Now (max_values: Some(1), max_size: Some(8), added: 503, mode: MaxEncodedLen)
	/// Storage: Permissions Permissions (r:3 w:0)
	/// Proof Skipped: Permissions Permissions (max_values: None, max_size: None, mode: Measured)
	/// Storage: Tokens Accounts (r:2 w:2)
	/// Proof: Tokens Accounts (max_values: None, max_size: Some(136), added: 2611, mode: MaxEncodedLen)
	/// Storage: Tokens TotalIssuance (r:2 w:2)
	/// Proof: Tokens TotalIssuance (max_values: None, max_size: Some(56), added: 2531, mode: MaxEncodedLen)
	/// Storage: Assets AssetInfos (r:1 w:0)
	/// Proof Skipped: Assets AssetInfos (max_values: None, max_size: None, mode: Measured)
	/// Storage: System Account (r:1 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	fn exchange() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `4819`
		//  Estimated: `80521`
		// Minimum execution time: 169_914_000 picoseconds.
		Weight::from_parts(171_470_000, 80521)
			.saturating_add(T::DbWeight::get().reads(18_u64))
			.saturating_add(T::DbWeight::get().writes(5_u64))
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	/// Storage: Assets AssetOwners (r:1 w:0)
	/// Proof Skipped: Assets AssetOwners (max_values: None, max_size: None, mode: Measured)
	/// Storage: Assets AssetInfos (r:1 w:0)
	/// Proof Skipped: Assets AssetInfos (max_values: None, max_size: None, mode: Measured)
	/// Storage: XSTPool ReferenceAssetId (r:0 w:1)
	/// Proof Skipped: XSTPool ReferenceAssetId (max_values: Some(1), max_size: None, mode: Measured)
	fn set_reference_asset() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `799`
		//  Estimated: `7347`
		// Minimum execution time: 25_370_000 picoseconds.
		Weight::from_parts(26_045_000, 7347)
			.saturating_add(RocksDbWeight::get().reads(2_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: XSTPool EnabledSymbols (r:1 w:1)
	/// Proof Skipped: XSTPool EnabledSymbols (max_values: None, max_size: None, mode: Measured)
	/// Storage: OracleProxy EnabledOracles (r:1 w:0)
	/// Proof Skipped: OracleProxy EnabledOracles (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Band SymbolRates (r:2 w:0)
	/// Proof Skipped: Band SymbolRates (max_values: None, max_size: None, mode: Measured)
	/// Storage: Assets AssetOwners (r:1 w:0)
	/// Proof Skipped: Assets AssetOwners (max_values: None, max_size: None, mode: Measured)
	/// Storage: Assets AssetInfos (r:1 w:0)
	/// Proof Skipped: Assets AssetInfos (max_values: None, max_size: None, mode: Measured)
	/// Storage: DEXManager DEXInfos (r:1 w:0)
	/// Proof Skipped: DEXManager DEXInfos (max_values: None, max_size: None, mode: Measured)
	/// Storage: TradingPair EnabledSources (r:1 w:0)
	/// Proof Skipped: TradingPair EnabledSources (max_values: None, max_size: None, mode: Measured)
	/// Storage: XSTPool EnabledSynthetics (r:0 w:1)
	/// Proof Skipped: XSTPool EnabledSynthetics (max_values: None, max_size: None, mode: Measured)
	fn enable_synthetic_asset() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1837`
		//  Estimated: `32516`
		// Minimum execution time: 82_989_000 picoseconds.
		Weight::from_parts(83_827_000, 32516)
			.saturating_add(RocksDbWeight::get().reads(8_u64))
			.saturating_add(RocksDbWeight::get().writes(2_u64))
	}
	/// Storage: XSTPool EnabledSynthetics (r:1 w:1)
	/// Proof Skipped: XSTPool EnabledSynthetics (max_values: None, max_size: None, mode: Measured)
	/// Storage: XSTPool EnabledSymbols (r:0 w:1)
	/// Proof Skipped: XSTPool EnabledSymbols (max_values: None, max_size: None, mode: Measured)
	fn disable_synthetic_asset() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `276`
		//  Estimated: `3027`
		// Minimum execution time: 25_784_000 picoseconds.
		Weight::from_parts(26_466_000, 3027)
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().writes(2_u64))
	}
	/// Storage: Assets AssetOwners (r:2 w:1)
	/// Proof Skipped: Assets AssetOwners (max_values: None, max_size: None, mode: Measured)
	/// Storage: System Account (r:1 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// Storage: Permissions Owners (r:2 w:2)
	/// Proof Skipped: Permissions Owners (max_values: None, max_size: None, mode: Measured)
	/// Storage: Permissions Permissions (r:3 w:1)
	/// Proof Skipped: Permissions Permissions (max_values: None, max_size: None, mode: Measured)
	/// Storage: XSTPool EnabledSymbols (r:1 w:1)
	/// Proof Skipped: XSTPool EnabledSymbols (max_values: None, max_size: None, mode: Measured)
	/// Storage: OracleProxy EnabledOracles (r:1 w:0)
	/// Proof Skipped: OracleProxy EnabledOracles (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Band SymbolRates (r:2 w:0)
	/// Proof Skipped: Band SymbolRates (max_values: None, max_size: None, mode: Measured)
	/// Storage: DEXManager DEXInfos (r:1 w:0)
	/// Proof Skipped: DEXManager DEXInfos (max_values: None, max_size: None, mode: Measured)
	/// Storage: TradingPair EnabledSources (r:1 w:1)
	/// Proof Skipped: TradingPair EnabledSources (max_values: None, max_size: None, mode: Measured)
	/// Storage: Assets AssetInfos (r:0 w:1)
	/// Proof Skipped: Assets AssetInfos (max_values: None, max_size: None, mode: Measured)
	/// Storage: XSTPool EnabledSynthetics (r:0 w:1)
	/// Proof Skipped: XSTPool EnabledSynthetics (max_values: None, max_size: None, mode: Measured)
	fn register_synthetic_asset() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `3040`
		//  Estimated: `63198`
		// Minimum execution time: 209_005_000 picoseconds.
		Weight::from_parts(211_092_000, 63198)
			.saturating_add(RocksDbWeight::get().reads(14_u64))
			.saturating_add(RocksDbWeight::get().writes(9_u64))
	}
	/// Storage: XSTPool EnabledSynthetics (r:1 w:1)
	/// Proof Skipped: XSTPool EnabledSynthetics (max_values: None, max_size: None, mode: Measured)
	fn set_synthetic_asset_fee() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `276`
		//  Estimated: `2751`
		// Minimum execution time: 23_339_000 picoseconds.
		Weight::from_parts(23_760_000, 2751)
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: XSTPool SyntheticBaseAssetFloorPrice (r:0 w:1)
	/// Proof Skipped: XSTPool SyntheticBaseAssetFloorPrice (max_values: Some(1), max_size: None, mode: Measured)
	fn set_synthetic_base_asset_floor_price() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 10_837_000 picoseconds.
		Weight::from_parts(11_100_000, 0)
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: XSTPool EnabledSynthetics (r:1 w:0)
	/// Proof Skipped: XSTPool EnabledSynthetics (max_values: None, max_size: None, mode: Measured)
	/// Storage: OracleProxy EnabledOracles (r:1 w:0)
	/// Proof Skipped: OracleProxy EnabledOracles (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: OracleProxy SymbolProviders (r:1 w:0)
	/// Proof Skipped: OracleProxy SymbolProviders (max_values: None, max_size: None, mode: Measured)
	/// Storage: Band SymbolRates (r:1 w:0)
	/// Proof Skipped: Band SymbolRates (max_values: None, max_size: None, mode: Measured)
	/// Storage: XSTPool ReferenceAssetId (r:1 w:0)
	/// Proof Skipped: XSTPool ReferenceAssetId (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: PriceTools PriceInfos (r:2 w:0)
	/// Proof Skipped: PriceTools PriceInfos (max_values: None, max_size: None, mode: Measured)
	/// Storage: XSTPool SyntheticBaseAssetFloorPrice (r:1 w:0)
	/// Proof Skipped: XSTPool SyntheticBaseAssetFloorPrice (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Timestamp Now (r:1 w:0)
	/// Proof: Timestamp Now (max_values: Some(1), max_size: Some(8), added: 503, mode: MaxEncodedLen)
	fn quote() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `3127`
		//  Estimated: `36252`
		// Minimum execution time: 73_811_000 picoseconds.
		Weight::from_parts(74_569_000, 36252)
			.saturating_add(RocksDbWeight::get().reads(9_u64))
	}
	/// Storage: XSTPool EnabledSynthetics (r:1 w:0)
	/// Proof Skipped: XSTPool EnabledSynthetics (max_values: None, max_size: None, mode: Measured)
	/// Storage: OracleProxy EnabledOracles (r:1 w:0)
	/// Proof Skipped: OracleProxy EnabledOracles (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: OracleProxy SymbolProviders (r:1 w:0)
	/// Proof Skipped: OracleProxy SymbolProviders (max_values: None, max_size: None, mode: Measured)
	/// Storage: Band SymbolRates (r:1 w:0)
	/// Proof Skipped: Band SymbolRates (max_values: None, max_size: None, mode: Measured)
	/// Storage: XSTPool ReferenceAssetId (r:1 w:0)
	/// Proof Skipped: XSTPool ReferenceAssetId (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: PriceTools PriceInfos (r:2 w:0)
	/// Proof Skipped: PriceTools PriceInfos (max_values: None, max_size: None, mode: Measured)
	/// Storage: XSTPool SyntheticBaseAssetFloorPrice (r:1 w:0)
	/// Proof Skipped: XSTPool SyntheticBaseAssetFloorPrice (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Timestamp Now (r:1 w:0)
	/// Proof: Timestamp Now (max_values: Some(1), max_size: Some(8), added: 503, mode: MaxEncodedLen)
	/// Storage: Permissions Permissions (r:3 w:0)
	/// Proof Skipped: Permissions Permissions (max_values: None, max_size: None, mode: Measured)
	/// Storage: Tokens Accounts (r:2 w:2)
	/// Proof: Tokens Accounts (max_values: None, max_size: Some(136), added: 2611, mode: MaxEncodedLen)
	/// Storage: Tokens TotalIssuance (r:2 w:2)
	/// Proof: Tokens TotalIssuance (max_values: None, max_size: Some(56), added: 2531, mode: MaxEncodedLen)
	/// Storage: Assets AssetInfos (r:1 w:0)
	/// Proof Skipped: Assets AssetInfos (max_values: None, max_size: None, mode: Measured)
	/// Storage: System Account (r:1 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	fn exchange() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `4819`
		//  Estimated: `80521`
		// Minimum execution time: 169_914_000 picoseconds.
		Weight::from_parts(171_470_000, 80521)
			.saturating_add(RocksDbWeight::get().reads(18_u64))
			.saturating_add(RocksDbWeight::get().writes(5_u64))
	}
}
