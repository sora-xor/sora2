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
use common::prelude::{Balance, FixedWrapper};
use common::CrowdloanTag;
use common::FromGenericPair;
use common::{balance, OnPswapBurned, PswapRemintInfo, RewardReason, VestedRewardsPallet, PSWAP};
use frame_support::dispatch::{DispatchError, DispatchResult};
use frame_support::ensure;
use frame_support::fail;
use frame_support::traits::{Get, IsType};
use frame_support::weights::Weight;
use serde::{Deserialize, Serialize};
use sp_runtime::traits::{CheckedSub, Zero};
use sp_runtime::{Permill, Perquintill};
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
pub const TECH_ACCOUNT_FARMING: &[u8] = b"farming";
pub const FARMING_REWARDS: Balance = balance!(3500000000);

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

/// Store information about crowdloan
#[derive(
    Encode, Decode, Deserialize, Serialize, Clone, Debug, Default, PartialEq, scale_info::TypeInfo,
)]
pub struct CrowdloanInfo<AssetId, BlockNumber, AccountId> {
    /// Total amount of DOT, KSM, etc. contributed
    pub total_contribution: Balance,
    /// Asset id and total rewards amount pairs
    pub rewards: Vec<(AssetId, Balance)>,
    /// Rewards distribution start block
    pub start_block: BlockNumber,
    /// Length of rewards distribution in blocks
    pub length: BlockNumber,
    /// Account with crowdloan rewards
    pub account: AccountId,
}

/// Information about user participation in crowdloan
#[derive(
    Encode, Decode, Deserialize, Serialize, Clone, Debug, Default, PartialEq, scale_info::TypeInfo,
)]
pub struct CrowdloanUserInfo<AssetId> {
    /// Amount of DOT, KSM, etc. contributed by user
    contribution: Balance,
    /// Amount of rewards which is already taken by user
    rewarded: Vec<(AssetId, Balance)>,
}

pub trait WeightInfo {
    fn claim_rewards() -> Weight;
    fn update_rewards(n: u32) -> Weight;
    fn register_crowdloan(m: u32) -> Weight;
    fn claim_crowdloan_rewards() -> Weight;
}

