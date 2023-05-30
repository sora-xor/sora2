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

//! Autogenerated weights for ceres_token_locker
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-05-30, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `b161ee0b2048`, CPU: `Intel(R) Xeon(R) Platinum 8275CL CPU @ 3.00GHz`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("local"), DB CACHE: 1024

// Executed Command:
// /usr/local/bin/framenode
// benchmark
// pallet
// --chain=local
// --steps=50
// --repeat=20
// --pallet=ceres_token_locker
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --header=./misc/file_header.txt
// --template=./misc/pallet-weight-template.hbs
// --output=./pallets/ceres-token-locker/src/weights.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use core::marker::PhantomData;

/// Weight functions needed for ceres_token_locker.
pub trait WeightInfo {
	fn lock_tokens() -> Weight;
	fn withdraw_tokens() -> Weight;
	fn change_fee() -> Weight;
}

/// Weights for ceres_token_locker using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	/// Storage: Timestamp Now (r:1 w:0)
	/// Proof: Timestamp Now (max_values: Some(1), max_size: Some(8), added: 503, mode: MaxEncodedLen)
	/// Storage: CeresTokenLocker FeeAmount (r:1 w:0)
	/// Proof Skipped: CeresTokenLocker FeeAmount (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Tokens Accounts (r:3 w:3)
	/// Proof: Tokens Accounts (max_values: None, max_size: Some(136), added: 2611, mode: MaxEncodedLen)
	/// Storage: System Account (r:3 w:2)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// Storage: CeresTokenLocker FeesAccount (r:1 w:0)
	/// Proof Skipped: CeresTokenLocker FeesAccount (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: CeresTokenLocker TokenLockerData (r:1 w:1)
	/// Proof Skipped: CeresTokenLocker TokenLockerData (max_values: None, max_size: None, mode: Measured)
	fn lock_tokens() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `914`
		//  Estimated: `22352`
		// Minimum execution time: 117_518_000 picoseconds.
		Weight::from_parts(118_581_000, 22352)
			.saturating_add(T::DbWeight::get().reads(10_u64))
			.saturating_add(T::DbWeight::get().writes(6_u64))
	}
	/// Storage: Timestamp Now (r:1 w:0)
	/// Proof: Timestamp Now (max_values: Some(1), max_size: Some(8), added: 503, mode: MaxEncodedLen)
	/// Storage: CeresTokenLocker TokenLockerData (r:1 w:1)
	/// Proof Skipped: CeresTokenLocker TokenLockerData (max_values: None, max_size: None, mode: Measured)
	/// Storage: Tokens Accounts (r:2 w:2)
	/// Proof: Tokens Accounts (max_values: None, max_size: Some(136), added: 2611, mode: MaxEncodedLen)
	/// Storage: System Account (r:1 w:0)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	fn withdraw_tokens() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1226`
		//  Estimated: `12029`
		// Minimum execution time: 67_150_000 picoseconds.
		Weight::from_parts(68_160_000, 12029)
			.saturating_add(T::DbWeight::get().reads(5_u64))
			.saturating_add(T::DbWeight::get().writes(3_u64))
	}
	/// Storage: CeresTokenLocker AuthorityAccount (r:1 w:0)
	/// Proof Skipped: CeresTokenLocker AuthorityAccount (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: CeresTokenLocker FeeAmount (r:0 w:1)
	/// Proof Skipped: CeresTokenLocker FeeAmount (max_values: Some(1), max_size: None, mode: Measured)
	fn change_fee() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `114`
		//  Estimated: `723`
		// Minimum execution time: 14_772_000 picoseconds.
		Weight::from_parts(15_023_000, 723)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	/// Storage: Timestamp Now (r:1 w:0)
	/// Proof: Timestamp Now (max_values: Some(1), max_size: Some(8), added: 503, mode: MaxEncodedLen)
	/// Storage: CeresTokenLocker FeeAmount (r:1 w:0)
	/// Proof Skipped: CeresTokenLocker FeeAmount (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Tokens Accounts (r:3 w:3)
	/// Proof: Tokens Accounts (max_values: None, max_size: Some(136), added: 2611, mode: MaxEncodedLen)
	/// Storage: System Account (r:3 w:2)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// Storage: CeresTokenLocker FeesAccount (r:1 w:0)
	/// Proof Skipped: CeresTokenLocker FeesAccount (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: CeresTokenLocker TokenLockerData (r:1 w:1)
	/// Proof Skipped: CeresTokenLocker TokenLockerData (max_values: None, max_size: None, mode: Measured)
	fn lock_tokens() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `914`
		//  Estimated: `22352`
		// Minimum execution time: 117_518_000 picoseconds.
		Weight::from_parts(118_581_000, 22352)
			.saturating_add(RocksDbWeight::get().reads(10_u64))
			.saturating_add(RocksDbWeight::get().writes(6_u64))
	}
	/// Storage: Timestamp Now (r:1 w:0)
	/// Proof: Timestamp Now (max_values: Some(1), max_size: Some(8), added: 503, mode: MaxEncodedLen)
	/// Storage: CeresTokenLocker TokenLockerData (r:1 w:1)
	/// Proof Skipped: CeresTokenLocker TokenLockerData (max_values: None, max_size: None, mode: Measured)
	/// Storage: Tokens Accounts (r:2 w:2)
	/// Proof: Tokens Accounts (max_values: None, max_size: Some(136), added: 2611, mode: MaxEncodedLen)
	/// Storage: System Account (r:1 w:0)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	fn withdraw_tokens() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1226`
		//  Estimated: `12029`
		// Minimum execution time: 67_150_000 picoseconds.
		Weight::from_parts(68_160_000, 12029)
			.saturating_add(RocksDbWeight::get().reads(5_u64))
			.saturating_add(RocksDbWeight::get().writes(3_u64))
	}
	/// Storage: CeresTokenLocker AuthorityAccount (r:1 w:0)
	/// Proof Skipped: CeresTokenLocker AuthorityAccount (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: CeresTokenLocker FeeAmount (r:0 w:1)
	/// Proof Skipped: CeresTokenLocker FeeAmount (max_values: Some(1), max_size: None, mode: Measured)
	fn change_fee() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `114`
		//  Estimated: `723`
		// Minimum execution time: 14_772_000 picoseconds.
		Weight::from_parts(15_023_000, 723)
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
}
