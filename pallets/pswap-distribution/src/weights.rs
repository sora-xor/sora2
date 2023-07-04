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

//! Autogenerated weights for pswap_distribution
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-07-04, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `da61bac67641`, CPU: `Intel(R) Xeon(R) Platinum 8275CL CPU @ 3.00GHz`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("local"), DB CACHE: 1024

// Executed Command:
// /usr/local/bin/framenode
// benchmark
// pallet
// --chain=local
// --steps=50
// --repeat=20
// --pallet=pswap_distribution
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --header=./misc/file_header.txt
// --template=./misc/pallet-weight-template.hbs
// --output=./pallets/pswap-distribution/src/weights.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use core::marker::PhantomData;

/// Weight functions needed for pswap_distribution.
pub trait WeightInfo {
	fn claim_incentive() -> Weight;
	fn on_initialize(a: u32, b: u32, c: u32, ) -> Weight;
}

/// Weights for pswap_distribution using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	/// Storage: PswapDistribution ShareholderAccounts (r:1 w:1)
	/// Proof Skipped: PswapDistribution ShareholderAccounts (max_values: None, max_size: None, mode: Measured)
	/// Storage: PswapDistribution ClaimableShares (r:1 w:1)
	/// Proof Skipped: PswapDistribution ClaimableShares (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Tokens Accounts (r:2 w:2)
	/// Proof: Tokens Accounts (max_values: None, max_size: Some(136), added: 2611, mode: MaxEncodedLen)
	/// Storage: System Account (r:2 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	fn claim_incentive() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1058`
		//  Estimated: `15514`
		// Minimum execution time: 70_791_000 picoseconds.
		Weight::from_parts(71_382_000, 15514)
			.saturating_add(T::DbWeight::get().reads(6_u64))
			.saturating_add(T::DbWeight::get().writes(5_u64))
	}
	/// Storage: PswapDistribution SubscribedAccounts (r:63 w:0)
	/// Proof Skipped: PswapDistribution SubscribedAccounts (max_values: None, max_size: None, mode: Measured)
	/// Storage: DEXManager DEXInfos (r:1 w:0)
	/// Proof Skipped: DEXManager DEXInfos (max_values: None, max_size: None, mode: Measured)
	/// Storage: System Account (r:15 w:5)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// Storage: Assets AssetOwners (r:2 w:0)
	/// Proof Skipped: Assets AssetOwners (max_values: None, max_size: None, mode: Measured)
	/// Storage: PoolXYK TotalIssuances (r:12 w:0)
	/// Proof Skipped: PoolXYK TotalIssuances (max_values: None, max_size: None, mode: Measured)
	/// Storage: Tokens Accounts (r:16 w:16)
	/// Proof: Tokens Accounts (max_values: None, max_size: Some(136), added: 2611, mode: MaxEncodedLen)
	/// Storage: PswapDistribution BuyBackXSTFraction (r:1 w:0)
	/// Proof Skipped: PswapDistribution BuyBackXSTFraction (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: PswapDistribution BurnRate (r:1 w:1)
	/// Proof Skipped: PswapDistribution BurnRate (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Permissions Permissions (r:3 w:0)
	/// Proof Skipped: Permissions Permissions (max_values: None, max_size: None, mode: Measured)
	/// Storage: Tokens TotalIssuance (r:2 w:2)
	/// Proof: Tokens TotalIssuance (max_values: None, max_size: Some(56), added: 2531, mode: MaxEncodedLen)
	/// Storage: VestedRewards TotalRewards (r:1 w:0)
	/// Proof Skipped: VestedRewards TotalRewards (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: PoolXYK PoolProviders (r:114 w:0)
	/// Proof Skipped: PoolXYK PoolProviders (max_values: None, max_size: None, mode: Measured)
	/// Storage: PswapDistribution ShareholderAccounts (r:11 w:11)
	/// Proof Skipped: PswapDistribution ShareholderAccounts (max_values: None, max_size: None, mode: Measured)
	/// Storage: PswapDistribution ClaimableShares (r:1 w:1)
	/// Proof Skipped: PswapDistribution ClaimableShares (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Assets AssetInfos (r:1 w:0)
	/// Proof Skipped: Assets AssetInfos (max_values: None, max_size: None, mode: Measured)
	/// Storage: XSTPool EnabledSynthetics (r:3 w:0)
	/// Proof Skipped: XSTPool EnabledSynthetics (max_values: None, max_size: None, mode: Measured)
	/// Storage: DEXAPI EnabledSourceTypes (r:1 w:0)
	/// Proof Skipped: DEXAPI EnabledSourceTypes (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: PoolXYK Properties (r:2 w:0)
	/// Proof Skipped: PoolXYK Properties (max_values: None, max_size: None, mode: Measured)
	/// Storage: TradingPair LockedLiquiditySources (r:1 w:0)
	/// Proof Skipped: TradingPair LockedLiquiditySources (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Technical TechAccounts (r:4 w:0)
	/// Proof Skipped: Technical TechAccounts (max_values: None, max_size: None, mode: Measured)
	/// Storage: PriceTools PriceInfos (r:2 w:0)
	/// Proof Skipped: PriceTools PriceInfos (max_values: None, max_size: None, mode: Measured)
	/// Storage: PswapDistribution BurnUpdateInfo (r:1 w:0)
	/// Proof Skipped: PswapDistribution BurnUpdateInfo (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: PoolXYK Reserves (r:0 w:2)
	/// Proof Skipped: PoolXYK Reserves (max_values: None, max_size: None, mode: Measured)
	/// The range of component `a` is `[1, 50]`.
	/// The range of component `b` is `[1, 10]`.
	/// The range of component `c` is `[10, 100]`.
	fn on_initialize(a: u32, b: u32, c: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `4852 + a * (244 ±0) + b * (971 ±0) + c * (39 ±0)`
		//  Estimated: `357290 + a * (7113 ±0) + b * (31545 ±4) + c * (3415 ±0)`
		// Minimum execution time: 2_840_747_000 picoseconds.
		Weight::from_parts(567_688_285, 357290)
			// Standard Error: 295_411
			.saturating_add(Weight::from_parts(9_497_511, 0).saturating_mul(a.into()))
			// Standard Error: 1_509_088
			.saturating_add(Weight::from_parts(586_874_262, 0).saturating_mul(b.into()))
			// Standard Error: 159_810
			.saturating_add(Weight::from_parts(12_724_421, 0).saturating_mul(c.into()))
			.saturating_add(T::DbWeight::get().reads(118_u64))
			.saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(a.into())))
			.saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(c.into())))
			.saturating_add(T::DbWeight::get().writes(91_u64))
			.saturating_add(Weight::from_parts(0, 7113).saturating_mul(a.into()))
			.saturating_add(Weight::from_parts(0, 31545).saturating_mul(b.into()))
			.saturating_add(Weight::from_parts(0, 3415).saturating_mul(c.into()))
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	/// Storage: PswapDistribution ShareholderAccounts (r:1 w:1)
	/// Proof Skipped: PswapDistribution ShareholderAccounts (max_values: None, max_size: None, mode: Measured)
	/// Storage: PswapDistribution ClaimableShares (r:1 w:1)
	/// Proof Skipped: PswapDistribution ClaimableShares (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Tokens Accounts (r:2 w:2)
	/// Proof: Tokens Accounts (max_values: None, max_size: Some(136), added: 2611, mode: MaxEncodedLen)
	/// Storage: System Account (r:2 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	fn claim_incentive() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1058`
		//  Estimated: `15514`
		// Minimum execution time: 70_791_000 picoseconds.
		Weight::from_parts(71_382_000, 15514)
			.saturating_add(RocksDbWeight::get().reads(6_u64))
			.saturating_add(RocksDbWeight::get().writes(5_u64))
	}
	/// Storage: PswapDistribution SubscribedAccounts (r:63 w:0)
	/// Proof Skipped: PswapDistribution SubscribedAccounts (max_values: None, max_size: None, mode: Measured)
	/// Storage: DEXManager DEXInfos (r:1 w:0)
	/// Proof Skipped: DEXManager DEXInfos (max_values: None, max_size: None, mode: Measured)
	/// Storage: System Account (r:15 w:5)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// Storage: Assets AssetOwners (r:2 w:0)
	/// Proof Skipped: Assets AssetOwners (max_values: None, max_size: None, mode: Measured)
	/// Storage: PoolXYK TotalIssuances (r:12 w:0)
	/// Proof Skipped: PoolXYK TotalIssuances (max_values: None, max_size: None, mode: Measured)
	/// Storage: Tokens Accounts (r:16 w:16)
	/// Proof: Tokens Accounts (max_values: None, max_size: Some(136), added: 2611, mode: MaxEncodedLen)
	/// Storage: PswapDistribution BuyBackXSTFraction (r:1 w:0)
	/// Proof Skipped: PswapDistribution BuyBackXSTFraction (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: PswapDistribution BurnRate (r:1 w:1)
	/// Proof Skipped: PswapDistribution BurnRate (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Permissions Permissions (r:3 w:0)
	/// Proof Skipped: Permissions Permissions (max_values: None, max_size: None, mode: Measured)
	/// Storage: Tokens TotalIssuance (r:2 w:2)
	/// Proof: Tokens TotalIssuance (max_values: None, max_size: Some(56), added: 2531, mode: MaxEncodedLen)
	/// Storage: VestedRewards TotalRewards (r:1 w:0)
	/// Proof Skipped: VestedRewards TotalRewards (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: PoolXYK PoolProviders (r:114 w:0)
	/// Proof Skipped: PoolXYK PoolProviders (max_values: None, max_size: None, mode: Measured)
	/// Storage: PswapDistribution ShareholderAccounts (r:11 w:11)
	/// Proof Skipped: PswapDistribution ShareholderAccounts (max_values: None, max_size: None, mode: Measured)
	/// Storage: PswapDistribution ClaimableShares (r:1 w:1)
	/// Proof Skipped: PswapDistribution ClaimableShares (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Assets AssetInfos (r:1 w:0)
	/// Proof Skipped: Assets AssetInfos (max_values: None, max_size: None, mode: Measured)
	/// Storage: XSTPool EnabledSynthetics (r:3 w:0)
	/// Proof Skipped: XSTPool EnabledSynthetics (max_values: None, max_size: None, mode: Measured)
	/// Storage: DEXAPI EnabledSourceTypes (r:1 w:0)
	/// Proof Skipped: DEXAPI EnabledSourceTypes (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: PoolXYK Properties (r:2 w:0)
	/// Proof Skipped: PoolXYK Properties (max_values: None, max_size: None, mode: Measured)
	/// Storage: TradingPair LockedLiquiditySources (r:1 w:0)
	/// Proof Skipped: TradingPair LockedLiquiditySources (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Technical TechAccounts (r:4 w:0)
	/// Proof Skipped: Technical TechAccounts (max_values: None, max_size: None, mode: Measured)
	/// Storage: PriceTools PriceInfos (r:2 w:0)
	/// Proof Skipped: PriceTools PriceInfos (max_values: None, max_size: None, mode: Measured)
	/// Storage: PswapDistribution BurnUpdateInfo (r:1 w:0)
	/// Proof Skipped: PswapDistribution BurnUpdateInfo (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: PoolXYK Reserves (r:0 w:2)
	/// Proof Skipped: PoolXYK Reserves (max_values: None, max_size: None, mode: Measured)
	/// The range of component `a` is `[1, 50]`.
	/// The range of component `b` is `[1, 10]`.
	/// The range of component `c` is `[10, 100]`.
	fn on_initialize(a: u32, b: u32, c: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `4852 + a * (244 ±0) + b * (971 ±0) + c * (39 ±0)`
		//  Estimated: `357290 + a * (7113 ±0) + b * (31545 ±4) + c * (3415 ±0)`
		// Minimum execution time: 2_840_747_000 picoseconds.
		Weight::from_parts(567_688_285, 357290)
			// Standard Error: 295_411
			.saturating_add(Weight::from_parts(9_497_511, 0).saturating_mul(a.into()))
			// Standard Error: 1_509_088
			.saturating_add(Weight::from_parts(586_874_262, 0).saturating_mul(b.into()))
			// Standard Error: 159_810
			.saturating_add(Weight::from_parts(12_724_421, 0).saturating_mul(c.into()))
			.saturating_add(RocksDbWeight::get().reads(118_u64))
			.saturating_add(RocksDbWeight::get().reads((1_u64).saturating_mul(a.into())))
			.saturating_add(RocksDbWeight::get().reads((1_u64).saturating_mul(c.into())))
			.saturating_add(RocksDbWeight::get().writes(91_u64))
			.saturating_add(Weight::from_parts(0, 7113).saturating_mul(a.into()))
			.saturating_add(Weight::from_parts(0, 31545).saturating_mul(b.into()))
			.saturating_add(Weight::from_parts(0, 3415).saturating_mul(c.into()))
	}
}
