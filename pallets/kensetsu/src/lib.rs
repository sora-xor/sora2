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

//! Kensetsu is a over collateralized lending protocol, clone of MakerDAO.
//! An individual can create a collateral debt positions (CDPs) for one of the listed token and
//! deposit or lock amount of the token in CDP as collateral. Then the individual is allowed to
//! borrow new minted Kensetsu USD (KUSD) in amount up to value of collateral corrected by
//! `liquidation_ratio` coefficient. The debt in KUSD is a subject of `stability_fee` interest rate.
//! Collateral may be unlocked only when the debt and the interest are payed back. If the value of
//! collateral has changed in a way that it does not secure the debt, the collateral is liquidated
//! to cover the debt and the interest.

pub use pallet::*;

use assets::AssetIdOf;
use codec::{Decode, Encode, MaxEncodedLen};
use common::{balance, Balance};
use frame_support::log::{debug, warn};
use scale_info::TypeInfo;
use sp_arithmetic::FixedU128;
use sp_arithmetic::Perbill;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(test)]
mod test_utils;

mod compounding;
pub mod weights;

pub const TECH_ACCOUNT_PREFIX: &[u8] = b"kensetsu";
pub const TECH_ACCOUNT_TREASURY_MAIN: &[u8] = b"treasury";

/// Custom errors for unsigned tx validation, InvalidTransaction::Custom(u8)
const VALIDATION_ERROR_ACCRUE: u8 = 1;
const VALIDATION_ERROR_ACCRUE_NO_DEBT: u8 = 2;
const VALIDATION_ERROR_CHECK_SAFE: u8 = 3;
const VALIDATION_ERROR_CDP_SAFE: u8 = 4;

/// Risk management parameters for the specific collateral type.
#[derive(
    Debug, Clone, Encode, Decode, MaxEncodedLen, TypeInfo, PartialEq, Eq, PartialOrd, Ord, Copy,
)]
pub struct CollateralRiskParameters {
    /// Hard cap of total KUSD issued for the collateral.
    pub hard_cap: Balance,

    /// Loan-to-value liquidation threshold
    pub liquidation_ratio: Perbill,

    /// The max amount of collateral can be liquidated in one round
    pub max_liquidation_lot: Balance,

    /// Protocol Interest rate per second
    pub stability_fee_rate: FixedU128,
}

/// Collateral parameters, includes risk info and additional data for interest rate calculation
#[derive(Debug, Clone, Encode, Decode, MaxEncodedLen, TypeInfo, PartialEq, Eq, PartialOrd, Ord)]
pub struct CollateralInfo<Moment> {
    /// Collateral Risk parameters set by risk management
    pub risk_parameters: CollateralRiskParameters,

    /// the last timestamp when stability fee was accrued
    pub last_fee_update_time: Moment,

    /// Interest accrued for collateral for all time
    pub interest_coefficient: FixedU128,
}

/// CDP - Collateralized Debt Position. It is a single collateral/debt record.
#[derive(Debug, Clone, Encode, Decode, MaxEncodedLen, TypeInfo, PartialEq, Eq, PartialOrd, Ord)]
pub struct CollateralizedDebtPosition<AccountId, AssetId> {
    /// CDP owner
    pub owner: AccountId,

    /// Collateral
    pub collateral_asset_id: AssetId,
    pub collateral_amount: Balance,

    /// normalized outstanding debt in KUSD
    pub debt: Balance,

    /// Interest accrued for CDP.
    /// Initializes on creation with collateral interest coefficient equal to 1.
    /// The coefficient is growing over time with interest rate.
    /// Actual interest is: (collateral.coefficient - cdp.coefficient) / cdp.coefficient
    pub interest_coefficient: FixedU128,
}

#[frame_support::pallet]
pub mod pallet {
    use super::*;
    use crate::compounding::compound;
    use crate::weights::WeightInfo;
    use common::prelude::{QuoteAmount, SwapAmount, SwapOutcome};
    use common::{
        AccountIdOf, AssetInfoProvider, AssetName, AssetSymbol, BalancePrecision, ContentSource,
        DEXId, Description, LiquidityProxyTrait, LiquiditySourceFilter, ReferencePriceProvider,
    };
    use frame_support::pallet_prelude::*;
    use frame_system::offchain::{SendTransactionTypes, SubmitTransaction};
    use frame_system::pallet_prelude::*;
    use pallet_timestamp as timestamp;
    use sp_arithmetic::traits::{CheckedMul, Saturating};
    use sp_arithmetic::Percent;
    use sp_core::U256;
    use sp_runtime::traits::{CheckedConversion, CheckedDiv, CheckedSub, One};
    use sp_std::collections::btree_set::BTreeSet;
    use sp_std::vec::Vec;

