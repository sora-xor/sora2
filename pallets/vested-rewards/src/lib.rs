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

#![cfg_attr(not(feature = "std"), no_std)]

#[allow(unused_imports)]
#[macro_use]
extern crate alloc;

use codec::{Decode, Encode};
use common::fixed_wrapper;
use common::prelude::{Balance, FixedWrapper, QuoteAmount};
use common::{
    balance, Fixed, OnPswapBurned, PswapRemintInfo, RewardReason, VestedRewardsPallet, PSWAP, VAL,
    XSTUSD,
};
use core::convert::TryFrom;
use frame_support::dispatch::{DispatchError, DispatchResult};
use frame_support::fail;
use frame_support::traits::{Get, IsType};
use frame_support::weights::Weight;
use liquidity_proxy::LiquidityProxyTrait;
use serde::{Deserialize, Serialize};
use sp_runtime::traits::{UniqueSaturatedInto, Zero};
use sp_std::collections::btree_map::BTreeMap;
use sp_std::convert::TryInto;
use sp_std::str;
use sp_std::vec::Vec;

pub mod weights;

mod benchmarking;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

pub mod migrations;

pub const TECH_ACCOUNT_PREFIX: &[u8] = b"vested-rewards";
pub const TECH_ACCOUNT_MARKET_MAKERS: &[u8] = b"market-makers";
pub const TECH_ACCOUNT_CROWDLOAN: &[u8] = b"crowdloan";
pub const TECH_ACCOUNT_FARMING: &[u8] = b"farming";
pub const MARKET_MAKER_ELIGIBILITY_TX_COUNT: u32 = 500;
pub const SINGLE_MARKET_MAKER_DISTRIBUTION_AMOUNT: Balance = balance!(20000000);
pub const FARMING_REWARDS: Balance = balance!(3500000000);
pub const VAL_CROWDLOAN_REWARDS: Balance = balance!(676393);
pub const PSWAP_CROWDLOAN_REWARDS: Balance = balance!(9363480);
pub const XSTUSD_CROWDLOAN_REWARDS: Balance = balance!(77050);
pub const MARKET_MAKER_REWARDS_DISTRIBUTION_FREQUENCY: u32 = 432000;
pub const BLOCKS_PER_DAY: u128 = 14400;
#[cfg(not(feature = "private-net"))]
pub const LEASE_START_BLOCK: u128 = 4_397_212;
#[cfg(feature = "private-net")]
pub const LEASE_START_BLOCK: u128 = 0;
pub const LEASE_TOTAL_DAYS: u128 = 318;

type Assets<T> = assets::Pallet<T>;
type AccountIdOf<T> = <T as frame_system::Config>::AccountId;

/// Denotes PSWAP rewards amounts of particular types available for user.
#[derive(
    Encode, Decode, Eq, PartialEq, Clone, PartialOrd, Ord, Debug, Default, scale_info::TypeInfo,
)]
pub struct RewardInfo {
    /// Reward amount vested, denotes portion of `total_avialable` which can be claimed.
    /// Reset to 0 after claim until more is vested over time.
    limit: Balance,
    /// Sum of reward amounts in `rewards`.
    total_available: Balance,
    /// Mapping between reward type represented by `RewardReason` and owned amount by user.
    pub rewards: BTreeMap<RewardReason, Balance>,
}

/// Denotes information about users who make transactions counted for market makers strategic
/// rewards programme. To participate in rewards distribution account needs to get 500+ tx's over 1
/// XOR in volume each.
#[derive(
    Encode, Decode, Eq, PartialEq, Clone, PartialOrd, Ord, Debug, Default, scale_info::TypeInfo,
)]
pub struct MarketMakerInfo {
    /// Number of eligible transactions - namely those with individual volume over 1 XOR.
    count: u32,
    /// Cumulative volume of eligible transactions.
    volume: Balance,
}

