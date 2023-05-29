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

//! Autogenerated weights for iroha_migration
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-05-29, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `2850b6372b51`, CPU: `Intel(R) Xeon(R) Platinum 8275CL CPU @ 3.00GHz`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("local"), DB CACHE: 1024

// Executed Command:
// /usr/local/bin/framenode
// benchmark
// pallet
// --chain=local
// --steps=50
// --repeat=20
// --pallet=iroha_migration
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --header=./misc/file_header.txt
// --template=./misc/pallet-weight-template.hbs
// --output=./pallets/iroha-migration/src/weights.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use core::marker::PhantomData;

/// Weight functions needed for iroha_migration.
pub trait WeightInfo {
	fn migrate() -> Weight;
	fn on_initialize() -> Weight;
}

/// Weights for iroha_migration using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	/// Storage: IrohaMigration MigratedAccounts (r:1 w:1)
	/// Proof Skipped: IrohaMigration MigratedAccounts (max_values: None, max_size: None, mode: Measured)
	/// Storage: IrohaMigration PublicKeys (r:1 w:1)
	/// Proof Skipped: IrohaMigration PublicKeys (max_values: None, max_size: None, mode: Measured)
	/// Storage: IrohaMigration Balances (r:1 w:1)
	/// Proof Skipped: IrohaMigration Balances (max_values: None, max_size: None, mode: Measured)
	/// Storage: Technical TechAccounts (r:1 w:0)
	/// Proof Skipped: Technical TechAccounts (max_values: None, max_size: None, mode: Measured)
	/// Storage: Tokens Accounts (r:2 w:2)
	/// Proof: Tokens Accounts (max_values: None, max_size: Some(136), added: 2611, mode: MaxEncodedLen)
	/// Storage: System Account (r:2 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// Storage: IrohaMigration Referrers (r:1 w:0)
	/// Proof Skipped: IrohaMigration Referrers (max_values: None, max_size: None, mode: Measured)
	/// Storage: IrohaMigration PendingReferrals (r:1 w:0)
	/// Proof Skipped: IrohaMigration PendingReferrals (max_values: None, max_size: None, mode: Measured)
	fn migrate() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `2025`
		//  Estimated: `37428`
		// Minimum execution time: 603_831_000 picoseconds.
		Weight::from_parts(611_296_000, 37428)
			.saturating_add(T::DbWeight::get().reads(10_u64))
			.saturating_add(T::DbWeight::get().writes(6_u64))
	}
	/// Storage: IrohaMigration PendingMultiSigAccounts (r:2 w:1)
	/// Proof Skipped: IrohaMigration PendingMultiSigAccounts (max_values: None, max_size: None, mode: Measured)
	/// Storage: IrohaMigration Quorums (r:1 w:1)
	/// Proof Skipped: IrohaMigration Quorums (max_values: None, max_size: None, mode: Measured)
	/// Storage: IrohaMigration Balances (r:1 w:1)
	/// Proof Skipped: IrohaMigration Balances (max_values: None, max_size: None, mode: Measured)
	/// Storage: Technical TechAccounts (r:1 w:0)
	/// Proof Skipped: Technical TechAccounts (max_values: None, max_size: None, mode: Measured)
	/// Storage: Tokens Accounts (r:2 w:2)
	/// Proof: Tokens Accounts (max_values: None, max_size: Some(136), added: 2611, mode: MaxEncodedLen)
	/// Storage: System Account (r:2 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// Storage: IrohaMigration Referrers (r:1 w:0)
	/// Proof Skipped: IrohaMigration Referrers (max_values: None, max_size: None, mode: Measured)
	/// Storage: IrohaMigration PendingReferrals (r:1 w:0)
	/// Proof Skipped: IrohaMigration PendingReferrals (max_values: None, max_size: None, mode: Measured)
	/// Storage: IrohaMigration MigratedAccounts (r:0 w:1)
	/// Proof Skipped: IrohaMigration MigratedAccounts (max_values: None, max_size: None, mode: Measured)
	/// Storage: IrohaMigration PublicKeys (r:0 w:1)
	/// Proof Skipped: IrohaMigration PublicKeys (max_values: None, max_size: None, mode: Measured)
	fn on_initialize() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `2058`
		//  Estimated: `44217`
		// Minimum execution time: 114_187_000 picoseconds.
		Weight::from_parts(115_956_000, 44217)
			.saturating_add(T::DbWeight::get().reads(11_u64))
			.saturating_add(T::DbWeight::get().writes(8_u64))
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	/// Storage: IrohaMigration MigratedAccounts (r:1 w:1)
	/// Proof Skipped: IrohaMigration MigratedAccounts (max_values: None, max_size: None, mode: Measured)
	/// Storage: IrohaMigration PublicKeys (r:1 w:1)
	/// Proof Skipped: IrohaMigration PublicKeys (max_values: None, max_size: None, mode: Measured)
	/// Storage: IrohaMigration Balances (r:1 w:1)
	/// Proof Skipped: IrohaMigration Balances (max_values: None, max_size: None, mode: Measured)
	/// Storage: Technical TechAccounts (r:1 w:0)
	/// Proof Skipped: Technical TechAccounts (max_values: None, max_size: None, mode: Measured)
	/// Storage: Tokens Accounts (r:2 w:2)
	/// Proof: Tokens Accounts (max_values: None, max_size: Some(136), added: 2611, mode: MaxEncodedLen)
	/// Storage: System Account (r:2 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// Storage: IrohaMigration Referrers (r:1 w:0)
	/// Proof Skipped: IrohaMigration Referrers (max_values: None, max_size: None, mode: Measured)
	/// Storage: IrohaMigration PendingReferrals (r:1 w:0)
	/// Proof Skipped: IrohaMigration PendingReferrals (max_values: None, max_size: None, mode: Measured)
	fn migrate() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `2025`
		//  Estimated: `37428`
		// Minimum execution time: 603_831_000 picoseconds.
		Weight::from_parts(611_296_000, 37428)
			.saturating_add(RocksDbWeight::get().reads(10_u64))
			.saturating_add(RocksDbWeight::get().writes(6_u64))
	}
	/// Storage: IrohaMigration PendingMultiSigAccounts (r:2 w:1)
	/// Proof Skipped: IrohaMigration PendingMultiSigAccounts (max_values: None, max_size: None, mode: Measured)
	/// Storage: IrohaMigration Quorums (r:1 w:1)
	/// Proof Skipped: IrohaMigration Quorums (max_values: None, max_size: None, mode: Measured)
	/// Storage: IrohaMigration Balances (r:1 w:1)
	/// Proof Skipped: IrohaMigration Balances (max_values: None, max_size: None, mode: Measured)
	/// Storage: Technical TechAccounts (r:1 w:0)
	/// Proof Skipped: Technical TechAccounts (max_values: None, max_size: None, mode: Measured)
	/// Storage: Tokens Accounts (r:2 w:2)
	/// Proof: Tokens Accounts (max_values: None, max_size: Some(136), added: 2611, mode: MaxEncodedLen)
	/// Storage: System Account (r:2 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// Storage: IrohaMigration Referrers (r:1 w:0)
	/// Proof Skipped: IrohaMigration Referrers (max_values: None, max_size: None, mode: Measured)
	/// Storage: IrohaMigration PendingReferrals (r:1 w:0)
	/// Proof Skipped: IrohaMigration PendingReferrals (max_values: None, max_size: None, mode: Measured)
	/// Storage: IrohaMigration MigratedAccounts (r:0 w:1)
	/// Proof Skipped: IrohaMigration MigratedAccounts (max_values: None, max_size: None, mode: Measured)
	/// Storage: IrohaMigration PublicKeys (r:0 w:1)
	/// Proof Skipped: IrohaMigration PublicKeys (max_values: None, max_size: None, mode: Measured)
	fn on_initialize() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `2058`
		//  Estimated: `44217`
		// Minimum execution time: 114_187_000 picoseconds.
		Weight::from_parts(115_956_000, 44217)
			.saturating_add(RocksDbWeight::get().reads(11_u64))
			.saturating_add(RocksDbWeight::get().writes(8_u64))
	}
}
