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

//! Autogenerated weights for oracle_proxy
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-05-29, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `2504872ed66f`, CPU: `Intel(R) Xeon(R) Platinum 8275CL CPU @ 3.00GHz`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("local"), DB CACHE: 1024

// Executed Command:
// /usr/local/bin/framenode
// benchmark
// pallet
// --chain=local
// --steps=50
// --repeat=20
// --pallet=oracle_proxy
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --header=./misc/file_header.txt
// --template=./misc/pallet-weight-template.hbs
// --output=./pallets/oracle-proxy/src/weights.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use core::marker::PhantomData;

/// Weight functions needed for oracle_proxy.
pub trait WeightInfo {
	fn enable_oracle() -> Weight;
	fn disable_oracle() -> Weight;
}

/// Weights for oracle_proxy using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	/// Storage: OracleProxy EnabledOracles (r:1 w:1)
	/// Proof Skipped: OracleProxy EnabledOracles (max_values: Some(1), max_size: None, mode: Measured)
	fn enable_oracle() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `76`
		//  Estimated: `571`
		// Minimum execution time: 14_819_000 picoseconds.
		Weight::from_parts(15_160_000, 571)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: OracleProxy EnabledOracles (r:1 w:1)
	/// Proof Skipped: OracleProxy EnabledOracles (max_values: Some(1), max_size: None, mode: Measured)
	fn disable_oracle() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `101`
		//  Estimated: `596`
		// Minimum execution time: 15_905_000 picoseconds.
		Weight::from_parts(16_231_000, 596)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	/// Storage: OracleProxy EnabledOracles (r:1 w:1)
	/// Proof Skipped: OracleProxy EnabledOracles (max_values: Some(1), max_size: None, mode: Measured)
	fn enable_oracle() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `76`
		//  Estimated: `571`
		// Minimum execution time: 14_819_000 picoseconds.
		Weight::from_parts(15_160_000, 571)
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: OracleProxy EnabledOracles (r:1 w:1)
	/// Proof Skipped: OracleProxy EnabledOracles (max_values: Some(1), max_size: None, mode: Measured)
	fn disable_oracle() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `101`
		//  Estimated: `596`
		// Minimum execution time: 15_905_000 picoseconds.
		Weight::from_parts(16_231_000, 596)
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
}