/// A vested reward for crowdloan.
#[derive(
    Encode, Decode, Deserialize, Serialize, Clone, Debug, Default, PartialEq, scale_info::TypeInfo,
)]
pub struct CrowdloanReward {
    /// The user id
    #[serde(with = "serde_bytes", rename = "ID")]
    pub id: Vec<u8>,
    /// The user address
    #[serde(with = "hex", rename = "Address")]
    pub address: Vec<u8>,
    /// Kusama contribution
    #[serde(rename = "Contribution")]
    pub contribution: Fixed,
    /// Reward in XOR
    #[serde(rename = "XOR Reward")]
    pub xor_reward: Fixed,
    /// Reward in VAL
    #[serde(rename = "Val Reward")]
    pub val_reward: Fixed,
    /// Reward in PSWAP
    #[serde(rename = "PSWAP Reward")]
    pub pswap_reward: Fixed,
    /// Reward in XSTUSD
    #[serde(rename = "XSTUSD Reward")]
    pub xstusd_reward: Fixed,
    /// Reward in percents of the total contribution
    #[serde(rename = "Percent")]
    pub percent: Fixed,
}

pub trait WeightInfo {
    fn claim_incentives() -> Weight;
    fn on_initialize(_n: u32) -> Weight;
    fn set_asset_pair() -> Weight;
    fn claim_crowdloan_rewards() -> Weight;
}

impl<T: Config> Pallet<T> {
    pub fn add_pending_reward(
        account_id: &T::AccountId,
        reason: RewardReason,
        amount: Balance,
    ) -> DispatchResult {
        if !Rewards::<T>::contains_key(account_id) {
            frame_system::Pallet::<T>::inc_consumers(account_id)
                .map_err(|_| Error::<T>::IncRefError)?;
        }
        Rewards::<T>::mutate(account_id, |info| {
            info.total_available = info.total_available.saturating_add(amount);
            info.rewards
                .entry(reason)
                .and_modify(|e| *e = e.saturating_add(amount))
                .or_insert(amount);
        });
        TotalRewards::<T>::mutate(|balance| *balance = balance.saturating_add(amount));
        Ok(())
    }

    /// General claim function, which updates user reward status.
    pub fn claim_rewards_inner(account_id: &T::AccountId) -> DispatchResult {
        let mut remove_after_mutate = false;
        let result = Rewards::<T>::mutate(account_id, |info| {
            if info.total_available.is_zero() {
                fail!(Error::<T>::NothingToClaim);
            } else if info.limit.is_zero() {
                fail!(Error::<T>::ClaimLimitExceeded);
            } else {
                let mut total_actual_claimed: Balance = 0;
                for (&reward_reason, amount) in info.rewards.iter_mut() {
                    let claimable = (*amount).min(info.limit);
                    let actual_claimed = Self::claim_reward_by_reason(
                        account_id,
                        reward_reason,
                        &PSWAP.into(),
                        claimable,
                    )
                    .unwrap_or(balance!(0));
                    info.limit = info.limit.saturating_sub(actual_claimed);
                    total_actual_claimed = total_actual_claimed.saturating_add(actual_claimed);
                    if claimable > actual_claimed {
                        Self::deposit_event(Event::<T>::ActualDoesntMatchAvailable(reward_reason));
                    }
                    *amount = amount.saturating_sub(actual_claimed);
                }
                // clear zeroed entries
                // NOTE: .retain() is an unstable feature yet
                info.rewards = info
                    .rewards
                    .clone()
                    .into_iter()
                    .filter(|&(_, reward)| reward > balance!(0))
                    .collect();
                if total_actual_claimed.is_zero() {
                    fail!(Error::<T>::RewardsSupplyShortage);
                }
                info.total_available = info.total_available.saturating_sub(total_actual_claimed);
                TotalRewards::<T>::mutate(|total| {
                    *total = total.saturating_sub(total_actual_claimed)
                });
                remove_after_mutate = info.total_available == 0;
                Ok(())
            }
        });
        if result.is_ok() && remove_after_mutate {
            Rewards::<T>::remove(account_id);
            frame_system::Pallet::<T>::dec_consumers(account_id);
        }
        result
    }

    /// Claim rewards from account with reserves dedicated for particular reward type.
    pub fn claim_reward_by_reason(
        account_id: &T::AccountId,
        reason: RewardReason,
        asset_id: &T::AssetId,
        amount: Balance,
    ) -> Result<Balance, DispatchError> {
        let source_account = match reason {
            RewardReason::BuyOnBondingCurve => T::GetBondingCurveRewardsAccountId::get(),
            RewardReason::LiquidityProvisionFarming => T::GetFarmingRewardsAccountId::get(),
            RewardReason::MarketMakerVolume => T::GetMarketMakerRewardsAccountId::get(),
            RewardReason::Crowdloan => T::GetCrowdloanRewardsAccountId::get(),
            _ => fail!(Error::<T>::UnhandledRewardType),
        };
        let available_rewards = Assets::<T>::free_balance(asset_id, &source_account)?;
        if available_rewards.is_zero() {
            fail!(Error::<T>::RewardsSupplyShortage);
        }
        let amount = amount.min(available_rewards);
        Assets::<T>::transfer_from(asset_id, &source_account, account_id, amount)?;
        Ok(amount)
    }