impl<T: Config> Pallet<T> {
    /// Stores a new reward for a given account_id, supported by a reward reason.
    /// Returns error in case of failure during incrementing the reference counter on an account.
    /// Interacts with the `Rewards` StorageMap and the `TotalRewards` StorageValue;
    /// also modifies the `System` pallet storage state.
    ///
    /// Used in this trait: `market_maker_rewards_distribution_routine`;
    /// in VestedRewardsPallet trait: `add_tbc_reward`, `add_farming_reward`, `add_market_maker_reward`;
    /// also in farming pallet: `vest_account_rewards`.
    ///
    /// - `account_id`: The account associated with the reward
    /// - `reason`: The reward reason
    /// - `amount`: The amount of reward
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
    /// Returns error in case if total available reward or
    /// its limit or total claimed result is equal to 0;
    /// Interacts with the `Rewards` StorageMap and the `TotalRewards` StorageValue;
    /// also modifies the `System` pallet storage state.
    /// Emits `ActualDoesntMatchAvailable` event if some of the rewards were not fully claimed
    /// for this account.
    ///
    /// Used in `claim_rewards` extrinsic.
    ///
    /// - `account_id`: The account associated with the reward
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
    /// Returns the actually transferred reward amount.
    /// Returns error if the reward `reason` is invalid, or if the available reward is equal to 0.
    /// Interacts with the `Asset` pallet storage state.
    ///
    /// Used in this trait: `claim_rewards_inner`;
    /// also in `claim_crowdloan_rewards` extrinsic.
    ///
    /// - `account_id`: The account id associated with the reward
    /// - `reason`: The reward reason
    /// - `asset_id`: The asset id associated with the reward
    /// - `amount`: The amount of the reward
    pub fn claim_reward_by_reason(
        account_id: &T::AccountId,
        reason: RewardReason,
        asset_id: &T::AssetId,
        amount: Balance,
    ) -> Result<Balance, DispatchError> {
        let source_account = match reason {
            RewardReason::BuyOnBondingCurve => T::GetBondingCurveRewardsAccountId::get(),
            RewardReason::LiquidityProvisionFarming => T::GetFarmingRewardsAccountId::get(),
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

    /// Distributes the vested PSWAP tokens.
    /// Interacts with the `Rewards` StorageMap.
    ///
    /// Used in `OnPswapBurned` trait: `on_pswap_burned`.
    ///
    /// - `vested_amount`: The amount to be distributed
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

    /// Helper function for runtime api
    pub fn get_claimable_crowdloan_reward(
        tag: &CrowdloanTag,
        user: &T::AccountId,
        asset_id: &T::AssetId,
    ) -> Option<Balance> {
        let info = CrowdloanInfos::<T>::get(tag)?;
        let total_rewards = info
            .rewards
            .iter()
            .find(|(a, _)| a == asset_id)
            .cloned()
            .map(|(_, r)| r)?;
        let user_info = CrowdloanUserInfos::<T>::get(user, tag)?;
        let rewarded = info
            .rewards
            .iter()
            .find(|(a, _)| a == asset_id)
            .cloned()
            .map(|(_, r)| r)
            .unwrap_or_default();
        let now = frame_system::Pallet::<T>::block_number();
        let claimable = Self::calculate_claimable_crowdloan_reward(
            &now,
            &info,
            total_rewards,
            user_info.contribution,
            rewarded,
        )
        .ok()?;
        Some(claimable)
    }

    /// Calculate amount of tokens to send to user
    pub fn calculate_claimable_crowdloan_reward(
        now: &T::BlockNumber,
        info: &CrowdloanInfo<T::AssetId, T::BlockNumber, T::AccountId>,
        total_rewards: Balance,
        contribution: Balance,
        rewarded: Balance,
    ) -> Result<Balance, DispatchError> {
        let elapsed = now
            .checked_sub(&info.start_block)
            .ok_or(Error::<T>::CrowdloanRewardsDistributionNotStarted)?;
        let rewards_part = Perquintill::from_rational(contribution, info.total_contribution);
        let user_reward = rewards_part.mul_floor(total_rewards);

        let user_reward_now = if elapsed >= info.length {
            user_reward
        } else {
            let length_days = info.length / T::BLOCKS_PER_DAY;
            let elapsed_days = elapsed / T::BLOCKS_PER_DAY;
            let elapsed_percent = Permill::from_rational(elapsed_days, length_days);
            elapsed_percent.mul_floor(user_reward)
        };

        if user_reward_now <= rewarded {
            return Ok(0);
        }
        let reward_to_send = user_reward_now - rewarded;
        Ok(reward_to_send)
    }

    /// Send crowdloan rewards with given asset to user
    ///
    /// Returns total amount of tokens sent to user for this crowdloan
    pub fn claim_crowdloan_reward_for_asset(
        user: &T::AccountId,
        now: &T::BlockNumber,
        info: &CrowdloanInfo<T::AssetId, T::BlockNumber, T::AccountId>,
        asset_id: &T::AssetId,
        total_rewards: Balance,
        contribution: Balance,
        rewarded: Balance,
    ) -> Result<Balance, DispatchError> {
        let claimable_reward = Self::calculate_claimable_crowdloan_reward(
            now,
            info,
            total_rewards,
            contribution,
            rewarded,
        )?;

        if claimable_reward.is_zero() {
            return Ok(0);
        }

        assets::Pallet::<T>::transfer_from(asset_id, &info.account, user, claimable_reward)?;

        Self::deposit_event(Event::<T>::CrowdloanClaimed(
            user.clone(),
            asset_id.clone(),
            claimable_reward,
        ));

        Ok(claimable_reward)
    }

    pub fn claim_crowdloan_rewards_for_user(
        user: &T::AccountId,
        crowdloan: CrowdloanTag,
    ) -> DispatchResult {
        let now = frame_system::Pallet::<T>::block_number();
        let info =
            CrowdloanInfos::<T>::get(&crowdloan).ok_or(Error::<T>::CrowdloanDoesNotExists)?;
        let mut user_info = CrowdloanUserInfos::<T>::get(user, &crowdloan)
            .ok_or(Error::<T>::NotCrowdloanParticipant)?;
        let user_rewarded = user_info
            .rewarded
            .iter()
            .cloned()
            .collect::<BTreeMap<_, _>>();
        let mut user_rewards = vec![];

        for (asset_id, total_rewards) in info.rewards.iter() {
            let mut rewarded = user_rewarded.get(asset_id).cloned().unwrap_or_default();
            rewarded += Self::claim_crowdloan_reward_for_asset(
                user,
                &now,
                &info,
                asset_id,
                *total_rewards,
                user_info.contribution,
                rewarded,
            )?;
            user_rewards.push((asset_id.clone(), rewarded));
        }
        user_info.rewarded = user_rewards;
        CrowdloanUserInfos::<T>::insert(user, &crowdloan, user_info);
        Ok(())
    }

    pub fn register_crowdloan_unchecked(
        tag: CrowdloanTag,
        start_block: T::BlockNumber,
        length: T::BlockNumber,
        rewards: Vec<(T::AssetId, Balance)>,
        contributions: Vec<(T::AccountId, Balance)>,
    ) -> DispatchResult {
        ensure!(
            !CrowdloanInfos::<T>::contains_key(&tag),
            Error::<T>::CrowdloanAlreadyExists
        );
        ensure!(
            !rewards.is_empty() && !contributions.is_empty(),
            Error::<T>::WrongCrowdloanInfo
        );
        let tech_account = T::TechAccountId::from_generic_pair(
            TECH_ACCOUNT_PREFIX.to_vec(),
            tag.0.clone().to_vec(),
        );
        technical::Pallet::<T>::register_tech_account_id_if_not_exist(&tech_account)?;
        let account = technical::Pallet::<T>::tech_account_id_to_account_id(&tech_account)?;

        let mut total_contribution = 0;
        for (user, contribution) in contributions {
            total_contribution += contribution;
            CrowdloanUserInfos::<T>::insert(
                &user,
                &tag,
                CrowdloanUserInfo {
                    contribution,
                    rewarded: Default::default(),
                },
            );
        }

        CrowdloanInfos::<T>::insert(
            &tag,
            CrowdloanInfo {
                total_contribution,
                rewards,
                start_block,
                length,
                account,
            },
        );
        Ok(().into())
    }
}

impl<T: Config> OnPswapBurned for Pallet<T> {
    /// Invoked when pswap is burned after being exchanged from collected liquidity provider fees.
    fn on_pswap_burned(distribution: PswapRemintInfo) {
        Pallet::<T>::distribute_limits(distribution.vesting)
    }
}

impl<T: Config> VestedRewardsPallet<T::AccountId, T::AssetId> for Pallet<T> {
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
    use sp_std::collections::btree_map::BTreeMap;

    #[pallet::config]
    pub trait Config:
        frame_system::Config
        + common::Config
        + assets::Config
        + multicollateral_bonding_curve_pool::Config
    {
        const BLOCKS_PER_DAY: BlockNumberFor<Self>;
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
        /// Accounts holding PSWAP dedicated for rewards.
        #[pallet::constant]
        type GetMarketMakerRewardsAccountId: Get<Self::AccountId>;
        #[pallet::constant]
        type GetFarmingRewardsAccountId: Get<Self::AccountId>;
        #[pallet::constant]
        type GetBondingCurveRewardsAccountId: Get<Self::AccountId>;
        /// Weight information for extrinsics in this pallet.
        type WeightInfo: WeightInfo;
    }

    /// The current storage version.
    const STORAGE_VERSION: StorageVersion = StorageVersion::new(2);

    #[pallet::pallet]
    #[pallet::generate_store(pub(super) trait Store)]
    #[pallet::storage_version(STORAGE_VERSION)]
    #[pallet::without_storage_info]
    pub struct Pallet<T>(PhantomData<T>);

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        /// Claim all available PSWAP rewards by account signing this transaction.
        #[transactional]
        #[pallet::call_index(0)]
        #[pallet::weight(<T as Config>::WeightInfo::claim_rewards())]
        pub fn claim_rewards(origin: OriginFor<T>) -> DispatchResultWithPostInfo {
            let who = ensure_signed(origin)?;
            Self::claim_rewards_inner(&who)?;
            Ok(().into())
        }

        #[transactional]
        #[pallet::call_index(1)]
        #[pallet::weight(<T as Config>::WeightInfo::claim_crowdloan_rewards())]
        pub fn claim_crowdloan_rewards(
            origin: OriginFor<T>,
            crowdloan: CrowdloanTag,
        ) -> DispatchResultWithPostInfo {
            let who = ensure_signed(origin)?;

            Self::claim_crowdloan_rewards_for_user(&who, crowdloan)?;

            Ok(().into())
        }

        #[transactional]
        #[pallet::call_index(2)]
        #[pallet::weight(<T as Config>::WeightInfo::update_rewards(rewards.len() as u32))]
        pub fn update_rewards(
            origin: OriginFor<T>,
            rewards: BTreeMap<T::AccountId, BTreeMap<RewardReason, Balance>>,
        ) -> DispatchResultWithPostInfo {
            ensure_root(origin)?;
            let mut total_rewards_diff = 0i128;
            for (account, reward) in rewards {
                Rewards::<T>::mutate(&account, |value| {
                    for (reason, amount) in reward {
                        let v = value.rewards.entry(reason).or_insert(0);
                        *v += amount;
                    }
                    let total: i128 = value
                        .rewards
                        .iter_mut()
                        .map(|(_, amount)| *amount as i128)
                        .sum();
                    total_rewards_diff += total - value.total_available as i128;
                });
            }
            TotalRewards::<T>::mutate(|value| {
                if total_rewards_diff < 0 {
                    *value -= total_rewards_diff.abs() as Balance;
                } else {
                    *value += total_rewards_diff as Balance;
                }
            });

            Ok(().into())
        }

        #[transactional]
        #[pallet::call_index(3)]
        #[pallet::weight(<T as Config>::WeightInfo::register_crowdloan(contributions.len() as u32))]
        pub fn register_crowdloan(
            origin: OriginFor<T>,
            tag: CrowdloanTag,
            start_block: T::BlockNumber,
            length: T::BlockNumber,
            rewards: Vec<(T::AssetId, Balance)>,
            contributions: Vec<(T::AccountId, Balance)>,
        ) -> DispatchResultWithPostInfo {
            ensure_root(origin)?;
            Pallet::<T>::register_crowdloan_unchecked(
                tag,
                start_block,
                length,
                rewards,
                contributions,
            )?;
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
        /// There are no rewards for the asset ID.
        NoRewardsForAsset,
        /// Something is wrong with arithmetic - overflow happened, for example.
        ArithmeticError,
        /// This error appears on wrong conversion of a number into another type.
        NumberConversionError,
        /// Unable to get base asset price in XOR. XOR-base asset pair should exist on Polkaswap DEX.
        UnableToGetBaseAssetPrice,
        /// Crowdloan with given tag already registered
        CrowdloanAlreadyExists,
        /// Wrong crowdloan data passed
        WrongCrowdloanInfo,
        /// Crowdloan rewards distribution is not started
        CrowdloanRewardsDistributionNotStarted,
        /// Crowdloan does not exists
        CrowdloanDoesNotExists,
        /// User is not crowdloan participant
        NotCrowdloanParticipant,
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
        /// Claimed crowdloan rewards
        CrowdloanClaimed(T::AccountId, T::AssetId, Balance),
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

    /// Information about crowdloan
    #[pallet::storage]
    #[pallet::getter(fn crowdloan_infos)]
    pub type CrowdloanInfos<T: Config> = StorageMap<
        _,
        Blake2_128Concat,
        CrowdloanTag,
        CrowdloanInfo<T::AssetId, T::BlockNumber, T::AccountId>,
        OptionQuery,
    >;

    /// Information about crowdloan rewards claimed by user
    #[pallet::storage]
    #[pallet::getter(fn crowdloan_user_infos)]
    pub type CrowdloanUserInfos<T: Config> = StorageDoubleMap<
        _,
        Blake2_128Concat,
        T::AccountId,
        Blake2_128Concat,
        CrowdloanTag,
        CrowdloanUserInfo<T::AssetId>,
        OptionQuery,
    >;
}
