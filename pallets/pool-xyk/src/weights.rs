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

//! Autogenerated weights for pool_xyk
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2024-06-22, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `dd788c093f61`, CPU: `Intel(R) Xeon(R) Platinum 8124M CPU @ 3.00GHz`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("local"), DB CACHE: 1024

// Executed Command:
// /usr/local/bin/framenode
// benchmark
// pallet
// --chain=local
// --steps=50
// --repeat=20
// --pallet=pool_xyk
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --header=./misc/file_header.txt
// --template=./misc/pallet-weight-template.hbs
// --output=./pallets/pool-xyk/src/weights.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use core::marker::PhantomData;

/// Weight functions needed for pool_xyk.
pub trait WeightInfo {
	fn swap_pair() -> Weight;
	fn can_exchange() -> Weight;
	fn quote() -> Weight;
	fn step_quote(a: u32, ) -> Weight;
	fn deposit_liquidity() -> Weight;
	fn withdraw_liquidity() -> Weight;
	fn initialize_pool() -> Weight;
}

/// Weights for pool_xyk using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	/// Storage: DEXManager DEXInfos (r:1 w:0)
	/// Proof Skipped: DEXManager DEXInfos (max_values: None, max_size: None, mode: Measured)
	/// Storage: Technical TechAccounts (r:2 w:0)
	/// Proof Skipped: Technical TechAccounts (max_values: None, max_size: None, mode: Measured)
	/// Storage: System Account (r:3 w:3)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// Storage: Tokens Accounts (r:2 w:2)
	/// Proof: Tokens Accounts (max_values: None, max_size: Some(136), added: 2611, mode: MaxEncodedLen)
	/// Storage: PriceTools PriceInfos (r:1 w:0)
	/// Proof Skipped: PriceTools PriceInfos (max_values: None, max_size: None, mode: Measured)
	/// Storage: PoolXYK Reserves (r:0 w:1)
	/// Proof Skipped: PoolXYK Reserves (max_values: None, max_size: None, mode: Measured)
	fn swap_pair() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `2612`
		//  Estimated: `33379`
		// Minimum execution time: 191_251_000 picoseconds.
		Weight::from_parts(193_456_000, 33379)
			.saturating_add(T::DbWeight::get().reads(9_u64))
			.saturating_add(T::DbWeight::get().writes(6_u64))
	}
	/// Storage: DEXManager DEXInfos (r:1 w:0)
	/// Proof Skipped: DEXManager DEXInfos (max_values: None, max_size: None, mode: Measured)
	/// Storage: PoolXYK Properties (r:1 w:0)
	/// Proof Skipped: PoolXYK Properties (max_values: None, max_size: None, mode: Measured)
	fn can_exchange() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `680`
		//  Estimated: `6310`
		// Minimum execution time: 19_920_000 picoseconds.
		Weight::from_parts(20_460_000, 6310)
			.saturating_add(T::DbWeight::get().reads(2_u64))
	}
	/// Storage: DEXManager DEXInfos (r:1 w:0)
	/// Proof Skipped: DEXManager DEXInfos (max_values: None, max_size: None, mode: Measured)
	/// Storage: System Account (r:1 w:0)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// Storage: Tokens Accounts (r:1 w:0)
	/// Proof: Tokens Accounts (max_values: None, max_size: Some(136), added: 2611, mode: MaxEncodedLen)
	fn quote() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1029`
		//  Estimated: `8718`
		// Minimum execution time: 33_983_000 picoseconds.
		Weight::from_parts(34_603_000, 8718)
			.saturating_add(T::DbWeight::get().reads(3_u64))
	}
	/// Storage: DEXManager DEXInfos (r:1 w:0)
	/// Proof Skipped: DEXManager DEXInfos (max_values: None, max_size: None, mode: Measured)
	/// Storage: System Account (r:1 w:0)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// Storage: Tokens Accounts (r:1 w:0)
	/// Proof: Tokens Accounts (max_values: None, max_size: Some(136), added: 2611, mode: MaxEncodedLen)
	/// The range of component `a` is `[10, 1000]`.
	fn step_quote(a: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1029`
		//  Estimated: `8718`
		// Minimum execution time: 52_326_000 picoseconds.
		Weight::from_parts(38_838_905, 8718)
			// Standard Error: 908
			.saturating_add(Weight::from_parts(1_867_349, 0).saturating_mul(a.into()))
			.saturating_add(T::DbWeight::get().reads(3_u64))
	}
	/// Storage: Assets AssetInfos (r:2 w:0)
	/// Proof Skipped: Assets AssetInfos (max_values: None, max_size: None, mode: Measured)
	/// Storage: DEXManager DEXInfos (r:1 w:0)
	/// Proof Skipped: DEXManager DEXInfos (max_values: None, max_size: None, mode: Measured)
	/// Storage: Technical TechAccounts (r:1 w:0)
	/// Proof Skipped: Technical TechAccounts (max_values: None, max_size: None, mode: Measured)
	/// Storage: System Account (r:2 w:2)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// Storage: Tokens Accounts (r:2 w:2)
	/// Proof: Tokens Accounts (max_values: None, max_size: Some(136), added: 2611, mode: MaxEncodedLen)
	/// Storage: PoolXYK TotalIssuances (r:1 w:1)
	/// Proof Skipped: PoolXYK TotalIssuances (max_values: None, max_size: None, mode: Measured)
	/// Storage: PoolXYK PoolProviders (r:1 w:1)
	/// Proof Skipped: PoolXYK PoolProviders (max_values: None, max_size: None, mode: Measured)
	/// Storage: PriceTools PriceInfos (r:1 w:0)
	/// Proof Skipped: PriceTools PriceInfos (max_values: None, max_size: None, mode: Measured)
	/// Storage: PoolXYK Reserves (r:0 w:1)
	/// Proof Skipped: PoolXYK Reserves (max_values: None, max_size: None, mode: Measured)
	fn deposit_liquidity() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `3221`
		//  Estimated: `50300`
		// Minimum execution time: 187_435_000 picoseconds.
		Weight::from_parts(189_997_000, 50300)
			.saturating_add(T::DbWeight::get().reads(11_u64))
			.saturating_add(T::DbWeight::get().writes(7_u64))
	}
	/// Storage: Assets AssetInfos (r:2 w:0)
	/// Proof Skipped: Assets AssetInfos (max_values: None, max_size: None, mode: Measured)
	/// Storage: DEXManager DEXInfos (r:1 w:0)
	/// Proof Skipped: DEXManager DEXInfos (max_values: None, max_size: None, mode: Measured)
	/// Storage: Technical TechAccounts (r:1 w:0)
	/// Proof Skipped: Technical TechAccounts (max_values: None, max_size: None, mode: Measured)
	/// Storage: PoolXYK PoolProviders (r:1 w:1)
	/// Proof Skipped: PoolXYK PoolProviders (max_values: None, max_size: None, mode: Measured)
	/// Storage: System Account (r:2 w:2)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// Storage: Tokens Accounts (r:2 w:2)
	/// Proof: Tokens Accounts (max_values: None, max_size: Some(136), added: 2611, mode: MaxEncodedLen)
	/// Storage: PoolXYK TotalIssuances (r:1 w:1)
	/// Proof Skipped: PoolXYK TotalIssuances (max_values: None, max_size: None, mode: Measured)
	/// Storage: CeresLiquidityLocker LockerData (r:1 w:0)
	/// Proof Skipped: CeresLiquidityLocker LockerData (max_values: None, max_size: None, mode: Measured)
	/// Storage: Timestamp Now (r:1 w:0)
	/// Proof: Timestamp Now (max_values: Some(1), max_size: Some(8), added: 503, mode: MaxEncodedLen)
	/// Storage: PoolXYK Properties (r:1 w:0)
	/// Proof Skipped: PoolXYK Properties (max_values: None, max_size: None, mode: Measured)
	/// Storage: DemeterFarmingPlatform UserInfos (r:1 w:0)
	/// Proof Skipped: DemeterFarmingPlatform UserInfos (max_values: None, max_size: None, mode: Measured)
	/// Storage: PriceTools PriceInfos (r:1 w:0)
	/// Proof Skipped: PriceTools PriceInfos (max_values: None, max_size: None, mode: Measured)
	/// Storage: PoolXYK Reserves (r:0 w:1)
	/// Proof Skipped: PoolXYK Reserves (max_values: None, max_size: None, mode: Measured)
	fn withdraw_liquidity() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `3558`
		//  Estimated: `71261`
		// Minimum execution time: 206_583_000 picoseconds.
		Weight::from_parts(209_720_000, 71261)
			.saturating_add(T::DbWeight::get().reads(15_u64))
			.saturating_add(T::DbWeight::get().writes(7_u64))
	}
	/// Storage: DEXManager DEXInfos (r:1 w:0)
	/// Proof Skipped: DEXManager DEXInfos (max_values: None, max_size: None, mode: Measured)
	/// Storage: Assets AssetInfos (r:2 w:0)
	/// Proof Skipped: Assets AssetInfos (max_values: None, max_size: None, mode: Measured)
	/// Storage: Technical TechAccounts (r:2 w:2)
	/// Proof Skipped: Technical TechAccounts (max_values: None, max_size: None, mode: Measured)
	/// Storage: System Account (r:2 w:2)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// Storage: TradingPair EnabledSources (r:1 w:1)
	/// Proof Skipped: TradingPair EnabledSources (max_values: None, max_size: None, mode: Measured)
	/// Storage: PswapDistribution SubscribedAccounts (r:1 w:1)
	/// Proof Skipped: PswapDistribution SubscribedAccounts (max_values: None, max_size: None, mode: Measured)
	/// Storage: Farming Pools (r:1 w:1)
	/// Proof Skipped: Farming Pools (max_values: None, max_size: None, mode: Measured)
	/// Storage: PoolXYK Properties (r:0 w:1)
	/// Proof Skipped: PoolXYK Properties (max_values: None, max_size: None, mode: Measured)
	fn initialize_pool() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `2147`
		//  Estimated: `40035`
		// Minimum execution time: 134_857_000 picoseconds.
		Weight::from_parts(136_524_000, 40035)
			.saturating_add(T::DbWeight::get().reads(10_u64))
			.saturating_add(T::DbWeight::get().writes(8_u64))
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	/// Storage: DEXManager DEXInfos (r:1 w:0)
	/// Proof Skipped: DEXManager DEXInfos (max_values: None, max_size: None, mode: Measured)
	/// Storage: Technical TechAccounts (r:2 w:0)
	/// Proof Skipped: Technical TechAccounts (max_values: None, max_size: None, mode: Measured)
	/// Storage: System Account (r:3 w:3)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// Storage: Tokens Accounts (r:2 w:2)
	/// Proof: Tokens Accounts (max_values: None, max_size: Some(136), added: 2611, mode: MaxEncodedLen)
	/// Storage: PriceTools PriceInfos (r:1 w:0)
	/// Proof Skipped: PriceTools PriceInfos (max_values: None, max_size: None, mode: Measured)
	/// Storage: PoolXYK Reserves (r:0 w:1)
	/// Proof Skipped: PoolXYK Reserves (max_values: None, max_size: None, mode: Measured)
	fn swap_pair() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `2612`
		//  Estimated: `33379`
		// Minimum execution time: 191_251_000 picoseconds.
		Weight::from_parts(193_456_000, 33379)
			.saturating_add(RocksDbWeight::get().reads(9_u64))
			.saturating_add(RocksDbWeight::get().writes(6_u64))
	}
	/// Storage: DEXManager DEXInfos (r:1 w:0)
	/// Proof Skipped: DEXManager DEXInfos (max_values: None, max_size: None, mode: Measured)
	/// Storage: PoolXYK Properties (r:1 w:0)
	/// Proof Skipped: PoolXYK Properties (max_values: None, max_size: None, mode: Measured)
	fn can_exchange() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `680`
		//  Estimated: `6310`
		// Minimum execution time: 19_920_000 picoseconds.
		Weight::from_parts(20_460_000, 6310)
			.saturating_add(RocksDbWeight::get().reads(2_u64))
	}
	/// Storage: DEXManager DEXInfos (r:1 w:0)
	/// Proof Skipped: DEXManager DEXInfos (max_values: None, max_size: None, mode: Measured)
	/// Storage: System Account (r:1 w:0)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// Storage: Tokens Accounts (r:1 w:0)
	/// Proof: Tokens Accounts (max_values: None, max_size: Some(136), added: 2611, mode: MaxEncodedLen)
	fn quote() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1029`
		//  Estimated: `8718`
		// Minimum execution time: 33_983_000 picoseconds.
		Weight::from_parts(34_603_000, 8718)
			.saturating_add(RocksDbWeight::get().reads(3_u64))
	}
	/// Storage: DEXManager DEXInfos (r:1 w:0)
	/// Proof Skipped: DEXManager DEXInfos (max_values: None, max_size: None, mode: Measured)
	/// Storage: System Account (r:1 w:0)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// Storage: Tokens Accounts (r:1 w:0)
	/// Proof: Tokens Accounts (max_values: None, max_size: Some(136), added: 2611, mode: MaxEncodedLen)
	/// The range of component `a` is `[10, 1000]`.
	fn step_quote(a: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1029`
		//  Estimated: `8718`
		// Minimum execution time: 52_326_000 picoseconds.
		Weight::from_parts(38_838_905, 8718)
			// Standard Error: 908
			.saturating_add(Weight::from_parts(1_867_349, 0).saturating_mul(a.into()))
			.saturating_add(RocksDbWeight::get().reads(3_u64))
	}
	/// Storage: Assets AssetInfos (r:2 w:0)
	/// Proof Skipped: Assets AssetInfos (max_values: None, max_size: None, mode: Measured)
	/// Storage: DEXManager DEXInfos (r:1 w:0)
	/// Proof Skipped: DEXManager DEXInfos (max_values: None, max_size: None, mode: Measured)
	/// Storage: Technical TechAccounts (r:1 w:0)
	/// Proof Skipped: Technical TechAccounts (max_values: None, max_size: None, mode: Measured)
	/// Storage: System Account (r:2 w:2)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// Storage: Tokens Accounts (r:2 w:2)
	/// Proof: Tokens Accounts (max_values: None, max_size: Some(136), added: 2611, mode: MaxEncodedLen)
	/// Storage: PoolXYK TotalIssuances (r:1 w:1)
	/// Proof Skipped: PoolXYK TotalIssuances (max_values: None, max_size: None, mode: Measured)
	/// Storage: PoolXYK PoolProviders (r:1 w:1)
	/// Proof Skipped: PoolXYK PoolProviders (max_values: None, max_size: None, mode: Measured)
	/// Storage: PriceTools PriceInfos (r:1 w:0)
	/// Proof Skipped: PriceTools PriceInfos (max_values: None, max_size: None, mode: Measured)
	/// Storage: PoolXYK Reserves (r:0 w:1)
	/// Proof Skipped: PoolXYK Reserves (max_values: None, max_size: None, mode: Measured)
	fn deposit_liquidity() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `3221`
		//  Estimated: `50300`
		// Minimum execution time: 187_435_000 picoseconds.
		Weight::from_parts(189_997_000, 50300)
			.saturating_add(RocksDbWeight::get().reads(11_u64))
			.saturating_add(RocksDbWeight::get().writes(7_u64))
	}
	/// Storage: Assets AssetInfos (r:2 w:0)
	/// Proof Skipped: Assets AssetInfos (max_values: None, max_size: None, mode: Measured)
	/// Storage: DEXManager DEXInfos (r:1 w:0)
	/// Proof Skipped: DEXManager DEXInfos (max_values: None, max_size: None, mode: Measured)
	/// Storage: Technical TechAccounts (r:1 w:0)
	/// Proof Skipped: Technical TechAccounts (max_values: None, max_size: None, mode: Measured)
	/// Storage: PoolXYK PoolProviders (r:1 w:1)
	/// Proof Skipped: PoolXYK PoolProviders (max_values: None, max_size: None, mode: Measured)
	/// Storage: System Account (r:2 w:2)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// Storage: Tokens Accounts (r:2 w:2)
	/// Proof: Tokens Accounts (max_values: None, max_size: Some(136), added: 2611, mode: MaxEncodedLen)
	/// Storage: PoolXYK TotalIssuances (r:1 w:1)
	/// Proof Skipped: PoolXYK TotalIssuances (max_values: None, max_size: None, mode: Measured)
	/// Storage: CeresLiquidityLocker LockerData (r:1 w:0)
	/// Proof Skipped: CeresLiquidityLocker LockerData (max_values: None, max_size: None, mode: Measured)
	/// Storage: Timestamp Now (r:1 w:0)
	/// Proof: Timestamp Now (max_values: Some(1), max_size: Some(8), added: 503, mode: MaxEncodedLen)
	/// Storage: PoolXYK Properties (r:1 w:0)
	/// Proof Skipped: PoolXYK Properties (max_values: None, max_size: None, mode: Measured)
	/// Storage: DemeterFarmingPlatform UserInfos (r:1 w:0)
	/// Proof Skipped: DemeterFarmingPlatform UserInfos (max_values: None, max_size: None, mode: Measured)
	/// Storage: PriceTools PriceInfos (r:1 w:0)
	/// Proof Skipped: PriceTools PriceInfos (max_values: None, max_size: None, mode: Measured)
	/// Storage: PoolXYK Reserves (r:0 w:1)
	/// Proof Skipped: PoolXYK Reserves (max_values: None, max_size: None, mode: Measured)
	fn withdraw_liquidity() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `3558`
		//  Estimated: `71261`
		// Minimum execution time: 206_583_000 picoseconds.
		Weight::from_parts(209_720_000, 71261)
			.saturating_add(RocksDbWeight::get().reads(15_u64))
			.saturating_add(RocksDbWeight::get().writes(7_u64))
	}
	/// Storage: DEXManager DEXInfos (r:1 w:0)
	/// Proof Skipped: DEXManager DEXInfos (max_values: None, max_size: None, mode: Measured)
	/// Storage: Assets AssetInfos (r:2 w:0)
	/// Proof Skipped: Assets AssetInfos (max_values: None, max_size: None, mode: Measured)
	/// Storage: Technical TechAccounts (r:2 w:2)
	/// Proof Skipped: Technical TechAccounts (max_values: None, max_size: None, mode: Measured)
	/// Storage: System Account (r:2 w:2)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// Storage: TradingPair EnabledSources (r:1 w:1)
	/// Proof Skipped: TradingPair EnabledSources (max_values: None, max_size: None, mode: Measured)
	/// Storage: PswapDistribution SubscribedAccounts (r:1 w:1)
	/// Proof Skipped: PswapDistribution SubscribedAccounts (max_values: None, max_size: None, mode: Measured)
	/// Storage: Farming Pools (r:1 w:1)
	/// Proof Skipped: Farming Pools (max_values: None, max_size: None, mode: Measured)
	/// Storage: PoolXYK Properties (r:0 w:1)
	/// Proof Skipped: PoolXYK Properties (max_values: None, max_size: None, mode: Measured)
	fn initialize_pool() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `2147`
		//  Estimated: `40035`
		// Minimum execution time: 134_857_000 picoseconds.
		Weight::from_parts(136_524_000, 40035)
			.saturating_add(RocksDbWeight::get().reads(10_u64))
			.saturating_add(RocksDbWeight::get().writes(8_u64))
	}
}