    pub fn distribute_limits(vested_amount: Balance) {
        let total_rewards = TotalRewards::<T>::get();

        // if there's no accounts to vest, then amount is not utilized nor stored
        if !total_rewards.is_zero() {
            Rewards::<T>::translate(|_key: T::AccountId, mut info: RewardInfo| {
                let share_of_the_vested_amount = FixedWrapper::from(info.total_available)
                    * FixedWrapper::from(vested_amount)
                    / FixedWrapper::from(total_rewards);

                let new_limit = (share_of_the_vested_amount + FixedWrapper::from(info.limit))
                    .try_into_balance()
                    .unwrap_or(info.limit);

                // don't vest more than available
                info.limit = new_limit.min(info.total_available);
                Some(info)
            })
        };
    }

    /// Returns number of accounts who received rewards.
    pub fn market_maker_rewards_distribution_routine() -> u32 {
        // collect list of accounts with volume info
        let mut eligible_accounts = Vec::new();
        let mut total_eligible_volume = balance!(0);
        for (account, info) in MarketMakersRegistry::<T>::drain() {
            if info.count >= MARKET_MAKER_ELIGIBILITY_TX_COUNT {
                eligible_accounts.push((account, info.volume));
                total_eligible_volume = total_eligible_volume.saturating_add(info.volume);
            }
        }
        let eligible_accounts_count = eligible_accounts.len();
        if total_eligible_volume > 0 {
            for (account, volume) in eligible_accounts {
                let reward = (FixedWrapper::from(volume)
                    * FixedWrapper::from(SINGLE_MARKET_MAKER_DISTRIBUTION_AMOUNT)
                    / FixedWrapper::from(total_eligible_volume))
                .try_into_balance()
                .unwrap_or(0);
                if reward > 0 {
                    let res =
                        Self::add_pending_reward(&account, RewardReason::MarketMakerVolume, reward);
                    if res.is_err() {
                        Self::deposit_event(Event::<T>::FailedToSaveCalculatedReward(account))
                    }
                } else {
                    Self::deposit_event(Event::<T>::AddingZeroMarketMakerReward(account));
                }
            }
        } else {
            Self::deposit_event(Event::<T>::NoEligibleMarketMakers);
        }
        eligible_accounts_count.try_into().unwrap_or(u32::MAX)
    }

    pub fn crowdloan_reward_for_asset(
        address: &T::AccountId,
        asset_id: &T::AssetId,
        current_block_number: u128,
    ) -> Result<Balance, DispatchError> {
        let rewards =
            CrowdloanRewards::<T>::try_get(address).map_err(|_| Error::<T>::NothingToClaim)?;
        let last_claim_block: u128 =
            CrowdloanClaimHistory::<T>::get(address, asset_id).unique_saturated_into();
        let claim_period = if last_claim_block.is_zero() {
            current_block_number.saturating_sub(LEASE_START_BLOCK)
        } else {
            current_block_number.saturating_sub(last_claim_block)
        };
        let claim_days = Fixed::try_from(claim_period / BLOCKS_PER_DAY)
            .map_err(|_| DispatchError::from(Error::<T>::NumberConversionError))?;
        let reward = if asset_id == &VAL.into() {
            rewards.val_reward
        } else if asset_id == &PSWAP.into() {
            rewards.pswap_reward
        } else if asset_id == &XSTUSD.into() {
            rewards.xstusd_reward
        } else {
            return Err(Error::<T>::NoRewardsForAsset.into());
        };
        let reward = reward
            / Fixed::try_from(LEASE_TOTAL_DAYS)
                .map_err(|_| DispatchError::from(Error::<T>::NumberConversionError))?
                .into();

        (reward * claim_days)
            .try_into_balance()
            .map_err(|_| Error::<T>::ArithmeticError.into())
    }
}

