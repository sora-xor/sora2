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

//! Autogenerated weights for kensetsu
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-12-13, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `PC-yex`, CPU: `11th Gen Intel(R) Core(TM) i7-11700K @ 3.60GHz`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("local"), DB CACHE: 1024

// Executed Command:
// target/release/framenode
// benchmark
// pallet
// --chain=local
// --execution=wasm
// --wasm-execution=compiled
// --pallet
// kensetsu
// --extrinsic
// *
// --steps
// 50
// --repeat
// 20
// --output
// ./pallets/kensetsu/src/weights.rs
// --template=./misc/pallet-weight-template.hbs
// --header=./misc/file_header.txt

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use core::marker::PhantomData;

/// Weight functions needed for kensetsu.
pub trait WeightInfo {
	fn create_cdp() -> Weight;
	fn close_cdp() -> Weight;
	fn deposit_collateral() -> Weight;
	fn update_collateral_risk_parameters() -> Weight;
	fn update_hard_cap_total_supply() -> Weight;
	fn update_liquidation_penalty() -> Weight;
	fn withdraw_profit() -> Weight;
	fn donate() -> Weight;
}

/// Weights for kensetsu using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	/// Storage: Kensetsu CollateralTypes (r:1 w:0)
	/// Proof: Kensetsu CollateralTypes (max_values: None, max_size: Some(84), added: 2559, mode: MaxEncodedLen)
	/// Storage: Kensetsu NextCDPId (r:1 w:1)
	/// Proof: Kensetsu NextCDPId (max_values: Some(1), max_size: Some(32), added: 527, mode: MaxEncodedLen)
	/// Storage: Timestamp Now (r:1 w:0)
	/// Proof: Timestamp Now (max_values: Some(1), max_size: Some(8), added: 503, mode: MaxEncodedLen)
	/// Storage: Kensetsu Treasury (r:0 w:1)
	/// Proof: Kensetsu Treasury (max_values: None, max_size: Some(136), added: 2611, mode: MaxEncodedLen)
	fn create_cdp() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `278`
		//  Estimated: `3589`
		// Minimum execution time: 15_187_000 picoseconds.
		Weight::from_parts(15_945_000, 3589)
			.saturating_add(T::DbWeight::get().reads(3_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
	/// Storage: Kensetsu Treasury (r:1 w:1)
	/// Proof: Kensetsu Treasury (max_values: None, max_size: Some(136), added: 2611, mode: MaxEncodedLen)
	/// Storage: Timestamp Now (r:1 w:0)
	/// Proof: Timestamp Now (max_values: Some(1), max_size: Some(8), added: 503, mode: MaxEncodedLen)
	/// Storage: Kensetsu CollateralTypes (r:1 w:0)
	/// Proof: Kensetsu CollateralTypes (max_values: None, max_size: Some(84), added: 2559, mode: MaxEncodedLen)
	/// Storage: Kensetsu BadDebt (r:1 w:0)
	/// Proof: Kensetsu BadDebt (max_values: Some(1), max_size: Some(16), added: 511, mode: MaxEncodedLen)
	/// Storage: Technical TechAccounts (r:1 w:0)
	/// Proof Skipped: Technical TechAccounts (max_values: None, max_size: None, mode: Measured)
	/// Storage: Assets AssetInfos (r:1 w:0)
	/// Proof Skipped: Assets AssetInfos (max_values: None, max_size: None, mode: Measured)
	/// Storage: Permissions Permissions (r:2 w:0)
	/// Proof Skipped: Permissions Permissions (max_values: None, max_size: None, mode: Measured)
	fn close_cdp() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `2126`
		//  Estimated: `22462`
		// Minimum execution time: 51_806_000 picoseconds.
		Weight::from_parts(52_831_000, 22462)
			.saturating_add(T::DbWeight::get().reads(8_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: Kensetsu Treasury (r:1 w:1)
	/// Proof: Kensetsu Treasury (max_values: None, max_size: Some(136), added: 2611, mode: MaxEncodedLen)
	/// Storage: Technical TechAccounts (r:1 w:0)
	/// Proof Skipped: Technical TechAccounts (max_values: None, max_size: None, mode: Measured)
	/// Storage: System Account (r:2 w:2)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	fn deposit_collateral() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1400`
		//  Estimated: `11692`
		// Minimum execution time: 47_122_000 picoseconds.
		Weight::from_parts(49_201_000, 11692)
			.saturating_add(T::DbWeight::get().reads(4_u64))
			.saturating_add(T::DbWeight::get().writes(3_u64))
	}
	/// Storage: Assets AssetOwners (r:1 w:0)
	/// Proof Skipped: Assets AssetOwners (max_values: None, max_size: None, mode: Measured)
	/// Storage: Kensetsu CollateralTypes (r:1 w:1)
	/// Proof: Kensetsu CollateralTypes (max_values: None, max_size: Some(84), added: 2559, mode: MaxEncodedLen)
	fn update_collateral_risk_parameters() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `696`
		//  Estimated: `5730`
		// Minimum execution time: 15_413_000 picoseconds.
		Weight::from_parts(15_987_000, 5730)
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: Kensetsu KusdHardCap (r:1 w:1)
	/// Proof: Kensetsu KusdHardCap (max_values: Some(1), max_size: Some(16), added: 511, mode: MaxEncodedLen)
	fn update_hard_cap_total_supply() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `147`
		//  Estimated: `511`
		// Minimum execution time: 9_148_000 picoseconds.
		Weight::from_parts(9_326_000, 511)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: Kensetsu LiquidationPenalty (r:1 w:1)
	/// Proof: Kensetsu LiquidationPenalty (max_values: Some(1), max_size: Some(1), added: 496, mode: MaxEncodedLen)
	fn update_liquidation_penalty() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `147`
		//  Estimated: `496`
		// Minimum execution time: 9_155_000 picoseconds.
		Weight::from_parts(9_462_000, 496)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: Technical TechAccounts (r:1 w:0)
	/// Proof Skipped: Technical TechAccounts (max_values: None, max_size: None, mode: Measured)
	/// Storage: Tokens Accounts (r:2 w:2)
	/// Proof: Tokens Accounts (max_values: None, max_size: Some(136), added: 2611, mode: MaxEncodedLen)
	/// Storage: System Account (r:2 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	fn withdraw_profit() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1466`
		//  Estimated: `14369`
		// Minimum execution time: 44_723_000 picoseconds.
		Weight::from_parts(45_852_000, 14369)
			.saturating_add(T::DbWeight::get().reads(5_u64))
			.saturating_add(T::DbWeight::get().writes(3_u64))
	}
	/// Storage: Kensetsu BadDebt (r:1 w:1)
	/// Proof: Kensetsu BadDebt (max_values: Some(1), max_size: Some(16), added: 511, mode: MaxEncodedLen)
	/// Storage: Technical TechAccounts (r:1 w:0)
	/// Proof Skipped: Technical TechAccounts (max_values: None, max_size: None, mode: Measured)
	/// Storage: Tokens Accounts (r:2 w:2)
	/// Proof: Tokens Accounts (max_values: None, max_size: Some(136), added: 2611, mode: MaxEncodedLen)
	/// Storage: System Account (r:2 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// Storage: Permissions Permissions (r:2 w:0)
	/// Proof Skipped: Permissions Permissions (max_values: None, max_size: None, mode: Measured)
	/// Storage: Tokens TotalIssuance (r:1 w:1)
	/// Proof: Tokens TotalIssuance (max_values: None, max_size: Some(56), added: 2531, mode: MaxEncodedLen)
	fn donate() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `2249`
		//  Estimated: `25393`
		// Minimum execution time: 71_653_000 picoseconds.
		Weight::from_parts(72_943_000, 25393)
			.saturating_add(T::DbWeight::get().reads(9_u64))
			.saturating_add(T::DbWeight::get().writes(5_u64))
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	/// Storage: Kensetsu CollateralTypes (r:1 w:0)
	/// Proof: Kensetsu CollateralTypes (max_values: None, max_size: Some(84), added: 2559, mode: MaxEncodedLen)
	/// Storage: Kensetsu NextCDPId (r:1 w:1)
	/// Proof: Kensetsu NextCDPId (max_values: Some(1), max_size: Some(32), added: 527, mode: MaxEncodedLen)
	/// Storage: Timestamp Now (r:1 w:0)
	/// Proof: Timestamp Now (max_values: Some(1), max_size: Some(8), added: 503, mode: MaxEncodedLen)
	/// Storage: Kensetsu Treasury (r:0 w:1)
	/// Proof: Kensetsu Treasury (max_values: None, max_size: Some(136), added: 2611, mode: MaxEncodedLen)
	fn create_cdp() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `278`
		//  Estimated: `3589`
		// Minimum execution time: 15_187_000 picoseconds.
		Weight::from_parts(15_945_000, 3589)
			.saturating_add(RocksDbWeight::get().reads(3_u64))
			.saturating_add(RocksDbWeight::get().writes(2_u64))
	}
	/// Storage: Kensetsu Treasury (r:1 w:1)
	/// Proof: Kensetsu Treasury (max_values: None, max_size: Some(136), added: 2611, mode: MaxEncodedLen)
	/// Storage: Timestamp Now (r:1 w:0)
	/// Proof: Timestamp Now (max_values: Some(1), max_size: Some(8), added: 503, mode: MaxEncodedLen)
	/// Storage: Kensetsu CollateralTypes (r:1 w:0)
	/// Proof: Kensetsu CollateralTypes (max_values: None, max_size: Some(84), added: 2559, mode: MaxEncodedLen)
	/// Storage: Kensetsu BadDebt (r:1 w:0)
	/// Proof: Kensetsu BadDebt (max_values: Some(1), max_size: Some(16), added: 511, mode: MaxEncodedLen)
	/// Storage: Technical TechAccounts (r:1 w:0)
	/// Proof Skipped: Technical TechAccounts (max_values: None, max_size: None, mode: Measured)
	/// Storage: Assets AssetInfos (r:1 w:0)
	/// Proof Skipped: Assets AssetInfos (max_values: None, max_size: None, mode: Measured)
	/// Storage: Permissions Permissions (r:2 w:0)
	/// Proof Skipped: Permissions Permissions (max_values: None, max_size: None, mode: Measured)
	fn close_cdp() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `2126`
		//  Estimated: `22462`
		// Minimum execution time: 51_806_000 picoseconds.
		Weight::from_parts(52_831_000, 22462)
			.saturating_add(RocksDbWeight::get().reads(8_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: Kensetsu Treasury (r:1 w:1)
	/// Proof: Kensetsu Treasury (max_values: None, max_size: Some(136), added: 2611, mode: MaxEncodedLen)
	/// Storage: Technical TechAccounts (r:1 w:0)
	/// Proof Skipped: Technical TechAccounts (max_values: None, max_size: None, mode: Measured)
	/// Storage: System Account (r:2 w:2)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	fn deposit_collateral() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1400`
		//  Estimated: `11692`
		// Minimum execution time: 47_122_000 picoseconds.
		Weight::from_parts(49_201_000, 11692)
			.saturating_add(RocksDbWeight::get().reads(4_u64))
			.saturating_add(RocksDbWeight::get().writes(3_u64))
	}
	/// Storage: Assets AssetOwners (r:1 w:0)
	/// Proof Skipped: Assets AssetOwners (max_values: None, max_size: None, mode: Measured)
	/// Storage: Kensetsu CollateralTypes (r:1 w:1)
	/// Proof: Kensetsu CollateralTypes (max_values: None, max_size: Some(84), added: 2559, mode: MaxEncodedLen)
	fn update_collateral_risk_parameters() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `696`
		//  Estimated: `5730`
		// Minimum execution time: 15_413_000 picoseconds.
		Weight::from_parts(15_987_000, 5730)
			.saturating_add(RocksDbWeight::get().reads(2_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: Kensetsu KusdHardCap (r:1 w:1)
	/// Proof: Kensetsu KusdHardCap (max_values: Some(1), max_size: Some(16), added: 511, mode: MaxEncodedLen)
	fn update_hard_cap_total_supply() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `147`
		//  Estimated: `511`
		// Minimum execution time: 9_148_000 picoseconds.
		Weight::from_parts(9_326_000, 511)
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: Kensetsu LiquidationPenalty (r:1 w:1)
	/// Proof: Kensetsu LiquidationPenalty (max_values: Some(1), max_size: Some(1), added: 496, mode: MaxEncodedLen)
	fn update_liquidation_penalty() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `147`
		//  Estimated: `496`
		// Minimum execution time: 9_155_000 picoseconds.
		Weight::from_parts(9_462_000, 496)
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: Technical TechAccounts (r:1 w:0)
	/// Proof Skipped: Technical TechAccounts (max_values: None, max_size: None, mode: Measured)
	/// Storage: Tokens Accounts (r:2 w:2)
	/// Proof: Tokens Accounts (max_values: None, max_size: Some(136), added: 2611, mode: MaxEncodedLen)
	/// Storage: System Account (r:2 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	fn withdraw_profit() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1466`
		//  Estimated: `14369`
		// Minimum execution time: 44_723_000 picoseconds.
		Weight::from_parts(45_852_000, 14369)
			.saturating_add(RocksDbWeight::get().reads(5_u64))
			.saturating_add(RocksDbWeight::get().writes(3_u64))
	}
	/// Storage: Kensetsu BadDebt (r:1 w:1)
	/// Proof: Kensetsu BadDebt (max_values: Some(1), max_size: Some(16), added: 511, mode: MaxEncodedLen)
	/// Storage: Technical TechAccounts (r:1 w:0)
	/// Proof Skipped: Technical TechAccounts (max_values: None, max_size: None, mode: Measured)
	/// Storage: Tokens Accounts (r:2 w:2)
	/// Proof: Tokens Accounts (max_values: None, max_size: Some(136), added: 2611, mode: MaxEncodedLen)
	/// Storage: System Account (r:2 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// Storage: Permissions Permissions (r:2 w:0)
	/// Proof Skipped: Permissions Permissions (max_values: None, max_size: None, mode: Measured)
	/// Storage: Tokens TotalIssuance (r:1 w:1)
	/// Proof: Tokens TotalIssuance (max_values: None, max_size: Some(56), added: 2531, mode: MaxEncodedLen)
	fn donate() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `2249`
		//  Estimated: `25393`
		// Minimum execution time: 71_653_000 picoseconds.
		Weight::from_parts(72_943_000, 25393)
			.saturating_add(RocksDbWeight::get().reads(9_u64))
			.saturating_add(RocksDbWeight::get().writes(5_u64))
	}
}
