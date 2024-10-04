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

//! Autogenerated weights for apollo_platform
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2024-10-04, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `b20179d3e305`, CPU: `Intel(R) Xeon(R) CPU E3-1240 v6 @ 3.70GHz`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("local"), DB CACHE: 1024

// Executed Command:
// /usr/local/bin/framenode
// benchmark
// pallet
// --chain=local
// --steps=50
// --repeat=20
// --pallet=apollo_platform
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --header=./misc/file_header.txt
// --template=./misc/pallet-weight-template.hbs
// --output=./pallets/apollo-platform/src/weights.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use core::marker::PhantomData;

/// Weight functions needed for apollo_platform.
pub trait WeightInfo {
	fn add_pool() -> Weight;
	fn lend() -> Weight;
	fn borrow() -> Weight;
	fn get_rewards() -> Weight;
	fn withdraw() -> Weight;
	fn repay() -> Weight;
	fn change_rewards_amount() -> Weight;
	fn change_rewards_per_block() -> Weight;
	fn liquidate() -> Weight;
	fn remove_pool() -> Weight;
	fn edit_pool_info() -> Weight;
	fn add_collateral() -> Weight;
}

/// Weights for apollo_platform using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	/// Storage: ApolloPlatform AuthorityAccount (r:1 w:0)
	/// Proof Skipped: ApolloPlatform AuthorityAccount (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: ApolloPlatform PoolData (r:2 w:1)
	/// Proof Skipped: ApolloPlatform PoolData (max_values: None, max_size: None, mode: Measured)
	/// Storage: ApolloPlatform LendingRewardsPerBlock (r:1 w:0)
	/// Proof Skipped: ApolloPlatform LendingRewardsPerBlock (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: ApolloPlatform BorrowingRewardsPerBlock (r:1 w:0)
	/// Proof Skipped: ApolloPlatform BorrowingRewardsPerBlock (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: ApolloPlatform PoolsByBlock (r:1 w:1)
	/// Proof Skipped: ApolloPlatform PoolsByBlock (max_values: None, max_size: None, mode: Measured)
	/// Storage: PriceTools PriceInfos (r:1 w:1)
	/// Proof Skipped: PriceTools PriceInfos (max_values: None, max_size: None, mode: Measured)
	/// Storage: PriceTools FastPriceInfos (r:1 w:1)
	/// Proof Skipped: PriceTools FastPriceInfos (max_values: None, max_size: None, mode: Measured)
	fn add_pool() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `569`
		//  Estimated: `17843`
		// Minimum execution time: 77_522_000 picoseconds.
		Weight::from_parts(77_899_000, 17843)
			.saturating_add(T::DbWeight::get().reads(8_u64))
			.saturating_add(T::DbWeight::get().writes(4_u64))
	}
	/// Storage: PriceTools PriceInfos (r:1 w:0)
	/// Proof Skipped: PriceTools PriceInfos (max_values: None, max_size: None, mode: Measured)
	/// Storage: ApolloPlatform PoolData (r:1 w:1)
	/// Proof Skipped: ApolloPlatform PoolData (max_values: None, max_size: None, mode: Measured)
	/// Storage: ApolloPlatform UserLendingInfo (r:1 w:1)
	/// Proof Skipped: ApolloPlatform UserLendingInfo (max_values: None, max_size: None, mode: Measured)
	/// Storage: ExtendedAssets SoulboundAsset (r:1 w:0)
	/// Proof: ExtendedAssets SoulboundAsset (max_values: None, max_size: Some(322091), added: 324566, mode: MaxEncodedLen)
	/// Storage: Assets AssetInfosV2 (r:1 w:0)
	/// Proof Skipped: Assets AssetInfosV2 (max_values: None, max_size: None, mode: Measured)
	/// Storage: System Account (r:2 w:2)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	fn lend() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `2935`
		//  Estimated: `351412`
		// Minimum execution time: 138_468_000 picoseconds.
		Weight::from_parts(141_813_000, 351412)
			.saturating_add(T::DbWeight::get().reads(7_u64))
			.saturating_add(T::DbWeight::get().writes(4_u64))
	}
	/// Storage: PriceTools PriceInfos (r:2 w:0)
	/// Proof Skipped: PriceTools PriceInfos (max_values: None, max_size: None, mode: Measured)
	/// Storage: ApolloPlatform PoolData (r:2 w:2)
	/// Proof Skipped: ApolloPlatform PoolData (max_values: None, max_size: None, mode: Measured)
	/// Storage: ApolloPlatform UserLendingInfo (r:1 w:1)
	/// Proof Skipped: ApolloPlatform UserLendingInfo (max_values: None, max_size: None, mode: Measured)
	/// Storage: ApolloPlatform UserBorrowingInfo (r:1 w:1)
	/// Proof Skipped: ApolloPlatform UserBorrowingInfo (max_values: None, max_size: None, mode: Measured)
	/// Storage: ExtendedAssets SoulboundAsset (r:1 w:0)
	/// Proof: ExtendedAssets SoulboundAsset (max_values: None, max_size: Some(322091), added: 324566, mode: MaxEncodedLen)
	/// Storage: Assets AssetInfosV2 (r:1 w:0)
	/// Proof Skipped: Assets AssetInfosV2 (max_values: None, max_size: None, mode: Measured)
	/// Storage: System Account (r:2 w:2)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	fn borrow() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `4517`
		//  Estimated: `369682`
		// Minimum execution time: 174_341_000 picoseconds.
		Weight::from_parts(179_484_000, 369682)
			.saturating_add(T::DbWeight::get().reads(10_u64))
			.saturating_add(T::DbWeight::get().writes(6_u64))
	}
	/// Storage: ApolloPlatform PoolData (r:1 w:0)
	/// Proof Skipped: ApolloPlatform PoolData (max_values: None, max_size: None, mode: Measured)
	/// Storage: ApolloPlatform UserLendingInfo (r:1 w:1)
	/// Proof Skipped: ApolloPlatform UserLendingInfo (max_values: None, max_size: None, mode: Measured)
	/// Storage: ExtendedAssets SoulboundAsset (r:1 w:0)
	/// Proof: ExtendedAssets SoulboundAsset (max_values: None, max_size: Some(322091), added: 324566, mode: MaxEncodedLen)
	/// Storage: Assets AssetInfosV2 (r:1 w:0)
	/// Proof Skipped: Assets AssetInfosV2 (max_values: None, max_size: None, mode: Measured)
	/// Storage: Tokens Accounts (r:2 w:2)
	/// Proof: Tokens Accounts (max_values: None, max_size: Some(136), added: 2611, mode: MaxEncodedLen)
	/// Storage: System Account (r:1 w:0)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	fn get_rewards() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `2367`
		//  Estimated: `346917`
		// Minimum execution time: 123_646_000 picoseconds.
		Weight::from_parts(127_898_000, 346917)
			.saturating_add(T::DbWeight::get().reads(7_u64))
			.saturating_add(T::DbWeight::get().writes(3_u64))
	}
	/// Storage: ApolloPlatform PoolData (r:1 w:1)
	/// Proof Skipped: ApolloPlatform PoolData (max_values: None, max_size: None, mode: Measured)
	/// Storage: ApolloPlatform UserLendingInfo (r:1 w:1)
	/// Proof Skipped: ApolloPlatform UserLendingInfo (max_values: None, max_size: None, mode: Measured)
	/// Storage: ExtendedAssets SoulboundAsset (r:2 w:0)
	/// Proof: ExtendedAssets SoulboundAsset (max_values: None, max_size: Some(322091), added: 324566, mode: MaxEncodedLen)
	/// Storage: Assets AssetInfosV2 (r:2 w:0)
	/// Proof Skipped: Assets AssetInfosV2 (max_values: None, max_size: None, mode: Measured)
	/// Storage: System Account (r:2 w:2)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// Storage: Tokens Accounts (r:2 w:2)
	/// Proof: Tokens Accounts (max_values: None, max_size: Some(136), added: 2611, mode: MaxEncodedLen)
	fn withdraw() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `2714`
		//  Estimated: `677602`
		// Minimum execution time: 178_970_000 picoseconds.
		Weight::from_parts(184_524_000, 677602)
			.saturating_add(T::DbWeight::get().reads(10_u64))
			.saturating_add(T::DbWeight::get().writes(6_u64))
	}
	/// Storage: ApolloPlatform PoolData (r:2 w:2)
	/// Proof Skipped: ApolloPlatform PoolData (max_values: None, max_size: None, mode: Measured)
	/// Storage: ApolloPlatform UserBorrowingInfo (r:1 w:1)
	/// Proof Skipped: ApolloPlatform UserBorrowingInfo (max_values: None, max_size: None, mode: Measured)
	/// Storage: ExtendedAssets SoulboundAsset (r:4 w:0)
	/// Proof: ExtendedAssets SoulboundAsset (max_values: None, max_size: Some(322091), added: 324566, mode: MaxEncodedLen)
	/// Storage: Assets AssetInfosV2 (r:4 w:0)
	/// Proof Skipped: Assets AssetInfosV2 (max_values: None, max_size: None, mode: Measured)
	/// Storage: System Account (r:8 w:8)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// Storage: Tokens Accounts (r:8 w:8)
	/// Proof: Tokens Accounts (max_values: None, max_size: Some(136), added: 2611, mode: MaxEncodedLen)
	/// Storage: DEXManager DEXInfos (r:1 w:0)
	/// Proof Skipped: DEXManager DEXInfos (max_values: None, max_size: None, mode: Measured)
	/// Storage: XSTPool EnabledSynthetics (r:2 w:0)
	/// Proof Skipped: XSTPool EnabledSynthetics (max_values: None, max_size: None, mode: Measured)
	/// Storage: DEXAPI EnabledSourceTypes (r:1 w:0)
	/// Proof Skipped: DEXAPI EnabledSourceTypes (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: PoolXYK Properties (r:2 w:0)
	/// Proof Skipped: PoolXYK Properties (max_values: None, max_size: None, mode: Measured)
	/// Storage: MulticollateralBondingCurvePool EnabledTargets (r:1 w:0)
	/// Proof Skipped: MulticollateralBondingCurvePool EnabledTargets (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: OrderBook OrderBooks (r:2 w:0)
	/// Proof: OrderBook OrderBooks (max_values: None, max_size: Some(238), added: 2713, mode: MaxEncodedLen)
	/// Storage: TradingPair LockedLiquiditySources (r:1 w:0)
	/// Proof Skipped: TradingPair LockedLiquiditySources (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Technical TechAccounts (r:4 w:0)
	/// Proof Skipped: Technical TechAccounts (max_values: None, max_size: None, mode: Measured)
	/// Storage: PriceTools PriceInfos (r:2 w:2)
	/// Proof Skipped: PriceTools PriceInfos (max_values: None, max_size: None, mode: Measured)
	/// Storage: PriceTools FastPriceInfos (r:2 w:2)
	/// Proof Skipped: PriceTools FastPriceInfos (max_values: None, max_size: None, mode: Measured)
	/// Storage: ApolloPlatform AuthorityAccount (r:1 w:0)
	/// Proof Skipped: ApolloPlatform AuthorityAccount (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: ApolloPlatform TreasuryAccount (r:1 w:0)
	/// Proof Skipped: ApolloPlatform TreasuryAccount (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Tokens TotalIssuance (r:1 w:1)
	/// Proof: Tokens TotalIssuance (max_values: None, max_size: Some(56), added: 2531, mode: MaxEncodedLen)
	/// Storage: PoolXYK Reserves (r:0 w:2)
	/// Proof Skipped: PoolXYK Reserves (max_values: None, max_size: None, mode: Measured)
	fn repay() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `8060`
		//  Estimated: `1520808`
		// Minimum execution time: 1_574_148_000 picoseconds.
		Weight::from_parts(1_634_997_000, 1520808)
			.saturating_add(T::DbWeight::get().reads(48_u64))
			.saturating_add(T::DbWeight::get().writes(26_u64))
	}
	/// Storage: ApolloPlatform AuthorityAccount (r:1 w:0)
	/// Proof Skipped: ApolloPlatform AuthorityAccount (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: ApolloPlatform LendingRewards (r:0 w:1)
	/// Proof Skipped: ApolloPlatform LendingRewards (max_values: Some(1), max_size: None, mode: Measured)
	fn change_rewards_amount() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `114`
		//  Estimated: `723`
		// Minimum execution time: 21_078_000 picoseconds.
		Weight::from_parts(21_617_000, 723)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: ApolloPlatform AuthorityAccount (r:1 w:0)
	/// Proof Skipped: ApolloPlatform AuthorityAccount (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: ApolloPlatform PoolData (r:2 w:1)
	/// Proof Skipped: ApolloPlatform PoolData (max_values: None, max_size: None, mode: Measured)
	/// Storage: ApolloPlatform LendingRewardsPerBlock (r:0 w:1)
	/// Proof Skipped: ApolloPlatform LendingRewardsPerBlock (max_values: Some(1), max_size: None, mode: Measured)
	fn change_rewards_per_block() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `484`
		//  Estimated: `6897`
		// Minimum execution time: 63_698_000 picoseconds.
		Weight::from_parts(64_168_000, 6897)
			.saturating_add(T::DbWeight::get().reads(3_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
	/// Storage: ApolloPlatform UserBorrowingInfo (r:1 w:1)
	/// Proof Skipped: ApolloPlatform UserBorrowingInfo (max_values: None, max_size: None, mode: Measured)
	/// Storage: ApolloPlatform PoolData (r:2 w:2)
	/// Proof Skipped: ApolloPlatform PoolData (max_values: None, max_size: None, mode: Measured)
	/// Storage: PriceTools PriceInfos (r:2 w:1)
	/// Proof Skipped: PriceTools PriceInfos (max_values: None, max_size: None, mode: Measured)
	/// Storage: DEXManager DEXInfos (r:1 w:0)
	/// Proof Skipped: DEXManager DEXInfos (max_values: None, max_size: None, mode: Measured)
	/// Storage: XSTPool EnabledSynthetics (r:2 w:0)
	/// Proof Skipped: XSTPool EnabledSynthetics (max_values: None, max_size: None, mode: Measured)
	/// Storage: DEXAPI EnabledSourceTypes (r:1 w:0)
	/// Proof Skipped: DEXAPI EnabledSourceTypes (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: PoolXYK Properties (r:3 w:0)
	/// Proof Skipped: PoolXYK Properties (max_values: None, max_size: None, mode: Measured)
	/// Storage: MulticollateralBondingCurvePool EnabledTargets (r:1 w:0)
	/// Proof Skipped: MulticollateralBondingCurvePool EnabledTargets (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: OrderBook OrderBooks (r:3 w:0)
	/// Proof: OrderBook OrderBooks (max_values: None, max_size: Some(238), added: 2713, mode: MaxEncodedLen)
	/// Storage: TradingPair LockedLiquiditySources (r:1 w:0)
	/// Proof Skipped: TradingPair LockedLiquiditySources (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: System Account (r:5 w:3)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// Storage: Tokens Accounts (r:4 w:2)
	/// Proof: Tokens Accounts (max_values: None, max_size: Some(136), added: 2611, mode: MaxEncodedLen)
	/// Storage: ApolloPlatform AuthorityAccount (r:1 w:0)
	/// Proof Skipped: ApolloPlatform AuthorityAccount (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: ExtendedAssets SoulboundAsset (r:3 w:0)
	/// Proof: ExtendedAssets SoulboundAsset (max_values: None, max_size: Some(322091), added: 324566, mode: MaxEncodedLen)
	/// Storage: Assets AssetInfosV2 (r:3 w:0)
	/// Proof Skipped: Assets AssetInfosV2 (max_values: None, max_size: None, mode: Measured)
	/// Storage: ApolloPlatform TreasuryAccount (r:1 w:0)
	/// Proof Skipped: ApolloPlatform TreasuryAccount (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Technical TechAccounts (r:1 w:0)
	/// Proof Skipped: Technical TechAccounts (max_values: None, max_size: None, mode: Measured)
	/// Storage: PriceTools FastPriceInfos (r:1 w:1)
	/// Proof Skipped: PriceTools FastPriceInfos (max_values: None, max_size: None, mode: Measured)
	/// Storage: PoolXYK Reserves (r:0 w:1)
	/// Proof Skipped: PoolXYK Reserves (max_values: None, max_size: None, mode: Measured)
	fn liquidate() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `11338`
		//  Estimated: `1217441`
		// Minimum execution time: 1_198_348_000 picoseconds.
		Weight::from_parts(1_231_687_000, 1217441)
			.saturating_add(T::DbWeight::get().reads(36_u64))
			.saturating_add(T::DbWeight::get().writes(11_u64))
	}
	/// Storage: ApolloPlatform AuthorityAccount (r:1 w:0)
	/// Proof Skipped: ApolloPlatform AuthorityAccount (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: ApolloPlatform PoolData (r:2 w:1)
	/// Proof Skipped: ApolloPlatform PoolData (max_values: None, max_size: None, mode: Measured)
	/// Storage: ApolloPlatform LendingRewardsPerBlock (r:1 w:0)
	/// Proof Skipped: ApolloPlatform LendingRewardsPerBlock (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: ApolloPlatform BorrowingRewardsPerBlock (r:1 w:0)
	/// Proof Skipped: ApolloPlatform BorrowingRewardsPerBlock (max_values: Some(1), max_size: None, mode: Measured)
	fn remove_pool() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `484`
		//  Estimated: `8371`
		// Minimum execution time: 70_808_000 picoseconds.
		Weight::from_parts(71_315_000, 8371)
			.saturating_add(T::DbWeight::get().reads(5_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: ApolloPlatform AuthorityAccount (r:1 w:0)
	/// Proof Skipped: ApolloPlatform AuthorityAccount (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: ApolloPlatform PoolData (r:1 w:1)
	/// Proof Skipped: ApolloPlatform PoolData (max_values: None, max_size: None, mode: Measured)
	fn edit_pool_info() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `481`
		//  Estimated: `3932`
		// Minimum execution time: 30_688_000 picoseconds.
		Weight::from_parts(31_149_000, 3932)
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: ApolloPlatform PoolData (r:2 w:1)
	/// Proof Skipped: ApolloPlatform PoolData (max_values: None, max_size: None, mode: Measured)
	/// Storage: ApolloPlatform UserLendingInfo (r:1 w:1)
	/// Proof Skipped: ApolloPlatform UserLendingInfo (max_values: None, max_size: None, mode: Measured)
	/// Storage: ApolloPlatform UserBorrowingInfo (r:1 w:1)
	/// Proof Skipped: ApolloPlatform UserBorrowingInfo (max_values: None, max_size: None, mode: Measured)
	fn add_collateral() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1210`
		//  Estimated: `13530`
		// Minimum execution time: 68_981_000 picoseconds.
		Weight::from_parts(71_008_000, 13530)
			.saturating_add(T::DbWeight::get().reads(4_u64))
			.saturating_add(T::DbWeight::get().writes(3_u64))
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	/// Storage: ApolloPlatform AuthorityAccount (r:1 w:0)
	/// Proof Skipped: ApolloPlatform AuthorityAccount (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: ApolloPlatform PoolData (r:2 w:1)
	/// Proof Skipped: ApolloPlatform PoolData (max_values: None, max_size: None, mode: Measured)
	/// Storage: ApolloPlatform LendingRewardsPerBlock (r:1 w:0)
	/// Proof Skipped: ApolloPlatform LendingRewardsPerBlock (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: ApolloPlatform BorrowingRewardsPerBlock (r:1 w:0)
	/// Proof Skipped: ApolloPlatform BorrowingRewardsPerBlock (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: ApolloPlatform PoolsByBlock (r:1 w:1)
	/// Proof Skipped: ApolloPlatform PoolsByBlock (max_values: None, max_size: None, mode: Measured)
	/// Storage: PriceTools PriceInfos (r:1 w:1)
	/// Proof Skipped: PriceTools PriceInfos (max_values: None, max_size: None, mode: Measured)
	/// Storage: PriceTools FastPriceInfos (r:1 w:1)
	/// Proof Skipped: PriceTools FastPriceInfos (max_values: None, max_size: None, mode: Measured)
	fn add_pool() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `569`
		//  Estimated: `17843`
		// Minimum execution time: 77_522_000 picoseconds.
		Weight::from_parts(77_899_000, 17843)
			.saturating_add(RocksDbWeight::get().reads(8_u64))
			.saturating_add(RocksDbWeight::get().writes(4_u64))
	}
	/// Storage: PriceTools PriceInfos (r:1 w:0)
	/// Proof Skipped: PriceTools PriceInfos (max_values: None, max_size: None, mode: Measured)
	/// Storage: ApolloPlatform PoolData (r:1 w:1)
	/// Proof Skipped: ApolloPlatform PoolData (max_values: None, max_size: None, mode: Measured)
	/// Storage: ApolloPlatform UserLendingInfo (r:1 w:1)
	/// Proof Skipped: ApolloPlatform UserLendingInfo (max_values: None, max_size: None, mode: Measured)
	/// Storage: ExtendedAssets SoulboundAsset (r:1 w:0)
	/// Proof: ExtendedAssets SoulboundAsset (max_values: None, max_size: Some(322091), added: 324566, mode: MaxEncodedLen)
	/// Storage: Assets AssetInfosV2 (r:1 w:0)
	/// Proof Skipped: Assets AssetInfosV2 (max_values: None, max_size: None, mode: Measured)
	/// Storage: System Account (r:2 w:2)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	fn lend() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `2935`
		//  Estimated: `351412`
		// Minimum execution time: 138_468_000 picoseconds.
		Weight::from_parts(141_813_000, 351412)
			.saturating_add(RocksDbWeight::get().reads(7_u64))
			.saturating_add(RocksDbWeight::get().writes(4_u64))
	}
	/// Storage: PriceTools PriceInfos (r:2 w:0)
	/// Proof Skipped: PriceTools PriceInfos (max_values: None, max_size: None, mode: Measured)
	/// Storage: ApolloPlatform PoolData (r:2 w:2)
	/// Proof Skipped: ApolloPlatform PoolData (max_values: None, max_size: None, mode: Measured)
	/// Storage: ApolloPlatform UserLendingInfo (r:1 w:1)
	/// Proof Skipped: ApolloPlatform UserLendingInfo (max_values: None, max_size: None, mode: Measured)
	/// Storage: ApolloPlatform UserBorrowingInfo (r:1 w:1)
	/// Proof Skipped: ApolloPlatform UserBorrowingInfo (max_values: None, max_size: None, mode: Measured)
	/// Storage: ExtendedAssets SoulboundAsset (r:1 w:0)
	/// Proof: ExtendedAssets SoulboundAsset (max_values: None, max_size: Some(322091), added: 324566, mode: MaxEncodedLen)
	/// Storage: Assets AssetInfosV2 (r:1 w:0)
	/// Proof Skipped: Assets AssetInfosV2 (max_values: None, max_size: None, mode: Measured)
	/// Storage: System Account (r:2 w:2)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	fn borrow() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `4517`
		//  Estimated: `369682`
		// Minimum execution time: 174_341_000 picoseconds.
		Weight::from_parts(179_484_000, 369682)
			.saturating_add(RocksDbWeight::get().reads(10_u64))
			.saturating_add(RocksDbWeight::get().writes(6_u64))
	}
	/// Storage: ApolloPlatform PoolData (r:1 w:0)
	/// Proof Skipped: ApolloPlatform PoolData (max_values: None, max_size: None, mode: Measured)
	/// Storage: ApolloPlatform UserLendingInfo (r:1 w:1)
	/// Proof Skipped: ApolloPlatform UserLendingInfo (max_values: None, max_size: None, mode: Measured)
	/// Storage: ExtendedAssets SoulboundAsset (r:1 w:0)
	/// Proof: ExtendedAssets SoulboundAsset (max_values: None, max_size: Some(322091), added: 324566, mode: MaxEncodedLen)
	/// Storage: Assets AssetInfosV2 (r:1 w:0)
	/// Proof Skipped: Assets AssetInfosV2 (max_values: None, max_size: None, mode: Measured)
	/// Storage: Tokens Accounts (r:2 w:2)
	/// Proof: Tokens Accounts (max_values: None, max_size: Some(136), added: 2611, mode: MaxEncodedLen)
	/// Storage: System Account (r:1 w:0)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	fn get_rewards() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `2367`
		//  Estimated: `346917`
		// Minimum execution time: 123_646_000 picoseconds.
		Weight::from_parts(127_898_000, 346917)
			.saturating_add(RocksDbWeight::get().reads(7_u64))
			.saturating_add(RocksDbWeight::get().writes(3_u64))
	}
	/// Storage: ApolloPlatform PoolData (r:1 w:1)
	/// Proof Skipped: ApolloPlatform PoolData (max_values: None, max_size: None, mode: Measured)
	/// Storage: ApolloPlatform UserLendingInfo (r:1 w:1)
	/// Proof Skipped: ApolloPlatform UserLendingInfo (max_values: None, max_size: None, mode: Measured)
	/// Storage: ExtendedAssets SoulboundAsset (r:2 w:0)
	/// Proof: ExtendedAssets SoulboundAsset (max_values: None, max_size: Some(322091), added: 324566, mode: MaxEncodedLen)
	/// Storage: Assets AssetInfosV2 (r:2 w:0)
	/// Proof Skipped: Assets AssetInfosV2 (max_values: None, max_size: None, mode: Measured)
	/// Storage: System Account (r:2 w:2)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// Storage: Tokens Accounts (r:2 w:2)
	/// Proof: Tokens Accounts (max_values: None, max_size: Some(136), added: 2611, mode: MaxEncodedLen)
	fn withdraw() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `2714`
		//  Estimated: `677602`
		// Minimum execution time: 178_970_000 picoseconds.
		Weight::from_parts(184_524_000, 677602)
			.saturating_add(RocksDbWeight::get().reads(10_u64))
			.saturating_add(RocksDbWeight::get().writes(6_u64))
	}
	/// Storage: ApolloPlatform PoolData (r:2 w:2)
	/// Proof Skipped: ApolloPlatform PoolData (max_values: None, max_size: None, mode: Measured)
	/// Storage: ApolloPlatform UserBorrowingInfo (r:1 w:1)
	/// Proof Skipped: ApolloPlatform UserBorrowingInfo (max_values: None, max_size: None, mode: Measured)
	/// Storage: ExtendedAssets SoulboundAsset (r:4 w:0)
	/// Proof: ExtendedAssets SoulboundAsset (max_values: None, max_size: Some(322091), added: 324566, mode: MaxEncodedLen)
	/// Storage: Assets AssetInfosV2 (r:4 w:0)
	/// Proof Skipped: Assets AssetInfosV2 (max_values: None, max_size: None, mode: Measured)
	/// Storage: System Account (r:8 w:8)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// Storage: Tokens Accounts (r:8 w:8)
	/// Proof: Tokens Accounts (max_values: None, max_size: Some(136), added: 2611, mode: MaxEncodedLen)
	/// Storage: DEXManager DEXInfos (r:1 w:0)
	/// Proof Skipped: DEXManager DEXInfos (max_values: None, max_size: None, mode: Measured)
	/// Storage: XSTPool EnabledSynthetics (r:2 w:0)
	/// Proof Skipped: XSTPool EnabledSynthetics (max_values: None, max_size: None, mode: Measured)
	/// Storage: DEXAPI EnabledSourceTypes (r:1 w:0)
	/// Proof Skipped: DEXAPI EnabledSourceTypes (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: PoolXYK Properties (r:2 w:0)
	/// Proof Skipped: PoolXYK Properties (max_values: None, max_size: None, mode: Measured)
	/// Storage: MulticollateralBondingCurvePool EnabledTargets (r:1 w:0)
	/// Proof Skipped: MulticollateralBondingCurvePool EnabledTargets (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: OrderBook OrderBooks (r:2 w:0)
	/// Proof: OrderBook OrderBooks (max_values: None, max_size: Some(238), added: 2713, mode: MaxEncodedLen)
	/// Storage: TradingPair LockedLiquiditySources (r:1 w:0)
	/// Proof Skipped: TradingPair LockedLiquiditySources (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Technical TechAccounts (r:4 w:0)
	/// Proof Skipped: Technical TechAccounts (max_values: None, max_size: None, mode: Measured)
	/// Storage: PriceTools PriceInfos (r:2 w:2)
	/// Proof Skipped: PriceTools PriceInfos (max_values: None, max_size: None, mode: Measured)
	/// Storage: PriceTools FastPriceInfos (r:2 w:2)
	/// Proof Skipped: PriceTools FastPriceInfos (max_values: None, max_size: None, mode: Measured)
	/// Storage: ApolloPlatform AuthorityAccount (r:1 w:0)
	/// Proof Skipped: ApolloPlatform AuthorityAccount (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: ApolloPlatform TreasuryAccount (r:1 w:0)
	/// Proof Skipped: ApolloPlatform TreasuryAccount (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Tokens TotalIssuance (r:1 w:1)
	/// Proof: Tokens TotalIssuance (max_values: None, max_size: Some(56), added: 2531, mode: MaxEncodedLen)
	/// Storage: PoolXYK Reserves (r:0 w:2)
	/// Proof Skipped: PoolXYK Reserves (max_values: None, max_size: None, mode: Measured)
	fn repay() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `8060`
		//  Estimated: `1520808`
		// Minimum execution time: 1_574_148_000 picoseconds.
		Weight::from_parts(1_634_997_000, 1520808)
			.saturating_add(RocksDbWeight::get().reads(48_u64))
			.saturating_add(RocksDbWeight::get().writes(26_u64))
	}
	/// Storage: ApolloPlatform AuthorityAccount (r:1 w:0)
	/// Proof Skipped: ApolloPlatform AuthorityAccount (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: ApolloPlatform LendingRewards (r:0 w:1)
	/// Proof Skipped: ApolloPlatform LendingRewards (max_values: Some(1), max_size: None, mode: Measured)
	fn change_rewards_amount() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `114`
		//  Estimated: `723`
		// Minimum execution time: 21_078_000 picoseconds.
		Weight::from_parts(21_617_000, 723)
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: ApolloPlatform AuthorityAccount (r:1 w:0)
	/// Proof Skipped: ApolloPlatform AuthorityAccount (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: ApolloPlatform PoolData (r:2 w:1)
	/// Proof Skipped: ApolloPlatform PoolData (max_values: None, max_size: None, mode: Measured)
	/// Storage: ApolloPlatform LendingRewardsPerBlock (r:0 w:1)
	/// Proof Skipped: ApolloPlatform LendingRewardsPerBlock (max_values: Some(1), max_size: None, mode: Measured)
	fn change_rewards_per_block() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `484`
		//  Estimated: `6897`
		// Minimum execution time: 63_698_000 picoseconds.
		Weight::from_parts(64_168_000, 6897)
			.saturating_add(RocksDbWeight::get().reads(3_u64))
			.saturating_add(RocksDbWeight::get().writes(2_u64))
	}
	/// Storage: ApolloPlatform UserBorrowingInfo (r:1 w:1)
	/// Proof Skipped: ApolloPlatform UserBorrowingInfo (max_values: None, max_size: None, mode: Measured)
	/// Storage: ApolloPlatform PoolData (r:2 w:2)
	/// Proof Skipped: ApolloPlatform PoolData (max_values: None, max_size: None, mode: Measured)
	/// Storage: PriceTools PriceInfos (r:2 w:1)
	/// Proof Skipped: PriceTools PriceInfos (max_values: None, max_size: None, mode: Measured)
	/// Storage: DEXManager DEXInfos (r:1 w:0)
	/// Proof Skipped: DEXManager DEXInfos (max_values: None, max_size: None, mode: Measured)
	/// Storage: XSTPool EnabledSynthetics (r:2 w:0)
	/// Proof Skipped: XSTPool EnabledSynthetics (max_values: None, max_size: None, mode: Measured)
	/// Storage: DEXAPI EnabledSourceTypes (r:1 w:0)
	/// Proof Skipped: DEXAPI EnabledSourceTypes (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: PoolXYK Properties (r:3 w:0)
	/// Proof Skipped: PoolXYK Properties (max_values: None, max_size: None, mode: Measured)
	/// Storage: MulticollateralBondingCurvePool EnabledTargets (r:1 w:0)
	/// Proof Skipped: MulticollateralBondingCurvePool EnabledTargets (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: OrderBook OrderBooks (r:3 w:0)
	/// Proof: OrderBook OrderBooks (max_values: None, max_size: Some(238), added: 2713, mode: MaxEncodedLen)
	/// Storage: TradingPair LockedLiquiditySources (r:1 w:0)
	/// Proof Skipped: TradingPair LockedLiquiditySources (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: System Account (r:5 w:3)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// Storage: Tokens Accounts (r:4 w:2)
	/// Proof: Tokens Accounts (max_values: None, max_size: Some(136), added: 2611, mode: MaxEncodedLen)
	/// Storage: ApolloPlatform AuthorityAccount (r:1 w:0)
	/// Proof Skipped: ApolloPlatform AuthorityAccount (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: ExtendedAssets SoulboundAsset (r:3 w:0)
	/// Proof: ExtendedAssets SoulboundAsset (max_values: None, max_size: Some(322091), added: 324566, mode: MaxEncodedLen)
	/// Storage: Assets AssetInfosV2 (r:3 w:0)
	/// Proof Skipped: Assets AssetInfosV2 (max_values: None, max_size: None, mode: Measured)
	/// Storage: ApolloPlatform TreasuryAccount (r:1 w:0)
	/// Proof Skipped: ApolloPlatform TreasuryAccount (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Technical TechAccounts (r:1 w:0)
	/// Proof Skipped: Technical TechAccounts (max_values: None, max_size: None, mode: Measured)
	/// Storage: PriceTools FastPriceInfos (r:1 w:1)
	/// Proof Skipped: PriceTools FastPriceInfos (max_values: None, max_size: None, mode: Measured)
	/// Storage: PoolXYK Reserves (r:0 w:1)
	/// Proof Skipped: PoolXYK Reserves (max_values: None, max_size: None, mode: Measured)
	fn liquidate() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `11338`
		//  Estimated: `1217441`
		// Minimum execution time: 1_198_348_000 picoseconds.
		Weight::from_parts(1_231_687_000, 1217441)
			.saturating_add(RocksDbWeight::get().reads(36_u64))
			.saturating_add(RocksDbWeight::get().writes(11_u64))
	}
	/// Storage: ApolloPlatform AuthorityAccount (r:1 w:0)
	/// Proof Skipped: ApolloPlatform AuthorityAccount (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: ApolloPlatform PoolData (r:2 w:1)
	/// Proof Skipped: ApolloPlatform PoolData (max_values: None, max_size: None, mode: Measured)
	/// Storage: ApolloPlatform LendingRewardsPerBlock (r:1 w:0)
	/// Proof Skipped: ApolloPlatform LendingRewardsPerBlock (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: ApolloPlatform BorrowingRewardsPerBlock (r:1 w:0)
	/// Proof Skipped: ApolloPlatform BorrowingRewardsPerBlock (max_values: Some(1), max_size: None, mode: Measured)
	fn remove_pool() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `484`
		//  Estimated: `8371`
		// Minimum execution time: 70_808_000 picoseconds.
		Weight::from_parts(71_315_000, 8371)
			.saturating_add(RocksDbWeight::get().reads(5_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: ApolloPlatform AuthorityAccount (r:1 w:0)
	/// Proof Skipped: ApolloPlatform AuthorityAccount (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: ApolloPlatform PoolData (r:1 w:1)
	/// Proof Skipped: ApolloPlatform PoolData (max_values: None, max_size: None, mode: Measured)
	fn edit_pool_info() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `481`
		//  Estimated: `3932`
		// Minimum execution time: 30_688_000 picoseconds.
		Weight::from_parts(31_149_000, 3932)
			.saturating_add(RocksDbWeight::get().reads(2_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: ApolloPlatform PoolData (r:2 w:1)
	/// Proof Skipped: ApolloPlatform PoolData (max_values: None, max_size: None, mode: Measured)
	/// Storage: ApolloPlatform UserLendingInfo (r:1 w:1)
	/// Proof Skipped: ApolloPlatform UserLendingInfo (max_values: None, max_size: None, mode: Measured)
	/// Storage: ApolloPlatform UserBorrowingInfo (r:1 w:1)
	/// Proof Skipped: ApolloPlatform UserBorrowingInfo (max_values: None, max_size: None, mode: Measured)
	fn add_collateral() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1210`
		//  Estimated: `13530`
		// Minimum execution time: 68_981_000 picoseconds.
		Weight::from_parts(71_008_000, 13530)
			.saturating_add(RocksDbWeight::get().reads(4_u64))
			.saturating_add(RocksDbWeight::get().writes(3_u64))
	}
}