impl<T: Config> OnPswapBurned for Pallet<T> {
    /// NOTE: currently is not invoked.
    /// Invoked when pswap is burned after being exchanged from collected liquidity provider fees.
    fn on_pswap_burned(distribution: PswapRemintInfo) {
        Pallet::<T>::distribute_limits(distribution.vesting)
    }
}

impl<T: Config> VestedRewardsPallet<T::AccountId, T::AssetId> for Pallet<T> {
    /// Check if volume is eligible to be counted for market maker rewards and add it to registry.
    /// `count` is used as a multiplier if multiple times same volume is transferred inside
    /// transaction.
    fn update_market_maker_records(
        account_id: &T::AccountId,
        base_asset: &T::AssetId,
        base_asset_volume: Balance,
        count: u32,
        from_asset_id: &T::AssetId,
        to_asset_id: &T::AssetId,
        intermediate_asset_id: Option<&T::AssetId>,
    ) -> DispatchResult {
        let allowed = if let Some(intermediate) = intermediate_asset_id {
            MarketMakingPairs::<T>::contains_key(from_asset_id, intermediate)
                && MarketMakingPairs::<T>::contains_key(intermediate, to_asset_id)
        } else {
            MarketMakingPairs::<T>::contains_key(from_asset_id, to_asset_id)
        };

        let xor_price = if base_asset == &common::XOR.into() {
            fixed_wrapper!(1)
        } else {
            let price = T::LiquidityProxy::quote(
                common::DEXId::Polkaswap.into(),
                base_asset,
                &common::XOR.into(),
                QuoteAmount::WithDesiredInput {
                    desired_amount_in: balance!(1),
                },
                common::LiquiditySourceFilter::empty(common::DEXId::Polkaswap.into()),
                false,
            )
            .map_err(|_| Error::<T>::UnableToGetBaseAssetPrice)?
            .amount;
            FixedWrapper::from(price)
        };
        let xor_volume = (xor_price * base_asset_volume)
            .try_into_balance()
            .map_err(|_| Error::<T>::ArithmeticError)?;

        if allowed && xor_volume >= balance!(1) {
            MarketMakersRegistry::<T>::mutate(account_id, |info| {
                info.count = info.count.saturating_add(count);
                info.volume = info
                    .volume
                    .saturating_add(xor_volume.saturating_mul(count as Balance));
            });
        }
        Ok(())
    }

    fn add_tbc_reward(account_id: &T::AccountId, pswap_amount: Balance) -> DispatchResult {
        Pallet::<T>::add_pending_reward(account_id, RewardReason::BuyOnBondingCurve, pswap_amount)
    }

    fn add_farming_reward(account_id: &T::AccountId, pswap_amount: Balance) -> DispatchResult {
        Pallet::<T>::add_pending_reward(
            account_id,
            RewardReason::LiquidityProvisionFarming,
            pswap_amount,
        )
    }

