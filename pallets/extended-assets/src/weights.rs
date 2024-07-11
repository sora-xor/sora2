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

//! Autogenerated weights for `extended_assets`
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
// extended_assets
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
	fn set_sbt_expiration() -> Weight;
	fn bind_regulated_asset_to_sbt() -> Weight;
}

/// Weight functions for `extended_assets`.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	/// Storage: Assets AssetOwners (r:1 w:0)
	/// Proof Skipped: Assets AssetOwners (max_values: None, max_size: None, mode: Measured)
	/// Storage: ExtendedAssets RegulatedAsset (r:1 w:1)
	/// Proof: ExtendedAssets RegulatedAsset (max_values: None, max_size: Some(33), added: 2508, mode: MaxEncodedLen)
	/// Storage: ExtendedAssets SoulboundAsset (r:1 w:0)
	/// Proof: ExtendedAssets SoulboundAsset (max_values: None, max_size: Some(322091), added: 324566, mode: MaxEncodedLen)
	/// Storage: Assets AssetInfosV2 (r:1 w:1)
	/// Proof Skipped: Assets AssetInfosV2 (max_values: None, max_size: None, mode: Measured)
	fn regulate_asset() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1215`
		//  Estimated: `334454`
		// Minimum execution time: 26_000 nanoseconds.
		Weight::from_parts(27_000_000, 334454)
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: Timestamp Now (r:1 w:0)
	/// Proof: Timestamp Now (max_values: Some(1), max_size: Some(8), added: 503, mode: MaxEncodedLen)
	/// Storage: System Account (r:1 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// Storage: Permissions Owners (r:2 w:2)
	/// Proof Skipped: Permissions Owners (max_values: None, max_size: None, mode: Measured)
	/// Storage: Permissions Permissions (r:2 w:1)
	/// Proof Skipped: Permissions Permissions (max_values: None, max_size: None, mode: Measured)
	/// Storage: ExtendedAssets SoulboundAsset (r:0 w:1)
	/// Proof: ExtendedAssets SoulboundAsset (max_values: None, max_size: Some(2089), added: 4564, mode: MaxEncodedLen)
	/// Storage: Assets AssetInfos (r:0 w:1)
	/// Proof Skipped: Assets AssetInfos (max_values: None, max_size: None, mode: Measured)
	fn issue_sbt() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `2247`
		//  Estimated: `24469`
		// Minimum execution time: 76_000 nanoseconds.
		Weight::from_parts(79_000_000, 24469)
			.saturating_add(T::DbWeight::get().reads(7))
			.saturating_add(T::DbWeight::get().writes(7))
	}
	/// Storage: ExtendedAssets SoulboundAsset (r:1 w:0)
	/// Proof: ExtendedAssets SoulboundAsset (max_values: None, max_size: Some(2089), added: 4564, mode: MaxEncodedLen)
	/// Storage: Assets AssetOwners (r:1 w:0)
	/// Proof Skipped: Assets AssetOwners (max_values: None, max_size: None, mode: Measured)
	/// Storage: ExtendedAssets SBTExpiration (r:1 w:1)
	/// Proof: ExtendedAssets SBTExpiration (max_values: None, max_size: Some(72), added: 2547, mode: MaxEncodedLen)
	fn set_sbt_expiration() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `928`
		//  Estimated: `10514`
		// Minimum execution time: 19_000 nanoseconds.
		Weight::from_parts(20_000_000, 10514)
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: ExtendedAssets SoulboundAsset (r:1 w:1)
	/// Proof: ExtendedAssets SoulboundAsset (max_values: None, max_size: Some(322091), added: 324566, mode: MaxEncodedLen)
	/// Storage: Assets AssetOwners (r:2 w:0)
	/// Proof Skipped: Assets AssetOwners (max_values: None, max_size: None, mode: Measured)
	/// Storage: ExtendedAssets RegulatedAsset (r:1 w:0)
	/// Proof: ExtendedAssets RegulatedAsset (max_values: None, max_size: Some(33), added: 2508, mode: MaxEncodedLen)
	/// Storage: ExtendedAssets RegulatedAssetToSoulboundAsset (r:1 w:1)
	/// Proof: ExtendedAssets RegulatedAssetToSoulboundAsset (max_values: None, max_size: Some(64), added: 2539, mode: MaxEncodedLen)
	fn bind_regulated_asset_to_sbt() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1031`
		//  Estimated: `335594`
		// Minimum execution time: 27_000 nanoseconds.
		Weight::from_parts(28_000_000, 335594)
			.saturating_add(T::DbWeight::get().reads(5))
			.saturating_add(T::DbWeight::get().writes(2))
	}
}



impl WeightInfo for () {
    fn regulate_asset() -> Weight {
        Weight::from_parts(20_000_000, 10308)
            .saturating_add(RocksDbWeight::get().reads(3_u64))
            .saturating_add(RocksDbWeight::get().writes(1_u64))
    }
	
	fn issue_sbt() -> Weight {
		Weight::from_parts(77_000_000, 24469)
			.saturating_add(RocksDbWeight::get().reads(7))
			.saturating_add(RocksDbWeight::get().writes(7))
	}
	
	fn set_sbt_expiration() -> Weight {
		Weight::from_parts(20_000_000, 10514)
			.saturating_add(RocksDbWeight::get().reads(3))
			.saturating_add(RocksDbWeight::get().writes(1))
	}
	
	fn bind_regulated_asset_to_sbt() -> Weight {
			Weight::from_parts(28_000_000, 335594)
			.saturating_add(RocksDbWeight::get().reads(5))
			.saturating_add(RocksDbWeight::get().writes(2))
		}
}
