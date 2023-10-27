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

//! Autogenerated weights for order_book
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-10-27, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `ef2f8146724e`, CPU: `Intel(R) Xeon(R) Platinum 8275CL CPU @ 3.00GHz`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("local"), DB CACHE: 1024

// Executed Command:
// /usr/local/bin/framenode
// benchmark
// pallet
// --chain=local
// --steps=50
// --repeat=20
// --pallet=order_book
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --header=./misc/file_header.txt
// --template=./misc/pallet-weight-template.hbs
// --output=./pallets/order-book/src/weights.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use core::marker::PhantomData;

/// Weight functions needed for order_book.
pub trait WeightInfo {
	fn create_orderbook() -> Weight;
	fn delete_orderbook() -> Weight;
	fn update_orderbook() -> Weight;
	fn change_orderbook_status() -> Weight;
	fn place_limit_order_without_cross_spread() -> Weight;
	fn cancel_limit_order_first_expiration() -> Weight;
	fn cancel_limit_order_last_expiration() -> Weight;
	fn execute_market_order() -> Weight;
	fn quote() -> Weight;
	fn exchange_single_order() -> Weight;
	fn service_base() -> Weight;
	fn service_block_base() -> Weight;
	fn service_single_expiration() -> Weight;
}

/// Weights for order_book using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	/// Storage: DEXManager DEXInfos (r:1 w:0)
	/// Proof Skipped: DEXManager DEXInfos (max_values: None, max_size: None, mode: Measured)
	/// Storage: Assets AssetOwners (r:1 w:0)
	/// Proof Skipped: Assets AssetOwners (max_values: None, max_size: None, mode: Measured)
	/// Storage: TradingPair EnabledSources (r:1 w:1)
	/// Proof Skipped: TradingPair EnabledSources (max_values: None, max_size: None, mode: Measured)
	/// Storage: OrderBook OrderBooks (r:1 w:1)
	/// Proof: OrderBook OrderBooks (max_values: None, max_size: Some(237), added: 2712, mode: MaxEncodedLen)
	/// Storage: Assets AssetInfos (r:1 w:0)
	/// Proof Skipped: Assets AssetInfos (max_values: None, max_size: None, mode: Measured)
	/// Storage: Tokens Accounts (r:1 w:0)
	/// Proof: Tokens Accounts (max_values: None, max_size: Some(136), added: 2611, mode: MaxEncodedLen)
	/// Storage: Technical TechAccounts (r:1 w:1)
	/// Proof Skipped: Technical TechAccounts (max_values: None, max_size: None, mode: Measured)
	/// Storage: System Account (r:1 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	fn create_orderbook() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `2964`
		//  Estimated: `35121`
		// Minimum execution time: 100_146_000 picoseconds.
		Weight::from_parts(101_293_000, 35121)
			.saturating_add(T::DbWeight::get().reads(8_u64))
			.saturating_add(T::DbWeight::get().writes(4_u64))
	}
	/// Storage: OrderBook OrderBooks (r:1 w:1)
	/// Proof: OrderBook OrderBooks (max_values: None, max_size: Some(237), added: 2712, mode: MaxEncodedLen)
	/// Storage: OrderBook LimitOrders (r:1 w:0)
	/// Proof: OrderBook LimitOrders (max_values: None, max_size: Some(236), added: 2711, mode: MaxEncodedLen)
	/// Storage: DEXManager DEXInfos (r:1 w:0)
	/// Proof Skipped: DEXManager DEXInfos (max_values: None, max_size: None, mode: Measured)
	/// Storage: TradingPair EnabledSources (r:1 w:1)
	/// Proof Skipped: TradingPair EnabledSources (max_values: None, max_size: None, mode: Measured)
	/// Storage: Technical TechAccounts (r:1 w:1)
	/// Proof Skipped: Technical TechAccounts (max_values: None, max_size: None, mode: Measured)
	/// Storage: System Account (r:1 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	fn delete_orderbook() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1856`
		//  Estimated: `21019`
		// Minimum execution time: 76_009_000 picoseconds.
		Weight::from_parts(77_281_000, 21019)
			.saturating_add(T::DbWeight::get().reads(6_u64))
			.saturating_add(T::DbWeight::get().writes(4_u64))
	}
	/// Storage: OrderBook OrderBooks (r:1 w:1)
	/// Proof: OrderBook OrderBooks (max_values: None, max_size: Some(237), added: 2712, mode: MaxEncodedLen)
	/// Storage: Assets AssetInfos (r:1 w:0)
	/// Proof Skipped: Assets AssetInfos (max_values: None, max_size: None, mode: Measured)
	/// Storage: Tokens TotalIssuance (r:1 w:0)
	/// Proof: Tokens TotalIssuance (max_values: None, max_size: Some(56), added: 2531, mode: MaxEncodedLen)
	/// Storage: OrderBook LimitOrders (r:13 w:11)
	/// Proof: OrderBook LimitOrders (max_values: None, max_size: Some(236), added: 2711, mode: MaxEncodedLen)
	/// Storage: Technical TechAccounts (r:1 w:0)
	/// Proof Skipped: Technical TechAccounts (max_values: None, max_size: None, mode: Measured)
	/// Storage: System Account (r:3 w:3)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// Storage: Tokens Accounts (r:3 w:3)
	/// Proof: Tokens Accounts (max_values: None, max_size: Some(136), added: 2611, mode: MaxEncodedLen)
	/// Storage: OrderBook AggregatedBids (r:1 w:1)
	/// Proof: OrderBook AggregatedBids (max_values: None, max_size: Some(34902), added: 37377, mode: MaxEncodedLen)
	/// Storage: OrderBook AggregatedAsks (r:1 w:1)
	/// Proof: OrderBook AggregatedAsks (max_values: None, max_size: Some(34902), added: 37377, mode: MaxEncodedLen)
	fn update_orderbook() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `6196`
		//  Estimated: `148224`
		// Minimum execution time: 446_249_000 picoseconds.
		Weight::from_parts(449_319_000, 148224)
			.saturating_add(T::DbWeight::get().reads(25_u64))
			.saturating_add(T::DbWeight::get().writes(20_u64))
	}
	/// Storage: OrderBook OrderBooks (r:1 w:1)
	/// Proof: OrderBook OrderBooks (max_values: None, max_size: Some(237), added: 2712, mode: MaxEncodedLen)
	fn change_orderbook_status() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `621`
		//  Estimated: `2712`
		// Minimum execution time: 24_411_000 picoseconds.
		Weight::from_parts(24_683_000, 2712)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: OrderBook OrderBooks (r:1 w:1)
	/// Proof: OrderBook OrderBooks (max_values: None, max_size: Some(237), added: 2712, mode: MaxEncodedLen)
	/// Storage: Timestamp Now (r:1 w:0)
	/// Proof: Timestamp Now (max_values: Some(1), max_size: Some(8), added: 503, mode: MaxEncodedLen)
	/// Storage: Assets AssetInfos (r:1 w:0)
	/// Proof Skipped: Assets AssetInfos (max_values: None, max_size: None, mode: Measured)
	/// Storage: OrderBook UserLimitOrders (r:1 w:1)
	/// Proof: OrderBook UserLimitOrders (max_values: None, max_size: Some(16518), added: 18993, mode: MaxEncodedLen)
	/// Storage: OrderBook Asks (r:1 w:1)
	/// Proof: OrderBook Asks (max_values: None, max_size: Some(16503), added: 18978, mode: MaxEncodedLen)
	/// Storage: OrderBook AggregatedAsks (r:1 w:1)
	/// Proof: OrderBook AggregatedAsks (max_values: None, max_size: Some(34902), added: 37377, mode: MaxEncodedLen)
	/// Storage: OrderBook AggregatedBids (r:1 w:0)
	/// Proof: OrderBook AggregatedBids (max_values: None, max_size: Some(34902), added: 37377, mode: MaxEncodedLen)
	/// Storage: Technical TechAccounts (r:1 w:0)
	/// Proof Skipped: Technical TechAccounts (max_values: None, max_size: None, mode: Measured)
	/// Storage: Tokens Accounts (r:2 w:2)
	/// Proof: Tokens Accounts (max_values: None, max_size: Some(136), added: 2611, mode: MaxEncodedLen)
	/// Storage: System Account (r:1 w:0)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// Storage: OrderBook LimitOrders (r:1 w:1)
	/// Proof: OrderBook LimitOrders (max_values: None, max_size: Some(236), added: 2711, mode: MaxEncodedLen)
	/// Storage: OrderBook ExpirationsAgenda (r:1 w:1)
	/// Proof: OrderBook ExpirationsAgenda (max_values: None, max_size: Some(43014), added: 45489, mode: MaxEncodedLen)
	fn place_limit_order_without_cross_spread() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `80739`
		//  Estimated: `338393`
		// Minimum execution time: 469_067_000 picoseconds.
		Weight::from_parts(485_279_000, 338393)
			.saturating_add(T::DbWeight::get().reads(13_u64))
			.saturating_add(T::DbWeight::get().writes(8_u64))
	}
	/// Storage: OrderBook LimitOrders (r:1 w:1)
	/// Proof: OrderBook LimitOrders (max_values: None, max_size: Some(236), added: 2711, mode: MaxEncodedLen)
	/// Storage: OrderBook OrderBooks (r:1 w:0)
	/// Proof: OrderBook OrderBooks (max_values: None, max_size: Some(237), added: 2712, mode: MaxEncodedLen)
	/// Storage: Technical TechAccounts (r:1 w:0)
	/// Proof Skipped: Technical TechAccounts (max_values: None, max_size: None, mode: Measured)
	/// Storage: System Account (r:2 w:2)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// Storage: OrderBook Bids (r:1 w:1)
	/// Proof: OrderBook Bids (max_values: None, max_size: Some(16503), added: 18978, mode: MaxEncodedLen)
	/// Storage: OrderBook AggregatedBids (r:1 w:1)
	/// Proof: OrderBook AggregatedBids (max_values: None, max_size: Some(34902), added: 37377, mode: MaxEncodedLen)
	/// Storage: OrderBook UserLimitOrders (r:1 w:1)
	/// Proof: OrderBook UserLimitOrders (max_values: None, max_size: Some(16518), added: 18993, mode: MaxEncodedLen)
	/// Storage: OrderBook ExpirationsAgenda (r:1 w:1)
	/// Proof: OrderBook ExpirationsAgenda (max_values: None, max_size: Some(43014), added: 45489, mode: MaxEncodedLen)
	fn cancel_limit_order_first_expiration() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `115651`
		//  Estimated: `249592`
		// Minimum execution time: 832_644_000 picoseconds.
		Weight::from_parts(851_019_000, 249592)
			.saturating_add(T::DbWeight::get().reads(9_u64))
			.saturating_add(T::DbWeight::get().writes(7_u64))
	}
	/// Storage: OrderBook LimitOrders (r:1 w:1)
	/// Proof: OrderBook LimitOrders (max_values: None, max_size: Some(236), added: 2711, mode: MaxEncodedLen)
	/// Storage: OrderBook OrderBooks (r:1 w:0)
	/// Proof: OrderBook OrderBooks (max_values: None, max_size: Some(237), added: 2712, mode: MaxEncodedLen)
	/// Storage: Technical TechAccounts (r:1 w:0)
	/// Proof Skipped: Technical TechAccounts (max_values: None, max_size: None, mode: Measured)
	/// Storage: System Account (r:2 w:2)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// Storage: OrderBook Bids (r:1 w:1)
	/// Proof: OrderBook Bids (max_values: None, max_size: Some(16503), added: 18978, mode: MaxEncodedLen)
	/// Storage: OrderBook AggregatedBids (r:1 w:1)
	/// Proof: OrderBook AggregatedBids (max_values: None, max_size: Some(34902), added: 37377, mode: MaxEncodedLen)
	/// Storage: OrderBook UserLimitOrders (r:1 w:1)
	/// Proof: OrderBook UserLimitOrders (max_values: None, max_size: Some(16518), added: 18993, mode: MaxEncodedLen)
	/// Storage: OrderBook ExpirationsAgenda (r:1 w:1)
	/// Proof: OrderBook ExpirationsAgenda (max_values: None, max_size: Some(43014), added: 45489, mode: MaxEncodedLen)
	fn cancel_limit_order_last_expiration() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `115651`
		//  Estimated: `249592`
		// Minimum execution time: 807_267_000 picoseconds.
		Weight::from_parts(841_496_000, 249592)
			.saturating_add(T::DbWeight::get().reads(9_u64))
			.saturating_add(T::DbWeight::get().writes(7_u64))
	}
	/// Storage: Assets AssetInfos (r:1 w:0)
	/// Proof Skipped: Assets AssetInfos (max_values: None, max_size: None, mode: Measured)
	/// Storage: OrderBook OrderBooks (r:1 w:0)
	/// Proof: OrderBook OrderBooks (max_values: None, max_size: Some(237), added: 2712, mode: MaxEncodedLen)
	/// Storage: OrderBook AggregatedBids (r:1 w:1)
	/// Proof: OrderBook AggregatedBids (max_values: None, max_size: Some(34902), added: 37377, mode: MaxEncodedLen)
	/// Storage: OrderBook Bids (r:4 w:4)
	/// Proof: OrderBook Bids (max_values: None, max_size: Some(16503), added: 18978, mode: MaxEncodedLen)
	/// Storage: OrderBook LimitOrders (r:4000 w:4000)
	/// Proof: OrderBook LimitOrders (max_values: None, max_size: Some(236), added: 2711, mode: MaxEncodedLen)
	/// Storage: Technical TechAccounts (r:1 w:0)
	/// Proof Skipped: Technical TechAccounts (max_values: None, max_size: None, mode: Measured)
	/// Storage: Tokens Accounts (r:6 w:6)
	/// Proof: Tokens Accounts (max_values: None, max_size: Some(136), added: 2611, mode: MaxEncodedLen)
	/// Storage: System Account (r:2 w:2)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// Storage: OrderBook UserLimitOrders (r:4 w:4)
	/// Proof: OrderBook UserLimitOrders (max_values: None, max_size: Some(16518), added: 18993, mode: MaxEncodedLen)
	/// Storage: OrderBook ExpirationsAgenda (r:8 w:8)
	/// Proof: OrderBook ExpirationsAgenda (max_values: None, max_size: Some(43014), added: 45489, mode: MaxEncodedLen)
	fn execute_market_order() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1237740`
		//  Estimated: `13901187`
		// Minimum execution time: 309_262_821_000 picoseconds.
		Weight::from_parts(310_778_621_000, 13901187)
			.saturating_add(T::DbWeight::get().reads(4028_u64))
			.saturating_add(T::DbWeight::get().writes(4025_u64))
	}
	/// Storage: DEXManager DEXInfos (r:1 w:0)
	/// Proof Skipped: DEXManager DEXInfos (max_values: None, max_size: None, mode: Measured)
	/// Storage: OrderBook OrderBooks (r:1 w:0)
	/// Proof: OrderBook OrderBooks (max_values: None, max_size: Some(237), added: 2712, mode: MaxEncodedLen)
	/// Storage: OrderBook AggregatedBids (r:1 w:0)
	/// Proof: OrderBook AggregatedBids (max_values: None, max_size: Some(34902), added: 37377, mode: MaxEncodedLen)
	fn quote() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `35829`
		//  Estimated: `78393`
		// Minimum execution time: 905_284_000 picoseconds.
		Weight::from_parts(935_448_000, 78393)
			.saturating_add(T::DbWeight::get().reads(3_u64))
	}
	/// Storage: DEXManager DEXInfos (r:1 w:0)
	/// Proof Skipped: DEXManager DEXInfos (max_values: None, max_size: None, mode: Measured)
	/// Storage: OrderBook OrderBooks (r:1 w:0)
	/// Proof: OrderBook OrderBooks (max_values: None, max_size: Some(237), added: 2712, mode: MaxEncodedLen)
	/// Storage: OrderBook AggregatedBids (r:1 w:1)
	/// Proof: OrderBook AggregatedBids (max_values: None, max_size: Some(34902), added: 37377, mode: MaxEncodedLen)
	/// Storage: OrderBook Bids (r:1 w:1)
	/// Proof: OrderBook Bids (max_values: None, max_size: Some(16503), added: 18978, mode: MaxEncodedLen)
	/// Storage: OrderBook LimitOrders (r:1 w:1)
	/// Proof: OrderBook LimitOrders (max_values: None, max_size: Some(236), added: 2711, mode: MaxEncodedLen)
	/// Storage: Technical TechAccounts (r:1 w:0)
	/// Proof Skipped: Technical TechAccounts (max_values: None, max_size: None, mode: Measured)
	/// Storage: Tokens Accounts (r:3 w:3)
	/// Proof: Tokens Accounts (max_values: None, max_size: Some(136), added: 2611, mode: MaxEncodedLen)
	/// Storage: System Account (r:2 w:2)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// Storage: OrderBook UserLimitOrders (r:1 w:1)
	/// Proof: OrderBook UserLimitOrders (max_values: None, max_size: Some(16518), added: 18993, mode: MaxEncodedLen)
	/// Storage: OrderBook ExpirationsAgenda (r:1 w:1)
	/// Proof: OrderBook ExpirationsAgenda (max_values: None, max_size: Some(43014), added: 45489, mode: MaxEncodedLen)
	fn exchange_single_order() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `5123`
		//  Estimated: `154495`
		// Minimum execution time: 229_529_000 picoseconds.
		Weight::from_parts(230_771_000, 154495)
			.saturating_add(T::DbWeight::get().reads(13_u64))
			.saturating_add(T::DbWeight::get().writes(10_u64))
	}
	/// Storage: OrderBook IncompleteExpirationsSince (r:1 w:0)
	/// Proof: OrderBook IncompleteExpirationsSince (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// Storage: OrderBook ExpirationsAgenda (r:1 w:0)
	/// Proof: OrderBook ExpirationsAgenda (max_values: None, max_size: Some(43014), added: 45489, mode: MaxEncodedLen)
	fn service_base() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `109`
		//  Estimated: `45988`
		// Minimum execution time: 5_245_000 picoseconds.
		Weight::from_parts(5_477_000, 45988)
			.saturating_add(T::DbWeight::get().reads(2_u64))
	}
	/// Storage: OrderBook ExpirationsAgenda (r:1 w:0)
	/// Proof: OrderBook ExpirationsAgenda (max_values: None, max_size: Some(43014), added: 45489, mode: MaxEncodedLen)
	fn service_block_base() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `109`
		//  Estimated: `45489`
		// Minimum execution time: 3_731_000 picoseconds.
		Weight::from_parts(3_947_000, 45489)
			.saturating_add(T::DbWeight::get().reads(1_u64))
	}
	/// Storage: OrderBook LimitOrders (r:1 w:0)
	/// Proof: OrderBook LimitOrders (max_values: None, max_size: Some(236), added: 2711, mode: MaxEncodedLen)
	/// Storage: OrderBook OrderBooks (r:1 w:0)
	/// Proof: OrderBook OrderBooks (max_values: None, max_size: Some(237), added: 2712, mode: MaxEncodedLen)
	/// Storage: Technical TechAccounts (r:1 w:0)
	/// Proof Skipped: Technical TechAccounts (max_values: None, max_size: None, mode: Measured)
	/// Storage: System Account (r:2 w:2)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// Storage: OrderBook Bids (r:1 w:0)
	/// Proof: OrderBook Bids (max_values: None, max_size: Some(16503), added: 18978, mode: MaxEncodedLen)
	/// Storage: OrderBook AggregatedBids (r:1 w:0)
	/// Proof: OrderBook AggregatedBids (max_values: None, max_size: Some(34902), added: 37377, mode: MaxEncodedLen)
	/// Storage: OrderBook UserLimitOrders (r:1 w:0)
	/// Proof: OrderBook UserLimitOrders (max_values: None, max_size: Some(16518), added: 18993, mode: MaxEncodedLen)
	/// Storage: OrderBook ExpirationsAgenda (r:1 w:1)
	/// Proof: OrderBook ExpirationsAgenda (max_values: None, max_size: Some(43014), added: 45489, mode: MaxEncodedLen)
	fn service_single_expiration() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `4017`
		//  Estimated: `137958`
		// Minimum execution time: 122_506_000 picoseconds.
		Weight::from_parts(124_248_000, 137958)
			.saturating_add(T::DbWeight::get().reads(9_u64))
			.saturating_add(T::DbWeight::get().writes(3_u64))
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	/// Storage: DEXManager DEXInfos (r:1 w:0)
	/// Proof Skipped: DEXManager DEXInfos (max_values: None, max_size: None, mode: Measured)
	/// Storage: Assets AssetOwners (r:1 w:0)
	/// Proof Skipped: Assets AssetOwners (max_values: None, max_size: None, mode: Measured)
	/// Storage: TradingPair EnabledSources (r:1 w:1)
	/// Proof Skipped: TradingPair EnabledSources (max_values: None, max_size: None, mode: Measured)
	/// Storage: OrderBook OrderBooks (r:1 w:1)
	/// Proof: OrderBook OrderBooks (max_values: None, max_size: Some(237), added: 2712, mode: MaxEncodedLen)
	/// Storage: Assets AssetInfos (r:1 w:0)
	/// Proof Skipped: Assets AssetInfos (max_values: None, max_size: None, mode: Measured)
	/// Storage: Tokens Accounts (r:1 w:0)
	/// Proof: Tokens Accounts (max_values: None, max_size: Some(136), added: 2611, mode: MaxEncodedLen)
	/// Storage: Technical TechAccounts (r:1 w:1)
	/// Proof Skipped: Technical TechAccounts (max_values: None, max_size: None, mode: Measured)
	/// Storage: System Account (r:1 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	fn create_orderbook() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `2964`
		//  Estimated: `35121`
		// Minimum execution time: 100_146_000 picoseconds.
		Weight::from_parts(101_293_000, 35121)
			.saturating_add(RocksDbWeight::get().reads(8_u64))
			.saturating_add(RocksDbWeight::get().writes(4_u64))
	}
	/// Storage: OrderBook OrderBooks (r:1 w:1)
	/// Proof: OrderBook OrderBooks (max_values: None, max_size: Some(237), added: 2712, mode: MaxEncodedLen)
	/// Storage: OrderBook LimitOrders (r:1 w:0)
	/// Proof: OrderBook LimitOrders (max_values: None, max_size: Some(236), added: 2711, mode: MaxEncodedLen)
	/// Storage: DEXManager DEXInfos (r:1 w:0)
	/// Proof Skipped: DEXManager DEXInfos (max_values: None, max_size: None, mode: Measured)
	/// Storage: TradingPair EnabledSources (r:1 w:1)
	/// Proof Skipped: TradingPair EnabledSources (max_values: None, max_size: None, mode: Measured)
	/// Storage: Technical TechAccounts (r:1 w:1)
	/// Proof Skipped: Technical TechAccounts (max_values: None, max_size: None, mode: Measured)
	/// Storage: System Account (r:1 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	fn delete_orderbook() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1856`
		//  Estimated: `21019`
		// Minimum execution time: 76_009_000 picoseconds.
		Weight::from_parts(77_281_000, 21019)
			.saturating_add(RocksDbWeight::get().reads(6_u64))
			.saturating_add(RocksDbWeight::get().writes(4_u64))
	}
	/// Storage: OrderBook OrderBooks (r:1 w:1)
	/// Proof: OrderBook OrderBooks (max_values: None, max_size: Some(237), added: 2712, mode: MaxEncodedLen)
	/// Storage: Assets AssetInfos (r:1 w:0)
	/// Proof Skipped: Assets AssetInfos (max_values: None, max_size: None, mode: Measured)
	/// Storage: Tokens TotalIssuance (r:1 w:0)
	/// Proof: Tokens TotalIssuance (max_values: None, max_size: Some(56), added: 2531, mode: MaxEncodedLen)
	/// Storage: OrderBook LimitOrders (r:13 w:11)
	/// Proof: OrderBook LimitOrders (max_values: None, max_size: Some(236), added: 2711, mode: MaxEncodedLen)
	/// Storage: Technical TechAccounts (r:1 w:0)
	/// Proof Skipped: Technical TechAccounts (max_values: None, max_size: None, mode: Measured)
	/// Storage: System Account (r:3 w:3)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// Storage: Tokens Accounts (r:3 w:3)
	/// Proof: Tokens Accounts (max_values: None, max_size: Some(136), added: 2611, mode: MaxEncodedLen)
	/// Storage: OrderBook AggregatedBids (r:1 w:1)
	/// Proof: OrderBook AggregatedBids (max_values: None, max_size: Some(34902), added: 37377, mode: MaxEncodedLen)
	/// Storage: OrderBook AggregatedAsks (r:1 w:1)
	/// Proof: OrderBook AggregatedAsks (max_values: None, max_size: Some(34902), added: 37377, mode: MaxEncodedLen)
	fn update_orderbook() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `6196`
		//  Estimated: `148224`
		// Minimum execution time: 446_249_000 picoseconds.
		Weight::from_parts(449_319_000, 148224)
			.saturating_add(RocksDbWeight::get().reads(25_u64))
			.saturating_add(RocksDbWeight::get().writes(20_u64))
	}
	/// Storage: OrderBook OrderBooks (r:1 w:1)
	/// Proof: OrderBook OrderBooks (max_values: None, max_size: Some(237), added: 2712, mode: MaxEncodedLen)
	fn change_orderbook_status() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `621`
		//  Estimated: `2712`
		// Minimum execution time: 24_411_000 picoseconds.
		Weight::from_parts(24_683_000, 2712)
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: OrderBook OrderBooks (r:1 w:1)
	/// Proof: OrderBook OrderBooks (max_values: None, max_size: Some(237), added: 2712, mode: MaxEncodedLen)
	/// Storage: Timestamp Now (r:1 w:0)
	/// Proof: Timestamp Now (max_values: Some(1), max_size: Some(8), added: 503, mode: MaxEncodedLen)
	/// Storage: Assets AssetInfos (r:1 w:0)
	/// Proof Skipped: Assets AssetInfos (max_values: None, max_size: None, mode: Measured)
	/// Storage: OrderBook UserLimitOrders (r:1 w:1)
	/// Proof: OrderBook UserLimitOrders (max_values: None, max_size: Some(16518), added: 18993, mode: MaxEncodedLen)
	/// Storage: OrderBook Asks (r:1 w:1)
	/// Proof: OrderBook Asks (max_values: None, max_size: Some(16503), added: 18978, mode: MaxEncodedLen)
	/// Storage: OrderBook AggregatedAsks (r:1 w:1)
	/// Proof: OrderBook AggregatedAsks (max_values: None, max_size: Some(34902), added: 37377, mode: MaxEncodedLen)
	/// Storage: OrderBook AggregatedBids (r:1 w:0)
	/// Proof: OrderBook AggregatedBids (max_values: None, max_size: Some(34902), added: 37377, mode: MaxEncodedLen)
	/// Storage: Technical TechAccounts (r:1 w:0)
	/// Proof Skipped: Technical TechAccounts (max_values: None, max_size: None, mode: Measured)
	/// Storage: Tokens Accounts (r:2 w:2)
	/// Proof: Tokens Accounts (max_values: None, max_size: Some(136), added: 2611, mode: MaxEncodedLen)
	/// Storage: System Account (r:1 w:0)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// Storage: OrderBook LimitOrders (r:1 w:1)
	/// Proof: OrderBook LimitOrders (max_values: None, max_size: Some(236), added: 2711, mode: MaxEncodedLen)
	/// Storage: OrderBook ExpirationsAgenda (r:1 w:1)
	/// Proof: OrderBook ExpirationsAgenda (max_values: None, max_size: Some(43014), added: 45489, mode: MaxEncodedLen)
	fn place_limit_order_without_cross_spread() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `80739`
		//  Estimated: `338393`
		// Minimum execution time: 469_067_000 picoseconds.
		Weight::from_parts(485_279_000, 338393)
			.saturating_add(RocksDbWeight::get().reads(13_u64))
			.saturating_add(RocksDbWeight::get().writes(8_u64))
	}
	/// Storage: OrderBook LimitOrders (r:1 w:1)
	/// Proof: OrderBook LimitOrders (max_values: None, max_size: Some(236), added: 2711, mode: MaxEncodedLen)
	/// Storage: OrderBook OrderBooks (r:1 w:0)
	/// Proof: OrderBook OrderBooks (max_values: None, max_size: Some(237), added: 2712, mode: MaxEncodedLen)
	/// Storage: Technical TechAccounts (r:1 w:0)
	/// Proof Skipped: Technical TechAccounts (max_values: None, max_size: None, mode: Measured)
	/// Storage: System Account (r:2 w:2)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// Storage: OrderBook Bids (r:1 w:1)
	/// Proof: OrderBook Bids (max_values: None, max_size: Some(16503), added: 18978, mode: MaxEncodedLen)
	/// Storage: OrderBook AggregatedBids (r:1 w:1)
	/// Proof: OrderBook AggregatedBids (max_values: None, max_size: Some(34902), added: 37377, mode: MaxEncodedLen)
	/// Storage: OrderBook UserLimitOrders (r:1 w:1)
	/// Proof: OrderBook UserLimitOrders (max_values: None, max_size: Some(16518), added: 18993, mode: MaxEncodedLen)
	/// Storage: OrderBook ExpirationsAgenda (r:1 w:1)
	/// Proof: OrderBook ExpirationsAgenda (max_values: None, max_size: Some(43014), added: 45489, mode: MaxEncodedLen)
	fn cancel_limit_order_first_expiration() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `115651`
		//  Estimated: `249592`
		// Minimum execution time: 832_644_000 picoseconds.
		Weight::from_parts(851_019_000, 249592)
			.saturating_add(RocksDbWeight::get().reads(9_u64))
			.saturating_add(RocksDbWeight::get().writes(7_u64))
	}
	/// Storage: OrderBook LimitOrders (r:1 w:1)
	/// Proof: OrderBook LimitOrders (max_values: None, max_size: Some(236), added: 2711, mode: MaxEncodedLen)
	/// Storage: OrderBook OrderBooks (r:1 w:0)
	/// Proof: OrderBook OrderBooks (max_values: None, max_size: Some(237), added: 2712, mode: MaxEncodedLen)
	/// Storage: Technical TechAccounts (r:1 w:0)
	/// Proof Skipped: Technical TechAccounts (max_values: None, max_size: None, mode: Measured)
	/// Storage: System Account (r:2 w:2)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// Storage: OrderBook Bids (r:1 w:1)
	/// Proof: OrderBook Bids (max_values: None, max_size: Some(16503), added: 18978, mode: MaxEncodedLen)
	/// Storage: OrderBook AggregatedBids (r:1 w:1)
	/// Proof: OrderBook AggregatedBids (max_values: None, max_size: Some(34902), added: 37377, mode: MaxEncodedLen)
	/// Storage: OrderBook UserLimitOrders (r:1 w:1)
	/// Proof: OrderBook UserLimitOrders (max_values: None, max_size: Some(16518), added: 18993, mode: MaxEncodedLen)
	/// Storage: OrderBook ExpirationsAgenda (r:1 w:1)
	/// Proof: OrderBook ExpirationsAgenda (max_values: None, max_size: Some(43014), added: 45489, mode: MaxEncodedLen)
	fn cancel_limit_order_last_expiration() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `115651`
		//  Estimated: `249592`
		// Minimum execution time: 807_267_000 picoseconds.
		Weight::from_parts(841_496_000, 249592)
			.saturating_add(RocksDbWeight::get().reads(9_u64))
			.saturating_add(RocksDbWeight::get().writes(7_u64))
	}
	/// Storage: Assets AssetInfos (r:1 w:0)
	/// Proof Skipped: Assets AssetInfos (max_values: None, max_size: None, mode: Measured)
	/// Storage: OrderBook OrderBooks (r:1 w:0)
	/// Proof: OrderBook OrderBooks (max_values: None, max_size: Some(237), added: 2712, mode: MaxEncodedLen)
	/// Storage: OrderBook AggregatedBids (r:1 w:1)
	/// Proof: OrderBook AggregatedBids (max_values: None, max_size: Some(34902), added: 37377, mode: MaxEncodedLen)
	/// Storage: OrderBook Bids (r:4 w:4)
	/// Proof: OrderBook Bids (max_values: None, max_size: Some(16503), added: 18978, mode: MaxEncodedLen)
	/// Storage: OrderBook LimitOrders (r:4000 w:4000)
	/// Proof: OrderBook LimitOrders (max_values: None, max_size: Some(236), added: 2711, mode: MaxEncodedLen)
	/// Storage: Technical TechAccounts (r:1 w:0)
	/// Proof Skipped: Technical TechAccounts (max_values: None, max_size: None, mode: Measured)
	/// Storage: Tokens Accounts (r:6 w:6)
	/// Proof: Tokens Accounts (max_values: None, max_size: Some(136), added: 2611, mode: MaxEncodedLen)
	/// Storage: System Account (r:2 w:2)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// Storage: OrderBook UserLimitOrders (r:4 w:4)
	/// Proof: OrderBook UserLimitOrders (max_values: None, max_size: Some(16518), added: 18993, mode: MaxEncodedLen)
	/// Storage: OrderBook ExpirationsAgenda (r:8 w:8)
	/// Proof: OrderBook ExpirationsAgenda (max_values: None, max_size: Some(43014), added: 45489, mode: MaxEncodedLen)
	fn execute_market_order() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1237740`
		//  Estimated: `13901187`
		// Minimum execution time: 309_262_821_000 picoseconds.
		Weight::from_parts(310_778_621_000, 13901187)
			.saturating_add(RocksDbWeight::get().reads(4028_u64))
			.saturating_add(RocksDbWeight::get().writes(4025_u64))
	}
	/// Storage: DEXManager DEXInfos (r:1 w:0)
	/// Proof Skipped: DEXManager DEXInfos (max_values: None, max_size: None, mode: Measured)
	/// Storage: OrderBook OrderBooks (r:1 w:0)
	/// Proof: OrderBook OrderBooks (max_values: None, max_size: Some(237), added: 2712, mode: MaxEncodedLen)
	/// Storage: OrderBook AggregatedBids (r:1 w:0)
	/// Proof: OrderBook AggregatedBids (max_values: None, max_size: Some(34902), added: 37377, mode: MaxEncodedLen)
	fn quote() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `35829`
		//  Estimated: `78393`
		// Minimum execution time: 905_284_000 picoseconds.
		Weight::from_parts(935_448_000, 78393)
			.saturating_add(RocksDbWeight::get().reads(3_u64))
	}
	/// Storage: DEXManager DEXInfos (r:1 w:0)
	/// Proof Skipped: DEXManager DEXInfos (max_values: None, max_size: None, mode: Measured)
	/// Storage: OrderBook OrderBooks (r:1 w:0)
	/// Proof: OrderBook OrderBooks (max_values: None, max_size: Some(237), added: 2712, mode: MaxEncodedLen)
	/// Storage: OrderBook AggregatedBids (r:1 w:1)
	/// Proof: OrderBook AggregatedBids (max_values: None, max_size: Some(34902), added: 37377, mode: MaxEncodedLen)
	/// Storage: OrderBook Bids (r:1 w:1)
	/// Proof: OrderBook Bids (max_values: None, max_size: Some(16503), added: 18978, mode: MaxEncodedLen)
	/// Storage: OrderBook LimitOrders (r:1 w:1)
	/// Proof: OrderBook LimitOrders (max_values: None, max_size: Some(236), added: 2711, mode: MaxEncodedLen)
	/// Storage: Technical TechAccounts (r:1 w:0)
	/// Proof Skipped: Technical TechAccounts (max_values: None, max_size: None, mode: Measured)
	/// Storage: Tokens Accounts (r:3 w:3)
	/// Proof: Tokens Accounts (max_values: None, max_size: Some(136), added: 2611, mode: MaxEncodedLen)
	/// Storage: System Account (r:2 w:2)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// Storage: OrderBook UserLimitOrders (r:1 w:1)
	/// Proof: OrderBook UserLimitOrders (max_values: None, max_size: Some(16518), added: 18993, mode: MaxEncodedLen)
	/// Storage: OrderBook ExpirationsAgenda (r:1 w:1)
	/// Proof: OrderBook ExpirationsAgenda (max_values: None, max_size: Some(43014), added: 45489, mode: MaxEncodedLen)
	fn exchange_single_order() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `5123`
		//  Estimated: `154495`
		// Minimum execution time: 229_529_000 picoseconds.
		Weight::from_parts(230_771_000, 154495)
			.saturating_add(RocksDbWeight::get().reads(13_u64))
			.saturating_add(RocksDbWeight::get().writes(10_u64))
	}
	/// Storage: OrderBook IncompleteExpirationsSince (r:1 w:0)
	/// Proof: OrderBook IncompleteExpirationsSince (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// Storage: OrderBook ExpirationsAgenda (r:1 w:0)
	/// Proof: OrderBook ExpirationsAgenda (max_values: None, max_size: Some(43014), added: 45489, mode: MaxEncodedLen)
	fn service_base() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `109`
		//  Estimated: `45988`
		// Minimum execution time: 5_245_000 picoseconds.
		Weight::from_parts(5_477_000, 45988)
			.saturating_add(RocksDbWeight::get().reads(2_u64))
	}
	/// Storage: OrderBook ExpirationsAgenda (r:1 w:0)
	/// Proof: OrderBook ExpirationsAgenda (max_values: None, max_size: Some(43014), added: 45489, mode: MaxEncodedLen)
	fn service_block_base() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `109`
		//  Estimated: `45489`
		// Minimum execution time: 3_731_000 picoseconds.
		Weight::from_parts(3_947_000, 45489)
			.saturating_add(RocksDbWeight::get().reads(1_u64))
	}
	/// Storage: OrderBook LimitOrders (r:1 w:0)
	/// Proof: OrderBook LimitOrders (max_values: None, max_size: Some(236), added: 2711, mode: MaxEncodedLen)
	/// Storage: OrderBook OrderBooks (r:1 w:0)
	/// Proof: OrderBook OrderBooks (max_values: None, max_size: Some(237), added: 2712, mode: MaxEncodedLen)
	/// Storage: Technical TechAccounts (r:1 w:0)
	/// Proof Skipped: Technical TechAccounts (max_values: None, max_size: None, mode: Measured)
	/// Storage: System Account (r:2 w:2)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// Storage: OrderBook Bids (r:1 w:0)
	/// Proof: OrderBook Bids (max_values: None, max_size: Some(16503), added: 18978, mode: MaxEncodedLen)
	/// Storage: OrderBook AggregatedBids (r:1 w:0)
	/// Proof: OrderBook AggregatedBids (max_values: None, max_size: Some(34902), added: 37377, mode: MaxEncodedLen)
	/// Storage: OrderBook UserLimitOrders (r:1 w:0)
	/// Proof: OrderBook UserLimitOrders (max_values: None, max_size: Some(16518), added: 18993, mode: MaxEncodedLen)
	/// Storage: OrderBook ExpirationsAgenda (r:1 w:1)
	/// Proof: OrderBook ExpirationsAgenda (max_values: None, max_size: Some(43014), added: 45489, mode: MaxEncodedLen)
	fn service_single_expiration() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `4017`
		//  Estimated: `137958`
		// Minimum execution time: 122_506_000 picoseconds.
		Weight::from_parts(124_248_000, 137958)
			.saturating_add(RocksDbWeight::get().reads(9_u64))
			.saturating_add(RocksDbWeight::get().writes(3_u64))
	}
}