    fn add_market_maker_reward(account_id: &T::AccountId, pswap_amount: Balance) -> DispatchResult {
        Pallet::<T>::add_pending_reward(account_id, RewardReason::MarketMakerVolume, pswap_amount)
    }
}

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
    use super::*;
    use frame_support::dispatch::DispatchResultWithPostInfo;
    use frame_support::pallet_prelude::*;
    use frame_support::traits::StorageVersion;
    use frame_support::transactional;
    use frame_system::pallet_prelude::*;
    use sp_runtime::traits::UniqueSaturatedFrom;

    #[pallet::config]
    pub trait Config:
        frame_system::Config
        + common::Config
        + assets::Config
        + multicollateral_bonding_curve_pool::Config
    {
        type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
        /// Accounts holding PSWAP dedicated for rewards.
        type GetMarketMakerRewardsAccountId: Get<Self::AccountId>;
        type GetFarmingRewardsAccountId: Get<Self::AccountId>;
        type GetBondingCurveRewardsAccountId: Get<Self::AccountId>;
        type GetCrowdloanRewardsAccountId: Get<Self::AccountId>;
        /// Weight information for extrinsics in this pallet.
        type WeightInfo: WeightInfo;
    }

    /// The current storage version.
    const STORAGE_VERSION: StorageVersion = StorageVersion::new(1);

    #[pallet::pallet]
    #[pallet::generate_store(pub(super) trait Store)]
    #[pallet::storage_version(STORAGE_VERSION)]
    #[pallet::without_storage_info]
    pub struct Pallet<T>(PhantomData<T>);

    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
        fn on_initialize(block_number: T::BlockNumber) -> Weight {
            if (block_number % MARKET_MAKER_REWARDS_DISTRIBUTION_FREQUENCY.into()).is_zero() {
                let elems = Pallet::<T>::market_maker_rewards_distribution_routine();
                <T as Config>::WeightInfo::on_initialize(elems)
            } else {
                <T as Config>::WeightInfo::on_initialize(0)
            }
        }
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        /// Claim all available PSWAP rewards by account signing this transaction.
        #[transactional]
        #[pallet::weight(<T as Config>::WeightInfo::claim_incentives())]

        pub fn claim_rewards(origin: OriginFor<T>) -> DispatchResultWithPostInfo {
            let who = ensure_signed(origin)?;
            Self::claim_rewards_inner(&who)?;
            Ok(().into())
        }

        #[transactional]
        #[pallet::weight(<T as Config>::WeightInfo::claim_crowdloan_rewards())]

        pub fn claim_crowdloan_rewards(
            origin: OriginFor<T>,
            asset_id: T::AssetId,
        ) -> DispatchResultWithPostInfo {
            let who = ensure_signed(origin)?;
            let current_block_number: u128 =
                <frame_system::Pallet<T>>::block_number().unique_saturated_into();
            let reward =
                Pallet::<T>::crowdloan_reward_for_asset(&who, &asset_id, current_block_number)?;

            Pallet::<T>::claim_reward_by_reason(&who, RewardReason::Crowdloan, &asset_id, reward)?;

            CrowdloanClaimHistory::<T>::mutate(who, asset_id, |value| {
                let offset = current_block_number % BLOCKS_PER_DAY;
                *value = T::BlockNumber::unique_saturated_from(
                    current_block_number.saturating_sub(offset),
                )
            });

            Ok(().into())
        }

        /// Allow/disallow a market making pair.
        #[transactional]
        #[pallet::weight(<T as Config>::WeightInfo::set_asset_pair())]

        pub fn set_asset_pair(
            origin: OriginFor<T>,
            from_asset_id: T::AssetId,
            to_asset_id: T::AssetId,
            market_making_rewards_allowed: bool,
        ) -> DispatchResultWithPostInfo {
            ensure_root(origin)?;
            let error = if market_making_rewards_allowed {
                Error::<T>::MarketMakingPairAlreadyAllowed
            } else {
                Error::<T>::MarketMakingPairAlreadyDisallowed
            };

            ensure!(
                MarketMakingPairs::<T>::contains_key(&from_asset_id, &to_asset_id)
                    != market_making_rewards_allowed,
                error
            );

            if market_making_rewards_allowed {
                MarketMakingPairs::<T>::insert(from_asset_id, to_asset_id, ());
            } else {
                MarketMakingPairs::<T>::remove(from_asset_id, to_asset_id);
            }

            Ok(().into())
        }
    }

    #[pallet::error]
    pub enum Error<T> {
        /// Account has no pending rewards to claim.
        NothingToClaim,
        /// Account has pending rewards but it has not been vested yet.
        ClaimLimitExceeded,
        /// Attempt to claim rewards of type, which is not handled.
        UnhandledRewardType,
        /// Account holding dedicated reward reserves is empty. This likely means that some of
        /// reward programmes have finished.
        RewardsSupplyShortage,
        /// Increment account reference error.
        IncRefError,
        /// Attempt to subtract more via snapshot than assigned to user.
        CantSubtractSnapshot,
        /// Failed to perform reward calculation.
        CantCalculateReward,
        /// The market making pair already allowed.
        MarketMakingPairAlreadyAllowed,
        /// The market making pair is disallowed.
        MarketMakingPairAlreadyDisallowed,
        /// There are no rewards for the asset ID.
        NoRewardsForAsset,
        /// Something is wrong with arithmetic - overflow happened, for example.
        ArithmeticError,
        /// This error appears on wrong conversion of a number into another type.
        NumberConversionError,
        /// Unable to get base asset price in XOR. XOR-base asset pair should exist on Polkaswap DEX.
        UnableToGetBaseAssetPrice,
    }

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        /// Rewards vested, limits were raised. [vested amount]
        RewardsVested(Balance),
        /// Attempted to claim reward, but actual claimed amount is less than expected. [reason for reward]
        ActualDoesntMatchAvailable(RewardReason),
        /// Saving reward for account has failed in a distribution series. [account]
        FailedToSaveCalculatedReward(AccountIdOf<T>),
        /// Account was chosen as eligible for market maker rewards, however calculated reward turned into 0. [account]
        AddingZeroMarketMakerReward(AccountIdOf<T>),
        /// Couldn't find any account with enough transactions to count market maker rewards.
        NoEligibleMarketMakers,
    }

    /// Reserved for future use
    /// Mapping between users and their owned rewards of different kinds, which are vested.
    #[pallet::storage]
    #[pallet::getter(fn rewards)]
    pub type Rewards<T: Config> =
        StorageMap<_, Blake2_128Concat, T::AccountId, RewardInfo, ValueQuery>;

    /// Reserved for future use
    /// Total amount of PSWAP pending rewards.
    #[pallet::storage]
    #[pallet::getter(fn total_rewards)]
    pub type TotalRewards<T: Config> = StorageValue<_, Balance, ValueQuery>;

    /// Registry of market makers with large transaction volumes (>1 XOR per transaction).
    #[pallet::storage]
    #[pallet::getter(fn market_makers_registry)]
    pub type MarketMakersRegistry<T: Config> =
        StorageMap<_, Blake2_128Concat, T::AccountId, MarketMakerInfo, ValueQuery>;

    /// Market making pairs storage.
    #[pallet::storage]
    #[pallet::getter(fn market_making_pairs)]
    pub type MarketMakingPairs<T: Config> = StorageDoubleMap<
        _,
        Blake2_128Concat,
        T::AssetId,
        Blake2_128Concat,
        T::AssetId,
        (),
        ValueQuery,
    >;

    /// Crowdloan vested rewards storage.
    #[pallet::storage]
    #[pallet::getter(fn crowdloan_rewards)]
    pub type CrowdloanRewards<T: Config> =
        StorageMap<_, Blake2_128Concat, T::AccountId, CrowdloanReward, ValueQuery>;

    /// This storage keeps the last block number, when the user (the first) claimed a reward for
    /// asset (the second key). The block is rounded to days.
    #[pallet::storage]
    #[pallet::getter(fn crowdloan_claim_history)]
    pub type CrowdloanClaimHistory<T: Config> = StorageDoubleMap<
        _,
        Blake2_128Concat,
        T::AccountId,
        Blake2_128Concat,
        T::AssetId,
        T::BlockNumber,
        ValueQuery,
    >;

    #[pallet::genesis_config]
    pub struct GenesisConfig {
        pub test_crowdloan_rewards: Vec<CrowdloanReward>,
    }

    #[cfg(feature = "std")]
    impl Default for GenesisConfig {
        fn default() -> Self {
            Self {
                test_crowdloan_rewards: Default::default(),
            }
        }
    }

    #[pallet::genesis_build]
    impl<T: Config> GenesisBuild<T> for GenesisConfig {
        fn build(&self) {
            use frame_support::log;
            use traits::MultiCurrency;

            self.test_crowdloan_rewards.iter().for_each(|reward| {
                CrowdloanRewards::<T>::insert(
                    T::AccountId::decode(&mut &reward.address[..])
                        .expect("Can't decode contributor address."),
                    reward.clone(),
                )
            });

            if let Err(e) = T::Currency::deposit(
                VAL.into(),
                &T::GetCrowdloanRewardsAccountId::get(),
                VAL_CROWDLOAN_REWARDS,
            ) {
                log::error!(target: "runtime", "Failed to add VAL crowdloan rewards: {:?}", e);
            }

            if let Err(e) = T::Currency::deposit(
                PSWAP.into(),
                &T::GetCrowdloanRewardsAccountId::get(),
                PSWAP_CROWDLOAN_REWARDS,
            ) {
                log::error!(target: "runtime", "Failed to add PSWAP crowdloan rewards: {:?}", e);
            }

            if let Err(e) = T::Currency::deposit(
                XSTUSD.into(),
                &T::GetCrowdloanRewardsAccountId::get(),
                XSTUSD_CROWDLOAN_REWARDS,
            ) {
                log::error!(target: "runtime", "Failed to add XSTUSD crowdloan rewards: {:?}", e);
            }
        }
    }
}
