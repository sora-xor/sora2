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
//! Autogenerated weights for ceres_staking
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
// --pallet=ceres_staking
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --header=./file_header.txt
// --template=./pallet-weight-template.hbs
// --output=./pallets/ceres-staking/src/weights.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use core::marker::PhantomData;

/// Weight functions needed for ceres_staking.
pub trait WeightInfo {
	fn deposit() -> Weight;
	fn withdraw() -> Weight;
	fn change_rewards_remaining() -> Weight;
}

/// Weights for ceres_staking using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	/// Storage: CeresStaking TotalDeposited (r:1 w:1)
	/// Proof Skipped: CeresStaking TotalDeposited (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Tokens Accounts (r:2 w:2)
	/// Proof: Tokens Accounts (max_values: None, max_size: Some(136), added: 2611, mode: MaxEncodedLen)
	/// Storage: System Account (r:2 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// Storage: CeresStaking Stakers (r:1 w:1)
	/// Proof Skipped: CeresStaking Stakers (max_values: None, max_size: None, mode: Measured)
	fn deposit() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `836`
		//  Estimated: `15070`
		// Minimum execution time: 95_834_000 picoseconds.
		Weight::from_parts(97_574_000, 15070)
			.saturating_add(T::DbWeight::get().reads(6_u64))
			.saturating_add(T::DbWeight::get().writes(5_u64))
	}
	/// Storage: CeresStaking Stakers (r:1 w:1)
	/// Proof Skipped: CeresStaking Stakers (max_values: None, max_size: None, mode: Measured)
	/// Storage: Tokens Accounts (r:2 w:2)
	/// Proof: Tokens Accounts (max_values: None, max_size: Some(136), added: 2611, mode: MaxEncodedLen)
	/// Storage: System Account (r:1 w:0)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// Storage: CeresStaking TotalDeposited (r:1 w:1)
	/// Proof Skipped: CeresStaking TotalDeposited (max_values: Some(1), max_size: None, mode: Measured)
	fn withdraw() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1130`
		//  Estimated: `13055`
		// Minimum execution time: 81_524_000 picoseconds.
		Weight::from_parts(81_954_000, 13055)
			.saturating_add(T::DbWeight::get().reads(5_u64))
			.saturating_add(T::DbWeight::get().writes(4_u64))
	}
	/// Storage: CeresStaking AuthorityAccount (r:1 w:0)
	/// Proof Skipped: CeresStaking AuthorityAccount (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: CeresStaking RewardsRemaining (r:0 w:1)
	/// Proof Skipped: CeresStaking RewardsRemaining (max_values: Some(1), max_size: None, mode: Measured)
	fn change_rewards_remaining() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `76`
		//  Estimated: `647`
		// Minimum execution time: 19_141_000 picoseconds.
		Weight::from_parts(19_531_000, 647)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	/// Storage: CeresStaking TotalDeposited (r:1 w:1)
	/// Proof Skipped: CeresStaking TotalDeposited (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Tokens Accounts (r:2 w:2)
	/// Proof: Tokens Accounts (max_values: None, max_size: Some(136), added: 2611, mode: MaxEncodedLen)
	/// Storage: System Account (r:2 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// Storage: CeresStaking Stakers (r:1 w:1)
	/// Proof Skipped: CeresStaking Stakers (max_values: None, max_size: None, mode: Measured)
	fn deposit() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `836`
		//  Estimated: `15070`
		// Minimum execution time: 95_834_000 picoseconds.
		Weight::from_parts(97_574_000, 15070)
			.saturating_add(RocksDbWeight::get().reads(6_u64))
			.saturating_add(RocksDbWeight::get().writes(5_u64))
	}
	/// Storage: CeresStaking Stakers (r:1 w:1)
	/// Proof Skipped: CeresStaking Stakers (max_values: None, max_size: None, mode: Measured)
	/// Storage: Tokens Accounts (r:2 w:2)
	/// Proof: Tokens Accounts (max_values: None, max_size: Some(136), added: 2611, mode: MaxEncodedLen)
	/// Storage: System Account (r:1 w:0)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// Storage: CeresStaking TotalDeposited (r:1 w:1)
	/// Proof Skipped: CeresStaking TotalDeposited (max_values: Some(1), max_size: None, mode: Measured)
	fn withdraw() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1130`
		//  Estimated: `13055`
		// Minimum execution time: 81_524_000 picoseconds.
		Weight::from_parts(81_954_000, 13055)
			.saturating_add(RocksDbWeight::get().reads(5_u64))
			.saturating_add(RocksDbWeight::get().writes(4_u64))
	}
	/// Storage: CeresStaking AuthorityAccount (r:1 w:0)
	/// Proof Skipped: CeresStaking AuthorityAccount (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: CeresStaking RewardsRemaining (r:0 w:1)
	/// Proof Skipped: CeresStaking RewardsRemaining (max_values: Some(1), max_size: None, mode: Measured)
	fn change_rewards_remaining() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `76`
		//  Estimated: `647`
		// Minimum execution time: 19_141_000 picoseconds.
		Weight::from_parts(19_531_000, 647)
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
}