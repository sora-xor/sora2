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

//! Autogenerated weights for ceres_launchpad
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-07-28, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `a331765a0ed8`, CPU: `Intel(R) Xeon(R) Platinum 8124M CPU @ 3.00GHz`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("local"), DB CACHE: 1024

// Executed Command:
// /usr/local/bin/framenode
// benchmark
// pallet
// --chain=local
// --steps=50
// --repeat=20
// --pallet=ceres_launchpad
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --header=./misc/file_header.txt
// --template=./misc/pallet-weight-template.hbs
// --output=./pallets/ceres-launchpad/src/weights.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use core::marker::PhantomData;

/// Weight functions needed for ceres_launchpad.
pub trait WeightInfo {
	fn create_ilo() -> Weight;
	fn contribute() -> Weight;
	fn emergency_withdraw() -> Weight;
	fn finish_ilo() -> Weight;
	fn claim_lp_tokens() -> Weight;
	fn claim() -> Weight;
	fn change_ceres_burn_fee() -> Weight;
	fn change_ceres_contribution_fee() -> Weight;
	fn claim_pswap_rewards() -> Weight;
	fn add_whitelisted_contributor() -> Weight;
	fn remove_whitelisted_contributor() -> Weight;
	fn add_whitelisted_ilo_organizer() -> Weight;
	fn remove_whitelisted_ilo_organizer() -> Weight;
}

