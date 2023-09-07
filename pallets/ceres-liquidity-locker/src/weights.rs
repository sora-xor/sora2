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

//! Autogenerated weights for ceres_liquidity_locker
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-09-07, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `81e33122e192`, CPU: `Intel(R) Xeon(R) Platinum 8275CL CPU @ 3.00GHz`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("local"), DB CACHE: 1024

// Executed Command:
// /usr/local/bin/framenode
// benchmark
// pallet
// --chain=local
// --steps=50
// --repeat=20
// --pallet=ceres_liquidity_locker
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --header=./misc/file_header.txt
// --template=./misc/pallet-weight-template.hbs
// --output=./pallets/ceres-liquidity-locker/src/weights.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use core::marker::PhantomData;

/// Weight functions needed for ceres_liquidity_locker.
pub trait WeightInfo {
	fn lock_liquidity() -> Weight;
	fn change_ceres_fee() -> Weight;
}

/// Weights for ceres_liquidity_locker using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	/// Storage: Timestamp Now (r:1 w:0)
	/// Proof: Timestamp Now (max_values: Some(1), max_size: Some(8), added: 503, mode: MaxEncodedLen)
	/// Storage: PoolXYK Properties (r:1 w:0)
	/// Proof Skipped: PoolXYK Properties (max_values: None, max_size: None, mode: Measured)
	/// Storage: PoolXYK PoolProviders (r:2 w:2)
	/// Proof Skipped: PoolXYK PoolProviders (max_values: None, max_size: None, mode: Measured)
	/// Storage: CeresLiquidityLocker LockerData (r:1 w:1)
	/// Proof Skipped: CeresLiquidityLocker LockerData (max_values: None, max_size: None, mode: Measured)
	/// Storage: CeresLiquidityLocker FeesOptionTwoAccount (r:1 w:0)
	/// Proof Skipped: CeresLiquidityLocker FeesOptionTwoAccount (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: CeresLiquidityLocker FeesOptionTwoCeresAmount (r:1 w:0)
	/// Proof Skipped: CeresLiquidityLocker FeesOptionTwoCeresAmount (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Tokens Accounts (r:2 w:2)
	/// Proof: Tokens Accounts (max_values: None, max_size: Some(136), added: 2611, mode: MaxEncodedLen)
	/// Storage: System Account (r:2 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// Storage: PoolXYK AccountPools (r:1 w:1)
	/// Proof Skipped: PoolXYK AccountPools (max_values: None, max_size: None, mode: Measured)
	/// Storage: DemeterFarmingPlatform UserInfos (r:1 w:1)
	/// Proof Skipped: DemeterFarmingPlatform UserInfos (max_values: None, max_size: None, mode: Measured)
	fn lock_liquidity() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1635`
		//  Estimated: `38216`
		// Minimum execution time: 116_236_000 picoseconds.
		Weight::from_parts(117_078_000, 38216)
			.saturating_add(T::DbWeight::get().reads(13_u64))
			.saturating_add(T::DbWeight::get().writes(8_u64))
	}
	/// Storage: CeresLiquidityLocker AuthorityAccount (r:1 w:0)
	/// Proof Skipped: CeresLiquidityLocker AuthorityAccount (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: CeresLiquidityLocker FeesOptionTwoCeresAmount (r:0 w:1)
	/// Proof Skipped: CeresLiquidityLocker FeesOptionTwoCeresAmount (max_values: Some(1), max_size: None, mode: Measured)
	fn change_ceres_fee() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `109`
		//  Estimated: `713`
		// Minimum execution time: 9_940_000 picoseconds.
		Weight::from_parts(10_328_000, 713)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	/// Storage: Timestamp Now (r:1 w:0)
	/// Proof: Timestamp Now (max_values: Some(1), max_size: Some(8), added: 503, mode: MaxEncodedLen)
	/// Storage: PoolXYK Properties (r:1 w:0)
	/// Proof Skipped: PoolXYK Properties (max_values: None, max_size: None, mode: Measured)
	/// Storage: PoolXYK PoolProviders (r:2 w:2)
	/// Proof Skipped: PoolXYK PoolProviders (max_values: None, max_size: None, mode: Measured)
	/// Storage: CeresLiquidityLocker LockerData (r:1 w:1)
	/// Proof Skipped: CeresLiquidityLocker LockerData (max_values: None, max_size: None, mode: Measured)
	/// Storage: CeresLiquidityLocker FeesOptionTwoAccount (r:1 w:0)
	/// Proof Skipped: CeresLiquidityLocker FeesOptionTwoAccount (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: CeresLiquidityLocker FeesOptionTwoCeresAmount (r:1 w:0)
	/// Proof Skipped: CeresLiquidityLocker FeesOptionTwoCeresAmount (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Tokens Accounts (r:2 w:2)
	/// Proof: Tokens Accounts (max_values: None, max_size: Some(136), added: 2611, mode: MaxEncodedLen)
	/// Storage: System Account (r:2 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// Storage: PoolXYK AccountPools (r:1 w:1)
	/// Proof Skipped: PoolXYK AccountPools (max_values: None, max_size: None, mode: Measured)
	/// Storage: DemeterFarmingPlatform UserInfos (r:1 w:1)
	/// Proof Skipped: DemeterFarmingPlatform UserInfos (max_values: None, max_size: None, mode: Measured)
	fn lock_liquidity() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1635`
		//  Estimated: `38216`
		// Minimum execution time: 116_236_000 picoseconds.
		Weight::from_parts(117_078_000, 38216)
			.saturating_add(RocksDbWeight::get().reads(13_u64))
			.saturating_add(RocksDbWeight::get().writes(8_u64))
	}
	/// Storage: CeresLiquidityLocker AuthorityAccount (r:1 w:0)
	/// Proof Skipped: CeresLiquidityLocker AuthorityAccount (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: CeresLiquidityLocker FeesOptionTwoCeresAmount (r:0 w:1)
	/// Proof Skipped: CeresLiquidityLocker FeesOptionTwoCeresAmount (max_values: Some(1), max_size: None, mode: Measured)
	fn change_ceres_fee() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `109`
		//  Estimated: `713`
		// Minimum execution time: 9_940_000 picoseconds.
		Weight::from_parts(10_328_000, 713)
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
}
