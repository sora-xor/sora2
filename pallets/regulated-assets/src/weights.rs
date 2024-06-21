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

//! Autogenerated weights for `regulated_assets`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2024-05-17, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `MacBook-Pro.local`, CPU: `<UNKNOWN>`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("local"), DB CACHE: 1024

// Executed Command:
// target/release/framenode
// benchmark
// pallet
// --chain=local
// --execution=wasm
// --wasm-execution=compiled
// --pallet
// regulated_assets
// --extrinsic
// *
// --steps
// 50
// --repeat
// 20
// --output
// ./benches

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::weights::constants::RocksDbWeight;
use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

pub trait WeightInfo {
    fn regulate_asset() -> Weight;
	fn issue_sbt() -> Weight;
	fn update_sbt_expiration() -> Weight;
}

/// Weight functions for `regulated_assets`.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	/// Storage: Assets AssetOwners (r:1 w:0)
	/// Proof Skipped: Assets AssetOwners (max_values: None, max_size: None, mode: Measured)
	/// Storage: RegulatedAssets RegulatedAsset (r:1 w:1)
	/// Proof: RegulatedAssets RegulatedAsset (max_values: None, max_size: Some(33), added: 2508, mode: MaxEncodedLen)
	/// Storage: RegulatedAssets SoulboundAsset (r:1 w:0)
	/// Proof: RegulatedAssets SoulboundAsset (max_values: None, max_size: Some(324727), added: 327202, mode: MaxEncodedLen)
	fn regulate_asset() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `761`
		//  Estimated: `332946`
		// Minimum execution time: 19_000 nanoseconds.
		Weight::from_parts(19_000_000, 332946)
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: Timestamp Now (r:1 w:0)
	/// Proof: Timestamp Now (max_values: Some(1), max_size: Some(8), added: 503, mode: MaxEncodedLen)
	/// Storage: Assets AssetOwners (r:2 w:1)
	/// Proof Skipped: Assets AssetOwners (max_values: None, max_size: None, mode: Measured)
	/// Storage: RegulatedAssets RegulatedAsset (r:1 w:0)
	/// Proof: RegulatedAssets RegulatedAsset (max_values: None, max_size: Some(33), added: 2508, mode: MaxEncodedLen)
	/// Storage: System Account (r:1 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// Storage: Permissions Owners (r:2 w:2)
	/// Proof Skipped: Permissions Owners (max_values: None, max_size: None, mode: Measured)
	/// Storage: Permissions Permissions (r:3 w:1)
	/// Proof Skipped: Permissions Permissions (max_values: None, max_size: None, mode: Measured)
	/// Storage: RegulatedAssets SBTsByAsset (r:1 w:1)
	/// Proof: RegulatedAssets SBTsByAsset (max_values: None, max_size: Some(320034), added: 322509, mode: MaxEncodedLen)
	/// Storage: RegulatedAssets SoulboundAsset (r:0 w:1)
	/// Proof: RegulatedAssets SoulboundAsset (max_values: None, max_size: Some(324727), added: 327202, mode: MaxEncodedLen)
	/// Storage: Assets AssetInfos (r:0 w:1)
	/// Proof Skipped: Assets AssetInfos (max_values: None, max_size: None, mode: Measured)
	fn issue_sbt() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `2575`
		//  Estimated: `355748`
		// Minimum execution time: 97_000 nanoseconds.
		Weight::from_parts(99_000_000, 355748)
			.saturating_add(T::DbWeight::get().reads(11))
			.saturating_add(T::DbWeight::get().writes(8))
	}
	/// Storage: RegulatedAssets SoulboundAsset (r:1 w:1)
	/// Proof: RegulatedAssets SoulboundAsset (max_values: None, max_size: Some(324727), added: 327202, mode: MaxEncodedLen)
	/// Storage: Timestamp Now (r:1 w:0)
	/// Proof: Timestamp Now (max_values: Some(1), max_size: Some(8), added: 503, mode: MaxEncodedLen)
	fn update_sbt_expiration() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `541`
		//  Estimated: `327705`
		// Minimum execution time: 15_000 nanoseconds.
		Weight::from_parts(16_000_000, 327705)
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
}



impl WeightInfo for () {
    fn regulate_asset() -> Weight {
        Weight::from_parts(19_000_000, 332946)
            .saturating_add(RocksDbWeight::get().reads(3_u64))
            .saturating_add(RocksDbWeight::get().writes(1_u64))
    }
	
	fn issue_sbt() -> Weight {
		Weight::from_parts(99_000_000, 355748)
			.saturating_add(RocksDbWeight::get().reads(11))
			.saturating_add(RocksDbWeight::get().writes(8))
	}
	
	fn update_sbt_expiration() -> Weight {
		Weight::from_parts(16_000_000, 327705)
			.saturating_add(RocksDbWeight::get().reads(2))
			.saturating_add(RocksDbWeight::get().writes(1))
	}
}

