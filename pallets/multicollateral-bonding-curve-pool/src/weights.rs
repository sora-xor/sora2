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

//! Autogenerated weights for multicollateral_bonding_curve_pool
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-06-01, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `58c787de2e36`, CPU: `Intel(R) Xeon(R) Platinum 8275CL CPU @ 3.00GHz`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("local"), DB CACHE: 1024

// Executed Command:
// /usr/local/bin/framenode
// benchmark
// pallet
// --chain=local
// --steps=50
// --repeat=20
// --pallet=multicollateral_bonding_curve_pool
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --header=./misc/file_header.txt
// --template=./misc/pallet-weight-template.hbs
// --output=./pallets/multicollateral-bonding-curve-pool/src/weights.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use core::marker::PhantomData;

/// Weight functions needed for multicollateral_bonding_curve_pool.
pub trait WeightInfo {
	fn initialize_pool() -> Weight;
	fn set_reference_asset() -> Weight;
	fn set_optional_reward_multiplier() -> Weight;
	fn on_initialize(n: u32, ) -> Weight;
	fn set_price_change_config() -> Weight;
	fn set_price_bias() -> Weight;
	fn quote() -> Weight;
	fn exchange() -> Weight;
	fn can_exchange() -> Weight;
	fn check_rewards() -> Weight;
}

