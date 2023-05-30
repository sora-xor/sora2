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

//! Autogenerated weights for demeter_farming_platform
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-05-30, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `537e5a713829`, CPU: `Intel(R) Xeon(R) Platinum 8275CL CPU @ 3.00GHz`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("local"), DB CACHE: 1024

// Executed Command:
// /usr/local/bin/framenode
// benchmark
// pallet
// --chain=local
// --steps=50
// --repeat=20
// --pallet=demeter_farming_platform
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --header=./misc/file_header.txt
// --template=./misc/pallet-weight-template.hbs
// --output=./pallets/demeter-farming-platform/src/weights.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use core::marker::PhantomData;

/// Weight functions needed for demeter_farming_platform.
pub trait WeightInfo {
	fn register_token() -> Weight;
	fn add_pool() -> Weight;
	fn deposit() -> Weight;
	fn get_rewards() -> Weight;
	fn withdraw() -> Weight;
	fn remove_pool() -> Weight;
	fn change_pool_multiplier() -> Weight;
	fn change_pool_deposit_fee() -> Weight;
	fn change_total_tokens() -> Weight;
	fn change_info() -> Weight;
	fn change_token_info() -> Weight;
}

/// Weights for demeter_farming_platform using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	/// Storage: DemeterFarmingPlatform AuthorityAccount (r:1 w:0)
	/// Proof Skipped: DemeterFarmingPlatform AuthorityAccount (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: DemeterFarmingPlatform TokenInfos (r:1 w:1)
	/// Proof Skipped: DemeterFarmingPlatform TokenInfos (max_values: None, max_size: None, mode: Measured)
	fn register_token() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `4`
		//  Estimated: `2978`
		// Minimum execution time: 16_180_000 picoseconds.
		Weight::from_parts(16_632_000, 2978)
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: DemeterFarmingPlatform AuthorityAccount (r:1 w:0)
	/// Proof Skipped: DemeterFarmingPlatform AuthorityAccount (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: DemeterFarmingPlatform TokenInfos (r:1 w:1)
	/// Proof Skipped: DemeterFarmingPlatform TokenInfos (max_values: None, max_size: None, mode: Measured)
	/// Storage: DemeterFarmingPlatform Pools (r:1 w:1)
	/// Proof Skipped: DemeterFarmingPlatform Pools (max_values: None, max_size: None, mode: Measured)
	fn add_pool() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `200`
		//  Estimated: `6045`
		// Minimum execution time: 24_689_000 picoseconds.
		Weight::from_parts(25_296_000, 6045)
			.saturating_add(T::DbWeight::get().reads(3_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
	/// Storage: DemeterFarmingPlatform Pools (r:1 w:1)
	/// Proof Skipped: DemeterFarmingPlatform Pools (max_values: None, max_size: None, mode: Measured)
	/// Storage: DemeterFarmingPlatform UserInfos (r:1 w:1)
	/// Proof Skipped: DemeterFarmingPlatform UserInfos (max_values: None, max_size: None, mode: Measured)
	/// Storage: Tokens Accounts (r:2 w:2)
	/// Proof: Tokens Accounts (max_values: None, max_size: Some(136), added: 2611, mode: MaxEncodedLen)
	/// Storage: DemeterFarmingPlatform FeeAccount (r:1 w:0)
	/// Proof Skipped: DemeterFarmingPlatform FeeAccount (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: System Account (r:2 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	fn deposit() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `988`
		//  Estimated: `18837`
		// Minimum execution time: 91_723_000 picoseconds.
		Weight::from_parts(93_031_000, 18837)
			.saturating_add(T::DbWeight::get().reads(7_u64))
			.saturating_add(T::DbWeight::get().writes(5_u64))
	}
	/// Storage: DemeterFarmingPlatform Pools (r:1 w:1)
	/// Proof Skipped: DemeterFarmingPlatform Pools (max_values: None, max_size: None, mode: Measured)
	/// Storage: DemeterFarmingPlatform UserInfos (r:1 w:1)
	/// Proof Skipped: DemeterFarmingPlatform UserInfos (max_values: None, max_size: None, mode: Measured)
	/// Storage: Tokens Accounts (r:2 w:2)
	/// Proof: Tokens Accounts (max_values: None, max_size: Some(136), added: 2611, mode: MaxEncodedLen)
	/// Storage: System Account (r:1 w:0)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	fn get_rewards() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1376`
		//  Estimated: `15527`
		// Minimum execution time: 77_263_000 picoseconds.
		Weight::from_parts(80_077_000, 15527)
			.saturating_add(T::DbWeight::get().reads(5_u64))
			.saturating_add(T::DbWeight::get().writes(4_u64))
	}
	/// Storage: DemeterFarmingPlatform UserInfos (r:1 w:1)
	/// Proof Skipped: DemeterFarmingPlatform UserInfos (max_values: None, max_size: None, mode: Measured)
	/// Storage: Tokens Accounts (r:2 w:2)
	/// Proof: Tokens Accounts (max_values: None, max_size: Some(136), added: 2611, mode: MaxEncodedLen)
	/// Storage: System Account (r:1 w:0)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// Storage: DemeterFarmingPlatform Pools (r:1 w:1)
	/// Proof Skipped: DemeterFarmingPlatform Pools (max_values: None, max_size: None, mode: Measured)
	fn withdraw() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1376`
		//  Estimated: `15527`
		// Minimum execution time: 73_754_000 picoseconds.
		Weight::from_parts(74_851_000, 15527)
			.saturating_add(T::DbWeight::get().reads(5_u64))
			.saturating_add(T::DbWeight::get().writes(4_u64))
	}
	/// Storage: DemeterFarmingPlatform AuthorityAccount (r:1 w:0)
	/// Proof Skipped: DemeterFarmingPlatform AuthorityAccount (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: DemeterFarmingPlatform Pools (r:1 w:1)
	/// Proof Skipped: DemeterFarmingPlatform Pools (max_values: None, max_size: None, mode: Measured)
	fn remove_pool() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `268`
		//  Estimated: `3506`
		// Minimum execution time: 20_517_000 picoseconds.
		Weight::from_parts(21_078_000, 3506)
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: DemeterFarmingPlatform AuthorityAccount (r:1 w:0)
	/// Proof Skipped: DemeterFarmingPlatform AuthorityAccount (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: DemeterFarmingPlatform Pools (r:1 w:1)
	/// Proof Skipped: DemeterFarmingPlatform Pools (max_values: None, max_size: None, mode: Measured)
	/// Storage: DemeterFarmingPlatform TokenInfos (r:1 w:1)
	/// Proof Skipped: DemeterFarmingPlatform TokenInfos (max_values: None, max_size: None, mode: Measured)
	fn change_pool_multiplier() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `427`
		//  Estimated: `6726`
		// Minimum execution time: 27_041_000 picoseconds.
		Weight::from_parts(27_503_000, 6726)
			.saturating_add(T::DbWeight::get().reads(3_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
	/// Storage: DemeterFarmingPlatform AuthorityAccount (r:1 w:0)
	/// Proof Skipped: DemeterFarmingPlatform AuthorityAccount (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: DemeterFarmingPlatform Pools (r:1 w:1)
	/// Proof Skipped: DemeterFarmingPlatform Pools (max_values: None, max_size: None, mode: Measured)
	fn change_pool_deposit_fee() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `268`
		//  Estimated: `3506`
		// Minimum execution time: 20_857_000 picoseconds.
		Weight::from_parts(21_341_000, 3506)
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: DemeterFarmingPlatform AuthorityAccount (r:1 w:0)
	/// Proof Skipped: DemeterFarmingPlatform AuthorityAccount (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: DemeterFarmingPlatform Pools (r:1 w:1)
	/// Proof Skipped: DemeterFarmingPlatform Pools (max_values: None, max_size: None, mode: Measured)
	fn change_total_tokens() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `268`
		//  Estimated: `3506`
		// Minimum execution time: 21_168_000 picoseconds.
		Weight::from_parts(21_724_000, 3506)
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: DemeterFarmingPlatform AuthorityAccount (r:1 w:0)
	/// Proof Skipped: DemeterFarmingPlatform AuthorityAccount (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: DemeterFarmingPlatform UserInfos (r:1 w:1)
	/// Proof Skipped: DemeterFarmingPlatform UserInfos (max_values: None, max_size: None, mode: Measured)
	fn change_info() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `296`
		//  Estimated: `3562`
		// Minimum execution time: 24_008_000 picoseconds.
		Weight::from_parts(24_236_000, 3562)
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: DemeterFarmingPlatform AuthorityAccount (r:1 w:0)
	/// Proof Skipped: DemeterFarmingPlatform AuthorityAccount (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: DemeterFarmingPlatform TokenInfos (r:1 w:1)
	/// Proof Skipped: DemeterFarmingPlatform TokenInfos (max_values: None, max_size: None, mode: Measured)
	fn change_token_info() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `200`
		//  Estimated: `3370`
		// Minimum execution time: 18_780_000 picoseconds.
		Weight::from_parts(19_407_000, 3370)
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	/// Storage: DemeterFarmingPlatform AuthorityAccount (r:1 w:0)
	/// Proof Skipped: DemeterFarmingPlatform AuthorityAccount (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: DemeterFarmingPlatform TokenInfos (r:1 w:1)
	/// Proof Skipped: DemeterFarmingPlatform TokenInfos (max_values: None, max_size: None, mode: Measured)
	fn register_token() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `4`
		//  Estimated: `2978`
		// Minimum execution time: 16_180_000 picoseconds.
		Weight::from_parts(16_632_000, 2978)
			.saturating_add(RocksDbWeight::get().reads(2_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: DemeterFarmingPlatform AuthorityAccount (r:1 w:0)
	/// Proof Skipped: DemeterFarmingPlatform AuthorityAccount (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: DemeterFarmingPlatform TokenInfos (r:1 w:1)
	/// Proof Skipped: DemeterFarmingPlatform TokenInfos (max_values: None, max_size: None, mode: Measured)
	/// Storage: DemeterFarmingPlatform Pools (r:1 w:1)
	/// Proof Skipped: DemeterFarmingPlatform Pools (max_values: None, max_size: None, mode: Measured)
	fn add_pool() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `200`
		//  Estimated: `6045`
		// Minimum execution time: 24_689_000 picoseconds.
		Weight::from_parts(25_296_000, 6045)
			.saturating_add(RocksDbWeight::get().reads(3_u64))
			.saturating_add(RocksDbWeight::get().writes(2_u64))
	}
	/// Storage: DemeterFarmingPlatform Pools (r:1 w:1)
	/// Proof Skipped: DemeterFarmingPlatform Pools (max_values: None, max_size: None, mode: Measured)
	/// Storage: DemeterFarmingPlatform UserInfos (r:1 w:1)
	/// Proof Skipped: DemeterFarmingPlatform UserInfos (max_values: None, max_size: None, mode: Measured)
	/// Storage: Tokens Accounts (r:2 w:2)
	/// Proof: Tokens Accounts (max_values: None, max_size: Some(136), added: 2611, mode: MaxEncodedLen)
	/// Storage: DemeterFarmingPlatform FeeAccount (r:1 w:0)
	/// Proof Skipped: DemeterFarmingPlatform FeeAccount (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: System Account (r:2 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	fn deposit() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `988`
		//  Estimated: `18837`
		// Minimum execution time: 91_723_000 picoseconds.
		Weight::from_parts(93_031_000, 18837)
			.saturating_add(RocksDbWeight::get().reads(7_u64))
			.saturating_add(RocksDbWeight::get().writes(5_u64))
	}
	/// Storage: DemeterFarmingPlatform Pools (r:1 w:1)
	/// Proof Skipped: DemeterFarmingPlatform Pools (max_values: None, max_size: None, mode: Measured)
	/// Storage: DemeterFarmingPlatform UserInfos (r:1 w:1)
	/// Proof Skipped: DemeterFarmingPlatform UserInfos (max_values: None, max_size: None, mode: Measured)
	/// Storage: Tokens Accounts (r:2 w:2)
	/// Proof: Tokens Accounts (max_values: None, max_size: Some(136), added: 2611, mode: MaxEncodedLen)
	/// Storage: System Account (r:1 w:0)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	fn get_rewards() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1376`
		//  Estimated: `15527`
		// Minimum execution time: 77_263_000 picoseconds.
		Weight::from_parts(80_077_000, 15527)
			.saturating_add(RocksDbWeight::get().reads(5_u64))
			.saturating_add(RocksDbWeight::get().writes(4_u64))
	}
	/// Storage: DemeterFarmingPlatform UserInfos (r:1 w:1)
	/// Proof Skipped: DemeterFarmingPlatform UserInfos (max_values: None, max_size: None, mode: Measured)
	/// Storage: Tokens Accounts (r:2 w:2)
	/// Proof: Tokens Accounts (max_values: None, max_size: Some(136), added: 2611, mode: MaxEncodedLen)
	/// Storage: System Account (r:1 w:0)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// Storage: DemeterFarmingPlatform Pools (r:1 w:1)
	/// Proof Skipped: DemeterFarmingPlatform Pools (max_values: None, max_size: None, mode: Measured)
	fn withdraw() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1376`
		//  Estimated: `15527`
		// Minimum execution time: 73_754_000 picoseconds.
		Weight::from_parts(74_851_000, 15527)
			.saturating_add(RocksDbWeight::get().reads(5_u64))
			.saturating_add(RocksDbWeight::get().writes(4_u64))
	}
	/// Storage: DemeterFarmingPlatform AuthorityAccount (r:1 w:0)
	/// Proof Skipped: DemeterFarmingPlatform AuthorityAccount (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: DemeterFarmingPlatform Pools (r:1 w:1)
	/// Proof Skipped: DemeterFarmingPlatform Pools (max_values: None, max_size: None, mode: Measured)
	fn remove_pool() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `268`
		//  Estimated: `3506`
		// Minimum execution time: 20_517_000 picoseconds.
		Weight::from_parts(21_078_000, 3506)
			.saturating_add(RocksDbWeight::get().reads(2_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: DemeterFarmingPlatform AuthorityAccount (r:1 w:0)
	/// Proof Skipped: DemeterFarmingPlatform AuthorityAccount (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: DemeterFarmingPlatform Pools (r:1 w:1)
	/// Proof Skipped: DemeterFarmingPlatform Pools (max_values: None, max_size: None, mode: Measured)
	/// Storage: DemeterFarmingPlatform TokenInfos (r:1 w:1)
	/// Proof Skipped: DemeterFarmingPlatform TokenInfos (max_values: None, max_size: None, mode: Measured)
	fn change_pool_multiplier() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `427`
		//  Estimated: `6726`
		// Minimum execution time: 27_041_000 picoseconds.
		Weight::from_parts(27_503_000, 6726)
			.saturating_add(RocksDbWeight::get().reads(3_u64))
			.saturating_add(RocksDbWeight::get().writes(2_u64))
	}
	/// Storage: DemeterFarmingPlatform AuthorityAccount (r:1 w:0)
	/// Proof Skipped: DemeterFarmingPlatform AuthorityAccount (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: DemeterFarmingPlatform Pools (r:1 w:1)
	/// Proof Skipped: DemeterFarmingPlatform Pools (max_values: None, max_size: None, mode: Measured)
	fn change_pool_deposit_fee() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `268`
		//  Estimated: `3506`
		// Minimum execution time: 20_857_000 picoseconds.
		Weight::from_parts(21_341_000, 3506)
			.saturating_add(RocksDbWeight::get().reads(2_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: DemeterFarmingPlatform AuthorityAccount (r:1 w:0)
	/// Proof Skipped: DemeterFarmingPlatform AuthorityAccount (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: DemeterFarmingPlatform Pools (r:1 w:1)
	/// Proof Skipped: DemeterFarmingPlatform Pools (max_values: None, max_size: None, mode: Measured)
	fn change_total_tokens() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `268`
		//  Estimated: `3506`
		// Minimum execution time: 21_168_000 picoseconds.
		Weight::from_parts(21_724_000, 3506)
			.saturating_add(RocksDbWeight::get().reads(2_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: DemeterFarmingPlatform AuthorityAccount (r:1 w:0)
	/// Proof Skipped: DemeterFarmingPlatform AuthorityAccount (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: DemeterFarmingPlatform UserInfos (r:1 w:1)
	/// Proof Skipped: DemeterFarmingPlatform UserInfos (max_values: None, max_size: None, mode: Measured)
	fn change_info() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `296`
		//  Estimated: `3562`
		// Minimum execution time: 24_008_000 picoseconds.
		Weight::from_parts(24_236_000, 3562)
			.saturating_add(RocksDbWeight::get().reads(2_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: DemeterFarmingPlatform AuthorityAccount (r:1 w:0)
	/// Proof Skipped: DemeterFarmingPlatform AuthorityAccount (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: DemeterFarmingPlatform TokenInfos (r:1 w:1)
	/// Proof Skipped: DemeterFarmingPlatform TokenInfos (max_values: None, max_size: None, mode: Measured)
	fn change_token_info() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `200`
		//  Estimated: `3370`
		// Minimum execution time: 18_780_000 picoseconds.
		Weight::from_parts(19_407_000, 3370)
			.saturating_add(RocksDbWeight::get().reads(2_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
}
