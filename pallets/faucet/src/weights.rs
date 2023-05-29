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

//! Autogenerated weights for faucet
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-05-29, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `326dfe77b0d9`, CPU: `Intel(R) Xeon(R) Platinum 8275CL CPU @ 3.00GHz`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("local"), DB CACHE: 1024

// Executed Command:
// /usr/local/bin/framenode
// benchmark
// pallet
// --chain=local
// --steps=50
// --repeat=20
// --pallet=faucet
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --header=./misc/file_header.txt
// --template=./misc/pallet-weight-template.hbs
// --output=./pallets/faucet/src/weights.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use core::marker::PhantomData;

/// Weight functions needed for faucet.
pub trait WeightInfo {
	fn transfer() -> Weight;
	fn reset_rewards() -> Weight;
	fn update_limit() -> Weight;
}

/// Weights for faucet using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	/// Storage: Faucet TransferLimit (r:1 w:0)
	/// Proof Skipped: Faucet TransferLimit (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Faucet Transfers (r:1 w:1)
	/// Proof Skipped: Faucet Transfers (max_values: None, max_size: None, mode: Measured)
	/// Storage: Faucet ReservesAcc (r:1 w:0)
	/// Proof Skipped: Faucet ReservesAcc (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: System Account (r:2 w:2)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// Storage: Referrals ReferrerBalances (r:1 w:0)
	/// Proof Skipped: Referrals ReferrerBalances (max_values: None, max_size: None, mode: Measured)
	/// Storage: Technical TechAccounts (r:1 w:0)
	/// Proof Skipped: Technical TechAccounts (max_values: None, max_size: None, mode: Measured)
	fn transfer() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1068`
		//  Estimated: `18961`
		// Minimum execution time: 91_265_000 picoseconds.
		Weight::from_parts(97_399_000, 18961)
			.saturating_add(T::DbWeight::get().reads(7_u64))
			.saturating_add(T::DbWeight::get().writes(3_u64))
	}
	/// Storage: Rewards ValOwners (r:0 w:3)
	/// Proof Skipped: Rewards ValOwners (max_values: None, max_size: None, mode: Measured)
	/// Storage: Rewards PswapWaifuOwners (r:0 w:1)
	/// Proof Skipped: Rewards PswapWaifuOwners (max_values: None, max_size: None, mode: Measured)
	/// Storage: Rewards PswapFarmOwners (r:0 w:2)
	/// Proof Skipped: Rewards PswapFarmOwners (max_values: None, max_size: None, mode: Measured)
	fn reset_rewards() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `699`
		//  Estimated: `2097`
		// Minimum execution time: 41_948_000 picoseconds.
		Weight::from_parts(43_377_000, 2097)
			.saturating_add(T::DbWeight::get().writes(6_u64))
	}
	/// Storage: Faucet TransferLimit (r:0 w:1)
	/// Proof Skipped: Faucet TransferLimit (max_values: Some(1), max_size: None, mode: Measured)
	fn update_limit() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 10_458_000 picoseconds.
		Weight::from_parts(10_817_000, 0)
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	/// Storage: Faucet TransferLimit (r:1 w:0)
	/// Proof Skipped: Faucet TransferLimit (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Faucet Transfers (r:1 w:1)
	/// Proof Skipped: Faucet Transfers (max_values: None, max_size: None, mode: Measured)
	/// Storage: Faucet ReservesAcc (r:1 w:0)
	/// Proof Skipped: Faucet ReservesAcc (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: System Account (r:2 w:2)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// Storage: Referrals ReferrerBalances (r:1 w:0)
	/// Proof Skipped: Referrals ReferrerBalances (max_values: None, max_size: None, mode: Measured)
	/// Storage: Technical TechAccounts (r:1 w:0)
	/// Proof Skipped: Technical TechAccounts (max_values: None, max_size: None, mode: Measured)
	fn transfer() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1068`
		//  Estimated: `18961`
		// Minimum execution time: 91_265_000 picoseconds.
		Weight::from_parts(97_399_000, 18961)
			.saturating_add(RocksDbWeight::get().reads(7_u64))
			.saturating_add(RocksDbWeight::get().writes(3_u64))
	}
	/// Storage: Rewards ValOwners (r:0 w:3)
	/// Proof Skipped: Rewards ValOwners (max_values: None, max_size: None, mode: Measured)
	/// Storage: Rewards PswapWaifuOwners (r:0 w:1)
	/// Proof Skipped: Rewards PswapWaifuOwners (max_values: None, max_size: None, mode: Measured)
	/// Storage: Rewards PswapFarmOwners (r:0 w:2)
	/// Proof Skipped: Rewards PswapFarmOwners (max_values: None, max_size: None, mode: Measured)
	fn reset_rewards() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `699`
		//  Estimated: `2097`
		// Minimum execution time: 41_948_000 picoseconds.
		Weight::from_parts(43_377_000, 2097)
			.saturating_add(RocksDbWeight::get().writes(6_u64))
	}
	/// Storage: Faucet TransferLimit (r:0 w:1)
	/// Proof Skipped: Faucet TransferLimit (max_values: Some(1), max_size: None, mode: Measured)
	fn update_limit() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 10_458_000 picoseconds.
		Weight::from_parts(10_817_000, 0)
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
}
