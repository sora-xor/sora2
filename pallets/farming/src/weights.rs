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
//! DATE: 2023-04-18, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `TRX40`, CPU: `AMD Ryzen Threadripper 3960X 24-Core Processor`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("local"), DB CACHE: 1024

// Executed Command:
// ./target/release/framenode
// benchmark
// pallet
// --chain=local
// --steps=50
// --repeat=20
// --pallet=farming
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --header=./file_header.txt
// --template=./pallet-weight-template.hbs
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
}

/// Weights for farming using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	/// Storage: Technical TechAccounts (r:1 w:0)
	/// Proof Skipped: Technical TechAccounts (max_values: None, max_size: None, mode: Measured)
	/// Storage: Farming PoolFarmers (r:1 w:1)
	/// Proof Skipped: Farming PoolFarmers (max_values: None, max_size: None, mode: Measured)
	/// Storage: PoolXYK PoolProviders (r:21 w:0)
	/// Proof Skipped: PoolXYK PoolProviders (max_values: None, max_size: None, mode: Measured)
	/// Storage: System Account (r:1 w:0)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// Storage: Tokens Accounts (r:1 w:0)
	/// Proof: Tokens Accounts (max_values: None, max_size: Some(136), added: 2611, mode: MaxEncodedLen)
	/// The range of component `a` is `[1, 20]`.
	fn refresh_pool(a: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1736 + a * (104 ±0)`
		//  Estimated: `18831 + a * (2619 ±2)`
		// Minimum execution time: 60_793_000 picoseconds.
		Weight::from_parts(49_771_134, 18831)
			// Standard Error: 21_081
			.saturating_add(Weight::from_parts(16_199_376, 0).saturating_mul(a.into()))
			.saturating_add(T::DbWeight::get().reads(5_u64))
			.saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(a.into())))
			.saturating_add(T::DbWeight::get().writes(1_u64))
			.saturating_add(Weight::from_parts(0, 2619).saturating_mul(a.into()))
	}
	/// Storage: Farming PoolFarmers (r:30 w:0)
	/// Proof Skipped: Farming PoolFarmers (max_values: None, max_size: None, mode: Measured)
	/// The range of component `a` is `[1, 29]`.
	/// The range of component `b` is `[1, 43]`.
	fn prepare_accounts_for_vesting(a: u32, b: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0 + a * (2292 ±0) + b * (1508 ±0)`
		//  Estimated: `7580 + a * (3219 ±26) + b * (473 ±17)`
		// Minimum execution time: 98_804_000 picoseconds.
		Weight::from_parts(99_275_000, 7580)
			// Standard Error: 829_886
			.saturating_add(Weight::from_parts(31_149_644, 0).saturating_mul(a.into()))
			// Standard Error: 559_407
			.saturating_add(Weight::from_parts(16_392_215, 0).saturating_mul(b.into()))
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(a.into())))
			.saturating_add(Weight::from_parts(0, 3219).saturating_mul(a.into()))
			.saturating_add(Weight::from_parts(0, 473).saturating_mul(b.into()))
	}
	/// Storage: VestedRewards Rewards (r:20 w:20)
	/// Proof Skipped: VestedRewards Rewards (max_values: None, max_size: None, mode: Measured)
	/// Storage: System Account (r:20 w:20)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// Storage: VestedRewards TotalRewards (r:1 w:1)
	/// Proof Skipped: VestedRewards TotalRewards (max_values: Some(1), max_size: None, mode: Measured)
	/// The range of component `a` is `[1, 20]`.
	fn vest_account_rewards(a: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `35 + a * (190 ±0)`
		//  Estimated: `587 + a * (5458 ±0)`
		// Minimum execution time: 27_171_000 picoseconds.
		Weight::from_parts(13_018_152, 587)
			// Standard Error: 16_611
			.saturating_add(Weight::from_parts(16_840_217, 0).saturating_mul(a.into()))
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().reads((2_u64).saturating_mul(a.into())))
			.saturating_add(T::DbWeight::get().writes(1_u64))
			.saturating_add(T::DbWeight::get().writes((2_u64).saturating_mul(a.into())))
			.saturating_add(Weight::from_parts(0, 5458).saturating_mul(a.into()))
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	/// Storage: Technical TechAccounts (r:1 w:0)
	/// Proof Skipped: Technical TechAccounts (max_values: None, max_size: None, mode: Measured)
	/// Storage: Farming PoolFarmers (r:1 w:1)
	/// Proof Skipped: Farming PoolFarmers (max_values: None, max_size: None, mode: Measured)
	/// Storage: PoolXYK PoolProviders (r:21 w:0)
	/// Proof Skipped: PoolXYK PoolProviders (max_values: None, max_size: None, mode: Measured)
	/// Storage: System Account (r:1 w:0)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// Storage: Tokens Accounts (r:1 w:0)
	/// Proof: Tokens Accounts (max_values: None, max_size: Some(136), added: 2611, mode: MaxEncodedLen)
	/// The range of component `a` is `[1, 20]`.
	fn refresh_pool(a: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1736 + a * (104 ±0)`
		//  Estimated: `18831 + a * (2619 ±2)`
		// Minimum execution time: 60_793_000 picoseconds.
		Weight::from_parts(49_771_134, 18831)
			// Standard Error: 21_081
			.saturating_add(Weight::from_parts(16_199_376, 0).saturating_mul(a.into()))
			.saturating_add(RocksDbWeight::get().reads(5_u64))
			.saturating_add(RocksDbWeight::get().reads((1_u64).saturating_mul(a.into())))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
			.saturating_add(Weight::from_parts(0, 2619).saturating_mul(a.into()))
	}
	/// Storage: Farming PoolFarmers (r:30 w:0)
	/// Proof Skipped: Farming PoolFarmers (max_values: None, max_size: None, mode: Measured)
	/// The range of component `a` is `[1, 29]`.
	/// The range of component `b` is `[1, 43]`.
	fn prepare_accounts_for_vesting(a: u32, b: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0 + a * (2292 ±0) + b * (1508 ±0)`
		//  Estimated: `7580 + a * (3219 ±26) + b * (473 ±17)`
		// Minimum execution time: 98_804_000 picoseconds.
		Weight::from_parts(99_275_000, 7580)
			// Standard Error: 829_886
			.saturating_add(Weight::from_parts(31_149_644, 0).saturating_mul(a.into()))
			// Standard Error: 559_407
			.saturating_add(Weight::from_parts(16_392_215, 0).saturating_mul(b.into()))
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().reads((1_u64).saturating_mul(a.into())))
			.saturating_add(Weight::from_parts(0, 3219).saturating_mul(a.into()))
			.saturating_add(Weight::from_parts(0, 473).saturating_mul(b.into()))
	}
	/// Storage: VestedRewards Rewards (r:20 w:20)
	/// Proof Skipped: VestedRewards Rewards (max_values: None, max_size: None, mode: Measured)
	/// Storage: System Account (r:20 w:20)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// Storage: VestedRewards TotalRewards (r:1 w:1)
	/// Proof Skipped: VestedRewards TotalRewards (max_values: Some(1), max_size: None, mode: Measured)
	/// The range of component `a` is `[1, 20]`.
	fn vest_account_rewards(a: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `35 + a * (190 ±0)`
		//  Estimated: `587 + a * (5458 ±0)`
		// Minimum execution time: 27_171_000 picoseconds.
		Weight::from_parts(13_018_152, 587)
			// Standard Error: 16_611
			.saturating_add(Weight::from_parts(16_840_217, 0).saturating_mul(a.into()))
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().reads((2_u64).saturating_mul(a.into())))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
			.saturating_add(RocksDbWeight::get().writes((2_u64).saturating_mul(a.into())))
			.saturating_add(Weight::from_parts(0, 5458).saturating_mul(a.into()))
	}
}