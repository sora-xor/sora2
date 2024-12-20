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

//! Autogenerated weights for farming
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2024-12-20, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `587b4c89f56b`, CPU: `Intel(R) Xeon(R) CPU E3-1240 v6 @ 3.70GHz`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("local"), DB CACHE: 1024

// Executed Command:
// /usr/local/bin/framenode
// benchmark
// pallet
// --chain=local
// --steps=50
// --repeat=20
// --pallet=farming
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --header=./misc/file_header.txt
// --template=./misc/pallet-weight-template.hbs
// --output=./pallets/farming/src/weights.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use core::marker::PhantomData;

/// Weight functions needed for farming.
pub trait WeightInfo {
	fn refresh_pool(a: u32, ) -> Weight;
	fn prepare_accounts_for_vesting(a: u32, b: u32, ) -> Weight;
	fn vest_account_rewards(a: u32, ) -> Weight;
	fn set_lp_min_xor_for_bonus_reward() -> Weight;
}

/// Weights for farming using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	/// Storage: Technical TechAccounts (r:1 w:0)
	/// Proof Skipped: Technical TechAccounts (max_values: None, max_size: None, mode: Measured)
	/// Storage: Farming PoolFarmers (r:1 w:0)
	/// Proof Skipped: Farming PoolFarmers (max_values: None, max_size: None, mode: Measured)
	/// Storage: PoolXYK TotalIssuances (r:1 w:0)
	/// Proof Skipped: PoolXYK TotalIssuances (max_values: None, max_size: None, mode: Measured)
	/// Storage: System Account (r:1 w:0)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// Storage: Tokens Accounts (r:1 w:0)
	/// Proof: Tokens Accounts (max_values: None, max_size: Some(136), added: 2611, mode: MaxEncodedLen)
	/// Storage: PoolXYK PoolProviders (r:21 w:0)
	/// Proof Skipped: PoolXYK PoolProviders (max_values: None, max_size: None, mode: Measured)
	/// Storage: Farming LpMinXorForBonusReward (r:1 w:0)
	/// Proof Skipped: Farming LpMinXorForBonusReward (max_values: Some(1), max_size: None, mode: Measured)
	/// The range of component `a` is `[1, 20]`.
	fn refresh_pool(a: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1840 + a * (104 ±0)`
		//  Estimated: `26449 + a * (2715 ±2)`
		// Minimum execution time: 82_678_000 picoseconds.
		Weight::from_parts(75_284_676, 26449)
			// Standard Error: 25_006
			.saturating_add(Weight::from_parts(13_826_287, 0).saturating_mul(a.into()))
			.saturating_add(T::DbWeight::get().reads(7_u64))
			.saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(a.into())))
			.saturating_add(Weight::from_parts(0, 2715).saturating_mul(a.into()))
	}
	/// Storage: Farming PoolFarmers (r:1 w:0)
	/// Proof Skipped: Farming PoolFarmers (max_values: None, max_size: None, mode: Measured)
	/// The range of component `a` is `[1, 29]`.
	/// The range of component `b` is `[1, 43]`.
	fn prepare_accounts_for_vesting(a: u32, b: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `189`
		//  Estimated: `2664`
		// Minimum execution time: 8_500_000 picoseconds.
		Weight::from_parts(5_608_082, 2664)
			// Standard Error: 2_158
			.saturating_add(Weight::from_parts(234_002, 0).saturating_mul(a.into()))
			// Standard Error: 1_446
			.saturating_add(Weight::from_parts(123_896, 0).saturating_mul(b.into()))
			.saturating_add(T::DbWeight::get().reads(1_u64))
	}
	/// The range of component `a` is `[1, 20]`.
	fn vest_account_rewards(a: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 2_433_000 picoseconds.
		Weight::from_parts(2_587_536, 0)
			// Standard Error: 321
			.saturating_add(Weight::from_parts(3_429, 0).saturating_mul(a.into()))
	}
	/// Storage: Farming LpMinXorForBonusReward (r:1 w:1)
	/// Proof Skipped: Farming LpMinXorForBonusReward (max_values: Some(1), max_size: None, mode: Measured)
	fn set_lp_min_xor_for_bonus_reward() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `109`
		//  Estimated: `604`
		// Minimum execution time: 21_240_000 picoseconds.
		Weight::from_parts(21_709_000, 604)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	/// Storage: Technical TechAccounts (r:1 w:0)
	/// Proof Skipped: Technical TechAccounts (max_values: None, max_size: None, mode: Measured)
	/// Storage: Farming PoolFarmers (r:1 w:0)
	/// Proof Skipped: Farming PoolFarmers (max_values: None, max_size: None, mode: Measured)
	/// Storage: PoolXYK TotalIssuances (r:1 w:0)
	/// Proof Skipped: PoolXYK TotalIssuances (max_values: None, max_size: None, mode: Measured)
	/// Storage: System Account (r:1 w:0)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// Storage: Tokens Accounts (r:1 w:0)
	/// Proof: Tokens Accounts (max_values: None, max_size: Some(136), added: 2611, mode: MaxEncodedLen)
	/// Storage: PoolXYK PoolProviders (r:21 w:0)
	/// Proof Skipped: PoolXYK PoolProviders (max_values: None, max_size: None, mode: Measured)
	/// Storage: Farming LpMinXorForBonusReward (r:1 w:0)
	/// Proof Skipped: Farming LpMinXorForBonusReward (max_values: Some(1), max_size: None, mode: Measured)
	/// The range of component `a` is `[1, 20]`.
	fn refresh_pool(a: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1840 + a * (104 ±0)`
		//  Estimated: `26449 + a * (2715 ±2)`
		// Minimum execution time: 82_678_000 picoseconds.
		Weight::from_parts(75_284_676, 26449)
			// Standard Error: 25_006
			.saturating_add(Weight::from_parts(13_826_287, 0).saturating_mul(a.into()))
			.saturating_add(RocksDbWeight::get().reads(7_u64))
			.saturating_add(RocksDbWeight::get().reads((1_u64).saturating_mul(a.into())))
			.saturating_add(Weight::from_parts(0, 2715).saturating_mul(a.into()))
	}
	/// Storage: Farming PoolFarmers (r:1 w:0)
	/// Proof Skipped: Farming PoolFarmers (max_values: None, max_size: None, mode: Measured)
	/// The range of component `a` is `[1, 29]`.
	/// The range of component `b` is `[1, 43]`.
	fn prepare_accounts_for_vesting(a: u32, b: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `189`
		//  Estimated: `2664`
		// Minimum execution time: 8_500_000 picoseconds.
		Weight::from_parts(5_608_082, 2664)
			// Standard Error: 2_158
			.saturating_add(Weight::from_parts(234_002, 0).saturating_mul(a.into()))
			// Standard Error: 1_446
			.saturating_add(Weight::from_parts(123_896, 0).saturating_mul(b.into()))
			.saturating_add(RocksDbWeight::get().reads(1_u64))
	}
	/// The range of component `a` is `[1, 20]`.
	fn vest_account_rewards(a: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 2_433_000 picoseconds.
		Weight::from_parts(2_587_536, 0)
			// Standard Error: 321
			.saturating_add(Weight::from_parts(3_429, 0).saturating_mul(a.into()))
	}
	/// Storage: Farming LpMinXorForBonusReward (r:1 w:1)
	/// Proof Skipped: Farming LpMinXorForBonusReward (max_values: Some(1), max_size: None, mode: Measured)
	fn set_lp_min_xor_for_bonus_reward() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `109`
		//  Estimated: `604`
		// Minimum execution time: 21_240_000 picoseconds.
		Weight::from_parts(21_709_000, 604)
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
}