    #[pallet::pallet]
    #[pallet::generate_store(pub(super) trait Store)]
    #[pallet::without_storage_info]
    pub struct Pallet<T>(PhantomData<T>);

    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
        /// Main off-chain worker procedure.
        ///
        /// Accrues fees and calls liquidations
        fn offchain_worker(block_number: T::BlockNumber) {
            debug!(
                "Entering off-chain worker, block number is {:?}",
                block_number
            );
            let now = Timestamp::<T>::get();
            let outdated_timestamp = now.saturating_sub(T::AccrueInterestPeriod::get());
            let mut collaterals_to_update = BTreeSet::new();
            for (collateral_asset_id, collateral_info) in <CollateralInfos<T>>::iter() {
                if collateral_info.last_fee_update_time <= outdated_timestamp {
                    collaterals_to_update.insert(collateral_asset_id);
                }
            }
            // TODO optimize CDP accrue
            for (cdp_id, cdp) in <CDPDepository<T>>::iter() {
                // Debt recalculation with interest
                if collaterals_to_update.contains(&cdp.collateral_asset_id) {
                    debug!("Accrue for CDP {:?}", cdp_id);
                    let call = Call::<T>::accrue { cdp_id };
                    if let Err(err) =
                        SubmitTransaction::<T, Call<T>>::submit_unsigned_transaction(call.into())
                    {
                        warn!(
                            "Failed in offchain_worker send accrue(cdp_id: {:?}): {:?}",
                            cdp_id, err
                        );
                    }
                }

                // Liquidation
                match Self::check_cdp_is_safe(
                    cdp.debt,
                    cdp.collateral_amount,
                    cdp.collateral_asset_id,
                ) {
                    Ok(cdp_is_safe) => {
                        if !cdp_is_safe {
                            debug!("Liquidation of CDP {:?}", cdp_id);
                            let call = Call::<T>::liquidate { cdp_id };
                            if let Err(err) =
                                SubmitTransaction::<T, Call<T>>::submit_unsigned_transaction(
                                    call.into(),
                                )
                            {
                                warn!(
                                    "Failed in offchain_worker send liquidate(cdp_id: {:?}): {:?}",
                                    cdp_id, err
                                );
                            }
                        }
                    }
                    Err(err) => {
                        warn!(
                            "Failed in offchain_worker check cdp {:?} safety: {:?}",
                            cdp_id, err
                        );
                    }
                }
            }
        }
    }

    #[pallet::config]
    pub trait Config:
        assets::Config
        + frame_system::Config
        + technical::Config
        + timestamp::Config
        + SendTransactionTypes<Call<Self>>
    {
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
        type AssetInfoProvider: AssetInfoProvider<
            Self::AssetId,
            Self::AccountId,
            AssetSymbol,
            AssetName,
            BalancePrecision,
            ContentSource,
            Description,
        >;
        type TreasuryTechAccount: Get<Self::TechAccountId>;
        type KusdAssetId: Get<Self::AssetId>;
        type ReferencePriceProvider: ReferencePriceProvider<AssetIdOf<Self>, Balance>;
        type LiquidityProxy: LiquidityProxyTrait<Self::DEXId, Self::AccountId, Self::AssetId>;

        /// Accrue() for a single CDP can be called once per this period
        #[pallet::constant]
        type AccrueInterestPeriod: Get<Self::Moment>;

        /// A configuration for base priority of unsigned transactions.
        #[pallet::constant]
        type UnsignedPriority: Get<TransactionPriority>;

        /// A configuration for longevity of unsigned transactions.
        #[pallet::constant]
        type UnsignedLongevity: Get<u64>;

        /// Weight information for extrinsics in this pallet.
        type WeightInfo: WeightInfo;
    }

    pub type Timestamp<T> = timestamp::Pallet<T>;

    /// System bad debt, the amount of KUSD not secured with collateral.
    #[pallet::storage]
    #[pallet::getter(fn bad_debt)]
    pub type BadDebt<T> = StorageValue<_, Balance, ValueQuery>;

    /// Parametes for collaterals, include risk parameters and interest recalculation coefficients
    #[pallet::storage]
    #[pallet::getter(fn collateral_infos)]
    pub type CollateralInfos<T: Config> =
        StorageMap<_, Identity, AssetIdOf<T>, CollateralInfo<T::Moment>>;

    /// Risk parameter
    /// Hard cap of KUSD may be minted by the system
    #[pallet::storage]
    #[pallet::getter(fn max_supply)]
    pub type KusdHardCap<T> = StorageValue<_, Balance, ValueQuery>;

    /// Risk parameter
    /// Liquidation penalty
    #[pallet::storage]
    #[pallet::getter(fn liquidation_penalty)]
    pub type LiquidationPenalty<T> = StorageValue<_, Percent, ValueQuery>;

    /// CDP counter used for CDP id
    #[pallet::storage]
    pub type NextCDPId<T> = StorageValue<_, U256, ValueQuery>;

    /// Storage of all CDPs, where key is an unique CDP identifier
    #[pallet::storage]
    #[pallet::getter(fn cdp)]
    pub type CDPDepository<T: Config> =
        StorageMap<_, Identity, U256, CollateralizedDebtPosition<AccountIdOf<T>, AssetIdOf<T>>>;

    /// Index links owner to CDP ids, not needed by protocol, but used by front-end
    #[pallet::storage]
    #[pallet::getter(fn cdp_owner_index)]
    pub type CdpOwnerIndex<T: Config> = StorageMap<_, Identity, AccountIdOf<T>, Vec<U256>>;

    /// Accounts of risk management team
    #[pallet::storage]
    #[pallet::getter(fn risk_managers)]
    pub type RiskManagers<T: Config> = StorageValue<_, BTreeSet<T::AccountId>>;

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        CDPCreated {
            cdp_id: U256,
            owner: AccountIdOf<T>,
            collateral_asset_id: AssetIdOf<T>,
        },
        CDPClosed {
            cdp_id: U256,
            owner: AccountIdOf<T>,
            collateral_asset_id: AssetIdOf<T>,
        },
        CollateralDeposit {
            cdp_id: U256,
            owner: AccountIdOf<T>,
            collateral_asset_id: AssetIdOf<T>,
            amount: Balance,
        },
        CollateralWithdrawn {
            cdp_id: U256,
            owner: AccountIdOf<T>,
            collateral_asset_id: AssetIdOf<T>,
            amount: Balance,
        },
        DebtIncreased {
            cdp_id: U256,
            owner: AccountIdOf<T>,
            collateral_asset_id: AssetIdOf<T>,
            // KUSD amount borrowed
            amount: Balance,
        },
        DebtPayment {
            cdp_id: U256,
            owner: AccountIdOf<T>,
            collateral_asset_id: AssetIdOf<T>,
            // KUSD amount payed off
            amount: Balance,
        },
        Liquidated {
            cdp_id: U256,
            // what was liquidated
            collateral_asset_id: AssetIdOf<T>,
            collateral_amount: Balance,
            // KUSD amount from liquidation to cover debt
            proceeds: Balance,
            // liquidation penalty
            penalty: Balance,
        },
        CollateralRiskParametersUpdated {
            collateral_asset_id: AssetIdOf<T>,
            risk_parameters: CollateralRiskParameters,
        },
        KusdHardCapUpdated {
            hard_cap: Balance,
        },
        LiquidationPenaltyUpdated {
            liquidation_penalty: Percent,
        },
        ProfitWithdrawn {
            amount: Balance,
        },
        Donation {
            amount: Balance,
        },
    }

    #[pallet::error]
    pub enum Error<T> {
        ArithmeticError,
        WrongAssetId,
        CDPNotFound,
        CollateralInfoNotFound,
        CDPSafe,
        CDPUnsafe,
        NotEnoughCollateral,
        OperationNotPermitted,
        OutstandingDebt,
        NoDebt,
        CDPsPerUserLimitReached,
        HardCapSupply,
        BalanceNotEnough,
        WrongCollateralAssetId,
        AccrueWrongTime,
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        /// Creates a Collateralized Debt Position (CDP) allowing users to lock collateral assets and borrow against them.
        ///
        /// ## Parameters
        ///
        /// - `origin`: The origin of the transaction.
        /// - `collateral_asset_id`: The identifier of the asset used as collateral.
        /// - `collateral_amount`: The amount of collateral to be deposited.
        /// - `borrow_amount`: The amount the user wants to borrow.
        #[pallet::call_index(0)]
        #[pallet::weight(<T as Config>::WeightInfo::create_cdp())]
        pub fn create_cdp(
            origin: OriginFor<T>,
            collateral_asset_id: AssetIdOf<T>,
            collateral_amount: Balance,
            borrow_amount: Balance,
        ) -> DispatchResult {
            let who = ensure_signed(origin)?;
            ensure!(
                <CollateralInfos<T>>::contains_key(collateral_asset_id),
                Error::<T>::CollateralInfoNotFound
            );
            let interest_coefficient = Self::collateral_infos(collateral_asset_id)
                .ok_or(Error::<T>::CollateralInfoNotFound)?
                .interest_coefficient;
            NextCDPId::<T>::try_mutate(|cdp_id| {
                *cdp_id = cdp_id
                    .checked_add(U256::from(1))
                    .ok_or(Error::<T>::ArithmeticError)?;
                Self::deposit_event(Event::CDPCreated {
                    cdp_id: *cdp_id,
                    owner: who.clone(),
                    collateral_asset_id,
                });
                <CDPDepository<T>>::insert(
                    *cdp_id,
                    CollateralizedDebtPosition {
                        owner: who.clone(),
                        collateral_asset_id,
                        collateral_amount: balance!(0),
                        debt: balance!(0),
                        interest_coefficient,
                    },
                );
                CdpOwnerIndex::<T>::append(&who, *cdp_id);
                if collateral_amount > 0 {
                    Self::deposit_internal(&who, *cdp_id, collateral_amount)?;
                }
                if borrow_amount > 0 {
                    Self::borrow_internal(&who, *cdp_id, borrow_amount)?;
                }
                DispatchResult::Ok(())
            })?;
            Ok(())
        }

        /// Closes a Collateralized Debt Position (CDP).
        ///
        /// ## Parameters
        ///
        /// - `origin`: The origin of the transaction.
        /// - `cdp_id`: The ID of the CDP to be closed.
        #[pallet::call_index(1)]
        #[pallet::weight(<T as Config>::WeightInfo::close_cdp())]
        pub fn close_cdp(origin: OriginFor<T>, cdp_id: U256) -> DispatchResult {
            let who = ensure_signed(origin)?;
            let cdp = Self::accrue_internal(cdp_id)?;
            ensure!(who == cdp.owner, Error::<T>::OperationNotPermitted);
            ensure!(cdp.debt == 0, Error::<T>::OutstandingDebt);
            technical::Pallet::<T>::transfer_out(
                &cdp.collateral_asset_id,
                &T::TreasuryTechAccount::get(),
                &who,
                cdp.collateral_amount,
            )?;
            Self::delete_cdp(cdp_id, &who);
            Self::deposit_event(Event::CDPClosed {
                cdp_id,
                owner: who,
                collateral_asset_id: cdp.collateral_asset_id,
            });
            Ok(())
        }

        /// Deposits collateral into a Collateralized Debt Position (CDP).
        ///
        /// ## Parameters
        ///
        /// - `origin`: The origin of the transaction.
        /// - `cdp_id`: The ID of the CDP to deposit collateral into.
        /// - `collateral_amount`: The amount of collateral to deposit.
        #[pallet::call_index(2)]
        #[pallet::weight(<T as Config>::WeightInfo::deposit_collateral())]
        pub fn deposit_collateral(
            origin: OriginFor<T>,
            cdp_id: U256,
            collateral_amount: Balance,
        ) -> DispatchResult {
            let who = ensure_signed(origin)?;
            Self::deposit_internal(&who, cdp_id, collateral_amount)
        }

        /// Withdraws collateral from a Collateralized Debt Position (CDP).
        ///
        /// ## Parameters
        ///
        /// - `origin`: The origin of the transaction.
        /// - `cdp_id`: The ID of the CDP to withdraw collateral from.
        /// - `collateral_amount`: The amount of collateral to withdraw.
        #[pallet::call_index(3)]
        #[pallet::weight(<T as Config>::WeightInfo::withdraw_collateral())]
        pub fn withdraw_collateral(
            origin: OriginFor<T>,
            cdp_id: U256,
            collateral_amount: Balance,
        ) -> DispatchResult {
            let who = ensure_signed(origin)?;
            let cdp = Self::accrue_internal(cdp_id)?;
            ensure!(who == cdp.owner, Error::<T>::OperationNotPermitted);
            let new_collateral_amount = cdp
                .collateral_amount
                .checked_sub(collateral_amount)
                .ok_or(Error::<T>::NotEnoughCollateral)?;
            ensure!(
                Self::check_cdp_is_safe(cdp.debt, new_collateral_amount, cdp.collateral_asset_id,)?,
                Error::<T>::CDPUnsafe
            );
            technical::Pallet::<T>::transfer_out(
                &cdp.collateral_asset_id,
                &T::TreasuryTechAccount::get(),
                &who,
                collateral_amount,
            )?;
            <CDPDepository<T>>::try_mutate(cdp_id, |cdp| {
                let cdp = cdp.as_mut().ok_or(Error::<T>::CDPNotFound)?;
                cdp.collateral_amount = new_collateral_amount;
                DispatchResult::Ok(())
            })?;
            Self::deposit_event(Event::CollateralWithdrawn {
                cdp_id,
                owner: who,
                collateral_asset_id: cdp.collateral_asset_id,
                amount: collateral_amount,
            });

            Ok(())
        }

        /// Borrows funds against a Collateralized Debt Position (CDP).
        ///
        /// ## Parameters
        ///
        /// - `origin`: The origin of the transaction.
        /// - `cdp_id`: The ID of the CDP to borrow against.
        /// - `will_to_borrow_amount`: The amount the user intends to borrow.
        #[pallet::call_index(4)]
        #[pallet::weight(<T as Config>::WeightInfo::borrow())]
        pub fn borrow(
            origin: OriginFor<T>,
            cdp_id: U256,
            will_to_borrow_amount: Balance,
        ) -> DispatchResult {
            let who = ensure_signed(origin)?;
            Self::borrow_internal(&who, cdp_id, will_to_borrow_amount)
        }

        /// Repays debt against a Collateralized Debt Position (CDP).
        ///
        /// ## Parameters
        ///
        /// - `origin`: The origin of the transaction.
        /// - `cdp_id`: The ID of the CDP to repay debt for.
        /// - `amount`: The amount to repay against the CDP's debt.
        #[pallet::call_index(5)]
        #[pallet::weight(<T as Config>::WeightInfo::repay_debt())]
        pub fn repay_debt(origin: OriginFor<T>, cdp_id: U256, amount: Balance) -> DispatchResult {
            let who = ensure_signed(origin)?;
            let cdp = Self::accrue_internal(cdp_id)?;
            // if repaying amount exceeds debt, leftover is not burned
            let to_cover_debt = amount.min(cdp.debt);
            Self::burn_from(&who, to_cover_debt)?;
            <CDPDepository<T>>::try_mutate(cdp_id, |cdp| {
                let cdp = cdp.as_mut().ok_or(Error::<T>::CDPNotFound)?;
                cdp.debt = cdp
                    .debt
                    .checked_sub(to_cover_debt)
                    .ok_or(Error::<T>::ArithmeticError)?;
                DispatchResult::Ok(())
            })?;
            Self::deposit_event(Event::DebtPayment {
                cdp_id,
                owner: who,
                collateral_asset_id: cdp.collateral_asset_id,
                amount: to_cover_debt,
            });

            Ok(())
        }

        /// Liquidates a Collateralized Debt Position (CDP) if it becomes unsafe.
        ///
        /// ## Parameters
        ///
        /// - `_origin`: The origin of the transaction (unused).
        /// - `cdp_id`: The ID of the CDP to be liquidated.
        #[pallet::call_index(6)]
        #[pallet::weight(<T as Config>::WeightInfo::liquidate())]
        pub fn liquidate(_origin: OriginFor<T>, cdp_id: U256) -> DispatchResult {
            let cdp = Self::accrue_internal(cdp_id)?;
            let cdp_debt = cdp.debt;
            let cdp_collateral_amount = cdp.collateral_amount;
            let cdp_owner = cdp.owner;
            ensure!(
                !Self::check_cdp_is_safe(cdp_debt, cdp_collateral_amount, cdp.collateral_asset_id)?,
                Error::<T>::CDPSafe
            );
            let risk_parameters = Self::collateral_infos(cdp.collateral_asset_id)
                .ok_or(Error::<T>::CollateralInfoNotFound)?
                .risk_parameters;
            let desired_kusd_amount = cdp_debt
                .checked_add(Self::liquidation_penalty() * cdp_debt)
                .ok_or(Error::<T>::ArithmeticError)?;
            // TODO if desired amount < LP liquidity, returns error which fails extrinsic
            // it must proceed with amount = cdp.collateral_amount
            let SwapOutcome { amount, .. } = T::LiquidityProxy::quote(
                DEXId::Polkaswap.into(),
                &cdp.collateral_asset_id,
                &T::KusdAssetId::get(),
                QuoteAmount::WithDesiredOutput {
                    desired_amount_out: desired_kusd_amount,
                },
                LiquiditySourceFilter::empty(DEXId::Polkaswap.into()),
                true,
            )?;
            let collateral_to_liquidate = amount
                .min(cdp.collateral_amount)
                .min(risk_parameters.max_liquidation_lot);
            let technical_account_id = technical::Pallet::<T>::tech_account_id_to_account_id(
                &T::TreasuryTechAccount::get(),
            )?;
            let swap_outcome = T::LiquidityProxy::exchange(
                DEXId::Polkaswap.into(),
                &technical_account_id,
                &technical_account_id,
                &cdp.collateral_asset_id,
                &T::KusdAssetId::get(),
                // desired output
                SwapAmount::with_desired_input(collateral_to_liquidate, balance!(0)),
                LiquiditySourceFilter::empty(DEXId::Polkaswap.into()),
            )?;
            <CDPDepository<T>>::try_mutate(cdp_id, |cdp| {
                let cdp = cdp.as_mut().ok_or(Error::<T>::CDPNotFound)?;
                cdp.collateral_amount = cdp
                    .collateral_amount
                    .checked_sub(collateral_to_liquidate)
                    .ok_or(Error::<T>::ArithmeticError)?;
                DispatchResult::Ok(())
            })?;
            // penalty is a protocol profit which stays on treasury tech account
            let penalty = Self::liquidation_penalty() * swap_outcome.amount.min(cdp_debt);
            let proceeds = swap_outcome.amount - penalty;
            if cdp_debt >= proceeds {
                Self::burn_treasury(proceeds)?;
                let shortage = cdp_debt
                    .checked_sub(proceeds)
                    .ok_or(Error::<T>::CDPNotFound)?;
                if cdp_collateral_amount <= collateral_to_liquidate {
                    // no collateral, total default
                    // CDP debt is not covered with liquidation, now it is a protocol bad debt
                    Self::cover_with_protocol(shortage)?;
                    // close empty CDP, debt == 0, collateral == 0
                    Self::delete_cdp(cdp_id, &cdp_owner);
                    Self::deposit_event(Event::CDPClosed {
                        cdp_id,
                        owner: cdp_owner,
                        collateral_asset_id: cdp.collateral_asset_id,
                    });
                } else {
                    // partly covered
                    <CDPDepository<T>>::try_mutate(cdp_id, |cdp| {
                        let cdp = cdp.as_mut().ok_or(Error::<T>::CDPNotFound)?;
                        cdp.debt = cdp
                            .debt
                            .checked_sub(proceeds)
                            .ok_or(Error::<T>::CDPNotFound)?;
                        DispatchResult::Ok(())
                    })?;
                }
            } else {
                Self::burn_treasury(cdp_debt)?;
                // CDP debt is covered
                <CDPDepository<T>>::try_mutate(cdp_id, |cdp| {
                    let cdp = cdp.as_mut().ok_or(Error::<T>::CDPNotFound)?;
                    cdp.debt = 0;
                    DispatchResult::Ok(())
                })?;
                // There is more KUSD than to cover debt and penalty, leftover goes to cdp.owner
                let leftover = proceeds
                    .checked_sub(cdp_debt)
                    .ok_or(Error::<T>::CDPNotFound)?;
                assets::Pallet::<T>::transfer_from(
                    &T::KusdAssetId::get(),
                    &technical_account_id,
                    &cdp_owner,
                    leftover,
                )?;
            };
            Self::deposit_event(Event::Liquidated {
                cdp_id,
                collateral_asset_id: cdp.collateral_asset_id,
                collateral_amount: collateral_to_liquidate,
                proceeds,
                penalty,
            });

            Ok(())
        }

        /// Accrues interest on a Collateralized Debt Position (CDP).
        ///
        /// ## Parameters
        ///
        /// - `_origin`: The origin of the transaction (unused).
        /// - `cdp_id`: The ID of the CDP to accrue interest on.
        #[pallet::call_index(7)]
        #[pallet::weight(<T as Config>::WeightInfo::accrue())]
        pub fn accrue(_origin: OriginFor<T>, cdp_id: U256) -> DispatchResult {
            ensure!(Self::is_accruable(&cdp_id)?, Error::<T>::NoDebt);
            Self::accrue_internal(cdp_id)?;
            Ok(())
        }

        /// Updates the risk parameters for a specific collateral asset.
        ///
        /// ## Parameters
        ///
        /// - `origin`: The origin of the transaction.
        /// - `collateral_asset_id`: The identifier of the collateral asset.
        /// - `new_risk_parameters`: The new risk parameters to be set for the collateral asset.
        #[pallet::call_index(8)]
        #[pallet::weight(<T as Config>::WeightInfo::update_collateral_risk_parameters())]
        pub fn update_collateral_risk_parameters(
            origin: OriginFor<T>,
            collateral_asset_id: AssetIdOf<T>,
            new_risk_parameters: CollateralRiskParameters,
        ) -> DispatchResult {
            let who = ensure_signed(origin)?;
            Self::ensure_risk_manager(&who)?;
            ensure!(
                T::AssetInfoProvider::asset_exists(&collateral_asset_id),
                Error::<T>::WrongAssetId
            );
            <CollateralInfos<T>>::try_mutate(collateral_asset_id, |option_collateral_info| {
                match option_collateral_info {
                    Some(collateral_info) => {
                        let mut new_info =
                            Self::update_collateral_interest_coefficient(collateral_asset_id)?;
                        new_info.risk_parameters = new_risk_parameters;
                        *collateral_info = new_info;
                    }
                    None => {
                        let _ = option_collateral_info.insert(CollateralInfo {
                            risk_parameters: new_risk_parameters,
                            last_fee_update_time: Timestamp::<T>::get(),
                            interest_coefficient: FixedU128::one(),
                        });
                    }
                }
                DispatchResult::Ok(())
            })?;
            Self::deposit_event(Event::CollateralRiskParametersUpdated {
                collateral_asset_id,
                risk_parameters: new_risk_parameters,
            });

            Ok(())
        }

        /// Updates the hard cap for the total supply of a stablecoin.
        ///
        /// ## Parameters
        ///
        /// - `origin`: The origin of the transaction.
        /// - `new_hard_cap`: The new hard cap value to be set for the total supply.
        #[pallet::call_index(9)]
        #[pallet::weight(<T as Config>::WeightInfo::update_hard_cap_total_supply())]
        pub fn update_hard_cap_total_supply(
            origin: OriginFor<T>,
            new_hard_cap: Balance,
        ) -> DispatchResult {
            let who = ensure_signed(origin)?;
            Self::ensure_risk_manager(&who)?;
            <KusdHardCap<T>>::mutate({
                |hard_cap| {
                    *hard_cap = new_hard_cap;
                }
            });
            Self::deposit_event(Event::KusdHardCapUpdated {
                hard_cap: new_hard_cap,
            });
            Ok(())
        }

        /// Updates the liquidation penalty applied during CDP liquidation.
        ///
        /// ## Parameters
        ///
        /// - `origin`: The origin of the transaction.
        /// - `new_liquidation_penalty`: The new liquidation penalty percentage to be set.
        #[pallet::call_index(10)]
        #[pallet::weight(<T as Config>::WeightInfo::update_liquidation_penalty())]
        pub fn update_liquidation_penalty(
            origin: OriginFor<T>,
            new_liquidation_penalty: Percent,
        ) -> DispatchResult {
            let who = ensure_signed(origin)?;
            Self::ensure_risk_manager(&who)?;
            <LiquidationPenalty<T>>::mutate(|liquidation_penalty| {
                *liquidation_penalty = new_liquidation_penalty;
            });
            Self::deposit_event(Event::LiquidationPenaltyUpdated {
                liquidation_penalty: new_liquidation_penalty,
            });

            Ok(())
        }
        /// Withdraws protocol profit in the form of stablecoin (KUSD).
        ///
        /// ## Parameters
        ///
        /// - `origin`: The origin of the transaction.
        /// - `kusd_amount`: The amount of stablecoin (KUSD) to withdraw as protocol profit.
        #[pallet::call_index(11)]
        #[pallet::weight(<T as Config>::WeightInfo::withdraw_profit())]
        pub fn withdraw_profit(origin: OriginFor<T>, kusd_amount: Balance) -> DispatchResult {
            let who = ensure_signed(origin)?;
            Self::ensure_protocol_owner(&who)?;
            technical::Pallet::<T>::transfer_out(
                &T::KusdAssetId::get(),
                &T::TreasuryTechAccount::get(),
                &who,
                kusd_amount,
            )?;
            Self::deposit_event(Event::ProfitWithdrawn {
                amount: kusd_amount,
            });

            Ok(())
        }

        /// Donates stablecoin (KUSD) to cover protocol bad debt.
        ///
        /// ## Parameters
        ///
        /// - `origin`: The origin of the transaction.
        /// - `kusd_amount`: The amount of stablecoin (KUSD) to donate to cover bad debt.
        #[pallet::call_index(12)]
        #[pallet::weight(<T as Config>::WeightInfo::donate())]
        pub fn donate(origin: OriginFor<T>, kusd_amount: Balance) -> DispatchResult {
            let who = ensure_signed(origin)?;
            Self::cover_bad_debt(&who, kusd_amount)?;
            Self::deposit_event(Event::Donation {
                amount: kusd_amount,
            });

            Ok(())
        }

        /// Adds a new account ID to the set of risk managers.
        ///
        /// ## Parameters
        ///
        /// - `origin`: The origin of the transaction.
        /// - `account_id`: The account ID to be added as a risk manager.
        #[pallet::call_index(13)]
        #[pallet::weight(<T as Config>::WeightInfo::add_risk_manager())]
        pub fn add_risk_manager(origin: OriginFor<T>, account_id: T::AccountId) -> DispatchResult {
            ensure_root(origin)?;
            <RiskManagers<T>>::mutate(|option_risk_managers| {
                let _ = option_risk_managers
                    .get_or_insert(BTreeSet::new())
                    .insert(account_id);
            });

            Ok(())
        }

        /// Removes an account ID from the set of risk managers.
        ///
        /// ## Parameters
        ///
        /// - `origin`: The origin of the transaction.
        /// - `account_id`: The account ID to be removed from the set of risk managers.
        #[pallet::call_index(14)]
        #[pallet::weight(<T as Config>::WeightInfo::remove_risk_manager())]
        pub fn remove_risk_manager(
            origin: OriginFor<T>,
            account_id: T::AccountId,
        ) -> DispatchResult {
            ensure_root(origin)?;
            <RiskManagers<T>>::mutate(|option_risk_managers| match option_risk_managers {
                Some(risk_managers) => {
                    let _ = risk_managers.remove(&account_id);
                }
                None => {}
            });

            Ok(())
        }
    }

    /// Validate unsigned call to this pallet.
    #[pallet::validate_unsigned]
    impl<T: Config> ValidateUnsigned for Pallet<T> {
        type Call = Call<T>;

        /// It is allowed to call only accrue() and liquidate() and only if
        /// it fulfills conditions.
        fn validate_unsigned(_source: TransactionSource, call: &Self::Call) -> TransactionValidity {
            match call {
                // TODO spamming with accrue calls, add some filter to not call too often
                // https://github.com/sora-xor/sora2-network/issues/878
                Call::accrue { cdp_id } => {
                    if Self::is_accruable(cdp_id)
                        .map_err(|_| InvalidTransaction::Custom(VALIDATION_ERROR_ACCRUE))?
                    {
                        ValidTransaction::with_tag_prefix("Kensetsu::accrue")
                            .priority(T::UnsignedPriority::get())
                            .longevity(T::UnsignedLongevity::get())
                            .and_provides([&cdp_id])
                            .propagate(true)
                            .build()
                    } else {
                        InvalidTransaction::Custom(VALIDATION_ERROR_ACCRUE_NO_DEBT).into()
                    }
                }
                Call::liquidate { cdp_id } => {
                    let cdp = Self::cdp(cdp_id)
                        .ok_or(InvalidTransaction::Custom(VALIDATION_ERROR_CHECK_SAFE))?;
                    if !Self::check_cdp_is_safe(
                        cdp.debt,
                        cdp.collateral_amount,
                        cdp.collateral_asset_id,
                    )
                    .map_err(|_| InvalidTransaction::Custom(VALIDATION_ERROR_CHECK_SAFE))?
                    {
                        ValidTransaction::with_tag_prefix("Kensetsu::liquidate")
                            .priority(T::UnsignedPriority::get())
                            .longevity(T::UnsignedLongevity::get())
                            .and_provides([&cdp_id])
                            .propagate(true)
                            .build()
                    } else {
                        InvalidTransaction::Custom(VALIDATION_ERROR_CDP_SAFE).into()
                    }
                }
                _ => {
                    warn!("Unknown unsigned call {:?}", call);
                    InvalidTransaction::Call.into()
                }
            }
        }
    }

    impl<T: Config> Pallet<T> {
        /// Ensures that `who` is a risk manager
        /// Risk manager can set protocol risk parameters.
        fn ensure_risk_manager(who: &AccountIdOf<T>) -> DispatchResult {
            if !Self::risk_managers().map_or(false, |risk_managers| risk_managers.contains(who)) {
                return Err(Error::<T>::OperationNotPermitted.into());
            }

            Ok(())
        }

        /// Ensures that `who` is a protocol owner
        /// Protocol owner can withdraw profit from the protocol.
        fn ensure_protocol_owner(who: &AccountIdOf<T>) -> DispatchResult {
            if !Self::risk_managers().map_or(false, |risk_managers| risk_managers.contains(who)) {
                return Err(Error::<T>::OperationNotPermitted.into());
            }

            Ok(())
        }

        /// Checks whether a Collateralized Debt Position (CDP) is currently considered safe based on its debt and collateral.
        /// The function evaluates the safety of a CDP based on predefined liquidation ratios and collateral values,
        /// providing an indication of its current safety status.
        ///
        /// ## Parameters
        ///
        /// - `debt`: The current debt amount in the CDP.
        /// - `collateral`: The current collateral amount in the CDP.
        /// - `collateral_asset_id`: The asset ID associated with the collateral in the CDP.
        pub(crate) fn check_cdp_is_safe(
            debt: Balance,
            collateral: Balance,
            collateral_asset_id: AssetIdOf<T>,
        ) -> Result<bool, DispatchError> {
            let liquidation_ratio = Self::collateral_infos(collateral_asset_id)
                .ok_or(Error::<T>::CollateralInfoNotFound)?
                .risk_parameters
                .liquidation_ratio;
            let collateral_reference_price = FixedU128::from_inner(
                T::ReferencePriceProvider::get_reference_price(&collateral_asset_id)?,
            );
            let collateral_value = collateral_reference_price
                .checked_mul(&FixedU128::from_inner(collateral))
                .ok_or(Error::<T>::ArithmeticError)?;
            let max_safe_debt = FixedU128::from_perbill(liquidation_ratio)
                .checked_mul(&collateral_value)
                .ok_or(Error::<T>::ArithmeticError)?;
            let debt = FixedU128::from_inner(debt);
            Ok(debt <= max_safe_debt)
        }

        /// Ensures that new emission will not exceed collateral hard cap
        fn ensure_collateral_cap(
            collateral_asset_id: AssetIdOf<T>,
            new_emission: Balance,
        ) -> DispatchResult {
            let hard_cap = Self::collateral_infos(collateral_asset_id)
                .ok_or(Error::<T>::CollateralInfoNotFound)?
                .risk_parameters
                .hard_cap;

            let current_supply_for_collateral = balance!(0);
            for cdp in <CDPDepository<T>>::iter_values() {
                if cdp.collateral_asset_id == collateral_asset_id {
                    current_supply_for_collateral
                        .checked_add(cdp.debt)
                        .ok_or(Error::<T>::ArithmeticError)?;
                }
            }
            ensure!(
                current_supply_for_collateral
                    .checked_add(new_emission)
                    .ok_or(Error::<T>::ArithmeticError)?
                    <= hard_cap,
                Error::<T>::HardCapSupply
            );
            Ok(())
        }

        /// Ensures that new emission will not exceed system KUSD hard cap
        fn ensure_protocol_cap(new_emission: Balance) -> DispatchResult {
            let current_supply = T::AssetInfoProvider::total_issuance(&T::KusdAssetId::get())?;
            ensure!(
                current_supply
                    .checked_add(new_emission)
                    .ok_or(Error::<T>::ArithmeticError)?
                    <= Self::max_supply(),
                Error::<T>::HardCapSupply
            );
            Ok(())
        }

        /// Removes CDP entry from the storage
        fn delete_cdp(cdp_id: U256, cdp_owner: &AccountIdOf<T>) {
            <CDPDepository<T>>::remove(cdp_id);
            if let Some(mut cdp_ids) = <CdpOwnerIndex<T>>::take(cdp_owner) {
                cdp_ids.retain(|&x| x != cdp_id);
                if !cdp_ids.is_empty() {
                    <CdpOwnerIndex<T>>::insert(cdp_owner, cdp_ids);
                }
            }
        }

        /// Deposits collateral to CDP.
        /// Handles internal deposit of collateral into a Collateralized Debt Position (CDP).
        ///
        /// ## Parameters
        ///
        /// - `who`: The account making the collateral deposit.
        /// - `cdp_id`: The ID of the CDP where the collateral is being deposited.
        /// - `collateral_amount`: The amount of collateral being deposited.
        fn deposit_internal(
            who: &AccountIdOf<T>,
            cdp_id: U256,
            collateral_amount: Balance,
        ) -> DispatchResult {
            let cdp = Self::cdp(cdp_id).ok_or(Error::<T>::CDPNotFound)?;
            technical::Pallet::<T>::transfer_in(
                &cdp.collateral_asset_id,
                who,
                &T::TreasuryTechAccount::get(),
                collateral_amount,
            )?;
            <CDPDepository<T>>::try_mutate(cdp_id, |cdp| {
                let cdp = cdp.as_mut().ok_or(Error::<T>::CDPNotFound)?;
                cdp.collateral_amount = cdp
                    .collateral_amount
                    .checked_add(collateral_amount)
                    .ok_or(Error::<T>::ArithmeticError)?;
                DispatchResult::Ok(())
            })?;
            Self::deposit_event(Event::CollateralDeposit {
                cdp_id,
                owner: who.clone(),
                collateral_asset_id: cdp.collateral_asset_id,
                amount: collateral_amount,
            });

            Ok(())
        }

        /// Handles the internal borrowing operation within a Collateralized Debt Position (CDP).
        ///
        /// ## Parameters
        ///
        /// - `who`: The account ID initiating the borrowing operation.
        /// - `cdp_id`: The ID of the CDP involved in the borrowing.
        /// - `will_to_borrow_amount`: The amount to be borrowed.
        fn borrow_internal(
            who: &AccountIdOf<T>,
            cdp_id: U256,
            will_to_borrow_amount: Balance,
        ) -> DispatchResult {
            let cdp = Self::accrue_internal(cdp_id)?;
            ensure!(*who == cdp.owner, Error::<T>::OperationNotPermitted);
            let new_debt = cdp
                .debt
                .checked_add(will_to_borrow_amount)
                .ok_or(Error::<T>::ArithmeticError)?;
            ensure!(
                Self::check_cdp_is_safe(new_debt, cdp.collateral_amount, cdp.collateral_asset_id)?,
                Error::<T>::CDPUnsafe
            );
            Self::ensure_collateral_cap(cdp.collateral_asset_id, will_to_borrow_amount)?;
            Self::ensure_protocol_cap(will_to_borrow_amount)?;
            Self::mint_to(who, will_to_borrow_amount)?;
            <CDPDepository<T>>::try_mutate(cdp_id, |cdp| {
                let cdp = cdp.as_mut().ok_or(Error::<T>::CDPNotFound)?;
                cdp.debt = new_debt;
                DispatchResult::Ok(())
            })?;
            Self::deposit_event(Event::DebtIncreased {
                cdp_id,
                owner: who.clone(),
                collateral_asset_id: cdp.collateral_asset_id,
                amount: will_to_borrow_amount,
            });

            Ok(())
        }

        /// Covers bad debt using a specified amount of stablecoin (KUSD).
        /// The function facilitates the covering of bad debt using stablecoin from a specific account,
        /// handling the transfer and burning of stablecoin as needed to cover the bad debt.
        ///
        /// ## Parameters
        ///
        /// - `from`: The account from which the stablecoin will be used to cover bad debt.
        /// - `kusd_amount`: The amount of stablecoin to cover bad debt.
        fn cover_bad_debt(from: &AccountIdOf<T>, kusd_amount: Balance) -> DispatchResult {
            let bad_debt = <BadDebt<T>>::get();
            let to_cover_debt = if kusd_amount < bad_debt {
                kusd_amount
            } else {
                technical::Pallet::<T>::transfer_in(
                    &T::KusdAssetId::get(),
                    from,
                    &T::TreasuryTechAccount::get(),
                    kusd_amount
                        .checked_sub(bad_debt)
                        .ok_or(Error::<T>::ArithmeticError)?,
                )?;
                bad_debt
            };
            Self::burn_from(from, to_cover_debt)?;
            <BadDebt<T>>::try_mutate(|bad_debt| {
                *bad_debt = bad_debt
                    .checked_sub(to_cover_debt)
                    .ok_or(Error::<T>::ArithmeticError)?;
                DispatchResult::Ok(())
            })?;

            Ok(())
        }

        /// Returns true if CDP has debt.
        fn is_accruable(cdp_id: &U256) -> Result<bool, DispatchError> {
            let cdp = Self::cdp(cdp_id).ok_or(Error::<T>::CDPNotFound)?;
            Ok(cdp.debt > 0)
        }

        /// Recalculates collateral interest coefficient with the current timestamp
        fn update_collateral_interest_coefficient(
            collateral_asset_id: AssetIdOf<T>,
        ) -> Result<CollateralInfo<T::Moment>, DispatchError> {
            let collateral_info =
                <CollateralInfos<T>>::try_mutate(collateral_asset_id, |collateral_info| {
                    let collateral_info = collateral_info
                        .as_mut()
                        .ok_or(Error::<T>::CollateralInfoNotFound)?;
                    let now = Timestamp::<T>::get();
                    ensure!(
                        now >= collateral_info.last_fee_update_time,
                        Error::<T>::AccrueWrongTime
                    );
                    // do not update if time is the same
                    if now > collateral_info.last_fee_update_time {
                        let time_passed = now
                            .checked_sub(&collateral_info.last_fee_update_time)
                            .ok_or(Error::<T>::ArithmeticError)?;
                        let new_coefficient = compound(
                            collateral_info.interest_coefficient.into_inner(),
                            collateral_info.risk_parameters.stability_fee_rate,
                            time_passed
                                .checked_into::<u64>()
                                .ok_or(Error::<T>::ArithmeticError)?,
                        )
                        .map_err(|_| Error::<T>::ArithmeticError)?;
                        collateral_info.last_fee_update_time = now;
                        collateral_info.interest_coefficient =
                            FixedU128::from_inner(new_coefficient);
                    }
                    Ok::<CollateralInfo<T::Moment>, DispatchError>(collateral_info.clone())
                })?;

            Ok(collateral_info)
        }

        /// Accrues interest on a Collateralized Debt Position (CDP) and updates relevant parameters.
        ///
        /// ## Parameters
        ///
        /// - `cdp_id`: The ID of the CDP for interest accrual.
        fn accrue_internal(
            cdp_id: U256,
        ) -> Result<CollateralizedDebtPosition<AccountIdOf<T>, AssetIdOf<T>>, DispatchError>
        {
            let mut cdp = Self::cdp(cdp_id).ok_or(Error::<T>::CDPNotFound)?;
            let collateral_info =
                Self::update_collateral_interest_coefficient(cdp.collateral_asset_id)?;
            let new_coefficient = collateral_info.interest_coefficient;
            let interest_percent = (new_coefficient
                .checked_sub(&cdp.interest_coefficient)
                .ok_or(Error::<T>::ArithmeticError)?)
            .checked_div(&cdp.interest_coefficient)
            .ok_or(Error::<T>::ArithmeticError)?;
            let mut stability_fee = FixedU128::from_inner(cdp.debt)
                .checked_mul(&interest_percent)
                .ok_or(Error::<T>::ArithmeticError)?
                .into_inner();
            let new_debt = cdp
                .debt
                .checked_add(stability_fee)
                .ok_or(Error::<T>::ArithmeticError)?;
            cdp = <CDPDepository<T>>::try_mutate(cdp_id, |cdp| {
                let cdp = cdp.as_mut().ok_or(Error::<T>::CDPNotFound)?;
                cdp.debt = new_debt;
                cdp.interest_coefficient = new_coefficient;
                Ok::<CollateralizedDebtPosition<T::AccountId, T::AssetId>, DispatchError>(
                    cdp.clone(),
                )
            })?;
            let mut new_bad_debt = <BadDebt<T>>::get();
            if new_bad_debt > 0 {
                if stability_fee <= new_bad_debt {
                    new_bad_debt = new_bad_debt
                        .checked_sub(stability_fee)
                        .ok_or(Error::<T>::ArithmeticError)?;
                    stability_fee = 0;
                } else {
                    stability_fee = stability_fee
                        .checked_sub(new_bad_debt)
                        .ok_or(Error::<T>::ArithmeticError)?;
                    new_bad_debt = balance!(0);
                };
                <BadDebt<T>>::try_mutate(|bad_debt| {
                    *bad_debt = new_bad_debt;
                    DispatchResult::Ok(())
                })?;
            }
            Self::mint_treasury(stability_fee)?;

            Ok(cdp)
        }

        /// Mint token to protocol technical account
        fn mint_treasury(amount: Balance) -> DispatchResult {
            technical::Pallet::<T>::mint(
                &T::KusdAssetId::get(),
                &T::TreasuryTechAccount::get(),
                amount,
            )?;
            Ok(())
        }

        /// Mint token to AccountId
        fn mint_to(account: &AccountIdOf<T>, amount: Balance) -> DispatchResult {
            let technical_account_id = technical::Pallet::<T>::tech_account_id_to_account_id(
                &T::TreasuryTechAccount::get(),
            )?;
            assets::Pallet::<T>::mint_to(
                &T::KusdAssetId::get(),
                &technical_account_id,
                account,
                amount,
            )?;
            Ok(())
        }

        /// Burns tokens from treasury technical account
        fn burn_treasury(to_burn: Balance) -> DispatchResult {
            let technical_account_id = technical::Pallet::<T>::tech_account_id_to_account_id(
                &T::TreasuryTechAccount::get(),
            )?;
            assets::Pallet::<T>::burn_from(
                &T::KusdAssetId::get(),
                &technical_account_id,
                &technical_account_id,
                to_burn,
            )?;
            Ok(())
        }

        /// Burns a specified amount of an asset from an account.
        ///
        /// ## Parameters
        ///
        /// - `account`: The account from which the asset will be burnt.
        /// - `amount`: The amount of the asset to be burnt.
        fn burn_from(account: &AccountIdOf<T>, amount: Balance) -> DispatchResult {
            let technical_account_id = technical::Pallet::<T>::tech_account_id_to_account_id(
                &T::TreasuryTechAccount::get(),
            )?;
            assets::Pallet::<T>::burn_from(
                &T::KusdAssetId::get(),
                &technical_account_id,
                account,
                amount,
            )?;
            Ok(())
        }

        /// Cover CDP debt with protocol balance
        /// If protocol balance is less than amount to cover, it is a bad debt
        fn cover_with_protocol(amount: Balance) -> DispatchResult {
            let treasury_account_id = technical::Pallet::<T>::tech_account_id_to_account_id(
                &T::TreasuryTechAccount::get(),
            )?;
            let protocol_positive_balance =
                T::AssetInfoProvider::free_balance(&T::KusdAssetId::get(), &treasury_account_id)?;
            let to_burn = if amount <= protocol_positive_balance {
                amount
            } else {
                <BadDebt<T>>::try_mutate(|bad_debt| {
                    *bad_debt = bad_debt
                        .checked_add(
                            amount
                                .checked_sub(protocol_positive_balance)
                                .ok_or(Error::<T>::ArithmeticError)?,
                        )
                        .ok_or(Error::<T>::ArithmeticError)?;
                    DispatchResult::Ok(())
                })?;
                protocol_positive_balance
            };
            Self::burn_treasury(to_burn)?;

            Ok(())
        }

        /// Returns CDP ids where the account id is owner
        pub fn get_account_cdp_ids(
            account_id: &AccountIdOf<T>,
        ) -> Result<Vec<U256>, DispatchError> {
            Ok(<CDPDepository<T>>::iter()
                .filter(|(_, cdp)| cdp.owner == *account_id)
                .map(|(cdp_id, _)| cdp_id)
                .collect())
        }
    }
}