/// Weights for ceres_launchpad using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	/// Storage: CeresLaunchpad WhitelistedIloOrganizers (r:1 w:0)
	/// Proof Skipped: CeresLaunchpad WhitelistedIloOrganizers (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: CeresLaunchpad ILOs (r:1 w:1)
	/// Proof Skipped: CeresLaunchpad ILOs (max_values: None, max_size: None, mode: Measured)
	/// Storage: DEXManager DEXInfos (r:1 w:0)
	/// Proof Skipped: DEXManager DEXInfos (max_values: None, max_size: None, mode: Measured)
	/// Storage: TradingPair EnabledSources (r:1 w:0)
	/// Proof Skipped: TradingPair EnabledSources (max_values: None, max_size: None, mode: Measured)
	/// Storage: Timestamp Now (r:1 w:0)
	/// Proof: Timestamp Now (max_values: Some(1), max_size: Some(8), added: 503, mode: MaxEncodedLen)
	/// Storage: CeresLaunchpad CeresBurnFeeAmount (r:1 w:0)
	/// Proof Skipped: CeresLaunchpad CeresBurnFeeAmount (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Tokens Accounts (r:2 w:2)
	/// Proof: Tokens Accounts (max_values: None, max_size: Some(136), added: 2611, mode: MaxEncodedLen)
	/// Storage: Tokens TotalIssuance (r:1 w:1)
	/// Proof: Tokens TotalIssuance (max_values: None, max_size: Some(56), added: 2531, mode: MaxEncodedLen)
	/// Storage: System Account (r:2 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	fn create_ilo() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1642`
		//  Estimated: `30087`
		// Minimum execution time: 137_235_000 picoseconds.
		Weight::from_parts(138_437_000, 30087)
			.saturating_add(T::DbWeight::get().reads(11_u64))
			.saturating_add(T::DbWeight::get().writes(5_u64))
	}
	/// Storage: CeresLaunchpad WhitelistedContributors (r:1 w:0)
	/// Proof Skipped: CeresLaunchpad WhitelistedContributors (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Timestamp Now (r:1 w:0)
	/// Proof: Timestamp Now (max_values: Some(1), max_size: Some(8), added: 503, mode: MaxEncodedLen)
	/// Storage: CeresLaunchpad CeresForContributionInILO (r:1 w:0)
	/// Proof Skipped: CeresLaunchpad CeresForContributionInILO (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Tokens Accounts (r:1 w:0)
	/// Proof: Tokens Accounts (max_values: None, max_size: Some(136), added: 2611, mode: MaxEncodedLen)
	/// Storage: CeresLaunchpad ILOs (r:1 w:1)
	/// Proof Skipped: CeresLaunchpad ILOs (max_values: None, max_size: None, mode: Measured)
	/// Storage: CeresLaunchpad Contributions (r:1 w:1)
	/// Proof Skipped: CeresLaunchpad Contributions (max_values: None, max_size: None, mode: Measured)
	/// Storage: System Account (r:2 w:2)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	fn contribute() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1713`
		//  Estimated: `21112`
		// Minimum execution time: 93_638_000 picoseconds.
		Weight::from_parts(95_381_000, 21112)
			.saturating_add(T::DbWeight::get().reads(8_u64))
			.saturating_add(T::DbWeight::get().writes(4_u64))
	}
	/// Storage: Timestamp Now (r:1 w:0)
	/// Proof: Timestamp Now (max_values: Some(1), max_size: Some(8), added: 503, mode: MaxEncodedLen)
	/// Storage: CeresLaunchpad ILOs (r:1 w:1)
	/// Proof Skipped: CeresLaunchpad ILOs (max_values: None, max_size: None, mode: Measured)
	/// Storage: CeresLaunchpad Contributions (r:1 w:1)
	/// Proof Skipped: CeresLaunchpad Contributions (max_values: None, max_size: None, mode: Measured)
	/// Storage: System Account (r:3 w:3)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// Storage: CeresLaunchpad PenaltiesAccount (r:1 w:0)
	/// Proof Skipped: CeresLaunchpad PenaltiesAccount (max_values: Some(1), max_size: None, mode: Measured)
	fn emergency_withdraw() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1358`
		//  Estimated: `17831`
		// Minimum execution time: 113_705_000 picoseconds.
		Weight::from_parts(114_974_000, 17831)
			.saturating_add(T::DbWeight::get().reads(7_u64))
			.saturating_add(T::DbWeight::get().writes(5_u64))
	}
	/// Storage: CeresLaunchpad ILOs (r:1 w:1)
	/// Proof Skipped: CeresLaunchpad ILOs (max_values: None, max_size: None, mode: Measured)
	/// Storage: Timestamp Now (r:1 w:0)
	/// Proof: Timestamp Now (max_values: Some(1), max_size: Some(8), added: 503, mode: MaxEncodedLen)
	/// Storage: CeresLaunchpad FeePercentOnRaisedFunds (r:1 w:0)
	/// Proof Skipped: CeresLaunchpad FeePercentOnRaisedFunds (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: CeresLaunchpad AuthorityAccount (r:1 w:0)
	/// Proof Skipped: CeresLaunchpad AuthorityAccount (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: System Account (r:6 w:6)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// Storage: DEXManager DEXInfos (r:1 w:0)
	/// Proof Skipped: DEXManager DEXInfos (max_values: None, max_size: None, mode: Measured)
	/// Storage: Assets AssetOwners (r:2 w:0)
	/// Proof Skipped: Assets AssetOwners (max_values: None, max_size: None, mode: Measured)
	/// Storage: TradingPair EnabledSources (r:1 w:1)
	/// Proof Skipped: TradingPair EnabledSources (max_values: None, max_size: None, mode: Measured)
	/// Storage: Assets AssetInfos (r:2 w:0)
	/// Proof Skipped: Assets AssetInfos (max_values: None, max_size: None, mode: Measured)
	/// Storage: Technical TechAccounts (r:2 w:2)
	/// Proof Skipped: Technical TechAccounts (max_values: None, max_size: None, mode: Measured)
	/// Storage: XSTPool EnabledSynthetics (r:2 w:0)
	/// Proof Skipped: XSTPool EnabledSynthetics (max_values: None, max_size: None, mode: Measured)
	/// Storage: PswapDistribution SubscribedAccounts (r:1 w:1)
	/// Proof Skipped: PswapDistribution SubscribedAccounts (max_values: None, max_size: None, mode: Measured)
	/// Storage: Farming Pools (r:1 w:1)
	/// Proof Skipped: Farming Pools (max_values: None, max_size: None, mode: Measured)
	/// Storage: Tokens Accounts (r:5 w:5)
	/// Proof: Tokens Accounts (max_values: None, max_size: Some(136), added: 2611, mode: MaxEncodedLen)
	/// Storage: PoolXYK PoolProviders (r:2 w:2)
	/// Proof Skipped: PoolXYK PoolProviders (max_values: None, max_size: None, mode: Measured)
	/// Storage: PoolXYK AccountPools (r:2 w:2)
	/// Proof Skipped: PoolXYK AccountPools (max_values: None, max_size: None, mode: Measured)
	/// Storage: PoolXYK TotalIssuances (r:1 w:1)
	/// Proof Skipped: PoolXYK TotalIssuances (max_values: None, max_size: None, mode: Measured)
	/// Storage: PriceTools PriceInfos (r:1 w:0)
	/// Proof Skipped: PriceTools PriceInfos (max_values: None, max_size: None, mode: Measured)
	/// Storage: Tokens TotalIssuance (r:1 w:1)
	/// Proof: Tokens TotalIssuance (max_values: None, max_size: Some(56), added: 2531, mode: MaxEncodedLen)
	/// Storage: CeresLiquidityLocker LockerData (r:1 w:1)
	/// Proof Skipped: CeresLiquidityLocker LockerData (max_values: None, max_size: None, mode: Measured)
	/// Storage: CeresLiquidityLocker FeesOptionOneAccount (r:1 w:0)
	/// Proof Skipped: CeresLiquidityLocker FeesOptionOneAccount (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: DemeterFarmingPlatform UserInfos (r:1 w:1)
	/// Proof Skipped: DemeterFarmingPlatform UserInfos (max_values: None, max_size: None, mode: Measured)
	/// Storage: CeresTokenLocker FeeAmount (r:1 w:0)
	/// Proof Skipped: CeresTokenLocker FeeAmount (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: CeresTokenLocker FeesAccount (r:1 w:0)
	/// Proof Skipped: CeresTokenLocker FeesAccount (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: CeresTokenLocker TokenLockerData (r:1 w:1)
	/// Proof Skipped: CeresTokenLocker TokenLockerData (max_values: None, max_size: None, mode: Measured)
	/// Storage: PoolXYK Reserves (r:0 w:1)
	/// Proof Skipped: PoolXYK Reserves (max_values: None, max_size: None, mode: Measured)
	/// Storage: PoolXYK Properties (r:0 w:1)
	/// Proof Skipped: PoolXYK Properties (max_values: None, max_size: None, mode: Measured)
	fn finish_ilo() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `4808`
		//  Estimated: `199216`
		// Minimum execution time: 809_432_000 picoseconds.
		Weight::from_parts(838_448_000, 199216)
			.saturating_add(T::DbWeight::get().reads(40_u64))
			.saturating_add(T::DbWeight::get().writes(28_u64))
	}
	/// Storage: Timestamp Now (r:1 w:0)
	/// Proof: Timestamp Now (max_values: Some(1), max_size: Some(8), added: 503, mode: MaxEncodedLen)
	/// Storage: CeresLaunchpad ILOs (r:1 w:1)
	/// Proof Skipped: CeresLaunchpad ILOs (max_values: None, max_size: None, mode: Measured)
	/// Storage: PoolXYK Properties (r:1 w:0)
	/// Proof Skipped: PoolXYK Properties (max_values: None, max_size: None, mode: Measured)
	/// Storage: PoolXYK PoolProviders (r:2 w:2)
	/// Proof Skipped: PoolXYK PoolProviders (max_values: None, max_size: None, mode: Measured)
	/// Storage: PoolXYK AccountPools (r:1 w:1)
	/// Proof Skipped: PoolXYK AccountPools (max_values: None, max_size: None, mode: Measured)
	fn claim_lp_tokens() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1336`
		//  Estimated: `18222`
		// Minimum execution time: 62_853_000 picoseconds.
		Weight::from_parts(64_024_000, 18222)
			.saturating_add(T::DbWeight::get().reads(6_u64))
			.saturating_add(T::DbWeight::get().writes(4_u64))
	}
	/// Storage: CeresLaunchpad ILOs (r:1 w:0)
	/// Proof Skipped: CeresLaunchpad ILOs (max_values: None, max_size: None, mode: Measured)
	/// Storage: CeresLaunchpad Contributions (r:1 w:1)
	/// Proof Skipped: CeresLaunchpad Contributions (max_values: None, max_size: None, mode: Measured)
	/// Storage: Tokens Accounts (r:2 w:2)
	/// Proof: Tokens Accounts (max_values: None, max_size: Some(136), added: 2611, mode: MaxEncodedLen)
	/// Storage: System Account (r:1 w:0)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	fn claim() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1653`
		//  Estimated: `16081`
		// Minimum execution time: 76_117_000 picoseconds.
		Weight::from_parts(78_059_000, 16081)
			.saturating_add(T::DbWeight::get().reads(5_u64))
			.saturating_add(T::DbWeight::get().writes(3_u64))
	}
	/// Storage: CeresLaunchpad AuthorityAccount (r:1 w:0)
	/// Proof Skipped: CeresLaunchpad AuthorityAccount (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: CeresLaunchpad CeresBurnFeeAmount (r:0 w:1)
	/// Proof Skipped: CeresLaunchpad CeresBurnFeeAmount (max_values: Some(1), max_size: None, mode: Measured)
	fn change_ceres_burn_fee() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `76`
		//  Estimated: `647`
		// Minimum execution time: 14_213_000 picoseconds.
		Weight::from_parts(14_637_000, 647)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: CeresLaunchpad AuthorityAccount (r:1 w:0)
	/// Proof Skipped: CeresLaunchpad AuthorityAccount (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: CeresLaunchpad CeresForContributionInILO (r:0 w:1)
	/// Proof Skipped: CeresLaunchpad CeresForContributionInILO (max_values: Some(1), max_size: None, mode: Measured)
	fn change_ceres_contribution_fee() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `76`
		//  Estimated: `647`
		// Minimum execution time: 14_201_000 picoseconds.
		Weight::from_parts(14_518_000, 647)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: CeresLaunchpad AuthorityAccount (r:1 w:0)
	/// Proof Skipped: CeresLaunchpad AuthorityAccount (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: PswapDistribution ShareholderAccounts (r:1 w:1)
	/// Proof Skipped: PswapDistribution ShareholderAccounts (max_values: None, max_size: None, mode: Measured)
	/// Storage: PswapDistribution ClaimableShares (r:1 w:1)
	/// Proof Skipped: PswapDistribution ClaimableShares (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Tokens Accounts (r:3 w:3)
	/// Proof: Tokens Accounts (max_values: None, max_size: Some(136), added: 2611, mode: MaxEncodedLen)
	/// Storage: System Account (r:3 w:2)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// Storage: VestedRewards Rewards (r:1 w:0)
	/// Proof Skipped: VestedRewards Rewards (max_values: None, max_size: None, mode: Measured)
	fn claim_pswap_rewards() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1696`
		//  Estimated: `28366`
		// Minimum execution time: 150_597_000 picoseconds.
		Weight::from_parts(152_877_000, 28366)
			.saturating_add(T::DbWeight::get().reads(10_u64))
			.saturating_add(T::DbWeight::get().writes(7_u64))
	}
	/// Storage: CeresLaunchpad AuthorityAccount (r:1 w:0)
	/// Proof Skipped: CeresLaunchpad AuthorityAccount (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: CeresLaunchpad WhitelistedContributors (r:1 w:1)
	/// Proof Skipped: CeresLaunchpad WhitelistedContributors (max_values: Some(1), max_size: None, mode: Measured)
	fn add_whitelisted_contributor() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `76`
		//  Estimated: `1142`
		// Minimum execution time: 15_301_000 picoseconds.
		Weight::from_parts(15_912_000, 1142)
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: CeresLaunchpad AuthorityAccount (r:1 w:0)
	/// Proof Skipped: CeresLaunchpad AuthorityAccount (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: CeresLaunchpad WhitelistedContributors (r:1 w:1)
	/// Proof Skipped: CeresLaunchpad WhitelistedContributors (max_values: Some(1), max_size: None, mode: Measured)
	fn remove_whitelisted_contributor() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `76`
		//  Estimated: `1142`
		// Minimum execution time: 16_076_000 picoseconds.
		Weight::from_parts(16_514_000, 1142)
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: CeresLaunchpad AuthorityAccount (r:1 w:0)
	/// Proof Skipped: CeresLaunchpad AuthorityAccount (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: CeresLaunchpad WhitelistedIloOrganizers (r:1 w:1)
	/// Proof Skipped: CeresLaunchpad WhitelistedIloOrganizers (max_values: Some(1), max_size: None, mode: Measured)
	fn add_whitelisted_ilo_organizer() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `76`
		//  Estimated: `1142`
		// Minimum execution time: 15_697_000 picoseconds.
		Weight::from_parts(16_050_000, 1142)
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: CeresLaunchpad AuthorityAccount (r:1 w:0)
	/// Proof Skipped: CeresLaunchpad AuthorityAccount (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: CeresLaunchpad WhitelistedIloOrganizers (r:1 w:1)
	/// Proof Skipped: CeresLaunchpad WhitelistedIloOrganizers (max_values: Some(1), max_size: None, mode: Measured)
	fn remove_whitelisted_ilo_organizer() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `76`
		//  Estimated: `1142`
		// Minimum execution time: 15_954_000 picoseconds.
		Weight::from_parts(16_488_000, 1142)
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	/// Storage: CeresLaunchpad WhitelistedIloOrganizers (r:1 w:0)
	/// Proof Skipped: CeresLaunchpad WhitelistedIloOrganizers (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: CeresLaunchpad ILOs (r:1 w:1)
	/// Proof Skipped: CeresLaunchpad ILOs (max_values: None, max_size: None, mode: Measured)
	/// Storage: DEXManager DEXInfos (r:1 w:0)
	/// Proof Skipped: DEXManager DEXInfos (max_values: None, max_size: None, mode: Measured)
	/// Storage: TradingPair EnabledSources (r:1 w:0)
	/// Proof Skipped: TradingPair EnabledSources (max_values: None, max_size: None, mode: Measured)
	/// Storage: Timestamp Now (r:1 w:0)
	/// Proof: Timestamp Now (max_values: Some(1), max_size: Some(8), added: 503, mode: MaxEncodedLen)
	/// Storage: CeresLaunchpad CeresBurnFeeAmount (r:1 w:0)
	/// Proof Skipped: CeresLaunchpad CeresBurnFeeAmount (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Tokens Accounts (r:2 w:2)
	/// Proof: Tokens Accounts (max_values: None, max_size: Some(136), added: 2611, mode: MaxEncodedLen)
	/// Storage: Tokens TotalIssuance (r:1 w:1)
	/// Proof: Tokens TotalIssuance (max_values: None, max_size: Some(56), added: 2531, mode: MaxEncodedLen)
	/// Storage: System Account (r:2 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	fn create_ilo() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1642`
		//  Estimated: `30087`
		// Minimum execution time: 137_235_000 picoseconds.
		Weight::from_parts(138_437_000, 30087)
			.saturating_add(RocksDbWeight::get().reads(11_u64))
			.saturating_add(RocksDbWeight::get().writes(5_u64))
	}
	/// Storage: CeresLaunchpad WhitelistedContributors (r:1 w:0)
	/// Proof Skipped: CeresLaunchpad WhitelistedContributors (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Timestamp Now (r:1 w:0)
	/// Proof: Timestamp Now (max_values: Some(1), max_size: Some(8), added: 503, mode: MaxEncodedLen)
	/// Storage: CeresLaunchpad CeresForContributionInILO (r:1 w:0)
	/// Proof Skipped: CeresLaunchpad CeresForContributionInILO (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Tokens Accounts (r:1 w:0)
	/// Proof: Tokens Accounts (max_values: None, max_size: Some(136), added: 2611, mode: MaxEncodedLen)
	/// Storage: CeresLaunchpad ILOs (r:1 w:1)
	/// Proof Skipped: CeresLaunchpad ILOs (max_values: None, max_size: None, mode: Measured)
	/// Storage: CeresLaunchpad Contributions (r:1 w:1)
	/// Proof Skipped: CeresLaunchpad Contributions (max_values: None, max_size: None, mode: Measured)
	/// Storage: System Account (r:2 w:2)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	fn contribute() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1713`
		//  Estimated: `21112`
		// Minimum execution time: 93_638_000 picoseconds.
		Weight::from_parts(95_381_000, 21112)
			.saturating_add(RocksDbWeight::get().reads(8_u64))
			.saturating_add(RocksDbWeight::get().writes(4_u64))
	}
	/// Storage: Timestamp Now (r:1 w:0)
	/// Proof: Timestamp Now (max_values: Some(1), max_size: Some(8), added: 503, mode: MaxEncodedLen)
	/// Storage: CeresLaunchpad ILOs (r:1 w:1)
	/// Proof Skipped: CeresLaunchpad ILOs (max_values: None, max_size: None, mode: Measured)
	/// Storage: CeresLaunchpad Contributions (r:1 w:1)
	/// Proof Skipped: CeresLaunchpad Contributions (max_values: None, max_size: None, mode: Measured)
	/// Storage: System Account (r:3 w:3)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// Storage: CeresLaunchpad PenaltiesAccount (r:1 w:0)
	/// Proof Skipped: CeresLaunchpad PenaltiesAccount (max_values: Some(1), max_size: None, mode: Measured)
	fn emergency_withdraw() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1358`
		//  Estimated: `17831`
		// Minimum execution time: 113_705_000 picoseconds.
		Weight::from_parts(114_974_000, 17831)
			.saturating_add(RocksDbWeight::get().reads(7_u64))
			.saturating_add(RocksDbWeight::get().writes(5_u64))
	}
	/// Storage: CeresLaunchpad ILOs (r:1 w:1)
	/// Proof Skipped: CeresLaunchpad ILOs (max_values: None, max_size: None, mode: Measured)
	/// Storage: Timestamp Now (r:1 w:0)
	/// Proof: Timestamp Now (max_values: Some(1), max_size: Some(8), added: 503, mode: MaxEncodedLen)
	/// Storage: CeresLaunchpad FeePercentOnRaisedFunds (r:1 w:0)
	/// Proof Skipped: CeresLaunchpad FeePercentOnRaisedFunds (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: CeresLaunchpad AuthorityAccount (r:1 w:0)
	/// Proof Skipped: CeresLaunchpad AuthorityAccount (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: System Account (r:6 w:6)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// Storage: DEXManager DEXInfos (r:1 w:0)
	/// Proof Skipped: DEXManager DEXInfos (max_values: None, max_size: None, mode: Measured)
	/// Storage: Assets AssetOwners (r:2 w:0)
	/// Proof Skipped: Assets AssetOwners (max_values: None, max_size: None, mode: Measured)
	/// Storage: TradingPair EnabledSources (r:1 w:1)
	/// Proof Skipped: TradingPair EnabledSources (max_values: None, max_size: None, mode: Measured)
	/// Storage: Assets AssetInfos (r:2 w:0)
	/// Proof Skipped: Assets AssetInfos (max_values: None, max_size: None, mode: Measured)
	/// Storage: Technical TechAccounts (r:2 w:2)
	/// Proof Skipped: Technical TechAccounts (max_values: None, max_size: None, mode: Measured)
	/// Storage: XSTPool EnabledSynthetics (r:2 w:0)
	/// Proof Skipped: XSTPool EnabledSynthetics (max_values: None, max_size: None, mode: Measured)
	/// Storage: PswapDistribution SubscribedAccounts (r:1 w:1)
	/// Proof Skipped: PswapDistribution SubscribedAccounts (max_values: None, max_size: None, mode: Measured)
	/// Storage: Farming Pools (r:1 w:1)
	/// Proof Skipped: Farming Pools (max_values: None, max_size: None, mode: Measured)
	/// Storage: Tokens Accounts (r:5 w:5)
	/// Proof: Tokens Accounts (max_values: None, max_size: Some(136), added: 2611, mode: MaxEncodedLen)
	/// Storage: PoolXYK PoolProviders (r:2 w:2)
	/// Proof Skipped: PoolXYK PoolProviders (max_values: None, max_size: None, mode: Measured)
	/// Storage: PoolXYK AccountPools (r:2 w:2)
	/// Proof Skipped: PoolXYK AccountPools (max_values: None, max_size: None, mode: Measured)
	/// Storage: PoolXYK TotalIssuances (r:1 w:1)
	/// Proof Skipped: PoolXYK TotalIssuances (max_values: None, max_size: None, mode: Measured)
	/// Storage: PriceTools PriceInfos (r:1 w:0)
	/// Proof Skipped: PriceTools PriceInfos (max_values: None, max_size: None, mode: Measured)
	/// Storage: Tokens TotalIssuance (r:1 w:1)
	/// Proof: Tokens TotalIssuance (max_values: None, max_size: Some(56), added: 2531, mode: MaxEncodedLen)
	/// Storage: CeresLiquidityLocker LockerData (r:1 w:1)
	/// Proof Skipped: CeresLiquidityLocker LockerData (max_values: None, max_size: None, mode: Measured)
	/// Storage: CeresLiquidityLocker FeesOptionOneAccount (r:1 w:0)
	/// Proof Skipped: CeresLiquidityLocker FeesOptionOneAccount (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: DemeterFarmingPlatform UserInfos (r:1 w:1)
	/// Proof Skipped: DemeterFarmingPlatform UserInfos (max_values: None, max_size: None, mode: Measured)
	/// Storage: CeresTokenLocker FeeAmount (r:1 w:0)
	/// Proof Skipped: CeresTokenLocker FeeAmount (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: CeresTokenLocker FeesAccount (r:1 w:0)
	/// Proof Skipped: CeresTokenLocker FeesAccount (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: CeresTokenLocker TokenLockerData (r:1 w:1)
	/// Proof Skipped: CeresTokenLocker TokenLockerData (max_values: None, max_size: None, mode: Measured)
	/// Storage: PoolXYK Reserves (r:0 w:1)
	/// Proof Skipped: PoolXYK Reserves (max_values: None, max_size: None, mode: Measured)
	/// Storage: PoolXYK Properties (r:0 w:1)
	/// Proof Skipped: PoolXYK Properties (max_values: None, max_size: None, mode: Measured)
	fn finish_ilo() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `4808`
		//  Estimated: `199216`
		// Minimum execution time: 809_432_000 picoseconds.
		Weight::from_parts(838_448_000, 199216)
			.saturating_add(RocksDbWeight::get().reads(40_u64))
			.saturating_add(RocksDbWeight::get().writes(28_u64))
	}
	/// Storage: Timestamp Now (r:1 w:0)
	/// Proof: Timestamp Now (max_values: Some(1), max_size: Some(8), added: 503, mode: MaxEncodedLen)
	/// Storage: CeresLaunchpad ILOs (r:1 w:1)
	/// Proof Skipped: CeresLaunchpad ILOs (max_values: None, max_size: None, mode: Measured)
	/// Storage: PoolXYK Properties (r:1 w:0)
	/// Proof Skipped: PoolXYK Properties (max_values: None, max_size: None, mode: Measured)
	/// Storage: PoolXYK PoolProviders (r:2 w:2)
	/// Proof Skipped: PoolXYK PoolProviders (max_values: None, max_size: None, mode: Measured)
	/// Storage: PoolXYK AccountPools (r:1 w:1)
	/// Proof Skipped: PoolXYK AccountPools (max_values: None, max_size: None, mode: Measured)
	fn claim_lp_tokens() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1336`
		//  Estimated: `18222`
		// Minimum execution time: 62_853_000 picoseconds.
		Weight::from_parts(64_024_000, 18222)
			.saturating_add(RocksDbWeight::get().reads(6_u64))
			.saturating_add(RocksDbWeight::get().writes(4_u64))
	}
	/// Storage: CeresLaunchpad ILOs (r:1 w:0)
	/// Proof Skipped: CeresLaunchpad ILOs (max_values: None, max_size: None, mode: Measured)
	/// Storage: CeresLaunchpad Contributions (r:1 w:1)
	/// Proof Skipped: CeresLaunchpad Contributions (max_values: None, max_size: None, mode: Measured)
	/// Storage: Tokens Accounts (r:2 w:2)
	/// Proof: Tokens Accounts (max_values: None, max_size: Some(136), added: 2611, mode: MaxEncodedLen)
	/// Storage: System Account (r:1 w:0)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	fn claim() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1653`
		//  Estimated: `16081`
		// Minimum execution time: 76_117_000 picoseconds.
		Weight::from_parts(78_059_000, 16081)
			.saturating_add(RocksDbWeight::get().reads(5_u64))
			.saturating_add(RocksDbWeight::get().writes(3_u64))
	}
	/// Storage: CeresLaunchpad AuthorityAccount (r:1 w:0)
	/// Proof Skipped: CeresLaunchpad AuthorityAccount (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: CeresLaunchpad CeresBurnFeeAmount (r:0 w:1)
	/// Proof Skipped: CeresLaunchpad CeresBurnFeeAmount (max_values: Some(1), max_size: None, mode: Measured)
	fn change_ceres_burn_fee() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `76`
		//  Estimated: `647`
		// Minimum execution time: 14_213_000 picoseconds.
		Weight::from_parts(14_637_000, 647)
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: CeresLaunchpad AuthorityAccount (r:1 w:0)
	/// Proof Skipped: CeresLaunchpad AuthorityAccount (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: CeresLaunchpad CeresForContributionInILO (r:0 w:1)
	/// Proof Skipped: CeresLaunchpad CeresForContributionInILO (max_values: Some(1), max_size: None, mode: Measured)
	fn change_ceres_contribution_fee() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `76`
		//  Estimated: `647`
		// Minimum execution time: 14_201_000 picoseconds.
		Weight::from_parts(14_518_000, 647)
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: CeresLaunchpad AuthorityAccount (r:1 w:0)
	/// Proof Skipped: CeresLaunchpad AuthorityAccount (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: PswapDistribution ShareholderAccounts (r:1 w:1)
	/// Proof Skipped: PswapDistribution ShareholderAccounts (max_values: None, max_size: None, mode: Measured)
	/// Storage: PswapDistribution ClaimableShares (r:1 w:1)
	/// Proof Skipped: PswapDistribution ClaimableShares (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Tokens Accounts (r:3 w:3)
	/// Proof: Tokens Accounts (max_values: None, max_size: Some(136), added: 2611, mode: MaxEncodedLen)
	/// Storage: System Account (r:3 w:2)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// Storage: VestedRewards Rewards (r:1 w:0)
	/// Proof Skipped: VestedRewards Rewards (max_values: None, max_size: None, mode: Measured)
	fn claim_pswap_rewards() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1696`
		//  Estimated: `28366`
		// Minimum execution time: 150_597_000 picoseconds.
		Weight::from_parts(152_877_000, 28366)
			.saturating_add(RocksDbWeight::get().reads(10_u64))
			.saturating_add(RocksDbWeight::get().writes(7_u64))
	}
	/// Storage: CeresLaunchpad AuthorityAccount (r:1 w:0)
	/// Proof Skipped: CeresLaunchpad AuthorityAccount (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: CeresLaunchpad WhitelistedContributors (r:1 w:1)
	/// Proof Skipped: CeresLaunchpad WhitelistedContributors (max_values: Some(1), max_size: None, mode: Measured)
	fn add_whitelisted_contributor() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `76`
		//  Estimated: `1142`
		// Minimum execution time: 15_301_000 picoseconds.
		Weight::from_parts(15_912_000, 1142)
			.saturating_add(RocksDbWeight::get().reads(2_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: CeresLaunchpad AuthorityAccount (r:1 w:0)
	/// Proof Skipped: CeresLaunchpad AuthorityAccount (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: CeresLaunchpad WhitelistedContributors (r:1 w:1)
	/// Proof Skipped: CeresLaunchpad WhitelistedContributors (max_values: Some(1), max_size: None, mode: Measured)
	fn remove_whitelisted_contributor() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `76`
		//  Estimated: `1142`
		// Minimum execution time: 16_076_000 picoseconds.
		Weight::from_parts(16_514_000, 1142)
			.saturating_add(RocksDbWeight::get().reads(2_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: CeresLaunchpad AuthorityAccount (r:1 w:0)
	/// Proof Skipped: CeresLaunchpad AuthorityAccount (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: CeresLaunchpad WhitelistedIloOrganizers (r:1 w:1)
	/// Proof Skipped: CeresLaunchpad WhitelistedIloOrganizers (max_values: Some(1), max_size: None, mode: Measured)
	fn add_whitelisted_ilo_organizer() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `76`
		//  Estimated: `1142`
		// Minimum execution time: 15_697_000 picoseconds.
		Weight::from_parts(16_050_000, 1142)
			.saturating_add(RocksDbWeight::get().reads(2_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: CeresLaunchpad AuthorityAccount (r:1 w:0)
	/// Proof Skipped: CeresLaunchpad AuthorityAccount (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: CeresLaunchpad WhitelistedIloOrganizers (r:1 w:1)
	/// Proof Skipped: CeresLaunchpad WhitelistedIloOrganizers (max_values: Some(1), max_size: None, mode: Measured)
	fn remove_whitelisted_ilo_organizer() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `76`
		//  Estimated: `1142`
		// Minimum execution time: 15_954_000 picoseconds.
		Weight::from_parts(16_488_000, 1142)
			.saturating_add(RocksDbWeight::get().reads(2_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
}
