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

//! Autogenerated weights for rewards
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-06-13, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `4338f3947e27`, CPU: `Intel(R) Xeon(R) Platinum 8275CL CPU @ 3.00GHz`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("local"), DB CACHE: 1024

// Executed Command:
// /usr/local/bin/framenode
// benchmark
// pallet
// --chain=local
// --steps=50
// --repeat=20
// --pallet=rewards
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --header=./misc/file_header.txt
// --template=./misc/pallet-weight-template.hbs
// --output=./pallets/rewards/src/weights.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use core::marker::PhantomData;

/// Weight functions needed for rewards.
pub trait WeightInfo {
	fn claim() -> Weight;
	fn add_umi_nfts_receivers(n: u32, ) -> Weight;
}

/// Weights for rewards using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	/// Storage: Rewards ReservesAcc (r:1 w:0)
	/// Proof Skipped: Rewards ReservesAcc (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Rewards ValOwners (r:1 w:1)
	/// Proof Skipped: Rewards ValOwners (max_values: None, max_size: None, mode: Measured)
	/// Storage: Technical TechAccounts (r:1 w:0)
	/// Proof Skipped: Technical TechAccounts (max_values: None, max_size: None, mode: Measured)
	/// Storage: Tokens Accounts (r:4 w:4)
	/// Proof: Tokens Accounts (max_values: None, max_size: Some(136), added: 2611, mode: MaxEncodedLen)
	/// Storage: System Account (r:2 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// Storage: Rewards TotalValRewards (r:1 w:1)
	/// Proof Skipped: Rewards TotalValRewards (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Rewards TotalClaimableVal (r:1 w:1)
	/// Proof Skipped: Rewards TotalClaimableVal (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Rewards PswapFarmOwners (r:1 w:1)
	/// Proof Skipped: Rewards PswapFarmOwners (max_values: None, max_size: None, mode: Measured)
	/// Storage: Rewards PswapWaifuOwners (r:1 w:1)
	/// Proof Skipped: Rewards PswapWaifuOwners (max_values: None, max_size: None, mode: Measured)
	/// Storage: Rewards UmiNftReceivers (r:1 w:0)
	/// Proof Skipped: Rewards UmiNftReceivers (max_values: None, max_size: None, mode: Measured)
	fn claim() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `2242`
		//  Estimated: `47446`
		// Minimum execution time: 552_442_000 picoseconds.
		Weight::from_parts(556_632_000, 47446)
			.saturating_add(T::DbWeight::get().reads(14_u64))
			.saturating_add(T::DbWeight::get().writes(10_u64))
	}
	/// Storage: Rewards UmiNftClaimed (r:999 w:0)
	/// Proof Skipped: Rewards UmiNftClaimed (max_values: None, max_size: None, mode: Measured)
	/// Storage: Rewards UmiNfts (r:1 w:0)
	/// Proof Skipped: Rewards UmiNfts (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Rewards UmiNftReceivers (r:0 w:999)
	/// Proof Skipped: Rewards UmiNftReceivers (max_values: None, max_size: None, mode: Measured)
	/// The range of component `n` is `[1, 1000]`.
	fn add_umi_nfts_receivers(n: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `501`
		//  Estimated: `1998 + n * (2475 ±0)`
		// Minimum execution time: 15_037_000 picoseconds.
		Weight::from_parts(15_278_000, 1998)
			// Standard Error: 3_036
			.saturating_add(Weight::from_parts(4_407_273, 0).saturating_mul(n.into()))
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(n.into())))
			.saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(n.into())))
			.saturating_add(Weight::from_parts(0, 2475).saturating_mul(n.into()))
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	/// Storage: Rewards ReservesAcc (r:1 w:0)
	/// Proof Skipped: Rewards ReservesAcc (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Rewards ValOwners (r:1 w:1)
	/// Proof Skipped: Rewards ValOwners (max_values: None, max_size: None, mode: Measured)
	/// Storage: Technical TechAccounts (r:1 w:0)
	/// Proof Skipped: Technical TechAccounts (max_values: None, max_size: None, mode: Measured)
	/// Storage: Tokens Accounts (r:4 w:4)
	/// Proof: Tokens Accounts (max_values: None, max_size: Some(136), added: 2611, mode: MaxEncodedLen)
	/// Storage: System Account (r:2 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// Storage: Rewards TotalValRewards (r:1 w:1)
	/// Proof Skipped: Rewards TotalValRewards (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Rewards TotalClaimableVal (r:1 w:1)
	/// Proof Skipped: Rewards TotalClaimableVal (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Rewards PswapFarmOwners (r:1 w:1)
	/// Proof Skipped: Rewards PswapFarmOwners (max_values: None, max_size: None, mode: Measured)
	/// Storage: Rewards PswapWaifuOwners (r:1 w:1)
	/// Proof Skipped: Rewards PswapWaifuOwners (max_values: None, max_size: None, mode: Measured)
	/// Storage: Rewards UmiNftReceivers (r:1 w:0)
	/// Proof Skipped: Rewards UmiNftReceivers (max_values: None, max_size: None, mode: Measured)
	fn claim() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `2242`
		//  Estimated: `47446`
		// Minimum execution time: 552_442_000 picoseconds.
		Weight::from_parts(556_632_000, 47446)
			.saturating_add(RocksDbWeight::get().reads(14_u64))
			.saturating_add(RocksDbWeight::get().writes(10_u64))
	}
	/// Storage: Rewards UmiNftClaimed (r:999 w:0)
	/// Proof Skipped: Rewards UmiNftClaimed (max_values: None, max_size: None, mode: Measured)
	/// Storage: Rewards UmiNfts (r:1 w:0)
	/// Proof Skipped: Rewards UmiNfts (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Rewards UmiNftReceivers (r:0 w:999)
	/// Proof Skipped: Rewards UmiNftReceivers (max_values: None, max_size: None, mode: Measured)
	/// The range of component `n` is `[1, 1000]`.
	fn add_umi_nfts_receivers(n: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `501`
		//  Estimated: `1998 + n * (2475 ±0)`
		// Minimum execution time: 15_037_000 picoseconds.
		Weight::from_parts(15_278_000, 1998)
			// Standard Error: 3_036
			.saturating_add(Weight::from_parts(4_407_273, 0).saturating_mul(n.into()))
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().reads((1_u64).saturating_mul(n.into())))
			.saturating_add(RocksDbWeight::get().writes((1_u64).saturating_mul(n.into())))
			.saturating_add(Weight::from_parts(0, 2475).saturating_mul(n.into()))
	}
}