/// Weights for multicollateral_bonding_curve_pool using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	/// Storage: DEXManager DEXInfos (r:1 w:0)
	/// Proof Skipped: DEXManager DEXInfos (max_values: None, max_size: None, mode: Measured)
	/// Storage: Permissions Permissions (r:1 w:0)
	/// Proof Skipped: Permissions Permissions (max_values: None, max_size: None, mode: Measured)
	/// Storage: MulticollateralBondingCurvePool EnabledTargets (r:1 w:1)
	/// Proof Skipped: MulticollateralBondingCurvePool EnabledTargets (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: PriceTools PriceInfos (r:1 w:1)
	/// Proof Skipped: PriceTools PriceInfos (max_values: None, max_size: None, mode: Measured)
	/// Storage: TradingPair EnabledSources (r:1 w:1)
	/// Proof Skipped: TradingPair EnabledSources (max_values: None, max_size: None, mode: Measured)
	/// Storage: MulticollateralBondingCurvePool IncentivisedCurrenciesNum (r:1 w:1)
	/// Proof Skipped: MulticollateralBondingCurvePool IncentivisedCurrenciesNum (max_values: Some(1), max_size: None, mode: Measured)
	fn initialize_pool() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1713`
		//  Estimated: `21168`
		// Minimum execution time: 77_639_000 picoseconds.
		Weight::from_parts(78_952_000, 21168)
			.saturating_add(T::DbWeight::get().reads(6_u64))
			.saturating_add(T::DbWeight::get().writes(4_u64))
	}
	/// Storage: DEXManager DEXInfos (r:1 w:0)
	/// Proof Skipped: DEXManager DEXInfos (max_values: None, max_size: None, mode: Measured)
	/// Storage: Permissions Permissions (r:1 w:0)
	/// Proof Skipped: Permissions Permissions (max_values: None, max_size: None, mode: Measured)
	/// Storage: MulticollateralBondingCurvePool ReferenceAssetId (r:0 w:1)
	/// Proof Skipped: MulticollateralBondingCurvePool ReferenceAssetId (max_values: Some(1), max_size: None, mode: Measured)
	fn set_reference_asset() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `726`
		//  Estimated: `7128`
		// Minimum execution time: 31_775_000 picoseconds.
		Weight::from_parts(32_070_000, 7128)
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: DEXManager DEXInfos (r:1 w:0)
	/// Proof Skipped: DEXManager DEXInfos (max_values: None, max_size: None, mode: Measured)
	/// Storage: Permissions Permissions (r:1 w:0)
	/// Proof Skipped: Permissions Permissions (max_values: None, max_size: None, mode: Measured)
	/// Storage: MulticollateralBondingCurvePool EnabledTargets (r:1 w:0)
	/// Proof Skipped: MulticollateralBondingCurvePool EnabledTargets (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: MulticollateralBondingCurvePool AssetsWithOptionalRewardMultiplier (r:1 w:1)
	/// Proof Skipped: MulticollateralBondingCurvePool AssetsWithOptionalRewardMultiplier (max_values: None, max_size: None, mode: Measured)
	fn set_optional_reward_multiplier() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1116`
		//  Estimated: `12384`
		// Minimum execution time: 41_076_000 picoseconds.
		Weight::from_parts(41_714_000, 12384)
			.saturating_add(T::DbWeight::get().reads(4_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: MulticollateralBondingCurvePool FreeReservesAccountId (r:1 w:0)
	/// Proof Skipped: MulticollateralBondingCurvePool FreeReservesAccountId (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: MulticollateralBondingCurvePool PendingFreeReserves (r:1 w:1)
	/// Proof Skipped: MulticollateralBondingCurvePool PendingFreeReserves (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: DEXManager DEXInfos (r:1 w:0)
	/// Proof Skipped: DEXManager DEXInfos (max_values: None, max_size: None, mode: Measured)
	/// Storage: XSTPool EnabledSynthetics (r:2 w:0)
	/// Proof Skipped: XSTPool EnabledSynthetics (max_values: None, max_size: None, mode: Measured)
	/// Storage: DEXAPI EnabledSourceTypes (r:1 w:0)
	/// Proof Skipped: DEXAPI EnabledSourceTypes (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: PoolXYK Properties (r:1 w:0)
	/// Proof Skipped: PoolXYK Properties (max_values: None, max_size: None, mode: Measured)
	/// Storage: TradingPair LockedLiquiditySources (r:1 w:0)
	/// Proof Skipped: TradingPair LockedLiquiditySources (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Tokens Accounts (r:2 w:0)
	/// Proof: Tokens Accounts (max_values: None, max_size: Some(136), added: 2611, mode: MaxEncodedLen)
	/// Storage: System Account (r:1 w:0)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// Storage: Technical TechAccounts (r:1 w:0)
	/// Proof Skipped: Technical TechAccounts (max_values: None, max_size: None, mode: Measured)
	/// Storage: Assets AssetOwners (r:1 w:0)
	/// Proof Skipped: Assets AssetOwners (max_values: None, max_size: None, mode: Measured)
	/// The range of component `n` is `[0, 10]`.
	fn on_initialize(n: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `3222 + n * (48 ±0)`
		//  Estimated: `51599 + n * (754 ±11)`
		// Minimum execution time: 11_338_000 picoseconds.
		Weight::from_parts(75_977_820, 51599)
			// Standard Error: 391_937
			.saturating_add(Weight::from_parts(110_487_151, 0).saturating_mul(n.into()))
			.saturating_add(T::DbWeight::get().reads(9_u64))
			.saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(n.into())))
			.saturating_add(T::DbWeight::get().writes(1_u64))
			.saturating_add(Weight::from_parts(0, 754).saturating_mul(n.into()))
	}
	/// Storage: MulticollateralBondingCurvePool PriceChangeStep (r:0 w:1)
	/// Proof Skipped: MulticollateralBondingCurvePool PriceChangeStep (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: MulticollateralBondingCurvePool PriceChangeRate (r:0 w:1)
	/// Proof Skipped: MulticollateralBondingCurvePool PriceChangeRate (max_values: Some(1), max_size: None, mode: Measured)
	fn set_price_change_config() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 13_779_000 picoseconds.
		Weight::from_parts(14_201_000, 0)
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
	/// Storage: MulticollateralBondingCurvePool InitialPrice (r:0 w:1)
	/// Proof Skipped: MulticollateralBondingCurvePool InitialPrice (max_values: Some(1), max_size: None, mode: Measured)
	fn set_price_bias() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 12_340_000 picoseconds.
		Weight::from_parts(12_651_000, 0)
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: MulticollateralBondingCurvePool EnabledTargets (r:1 w:0)
	/// Proof Skipped: MulticollateralBondingCurvePool EnabledTargets (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: MulticollateralBondingCurvePool PriceChangeStep (r:1 w:0)
	/// Proof Skipped: MulticollateralBondingCurvePool PriceChangeStep (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: MulticollateralBondingCurvePool PriceChangeRate (r:1 w:0)
	/// Proof Skipped: MulticollateralBondingCurvePool PriceChangeRate (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: MulticollateralBondingCurvePool InitialPrice (r:1 w:0)
	/// Proof Skipped: MulticollateralBondingCurvePool InitialPrice (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: MulticollateralBondingCurvePool ReferenceAssetId (r:1 w:0)
	/// Proof Skipped: MulticollateralBondingCurvePool ReferenceAssetId (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: PriceTools PriceInfos (r:2 w:0)
	/// Proof Skipped: PriceTools PriceInfos (max_values: None, max_size: None, mode: Measured)
	/// Storage: MulticollateralBondingCurvePool BaseFee (r:1 w:0)
	/// Proof Skipped: MulticollateralBondingCurvePool BaseFee (max_values: Some(1), max_size: None, mode: Measured)
	fn quote() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `2878`
		//  Estimated: `28066`
		// Minimum execution time: 46_379_000 picoseconds.
		Weight::from_parts(47_330_000, 28066)
			.saturating_add(T::DbWeight::get().reads(8_u64))
	}
	/// Storage: MulticollateralBondingCurvePool EnabledTargets (r:1 w:0)
	/// Proof Skipped: MulticollateralBondingCurvePool EnabledTargets (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: MulticollateralBondingCurvePool ReservesAcc (r:1 w:0)
	/// Proof Skipped: MulticollateralBondingCurvePool ReservesAcc (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: MulticollateralBondingCurvePool PriceChangeStep (r:1 w:0)
	/// Proof Skipped: MulticollateralBondingCurvePool PriceChangeStep (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: MulticollateralBondingCurvePool PriceChangeRate (r:1 w:0)
	/// Proof Skipped: MulticollateralBondingCurvePool PriceChangeRate (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: MulticollateralBondingCurvePool InitialPrice (r:1 w:0)
	/// Proof Skipped: MulticollateralBondingCurvePool InitialPrice (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: MulticollateralBondingCurvePool ReferenceAssetId (r:1 w:0)
	/// Proof Skipped: MulticollateralBondingCurvePool ReferenceAssetId (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: PriceTools PriceInfos (r:2 w:0)
	/// Proof Skipped: PriceTools PriceInfos (max_values: None, max_size: None, mode: Measured)
	/// Storage: MulticollateralBondingCurvePool BaseFee (r:1 w:0)
	/// Proof Skipped: MulticollateralBondingCurvePool BaseFee (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Tokens Accounts (r:3 w:3)
	/// Proof: Tokens Accounts (max_values: None, max_size: Some(136), added: 2611, mode: MaxEncodedLen)
	/// Storage: Assets AssetOwners (r:1 w:0)
	/// Proof Skipped: Assets AssetOwners (max_values: None, max_size: None, mode: Measured)
	/// Storage: MulticollateralBondingCurvePool IncentivisedCurrenciesNum (r:1 w:0)
	/// Proof Skipped: MulticollateralBondingCurvePool IncentivisedCurrenciesNum (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: MulticollateralBondingCurvePool InitialPswapRewardsSupply (r:1 w:0)
	/// Proof Skipped: MulticollateralBondingCurvePool InitialPswapRewardsSupply (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: MulticollateralBondingCurvePool AssetsWithOptionalRewardMultiplier (r:1 w:0)
	/// Proof Skipped: MulticollateralBondingCurvePool AssetsWithOptionalRewardMultiplier (max_values: None, max_size: None, mode: Measured)
	/// Storage: VestedRewards Rewards (r:1 w:1)
	/// Proof Skipped: VestedRewards Rewards (max_values: None, max_size: None, mode: Measured)
	/// Storage: System Account (r:3 w:3)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// Storage: VestedRewards TotalRewards (r:1 w:1)
	/// Proof Skipped: VestedRewards TotalRewards (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Technical TechAccounts (r:1 w:0)
	/// Proof Skipped: Technical TechAccounts (max_values: None, max_size: None, mode: Measured)
	/// Storage: MulticollateralBondingCurvePool AlwaysDistributeCoefficient (r:1 w:0)
	/// Proof Skipped: MulticollateralBondingCurvePool AlwaysDistributeCoefficient (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: DEXManager DEXInfos (r:1 w:0)
	/// Proof Skipped: DEXManager DEXInfos (max_values: None, max_size: None, mode: Measured)
	/// Storage: XSTPool EnabledSynthetics (r:2 w:0)
	/// Proof Skipped: XSTPool EnabledSynthetics (max_values: None, max_size: None, mode: Measured)
	/// Storage: DEXAPI EnabledSourceTypes (r:1 w:0)
	/// Proof Skipped: DEXAPI EnabledSourceTypes (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: PoolXYK Properties (r:1 w:0)
	/// Proof Skipped: PoolXYK Properties (max_values: None, max_size: None, mode: Measured)
	/// Storage: TradingPair LockedLiquiditySources (r:1 w:0)
	/// Proof Skipped: TradingPair LockedLiquiditySources (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: MulticollateralBondingCurvePool FreeReservesAccountId (r:1 w:0)
	/// Proof Skipped: MulticollateralBondingCurvePool FreeReservesAccountId (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: MulticollateralBondingCurvePool PendingFreeReserves (r:1 w:1)
	/// Proof Skipped: MulticollateralBondingCurvePool PendingFreeReserves (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Assets AssetInfos (r:1 w:0)
	/// Proof Skipped: Assets AssetInfos (max_values: None, max_size: None, mode: Measured)
	/// Storage: Permissions Permissions (r:2 w:0)
	/// Proof Skipped: Permissions Permissions (max_values: None, max_size: None, mode: Measured)
	/// Storage: MulticollateralBondingCurvePool CollateralReserves (r:0 w:1)
	/// Proof Skipped: MulticollateralBondingCurvePool CollateralReserves (max_values: None, max_size: None, mode: Measured)
	fn exchange() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `6774`
		//  Estimated: `231366`
		// Minimum execution time: 352_956_000 picoseconds.
		Weight::from_parts(355_138_000, 231366)
			.saturating_add(T::DbWeight::get().reads(34_u64))
			.saturating_add(T::DbWeight::get().writes(10_u64))
	}
	/// Storage: MulticollateralBondingCurvePool EnabledTargets (r:1 w:0)
	/// Proof Skipped: MulticollateralBondingCurvePool EnabledTargets (max_values: Some(1), max_size: None, mode: Measured)
	fn can_exchange() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `390`
		//  Estimated: `885`
		// Minimum execution time: 6_861_000 picoseconds.
		Weight::from_parts(7_056_000, 885)
			.saturating_add(T::DbWeight::get().reads(1_u64))
	}
	/// Storage: MulticollateralBondingCurvePool EnabledTargets (r:1 w:0)
	/// Proof Skipped: MulticollateralBondingCurvePool EnabledTargets (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: MulticollateralBondingCurvePool ReservesAcc (r:1 w:0)
	/// Proof Skipped: MulticollateralBondingCurvePool ReservesAcc (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: MulticollateralBondingCurvePool InitialPrice (r:1 w:0)
	/// Proof Skipped: MulticollateralBondingCurvePool InitialPrice (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: MulticollateralBondingCurvePool PriceChangeStep (r:1 w:0)
	/// Proof Skipped: MulticollateralBondingCurvePool PriceChangeStep (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: MulticollateralBondingCurvePool PriceChangeRate (r:1 w:0)
	/// Proof Skipped: MulticollateralBondingCurvePool PriceChangeRate (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Tokens Accounts (r:1 w:0)
	/// Proof: Tokens Accounts (max_values: None, max_size: Some(136), added: 2611, mode: MaxEncodedLen)
	/// Storage: Assets AssetOwners (r:1 w:0)
	/// Proof Skipped: Assets AssetOwners (max_values: None, max_size: None, mode: Measured)
	/// Storage: MulticollateralBondingCurvePool ReferenceAssetId (r:1 w:0)
	/// Proof Skipped: MulticollateralBondingCurvePool ReferenceAssetId (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: PriceTools PriceInfos (r:2 w:0)
	/// Proof Skipped: PriceTools PriceInfos (max_values: None, max_size: None, mode: Measured)
	/// Storage: MulticollateralBondingCurvePool IncentivisedCurrenciesNum (r:1 w:0)
	/// Proof Skipped: MulticollateralBondingCurvePool IncentivisedCurrenciesNum (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: MulticollateralBondingCurvePool InitialPswapRewardsSupply (r:1 w:0)
	/// Proof Skipped: MulticollateralBondingCurvePool InitialPswapRewardsSupply (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: MulticollateralBondingCurvePool AssetsWithOptionalRewardMultiplier (r:1 w:0)
	/// Proof Skipped: MulticollateralBondingCurvePool AssetsWithOptionalRewardMultiplier (max_values: None, max_size: None, mode: Measured)
	fn check_rewards() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `3791`
		//  Estimated: `58172`
		// Minimum execution time: 78_041_000 picoseconds.
		Weight::from_parts(79_738_000, 58172)
			.saturating_add(T::DbWeight::get().reads(13_u64))
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	/// Storage: DEXManager DEXInfos (r:1 w:0)
	/// Proof Skipped: DEXManager DEXInfos (max_values: None, max_size: None, mode: Measured)
	/// Storage: Permissions Permissions (r:1 w:0)
	/// Proof Skipped: Permissions Permissions (max_values: None, max_size: None, mode: Measured)
	/// Storage: MulticollateralBondingCurvePool EnabledTargets (r:1 w:1)
	/// Proof Skipped: MulticollateralBondingCurvePool EnabledTargets (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: PriceTools PriceInfos (r:1 w:1)
	/// Proof Skipped: PriceTools PriceInfos (max_values: None, max_size: None, mode: Measured)
	/// Storage: TradingPair EnabledSources (r:1 w:1)
	/// Proof Skipped: TradingPair EnabledSources (max_values: None, max_size: None, mode: Measured)
	/// Storage: MulticollateralBondingCurvePool IncentivisedCurrenciesNum (r:1 w:1)
	/// Proof Skipped: MulticollateralBondingCurvePool IncentivisedCurrenciesNum (max_values: Some(1), max_size: None, mode: Measured)
	fn initialize_pool() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1713`
		//  Estimated: `21168`
		// Minimum execution time: 77_639_000 picoseconds.
		Weight::from_parts(78_952_000, 21168)
			.saturating_add(RocksDbWeight::get().reads(6_u64))
			.saturating_add(RocksDbWeight::get().writes(4_u64))
	}
	/// Storage: DEXManager DEXInfos (r:1 w:0)
	/// Proof Skipped: DEXManager DEXInfos (max_values: None, max_size: None, mode: Measured)
	/// Storage: Permissions Permissions (r:1 w:0)
	/// Proof Skipped: Permissions Permissions (max_values: None, max_size: None, mode: Measured)
	/// Storage: MulticollateralBondingCurvePool ReferenceAssetId (r:0 w:1)
	/// Proof Skipped: MulticollateralBondingCurvePool ReferenceAssetId (max_values: Some(1), max_size: None, mode: Measured)
	fn set_reference_asset() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `726`
		//  Estimated: `7128`
		// Minimum execution time: 31_775_000 picoseconds.
		Weight::from_parts(32_070_000, 7128)
			.saturating_add(RocksDbWeight::get().reads(2_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: DEXManager DEXInfos (r:1 w:0)
	/// Proof Skipped: DEXManager DEXInfos (max_values: None, max_size: None, mode: Measured)
	/// Storage: Permissions Permissions (r:1 w:0)
	/// Proof Skipped: Permissions Permissions (max_values: None, max_size: None, mode: Measured)
	/// Storage: MulticollateralBondingCurvePool EnabledTargets (r:1 w:0)
	/// Proof Skipped: MulticollateralBondingCurvePool EnabledTargets (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: MulticollateralBondingCurvePool AssetsWithOptionalRewardMultiplier (r:1 w:1)
	/// Proof Skipped: MulticollateralBondingCurvePool AssetsWithOptionalRewardMultiplier (max_values: None, max_size: None, mode: Measured)
	fn set_optional_reward_multiplier() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1116`
		//  Estimated: `12384`
		// Minimum execution time: 41_076_000 picoseconds.
		Weight::from_parts(41_714_000, 12384)
			.saturating_add(RocksDbWeight::get().reads(4_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: MulticollateralBondingCurvePool FreeReservesAccountId (r:1 w:0)
	/// Proof Skipped: MulticollateralBondingCurvePool FreeReservesAccountId (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: MulticollateralBondingCurvePool PendingFreeReserves (r:1 w:1)
	/// Proof Skipped: MulticollateralBondingCurvePool PendingFreeReserves (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: DEXManager DEXInfos (r:1 w:0)
	/// Proof Skipped: DEXManager DEXInfos (max_values: None, max_size: None, mode: Measured)
	/// Storage: XSTPool EnabledSynthetics (r:2 w:0)
	/// Proof Skipped: XSTPool EnabledSynthetics (max_values: None, max_size: None, mode: Measured)
	/// Storage: DEXAPI EnabledSourceTypes (r:1 w:0)
	/// Proof Skipped: DEXAPI EnabledSourceTypes (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: PoolXYK Properties (r:1 w:0)
	/// Proof Skipped: PoolXYK Properties (max_values: None, max_size: None, mode: Measured)
	/// Storage: TradingPair LockedLiquiditySources (r:1 w:0)
	/// Proof Skipped: TradingPair LockedLiquiditySources (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Tokens Accounts (r:2 w:0)
	/// Proof: Tokens Accounts (max_values: None, max_size: Some(136), added: 2611, mode: MaxEncodedLen)
	/// Storage: System Account (r:1 w:0)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// Storage: Technical TechAccounts (r:1 w:0)
	/// Proof Skipped: Technical TechAccounts (max_values: None, max_size: None, mode: Measured)
	/// Storage: Assets AssetOwners (r:1 w:0)
	/// Proof Skipped: Assets AssetOwners (max_values: None, max_size: None, mode: Measured)
	/// The range of component `n` is `[0, 10]`.
	fn on_initialize(n: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `3222 + n * (48 ±0)`
		//  Estimated: `51599 + n * (754 ±11)`
		// Minimum execution time: 11_338_000 picoseconds.
		Weight::from_parts(75_977_820, 51599)
			// Standard Error: 391_937
			.saturating_add(Weight::from_parts(110_487_151, 0).saturating_mul(n.into()))
			.saturating_add(RocksDbWeight::get().reads(9_u64))
			.saturating_add(RocksDbWeight::get().reads((1_u64).saturating_mul(n.into())))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
			.saturating_add(Weight::from_parts(0, 754).saturating_mul(n.into()))
	}
	/// Storage: MulticollateralBondingCurvePool PriceChangeStep (r:0 w:1)
	/// Proof Skipped: MulticollateralBondingCurvePool PriceChangeStep (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: MulticollateralBondingCurvePool PriceChangeRate (r:0 w:1)
	/// Proof Skipped: MulticollateralBondingCurvePool PriceChangeRate (max_values: Some(1), max_size: None, mode: Measured)
	fn set_price_change_config() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 13_779_000 picoseconds.
		Weight::from_parts(14_201_000, 0)
			.saturating_add(RocksDbWeight::get().writes(2_u64))
	}
	/// Storage: MulticollateralBondingCurvePool InitialPrice (r:0 w:1)
	/// Proof Skipped: MulticollateralBondingCurvePool InitialPrice (max_values: Some(1), max_size: None, mode: Measured)
	fn set_price_bias() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 12_340_000 picoseconds.
		Weight::from_parts(12_651_000, 0)
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: MulticollateralBondingCurvePool EnabledTargets (r:1 w:0)
	/// Proof Skipped: MulticollateralBondingCurvePool EnabledTargets (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: MulticollateralBondingCurvePool PriceChangeStep (r:1 w:0)
	/// Proof Skipped: MulticollateralBondingCurvePool PriceChangeStep (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: MulticollateralBondingCurvePool PriceChangeRate (r:1 w:0)
	/// Proof Skipped: MulticollateralBondingCurvePool PriceChangeRate (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: MulticollateralBondingCurvePool InitialPrice (r:1 w:0)
	/// Proof Skipped: MulticollateralBondingCurvePool InitialPrice (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: MulticollateralBondingCurvePool ReferenceAssetId (r:1 w:0)
	/// Proof Skipped: MulticollateralBondingCurvePool ReferenceAssetId (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: PriceTools PriceInfos (r:2 w:0)
	/// Proof Skipped: PriceTools PriceInfos (max_values: None, max_size: None, mode: Measured)
	/// Storage: MulticollateralBondingCurvePool BaseFee (r:1 w:0)
	/// Proof Skipped: MulticollateralBondingCurvePool BaseFee (max_values: Some(1), max_size: None, mode: Measured)
	fn quote() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `2878`
		//  Estimated: `28066`
		// Minimum execution time: 46_379_000 picoseconds.
		Weight::from_parts(47_330_000, 28066)
			.saturating_add(RocksDbWeight::get().reads(8_u64))
	}
	/// Storage: MulticollateralBondingCurvePool EnabledTargets (r:1 w:0)
	/// Proof Skipped: MulticollateralBondingCurvePool EnabledTargets (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: MulticollateralBondingCurvePool ReservesAcc (r:1 w:0)
	/// Proof Skipped: MulticollateralBondingCurvePool ReservesAcc (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: MulticollateralBondingCurvePool PriceChangeStep (r:1 w:0)
	/// Proof Skipped: MulticollateralBondingCurvePool PriceChangeStep (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: MulticollateralBondingCurvePool PriceChangeRate (r:1 w:0)
	/// Proof Skipped: MulticollateralBondingCurvePool PriceChangeRate (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: MulticollateralBondingCurvePool InitialPrice (r:1 w:0)
	/// Proof Skipped: MulticollateralBondingCurvePool InitialPrice (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: MulticollateralBondingCurvePool ReferenceAssetId (r:1 w:0)
	/// Proof Skipped: MulticollateralBondingCurvePool ReferenceAssetId (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: PriceTools PriceInfos (r:2 w:0)
	/// Proof Skipped: PriceTools PriceInfos (max_values: None, max_size: None, mode: Measured)
	/// Storage: MulticollateralBondingCurvePool BaseFee (r:1 w:0)
	/// Proof Skipped: MulticollateralBondingCurvePool BaseFee (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Tokens Accounts (r:3 w:3)
	/// Proof: Tokens Accounts (max_values: None, max_size: Some(136), added: 2611, mode: MaxEncodedLen)
	/// Storage: Assets AssetOwners (r:1 w:0)
	/// Proof Skipped: Assets AssetOwners (max_values: None, max_size: None, mode: Measured)
	/// Storage: MulticollateralBondingCurvePool IncentivisedCurrenciesNum (r:1 w:0)
	/// Proof Skipped: MulticollateralBondingCurvePool IncentivisedCurrenciesNum (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: MulticollateralBondingCurvePool InitialPswapRewardsSupply (r:1 w:0)
	/// Proof Skipped: MulticollateralBondingCurvePool InitialPswapRewardsSupply (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: MulticollateralBondingCurvePool AssetsWithOptionalRewardMultiplier (r:1 w:0)
	/// Proof Skipped: MulticollateralBondingCurvePool AssetsWithOptionalRewardMultiplier (max_values: None, max_size: None, mode: Measured)
	/// Storage: VestedRewards Rewards (r:1 w:1)
	/// Proof Skipped: VestedRewards Rewards (max_values: None, max_size: None, mode: Measured)
	/// Storage: System Account (r:3 w:3)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// Storage: VestedRewards TotalRewards (r:1 w:1)
	/// Proof Skipped: VestedRewards TotalRewards (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Technical TechAccounts (r:1 w:0)
	/// Proof Skipped: Technical TechAccounts (max_values: None, max_size: None, mode: Measured)
	/// Storage: MulticollateralBondingCurvePool AlwaysDistributeCoefficient (r:1 w:0)
	/// Proof Skipped: MulticollateralBondingCurvePool AlwaysDistributeCoefficient (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: DEXManager DEXInfos (r:1 w:0)
	/// Proof Skipped: DEXManager DEXInfos (max_values: None, max_size: None, mode: Measured)
	/// Storage: XSTPool EnabledSynthetics (r:2 w:0)
	/// Proof Skipped: XSTPool EnabledSynthetics (max_values: None, max_size: None, mode: Measured)
	/// Storage: DEXAPI EnabledSourceTypes (r:1 w:0)
	/// Proof Skipped: DEXAPI EnabledSourceTypes (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: PoolXYK Properties (r:1 w:0)
	/// Proof Skipped: PoolXYK Properties (max_values: None, max_size: None, mode: Measured)
	/// Storage: TradingPair LockedLiquiditySources (r:1 w:0)
	/// Proof Skipped: TradingPair LockedLiquiditySources (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: MulticollateralBondingCurvePool FreeReservesAccountId (r:1 w:0)
	/// Proof Skipped: MulticollateralBondingCurvePool FreeReservesAccountId (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: MulticollateralBondingCurvePool PendingFreeReserves (r:1 w:1)
	/// Proof Skipped: MulticollateralBondingCurvePool PendingFreeReserves (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Assets AssetInfos (r:1 w:0)
	/// Proof Skipped: Assets AssetInfos (max_values: None, max_size: None, mode: Measured)
	/// Storage: Permissions Permissions (r:2 w:0)
	/// Proof Skipped: Permissions Permissions (max_values: None, max_size: None, mode: Measured)
	/// Storage: MulticollateralBondingCurvePool CollateralReserves (r:0 w:1)
	/// Proof Skipped: MulticollateralBondingCurvePool CollateralReserves (max_values: None, max_size: None, mode: Measured)
	fn exchange() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `6774`
		//  Estimated: `231366`
		// Minimum execution time: 352_956_000 picoseconds.
		Weight::from_parts(355_138_000, 231366)
			.saturating_add(RocksDbWeight::get().reads(34_u64))
			.saturating_add(RocksDbWeight::get().writes(10_u64))
	}
	/// Storage: MulticollateralBondingCurvePool EnabledTargets (r:1 w:0)
	/// Proof Skipped: MulticollateralBondingCurvePool EnabledTargets (max_values: Some(1), max_size: None, mode: Measured)
	fn can_exchange() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `390`
		//  Estimated: `885`
		// Minimum execution time: 6_861_000 picoseconds.
		Weight::from_parts(7_056_000, 885)
			.saturating_add(RocksDbWeight::get().reads(1_u64))
	}
	/// Storage: MulticollateralBondingCurvePool EnabledTargets (r:1 w:0)
	/// Proof Skipped: MulticollateralBondingCurvePool EnabledTargets (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: MulticollateralBondingCurvePool ReservesAcc (r:1 w:0)
	/// Proof Skipped: MulticollateralBondingCurvePool ReservesAcc (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: MulticollateralBondingCurvePool InitialPrice (r:1 w:0)
	/// Proof Skipped: MulticollateralBondingCurvePool InitialPrice (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: MulticollateralBondingCurvePool PriceChangeStep (r:1 w:0)
	/// Proof Skipped: MulticollateralBondingCurvePool PriceChangeStep (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: MulticollateralBondingCurvePool PriceChangeRate (r:1 w:0)
	/// Proof Skipped: MulticollateralBondingCurvePool PriceChangeRate (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Tokens Accounts (r:1 w:0)
	/// Proof: Tokens Accounts (max_values: None, max_size: Some(136), added: 2611, mode: MaxEncodedLen)
	/// Storage: Assets AssetOwners (r:1 w:0)
	/// Proof Skipped: Assets AssetOwners (max_values: None, max_size: None, mode: Measured)
	/// Storage: MulticollateralBondingCurvePool ReferenceAssetId (r:1 w:0)
	/// Proof Skipped: MulticollateralBondingCurvePool ReferenceAssetId (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: PriceTools PriceInfos (r:2 w:0)
	/// Proof Skipped: PriceTools PriceInfos (max_values: None, max_size: None, mode: Measured)
	/// Storage: MulticollateralBondingCurvePool IncentivisedCurrenciesNum (r:1 w:0)
	/// Proof Skipped: MulticollateralBondingCurvePool IncentivisedCurrenciesNum (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: MulticollateralBondingCurvePool InitialPswapRewardsSupply (r:1 w:0)
	/// Proof Skipped: MulticollateralBondingCurvePool InitialPswapRewardsSupply (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: MulticollateralBondingCurvePool AssetsWithOptionalRewardMultiplier (r:1 w:0)
	/// Proof Skipped: MulticollateralBondingCurvePool AssetsWithOptionalRewardMultiplier (max_values: None, max_size: None, mode: Measured)
	fn check_rewards() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `3791`
		//  Estimated: `58172`
		// Minimum execution time: 78_041_000 picoseconds.
		Weight::from_parts(79_738_000, 58172)
			.saturating_add(RocksDbWeight::get().reads(13_u64))
	}
}
