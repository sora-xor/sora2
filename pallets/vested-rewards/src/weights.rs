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

//! Autogenerated weights for vested_rewards
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-06-01, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `58c787de2e36`, CPU: `Intel(R) Xeon(R) Platinum 8275CL CPU @ 3.00GHz`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("local"), DB CACHE: 1024

// Executed Command:
// /usr/local/bin/framenode
// benchmark
// pallet
// --chain=local
// --steps=50
// --repeat=20
// --pallet=vested_rewards
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --header=./misc/file_header.txt
// --template=./misc/pallet-weight-template.hbs
// --output=./pallets/vested-rewards/src/weights.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use core::marker::PhantomData;

/// Weight functions needed for vested_rewards.
pub trait WeightInfo {
	fn claim_rewards() -> Weight;
	fn update_rewards(n: u32, ) -> Weight;
	fn register_crowdloan(m: u32, ) -> Weight;
	fn claim_crowdloan_rewards() -> Weight;
}

/// Weights for vested_rewards using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	/// Storage: VestedRewards Rewards (r:1 w:1)
	/// Proof Skipped: VestedRewards Rewards (max_values: None, max_size: None, mode: Measured)
	/// Storage: Tokens Accounts (r:2 w:2)
	/// Proof: Tokens Accounts (max_values: None, max_size: Some(136), added: 2611, mode: MaxEncodedLen)
	/// Storage: System Account (r:2 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// Storage: VestedRewards TotalRewards (r:1 w:1)
	/// Proof Skipped: VestedRewards TotalRewards (max_values: Some(1), max_size: None, mode: Measured)
	fn claim_rewards() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1164`
		//  Estimated: `15726`
		// Minimum execution time: 85_061_000 picoseconds.
		Weight::from_parts(86_383_000, 15726)
			.saturating_add(T::DbWeight::get().reads(6_u64))
			.saturating_add(T::DbWeight::get().writes(5_u64))
	}
	/// Storage: VestedRewards Rewards (r:100 w:100)
	/// Proof Skipped: VestedRewards Rewards (max_values: None, max_size: None, mode: Measured)
	/// Storage: VestedRewards TotalRewards (r:1 w:1)
	/// Proof Skipped: VestedRewards TotalRewards (max_values: Some(1), max_size: None, mode: Measured)
	/// The range of component `n` is `[0, 100]`.
	fn update_rewards(n: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `42`
		//  Estimated: `579 + n * (2475 ±0)`
		// Minimum execution time: 9_452_000 picoseconds.
		Weight::from_parts(11_860_580, 579)
			// Standard Error: 2_733
			.saturating_add(Weight::from_parts(3_840_847, 0).saturating_mul(n.into()))
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(n.into())))
			.saturating_add(T::DbWeight::get().writes(1_u64))
			.saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(n.into())))
			.saturating_add(Weight::from_parts(0, 2475).saturating_mul(n.into()))
	}
	/// Storage: VestedRewards CrowdloanInfos (r:1 w:1)
	/// Proof Skipped: VestedRewards CrowdloanInfos (max_values: None, max_size: None, mode: Measured)
	/// Storage: Technical TechAccounts (r:1 w:1)
	/// Proof Skipped: Technical TechAccounts (max_values: None, max_size: None, mode: Measured)
	/// Storage: System Account (r:1 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// Storage: VestedRewards CrowdloanUserInfos (r:0 w:999)
	/// Proof Skipped: VestedRewards CrowdloanUserInfos (max_values: None, max_size: None, mode: Measured)
	/// The range of component `m` is `[1, 1000]`.
	fn register_crowdloan(m: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `612`
		//  Estimated: `9389`
		// Minimum execution time: 49_489_000 picoseconds.
		Weight::from_parts(50_191_000, 9389)
			// Standard Error: 3_536
			.saturating_add(Weight::from_parts(3_382_901, 0).saturating_mul(m.into()))
			.saturating_add(T::DbWeight::get().reads(3_u64))
			.saturating_add(T::DbWeight::get().writes(3_u64))
			.saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(m.into())))
	}
	/// Storage: VestedRewards CrowdloanInfos (r:1 w:0)
	/// Proof Skipped: VestedRewards CrowdloanInfos (max_values: None, max_size: None, mode: Measured)
	/// Storage: VestedRewards CrowdloanUserInfos (r:1 w:1)
	/// Proof Skipped: VestedRewards CrowdloanUserInfos (max_values: None, max_size: None, mode: Measured)
	/// Storage: Tokens Accounts (r:10 w:10)
	/// Proof: Tokens Accounts (max_values: None, max_size: Some(136), added: 2611, mode: MaxEncodedLen)
	/// Storage: System Account (r:2 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	fn claim_crowdloan_rewards() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `2402`
		//  Estimated: `41070`
		// Minimum execution time: 259_003_000 picoseconds.
		Weight::from_parts(261_604_000, 41070)
			.saturating_add(T::DbWeight::get().reads(14_u64))
			.saturating_add(T::DbWeight::get().writes(12_u64))
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	/// Storage: VestedRewards Rewards (r:1 w:1)
	/// Proof Skipped: VestedRewards Rewards (max_values: None, max_size: None, mode: Measured)
	/// Storage: Tokens Accounts (r:2 w:2)
	/// Proof: Tokens Accounts (max_values: None, max_size: Some(136), added: 2611, mode: MaxEncodedLen)
	/// Storage: System Account (r:2 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// Storage: VestedRewards TotalRewards (r:1 w:1)
	/// Proof Skipped: VestedRewards TotalRewards (max_values: Some(1), max_size: None, mode: Measured)
	fn claim_rewards() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1164`
		//  Estimated: `15726`
		// Minimum execution time: 85_061_000 picoseconds.
		Weight::from_parts(86_383_000, 15726)
			.saturating_add(RocksDbWeight::get().reads(6_u64))
			.saturating_add(RocksDbWeight::get().writes(5_u64))
	}
	/// Storage: VestedRewards Rewards (r:100 w:100)
	/// Proof Skipped: VestedRewards Rewards (max_values: None, max_size: None, mode: Measured)
	/// Storage: VestedRewards TotalRewards (r:1 w:1)
	/// Proof Skipped: VestedRewards TotalRewards (max_values: Some(1), max_size: None, mode: Measured)
	/// The range of component `n` is `[0, 100]`.
	fn update_rewards(n: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `42`
		//  Estimated: `579 + n * (2475 ±0)`
		// Minimum execution time: 9_452_000 picoseconds.
		Weight::from_parts(11_860_580, 579)
			// Standard Error: 2_733
			.saturating_add(Weight::from_parts(3_840_847, 0).saturating_mul(n.into()))
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().reads((1_u64).saturating_mul(n.into())))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
			.saturating_add(RocksDbWeight::get().writes((1_u64).saturating_mul(n.into())))
			.saturating_add(Weight::from_parts(0, 2475).saturating_mul(n.into()))
	}
	/// Storage: VestedRewards CrowdloanInfos (r:1 w:1)
	/// Proof Skipped: VestedRewards CrowdloanInfos (max_values: None, max_size: None, mode: Measured)
	/// Storage: Technical TechAccounts (r:1 w:1)
	/// Proof Skipped: Technical TechAccounts (max_values: None, max_size: None, mode: Measured)
	/// Storage: System Account (r:1 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// Storage: VestedRewards CrowdloanUserInfos (r:0 w:999)
	/// Proof Skipped: VestedRewards CrowdloanUserInfos (max_values: None, max_size: None, mode: Measured)
	/// The range of component `m` is `[1, 1000]`.
	fn register_crowdloan(m: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `612`
		//  Estimated: `9389`
		// Minimum execution time: 49_489_000 picoseconds.
		Weight::from_parts(50_191_000, 9389)
			// Standard Error: 3_536
			.saturating_add(Weight::from_parts(3_382_901, 0).saturating_mul(m.into()))
			.saturating_add(RocksDbWeight::get().reads(3_u64))
			.saturating_add(RocksDbWeight::get().writes(3_u64))
			.saturating_add(RocksDbWeight::get().writes((1_u64).saturating_mul(m.into())))
	}
	/// Storage: VestedRewards CrowdloanInfos (r:1 w:0)
	/// Proof Skipped: VestedRewards CrowdloanInfos (max_values: None, max_size: None, mode: Measured)
	/// Storage: VestedRewards CrowdloanUserInfos (r:1 w:1)
	/// Proof Skipped: VestedRewards CrowdloanUserInfos (max_values: None, max_size: None, mode: Measured)
	/// Storage: Tokens Accounts (r:10 w:10)
	/// Proof: Tokens Accounts (max_values: None, max_size: Some(136), added: 2611, mode: MaxEncodedLen)
	/// Storage: System Account (r:2 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	fn claim_crowdloan_rewards() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `2402`
		//  Estimated: `41070`
		// Minimum execution time: 259_003_000 picoseconds.
		Weight::from_parts(261_604_000, 41070)
			.saturating_add(RocksDbWeight::get().reads(14_u64))
			.saturating_add(RocksDbWeight::get().writes(12_u64))
	}
}
