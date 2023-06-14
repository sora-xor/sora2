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

//! Autogenerated weights for eth_bridge
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-06-14, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `3f2d25f94700`, CPU: `Intel(R) Xeon(R) Platinum 8275CL CPU @ 3.00GHz`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("local"), DB CACHE: 1024

// Executed Command:
// /usr/local/bin/framenode
// benchmark
// pallet
// --chain=local
// --steps=50
// --repeat=20
// --pallet=eth_bridge
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --header=./misc/file_header.txt
// --template=./misc/pallet-weight-template.hbs
// --output=./pallets/eth-bridge/src/weights.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use core::marker::PhantomData;

/// Weight functions needed for eth_bridge.
pub trait WeightInfo {
	fn transfer_to_sidechain() -> Weight;
	fn request_from_sidechain() -> Weight;
	fn register_incoming_request() -> Weight;
	fn finalize_incoming_request() -> Weight;
	fn approve_request() -> Weight;
	fn approve_request_finalize() -> Weight;
	fn abort_request() -> Weight;
}

/// Weights for eth_bridge using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	/// Storage: System Account (r:2 w:2)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// Storage: EthBridge BridgeStatuses (r:1 w:0)
	/// Proof Skipped: EthBridge BridgeStatuses (max_values: None, max_size: None, mode: Measured)
	/// Storage: EthBridge RequestStatuses (r:1 w:1)
	/// Proof Skipped: EthBridge RequestStatuses (max_values: None, max_size: None, mode: Measured)
	/// Storage: EthBridge Requests (r:1 w:1)
	/// Proof Skipped: EthBridge Requests (max_values: None, max_size: None, mode: Measured)
	/// Storage: EthBridge RegisteredAsset (r:1 w:0)
	/// Proof Skipped: EthBridge RegisteredAsset (max_values: None, max_size: None, mode: Measured)
	/// Storage: EthBridge BridgeAccount (r:1 w:0)
	/// Proof Skipped: EthBridge BridgeAccount (max_values: None, max_size: None, mode: Measured)
	/// Storage: EthBridge AccountRequests (r:1 w:1)
	/// Proof Skipped: EthBridge AccountRequests (max_values: None, max_size: None, mode: Measured)
	/// Storage: EthBridge RequestsQueue (r:1 w:1)
	/// Proof Skipped: EthBridge RequestsQueue (max_values: None, max_size: None, mode: Measured)
	/// Storage: EthBridge RequestSubmissionHeight (r:0 w:1)
	/// Proof Skipped: EthBridge RequestSubmissionHeight (max_values: None, max_size: None, mode: Measured)
	fn transfer_to_sidechain() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1220`
		//  Estimated: `32291`
		// Minimum execution time: 114_728_000 picoseconds.
		Weight::from_parts(115_587_000, 32291)
			.saturating_add(T::DbWeight::get().reads(9_u64))
			.saturating_add(T::DbWeight::get().writes(7_u64))
	}
	/// Storage: EthBridge BridgeStatuses (r:1 w:0)
	/// Proof Skipped: EthBridge BridgeStatuses (max_values: None, max_size: None, mode: Measured)
	/// Storage: EthBridge RequestStatuses (r:1 w:1)
	/// Proof Skipped: EthBridge RequestStatuses (max_values: None, max_size: None, mode: Measured)
	/// Storage: EthBridge Requests (r:1 w:1)
	/// Proof Skipped: EthBridge Requests (max_values: None, max_size: None, mode: Measured)
	/// Storage: EthBridge AccountRequests (r:1 w:1)
	/// Proof Skipped: EthBridge AccountRequests (max_values: None, max_size: None, mode: Measured)
	/// Storage: EthBridge RequestsQueue (r:1 w:1)
	/// Proof Skipped: EthBridge RequestsQueue (max_values: None, max_size: None, mode: Measured)
	/// Storage: EthBridge RequestSubmissionHeight (r:0 w:1)
	/// Proof Skipped: EthBridge RequestSubmissionHeight (max_values: None, max_size: None, mode: Measured)
	fn request_from_sidechain() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `566`
		//  Estimated: `15771`
		// Minimum execution time: 45_471_000 picoseconds.
		Weight::from_parts(46_353_000, 15771)
			.saturating_add(T::DbWeight::get().reads(5_u64))
			.saturating_add(T::DbWeight::get().writes(5_u64))
	}
	/// Storage: EthBridge BridgeAccount (r:1 w:0)
	/// Proof Skipped: EthBridge BridgeAccount (max_values: None, max_size: None, mode: Measured)
	/// Storage: EthBridge Requests (r:1 w:1)
	/// Proof Skipped: EthBridge Requests (max_values: None, max_size: None, mode: Measured)
	/// Storage: EthBridge RequestsQueue (r:1 w:1)
	/// Proof Skipped: EthBridge RequestsQueue (max_values: None, max_size: None, mode: Measured)
	/// Storage: System Account (r:1 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// Storage: EthBridge AccountRequests (r:1 w:1)
	/// Proof Skipped: EthBridge AccountRequests (max_values: None, max_size: None, mode: Measured)
	/// Storage: EthBridge RequestStatuses (r:0 w:2)
	/// Proof Skipped: EthBridge RequestStatuses (max_values: None, max_size: None, mode: Measured)
	/// Storage: EthBridge LoadToIncomingRequestHash (r:0 w:1)
	/// Proof Skipped: EthBridge LoadToIncomingRequestHash (max_values: None, max_size: None, mode: Measured)
	fn register_incoming_request() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `854`
		//  Estimated: `17627`
		// Minimum execution time: 60_148_000 picoseconds.
		Weight::from_parts(61_336_000, 17627)
			.saturating_add(T::DbWeight::get().reads(5_u64))
			.saturating_add(T::DbWeight::get().writes(7_u64))
	}
	/// Storage: EthBridge BridgeAccount (r:1 w:0)
	/// Proof Skipped: EthBridge BridgeAccount (max_values: None, max_size: None, mode: Measured)
	/// Storage: EthBridge Requests (r:1 w:0)
	/// Proof Skipped: EthBridge Requests (max_values: None, max_size: None, mode: Measured)
	/// Storage: EthBridge RequestStatuses (r:1 w:1)
	/// Proof Skipped: EthBridge RequestStatuses (max_values: None, max_size: None, mode: Measured)
	/// Storage: XorFee Multiplier (r:1 w:0)
	/// Proof Skipped: XorFee Multiplier (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: System Account (r:2 w:2)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// Storage: EthBridge RequestsQueue (r:1 w:1)
	/// Proof Skipped: EthBridge RequestsQueue (max_values: None, max_size: None, mode: Measured)
	fn finalize_incoming_request() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1466`
		//  Estimated: `22931`
		// Minimum execution time: 104_082_000 picoseconds.
		Weight::from_parts(105_343_000, 22931)
			.saturating_add(T::DbWeight::get().reads(7_u64))
			.saturating_add(T::DbWeight::get().writes(4_u64))
	}
	/// Storage: EthBridge Peers (r:1 w:0)
	/// Proof Skipped: EthBridge Peers (max_values: None, max_size: None, mode: Measured)
	/// Storage: EthBridge Requests (r:1 w:0)
	/// Proof Skipped: EthBridge Requests (max_values: None, max_size: None, mode: Measured)
	/// Storage: EthBridge RegisteredSidechainToken (r:1 w:0)
	/// Proof Skipped: EthBridge RegisteredSidechainToken (max_values: None, max_size: None, mode: Measured)
	/// Storage: EthBridge SidechainAssetPrecision (r:1 w:0)
	/// Proof Skipped: EthBridge SidechainAssetPrecision (max_values: None, max_size: None, mode: Measured)
	/// Storage: Assets AssetInfos (r:1 w:0)
	/// Proof Skipped: Assets AssetInfos (max_values: None, max_size: None, mode: Measured)
	/// Storage: EthBridge RequestApprovals (r:1 w:1)
	/// Proof Skipped: EthBridge RequestApprovals (max_values: None, max_size: None, mode: Measured)
	/// Storage: EthBridge PendingPeer (r:1 w:0)
	/// Proof Skipped: EthBridge PendingPeer (max_values: None, max_size: None, mode: Measured)
	/// Storage: EthBridge RequestStatuses (r:1 w:0)
	/// Proof Skipped: EthBridge RequestStatuses (max_values: None, max_size: None, mode: Measured)
	fn approve_request() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1905`
		//  Estimated: `35040`
		// Minimum execution time: 426_112_000 picoseconds.
		Weight::from_parts(428_134_000, 35040)
			.saturating_add(T::DbWeight::get().reads(8_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: EthBridge Peers (r:1 w:0)
	/// Proof Skipped: EthBridge Peers (max_values: None, max_size: None, mode: Measured)
	/// Storage: EthBridge Requests (r:1 w:0)
	/// Proof Skipped: EthBridge Requests (max_values: None, max_size: None, mode: Measured)
	/// Storage: EthBridge RegisteredSidechainToken (r:1 w:0)
	/// Proof Skipped: EthBridge RegisteredSidechainToken (max_values: None, max_size: None, mode: Measured)
	/// Storage: EthBridge SidechainAssetPrecision (r:1 w:0)
	/// Proof Skipped: EthBridge SidechainAssetPrecision (max_values: None, max_size: None, mode: Measured)
	/// Storage: Assets AssetInfos (r:1 w:0)
	/// Proof Skipped: Assets AssetInfos (max_values: None, max_size: None, mode: Measured)
	/// Storage: EthBridge RequestApprovals (r:1 w:1)
	/// Proof Skipped: EthBridge RequestApprovals (max_values: None, max_size: None, mode: Measured)
	/// Storage: EthBridge PendingPeer (r:1 w:0)
	/// Proof Skipped: EthBridge PendingPeer (max_values: None, max_size: None, mode: Measured)
	/// Storage: EthBridge RequestStatuses (r:1 w:1)
	/// Proof Skipped: EthBridge RequestStatuses (max_values: None, max_size: None, mode: Measured)
	/// Storage: EthBridge RegisteredAsset (r:1 w:0)
	/// Proof Skipped: EthBridge RegisteredAsset (max_values: None, max_size: None, mode: Measured)
	/// Storage: EthBridge BridgeAccount (r:1 w:0)
	/// Proof Skipped: EthBridge BridgeAccount (max_values: None, max_size: None, mode: Measured)
	/// Storage: System Account (r:1 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// Storage: EthBridge RequestsQueue (r:1 w:1)
	/// Proof Skipped: EthBridge RequestsQueue (max_values: None, max_size: None, mode: Measured)
	fn approve_request_finalize() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `3069`
		//  Estimated: `63587`
		// Minimum execution time: 481_335_000 picoseconds.
		Weight::from_parts(484_649_000, 63587)
			.saturating_add(T::DbWeight::get().reads(12_u64))
			.saturating_add(T::DbWeight::get().writes(4_u64))
	}
	/// Storage: EthBridge BridgeAccount (r:1 w:0)
	/// Proof Skipped: EthBridge BridgeAccount (max_values: None, max_size: None, mode: Measured)
	/// Storage: EthBridge Requests (r:1 w:0)
	/// Proof Skipped: EthBridge Requests (max_values: None, max_size: None, mode: Measured)
	/// Storage: EthBridge RequestStatuses (r:1 w:1)
	/// Proof Skipped: EthBridge RequestStatuses (max_values: None, max_size: None, mode: Measured)
	/// Storage: System Account (r:1 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// Storage: EthBridge RequestsQueue (r:1 w:1)
	/// Proof Skipped: EthBridge RequestsQueue (max_values: None, max_size: None, mode: Measured)
	fn abort_request() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1145`
		//  Estimated: `17083`
		// Minimum execution time: 67_102_000 picoseconds.
		Weight::from_parts(67_959_000, 17083)
			.saturating_add(T::DbWeight::get().reads(5_u64))
			.saturating_add(T::DbWeight::get().writes(3_u64))
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	/// Storage: System Account (r:2 w:2)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// Storage: EthBridge BridgeStatuses (r:1 w:0)
	/// Proof Skipped: EthBridge BridgeStatuses (max_values: None, max_size: None, mode: Measured)
	/// Storage: EthBridge RequestStatuses (r:1 w:1)
	/// Proof Skipped: EthBridge RequestStatuses (max_values: None, max_size: None, mode: Measured)
	/// Storage: EthBridge Requests (r:1 w:1)
	/// Proof Skipped: EthBridge Requests (max_values: None, max_size: None, mode: Measured)
	/// Storage: EthBridge RegisteredAsset (r:1 w:0)
	/// Proof Skipped: EthBridge RegisteredAsset (max_values: None, max_size: None, mode: Measured)
	/// Storage: EthBridge BridgeAccount (r:1 w:0)
	/// Proof Skipped: EthBridge BridgeAccount (max_values: None, max_size: None, mode: Measured)
	/// Storage: EthBridge AccountRequests (r:1 w:1)
	/// Proof Skipped: EthBridge AccountRequests (max_values: None, max_size: None, mode: Measured)
	/// Storage: EthBridge RequestsQueue (r:1 w:1)
	/// Proof Skipped: EthBridge RequestsQueue (max_values: None, max_size: None, mode: Measured)
	/// Storage: EthBridge RequestSubmissionHeight (r:0 w:1)
	/// Proof Skipped: EthBridge RequestSubmissionHeight (max_values: None, max_size: None, mode: Measured)
	fn transfer_to_sidechain() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1220`
		//  Estimated: `32291`
		// Minimum execution time: 114_728_000 picoseconds.
		Weight::from_parts(115_587_000, 32291)
			.saturating_add(RocksDbWeight::get().reads(9_u64))
			.saturating_add(RocksDbWeight::get().writes(7_u64))
	}
	/// Storage: EthBridge BridgeStatuses (r:1 w:0)
	/// Proof Skipped: EthBridge BridgeStatuses (max_values: None, max_size: None, mode: Measured)
	/// Storage: EthBridge RequestStatuses (r:1 w:1)
	/// Proof Skipped: EthBridge RequestStatuses (max_values: None, max_size: None, mode: Measured)
	/// Storage: EthBridge Requests (r:1 w:1)
	/// Proof Skipped: EthBridge Requests (max_values: None, max_size: None, mode: Measured)
	/// Storage: EthBridge AccountRequests (r:1 w:1)
	/// Proof Skipped: EthBridge AccountRequests (max_values: None, max_size: None, mode: Measured)
	/// Storage: EthBridge RequestsQueue (r:1 w:1)
	/// Proof Skipped: EthBridge RequestsQueue (max_values: None, max_size: None, mode: Measured)
	/// Storage: EthBridge RequestSubmissionHeight (r:0 w:1)
	/// Proof Skipped: EthBridge RequestSubmissionHeight (max_values: None, max_size: None, mode: Measured)
	fn request_from_sidechain() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `566`
		//  Estimated: `15771`
		// Minimum execution time: 45_471_000 picoseconds.
		Weight::from_parts(46_353_000, 15771)
			.saturating_add(RocksDbWeight::get().reads(5_u64))
			.saturating_add(RocksDbWeight::get().writes(5_u64))
	}
	/// Storage: EthBridge BridgeAccount (r:1 w:0)
	/// Proof Skipped: EthBridge BridgeAccount (max_values: None, max_size: None, mode: Measured)
	/// Storage: EthBridge Requests (r:1 w:1)
	/// Proof Skipped: EthBridge Requests (max_values: None, max_size: None, mode: Measured)
	/// Storage: EthBridge RequestsQueue (r:1 w:1)
	/// Proof Skipped: EthBridge RequestsQueue (max_values: None, max_size: None, mode: Measured)
	/// Storage: System Account (r:1 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// Storage: EthBridge AccountRequests (r:1 w:1)
	/// Proof Skipped: EthBridge AccountRequests (max_values: None, max_size: None, mode: Measured)
	/// Storage: EthBridge RequestStatuses (r:0 w:2)
	/// Proof Skipped: EthBridge RequestStatuses (max_values: None, max_size: None, mode: Measured)
	/// Storage: EthBridge LoadToIncomingRequestHash (r:0 w:1)
	/// Proof Skipped: EthBridge LoadToIncomingRequestHash (max_values: None, max_size: None, mode: Measured)
	fn register_incoming_request() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `854`
		//  Estimated: `17627`
		// Minimum execution time: 60_148_000 picoseconds.
		Weight::from_parts(61_336_000, 17627)
			.saturating_add(RocksDbWeight::get().reads(5_u64))
			.saturating_add(RocksDbWeight::get().writes(7_u64))
	}
	/// Storage: EthBridge BridgeAccount (r:1 w:0)
	/// Proof Skipped: EthBridge BridgeAccount (max_values: None, max_size: None, mode: Measured)
	/// Storage: EthBridge Requests (r:1 w:0)
	/// Proof Skipped: EthBridge Requests (max_values: None, max_size: None, mode: Measured)
	/// Storage: EthBridge RequestStatuses (r:1 w:1)
	/// Proof Skipped: EthBridge RequestStatuses (max_values: None, max_size: None, mode: Measured)
	/// Storage: XorFee Multiplier (r:1 w:0)
	/// Proof Skipped: XorFee Multiplier (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: System Account (r:2 w:2)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// Storage: EthBridge RequestsQueue (r:1 w:1)
	/// Proof Skipped: EthBridge RequestsQueue (max_values: None, max_size: None, mode: Measured)
	fn finalize_incoming_request() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1466`
		//  Estimated: `22931`
		// Minimum execution time: 104_082_000 picoseconds.
		Weight::from_parts(105_343_000, 22931)
			.saturating_add(RocksDbWeight::get().reads(7_u64))
			.saturating_add(RocksDbWeight::get().writes(4_u64))
	}
	/// Storage: EthBridge Peers (r:1 w:0)
	/// Proof Skipped: EthBridge Peers (max_values: None, max_size: None, mode: Measured)
	/// Storage: EthBridge Requests (r:1 w:0)
	/// Proof Skipped: EthBridge Requests (max_values: None, max_size: None, mode: Measured)
	/// Storage: EthBridge RegisteredSidechainToken (r:1 w:0)
	/// Proof Skipped: EthBridge RegisteredSidechainToken (max_values: None, max_size: None, mode: Measured)
	/// Storage: EthBridge SidechainAssetPrecision (r:1 w:0)
	/// Proof Skipped: EthBridge SidechainAssetPrecision (max_values: None, max_size: None, mode: Measured)
	/// Storage: Assets AssetInfos (r:1 w:0)
	/// Proof Skipped: Assets AssetInfos (max_values: None, max_size: None, mode: Measured)
	/// Storage: EthBridge RequestApprovals (r:1 w:1)
	/// Proof Skipped: EthBridge RequestApprovals (max_values: None, max_size: None, mode: Measured)
	/// Storage: EthBridge PendingPeer (r:1 w:0)
	/// Proof Skipped: EthBridge PendingPeer (max_values: None, max_size: None, mode: Measured)
	/// Storage: EthBridge RequestStatuses (r:1 w:0)
	/// Proof Skipped: EthBridge RequestStatuses (max_values: None, max_size: None, mode: Measured)
	fn approve_request() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1905`
		//  Estimated: `35040`
		// Minimum execution time: 426_112_000 picoseconds.
		Weight::from_parts(428_134_000, 35040)
			.saturating_add(RocksDbWeight::get().reads(8_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: EthBridge Peers (r:1 w:0)
	/// Proof Skipped: EthBridge Peers (max_values: None, max_size: None, mode: Measured)
	/// Storage: EthBridge Requests (r:1 w:0)
	/// Proof Skipped: EthBridge Requests (max_values: None, max_size: None, mode: Measured)
	/// Storage: EthBridge RegisteredSidechainToken (r:1 w:0)
	/// Proof Skipped: EthBridge RegisteredSidechainToken (max_values: None, max_size: None, mode: Measured)
	/// Storage: EthBridge SidechainAssetPrecision (r:1 w:0)
	/// Proof Skipped: EthBridge SidechainAssetPrecision (max_values: None, max_size: None, mode: Measured)
	/// Storage: Assets AssetInfos (r:1 w:0)
	/// Proof Skipped: Assets AssetInfos (max_values: None, max_size: None, mode: Measured)
	/// Storage: EthBridge RequestApprovals (r:1 w:1)
	/// Proof Skipped: EthBridge RequestApprovals (max_values: None, max_size: None, mode: Measured)
	/// Storage: EthBridge PendingPeer (r:1 w:0)
	/// Proof Skipped: EthBridge PendingPeer (max_values: None, max_size: None, mode: Measured)
	/// Storage: EthBridge RequestStatuses (r:1 w:1)
	/// Proof Skipped: EthBridge RequestStatuses (max_values: None, max_size: None, mode: Measured)
	/// Storage: EthBridge RegisteredAsset (r:1 w:0)
	/// Proof Skipped: EthBridge RegisteredAsset (max_values: None, max_size: None, mode: Measured)
	/// Storage: EthBridge BridgeAccount (r:1 w:0)
	/// Proof Skipped: EthBridge BridgeAccount (max_values: None, max_size: None, mode: Measured)
	/// Storage: System Account (r:1 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// Storage: EthBridge RequestsQueue (r:1 w:1)
	/// Proof Skipped: EthBridge RequestsQueue (max_values: None, max_size: None, mode: Measured)
	fn approve_request_finalize() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `3069`
		//  Estimated: `63587`
		// Minimum execution time: 481_335_000 picoseconds.
		Weight::from_parts(484_649_000, 63587)
			.saturating_add(RocksDbWeight::get().reads(12_u64))
			.saturating_add(RocksDbWeight::get().writes(4_u64))
	}
	/// Storage: EthBridge BridgeAccount (r:1 w:0)
	/// Proof Skipped: EthBridge BridgeAccount (max_values: None, max_size: None, mode: Measured)
	/// Storage: EthBridge Requests (r:1 w:0)
	/// Proof Skipped: EthBridge Requests (max_values: None, max_size: None, mode: Measured)
	/// Storage: EthBridge RequestStatuses (r:1 w:1)
	/// Proof Skipped: EthBridge RequestStatuses (max_values: None, max_size: None, mode: Measured)
	/// Storage: System Account (r:1 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// Storage: EthBridge RequestsQueue (r:1 w:1)
	/// Proof Skipped: EthBridge RequestsQueue (max_values: None, max_size: None, mode: Measured)
	fn abort_request() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1145`
		//  Estimated: `17083`
		// Minimum execution time: 67_102_000 picoseconds.
		Weight::from_parts(67_959_000, 17083)
			.saturating_add(RocksDbWeight::get().reads(5_u64))
			.saturating_add(RocksDbWeight::get().writes(3_u64))
	}
}
