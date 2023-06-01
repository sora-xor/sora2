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

//! Autogenerated weights for referrals
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-06-01, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `73a108ced913`, CPU: `Intel(R) Xeon(R) Platinum 8275CL CPU @ 3.00GHz`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("local"), DB CACHE: 1024

// Executed Command:
// /usr/local/bin/framenode
// benchmark
// pallet
// --chain=local
// --steps=50
// --repeat=20
// --pallet=referrals
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --header=./misc/file_header.txt
// --template=./misc/pallet-weight-template.hbs
// --output=./pallets/referrals/src/weights.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use core::marker::PhantomData;

/// Weight functions needed for referrals.
pub trait WeightInfo {
	fn reserve() -> Weight;
	fn unreserve() -> Weight;
	fn set_referrer() -> Weight;
}

/// Weights for referrals using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	/// Storage: System Account (r:2 w:2)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// Storage: Referrals ReferrerBalances (r:1 w:1)
	/// Proof Skipped: Referrals ReferrerBalances (max_values: None, max_size: None, mode: Measured)
	fn reserve() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `291`
		//  Estimated: `7972`
		// Minimum execution time: 57_119_000 picoseconds.
		Weight::from_parts(57_552_000, 7972)
			.saturating_add(T::DbWeight::get().reads(3_u64))
			.saturating_add(T::DbWeight::get().writes(3_u64))
	}
	/// Storage: Referrals ReferrerBalances (r:1 w:1)
	/// Proof Skipped: Referrals ReferrerBalances (max_values: None, max_size: None, mode: Measured)
	/// Storage: System Account (r:2 w:2)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	fn unreserve() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `548`
		//  Estimated: `8229`
		// Minimum execution time: 48_267_000 picoseconds.
		Weight::from_parts(48_719_000, 8229)
			.saturating_add(T::DbWeight::get().reads(3_u64))
			.saturating_add(T::DbWeight::get().writes(3_u64))
	}
	/// Storage: Referrals Referrers (r:1 w:1)
	/// Proof Skipped: Referrals Referrers (max_values: None, max_size: None, mode: Measured)
	/// Storage: System Account (r:2 w:2)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// Storage: Referrals Referrals (r:1 w:1)
	/// Proof Skipped: Referrals Referrals (max_values: None, max_size: None, mode: Measured)
	fn set_referrer() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `375`
		//  Estimated: `10906`
		// Minimum execution time: 37_397_000 picoseconds.
		Weight::from_parts(38_103_000, 10906)
			.saturating_add(T::DbWeight::get().reads(4_u64))
			.saturating_add(T::DbWeight::get().writes(4_u64))
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	/// Storage: System Account (r:2 w:2)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// Storage: Referrals ReferrerBalances (r:1 w:1)
	/// Proof Skipped: Referrals ReferrerBalances (max_values: None, max_size: None, mode: Measured)
	fn reserve() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `291`
		//  Estimated: `7972`
		// Minimum execution time: 57_119_000 picoseconds.
		Weight::from_parts(57_552_000, 7972)
			.saturating_add(RocksDbWeight::get().reads(3_u64))
			.saturating_add(RocksDbWeight::get().writes(3_u64))
	}
	/// Storage: Referrals ReferrerBalances (r:1 w:1)
	/// Proof Skipped: Referrals ReferrerBalances (max_values: None, max_size: None, mode: Measured)
	/// Storage: System Account (r:2 w:2)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	fn unreserve() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `548`
		//  Estimated: `8229`
		// Minimum execution time: 48_267_000 picoseconds.
		Weight::from_parts(48_719_000, 8229)
			.saturating_add(RocksDbWeight::get().reads(3_u64))
			.saturating_add(RocksDbWeight::get().writes(3_u64))
	}
	/// Storage: Referrals Referrers (r:1 w:1)
	/// Proof Skipped: Referrals Referrers (max_values: None, max_size: None, mode: Measured)
	/// Storage: System Account (r:2 w:2)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// Storage: Referrals Referrals (r:1 w:1)
	/// Proof Skipped: Referrals Referrals (max_values: None, max_size: None, mode: Measured)
	fn set_referrer() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `375`
		//  Estimated: `10906`
		// Minimum execution time: 37_397_000 picoseconds.
		Weight::from_parts(38_103_000, 10906)
			.saturating_add(RocksDbWeight::get().reads(4_u64))
			.saturating_add(RocksDbWeight::get().writes(4_u64))
	}
}
