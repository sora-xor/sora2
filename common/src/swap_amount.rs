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

use core::convert::{TryFrom, TryInto};
use core::ops::{Mul, MulAssign};
use core::result::Result;

use alloc::collections::BTreeMap;
use codec::{Decode, Encode, MaxEncodedLen};
use fixnum::ops::RoundMode::*;
use fixnum::ops::RoundingMul;
use frame_support::RuntimeDebug;
#[cfg(feature = "std")]
use serde::{Deserialize, Serialize};
use sp_runtime::traits::{CheckedAdd, CheckedSub, UniqueSaturatedFrom, UniqueSaturatedInto};
use sp_std::mem;
use sp_std::ops::{Add, Sub};

use crate::primitives::Balance;
use crate::Fixed;

#[derive(
    Encode, Decode, Copy, Clone, RuntimeDebug, PartialEq, Eq, PartialOrd, Ord, scale_info::TypeInfo,
)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub enum QuoteAmount<AmountType> {
    WithDesiredInput { desired_amount_in: AmountType },
    WithDesiredOutput { desired_amount_out: AmountType },
}

impl<T> QuoteAmount<T> {
    pub fn with_desired_input(desired_amount_in: T) -> Self {
        Self::WithDesiredInput { desired_amount_in }
    }

    pub fn with_desired_output(desired_amount_out: T) -> Self {
        Self::WithDesiredOutput { desired_amount_out }
    }

    pub fn with_variant(variant: SwapVariant, amount: T) -> Self {
        match variant {
            SwapVariant::WithDesiredInput => Self::WithDesiredInput {
                desired_amount_in: amount,
            },
            SwapVariant::WithDesiredOutput => Self::WithDesiredOutput {
                desired_amount_out: amount,
            },
        }
    }

    /// Return inner amount value of either desired_amount_in or desired_amount_out to put away enum variant.
    pub fn amount(self) -> T {
        match self {
            QuoteAmount::WithDesiredInput {
                desired_amount_in: amount,
                ..
            }
            | QuoteAmount::WithDesiredOutput {
                desired_amount_out: amount,
                ..
            } => amount,
        }
    }

    /// Returns desired input/output variant of the value
    pub fn variant(&self) -> SwapVariant {
        match self {
            QuoteAmount::WithDesiredInput { .. } => SwapVariant::WithDesiredInput,
            QuoteAmount::WithDesiredOutput { .. } => SwapVariant::WithDesiredOutput,
        }
    }

    /// Position desired amount with outcome such that input and output values are aligned.
    pub fn place_input_and_output<AssetId: Ord>(self, outcome: SwapOutcome<T, AssetId>) -> (T, T) {
        match self {
            Self::WithDesiredInput { .. } => (self.amount(), outcome.amount),
            Self::WithDesiredOutput { .. } => (outcome.amount, self.amount()),
        }
    }

    /// Create new value with same direction as `self`.
    pub fn copy_direction(&self, amount: T) -> Self {
        match self {
            Self::WithDesiredInput { .. } => Self::with_desired_input(amount),
            Self::WithDesiredOutput { .. } => Self::with_desired_output(amount),
        }
    }
}

/// Provide addition for QuoteAmount type. Only values of same enum variant can be added,
/// otherwise panic. Arithmetic failures, e.g. overflow, underflow will panic.
impl<T: Add<Output = T>> Add for QuoteAmount<T> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (
                Self::WithDesiredInput {
                    desired_amount_in: in_a,
                },
                Self::WithDesiredInput {
                    desired_amount_in: in_b,
                },
            ) => Self::with_desired_input(in_a + in_b),
            (
                Self::WithDesiredOutput {
                    desired_amount_out: out_a,
                },
                Self::WithDesiredOutput {
                    desired_amount_out: out_b,
                },
            ) => Self::with_desired_output(out_a + out_b),
            (_, _) => panic!("cannot add non-uniform variants"),
        }
    }
}

/// Provide subtraction for QuoteAmount type. Only values of same enum variant can be subtracted,
/// otherwise panic. Arithmetic failures, e.g. overflow, underflow will panic.
impl<T: Sub<Output = T>> Sub for QuoteAmount<T> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (
                Self::WithDesiredInput {
                    desired_amount_in: in_a,
                },
                Self::WithDesiredInput {
                    desired_amount_in: in_b,
                },
            ) => Self::with_desired_input(in_a - in_b),
            (
                Self::WithDesiredOutput {
                    desired_amount_out: out_a,
                },
                Self::WithDesiredOutput {
                    desired_amount_out: out_b,
                },
            ) => Self::with_desired_output(out_a - out_b),
            (_, _) => panic!("cannot subtract non-uniform variants"),
        }
    }
}

/// Provide checked addition for QuoteAmount type. Only values of same enum variant can be added,
/// otherwise return `None`. Arithmetic failures, e.g. overflow, underflow will return `None`.
impl<T: CheckedAdd> CheckedAdd for QuoteAmount<T> {
    fn checked_add(&self, rhs: &QuoteAmount<T>) -> Option<Self::Output> {
        match (self, rhs) {
            (
                Self::WithDesiredInput {
                    desired_amount_in: in_a,
                },
                Self::WithDesiredInput {
                    desired_amount_in: in_b,
                },
            ) => Some(Self::with_desired_input(in_a.checked_add(in_b)?)),
            (
                Self::WithDesiredOutput {
                    desired_amount_out: out_a,
                },
                Self::WithDesiredOutput {
                    desired_amount_out: out_b,
                },
            ) => Some(Self::with_desired_output(out_a.checked_add(out_b)?)),
            (_, _) => None,
        }
    }
}

/// Provide checked subtraction for QuoteAmount type. Only values of same enum variant can be subtracted,
/// otherwise return `None`. Arithmetic failures, e.g. overflow, underflow will return `None`.
impl<T: CheckedSub<Output = T>> CheckedSub for QuoteAmount<T> {
    fn checked_sub(&self, rhs: &QuoteAmount<T>) -> Option<Self::Output> {
        match (self, rhs) {
            (
                Self::WithDesiredInput {
                    desired_amount_in: in_a,
                },
                Self::WithDesiredInput {
                    desired_amount_in: in_b,
                },
            ) => Some(Self::with_desired_input(in_a.checked_sub(in_b)?)),
            (
                Self::WithDesiredOutput {
                    desired_amount_out: out_a,
                },
                Self::WithDesiredOutput {
                    desired_amount_out: out_b,
                },
            ) => Some(Self::with_desired_output(out_a.checked_sub(out_b)?)),
            (_, _) => None,
        }
    }
}

impl<T> From<SwapAmount<T>> for QuoteAmount<T> {
    fn from(swap_amount: SwapAmount<T>) -> Self {
        match swap_amount {
            SwapAmount::WithDesiredInput {
                desired_amount_in, ..
            } => Self::with_desired_input(desired_amount_in),
            SwapAmount::WithDesiredOutput {
                desired_amount_out, ..
            } => Self::with_desired_output(desired_amount_out),
        }
    }
}

#[derive(Clone, Copy)]
#[cfg_attr(feature = "std", derive(Debug))]
pub struct TryFromQuoteAmountError;

impl TryFrom<QuoteAmount<Fixed>> for QuoteAmount<Balance> {
    type Error = TryFromQuoteAmountError;

    fn try_from(v: QuoteAmount<Fixed>) -> Result<Self, Self::Error> {
        Ok(match v {
            QuoteAmount::WithDesiredInput { desired_amount_in } => Self::WithDesiredInput {
                desired_amount_in: desired_amount_in
                    .into_bits()
                    .try_into()
                    .map_err(|_| TryFromQuoteAmountError)?,
            },
            QuoteAmount::WithDesiredOutput { desired_amount_out } => Self::WithDesiredOutput {
                desired_amount_out: desired_amount_out
                    .into_bits()
                    .try_into()
                    .map_err(|_| TryFromQuoteAmountError)?,
            },
        })
    }
}

impl TryFrom<QuoteAmount<Balance>> for QuoteAmount<Fixed> {
    type Error = TryFromQuoteAmountError;

    fn try_from(v: QuoteAmount<Balance>) -> Result<Self, Self::Error> {
        Ok(match v {
            QuoteAmount::WithDesiredInput { desired_amount_in } => Self::WithDesiredInput {
                desired_amount_in: Fixed::from_bits(
                    desired_amount_in
                        .try_into()
                        .map_err(|_| TryFromQuoteAmountError)?,
                ),
            },
            QuoteAmount::WithDesiredOutput { desired_amount_out } => Self::WithDesiredOutput {
                desired_amount_out: Fixed::from_bits(
                    desired_amount_out
                        .try_into()
                        .map_err(|_| TryFromQuoteAmountError)?,
                ),
            },
        })
    }
}

/// Used to identify intention of caller to indicate desired input amount or desired output amount.
/// Similar to SwapAmount, does not hold value in order to be used in external API.
#[derive(
    Encode, Decode, Copy, Clone, RuntimeDebug, PartialEq, Eq, PartialOrd, Ord, scale_info::TypeInfo,
)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub enum SwapVariant {
    WithDesiredInput,
    WithDesiredOutput,
}

/// Used to identify intention of caller either to transfer tokens based on exact input amount or
/// exact output amount.
#[derive(
    Encode,
    Decode,
    Copy,
    Clone,
    RuntimeDebug,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    scale_info::TypeInfo,
    MaxEncodedLen,
)]
pub enum SwapAmount<AmountType> {
    WithDesiredInput {
        desired_amount_in: AmountType,
        min_amount_out: AmountType,
    },
    WithDesiredOutput {
        desired_amount_out: AmountType,
        max_amount_in: AmountType,
    },
}

impl<T> SwapAmount<T> {
    pub fn with_desired_input(desired_amount_in: T, min_amount_out: T) -> Self {
        Self::WithDesiredInput {
            desired_amount_in,
            min_amount_out,
        }
    }

    pub fn with_desired_output(desired_amount_out: T, max_amount_in: T) -> Self {
        Self::WithDesiredOutput {
            desired_amount_out,
            max_amount_in,
        }
    }

    pub fn with_variant(variant: SwapVariant, amount: T, limit: T) -> Self {
        match variant {
            SwapVariant::WithDesiredInput => Self::WithDesiredInput {
                desired_amount_in: amount,
                min_amount_out: limit,
            },
            SwapVariant::WithDesiredOutput => Self::WithDesiredOutput {
                desired_amount_out: amount,
                max_amount_in: limit,
            },
        }
    }

    /// Return inner amount value of either desired_amount_in or desired_amount_out to put away enum variant.
    pub fn amount(self) -> T {
        match self {
            SwapAmount::WithDesiredInput {
                desired_amount_in: amount,
                ..
            }
            | SwapAmount::WithDesiredOutput {
                desired_amount_out: amount,
                ..
            } => amount,
        }
    }

    /// Return inner limit value of either min_amount_out or max_amount_in to put away enum variant.
    pub fn limit(self) -> T {
        match self {
            SwapAmount::WithDesiredInput {
                min_amount_out: amount,
                ..
            }
            | SwapAmount::WithDesiredOutput {
                max_amount_in: amount,
                ..
            } => amount,
        }
    }

    // Position desired amount with outcome such that input and output values are aligned.
    pub fn place_input_and_output<AssetId: Ord>(self, outcome: SwapOutcome<T, AssetId>) -> (T, T) {
        match self {
            Self::WithDesiredInput { .. } => (self.amount(), outcome.amount),
            Self::WithDesiredOutput { .. } => (outcome.amount, self.amount()),
        }
    }

    // Create new value with same direction as `self`.
    pub fn copy_direction(&self, amount: T, limit: T) -> Self {
        match self {
            Self::WithDesiredInput { .. } => Self::with_desired_input(amount, limit),
            Self::WithDesiredOutput { .. } => Self::with_desired_output(amount, limit),
        }
    }
}

#[derive(Clone, Copy)]
#[cfg_attr(feature = "std", derive(Debug))]
pub struct TryFromSwapAmountError;

impl TryFrom<SwapAmount<Fixed>> for SwapAmount<Balance> {
    type Error = TryFromSwapAmountError;

    fn try_from(v: SwapAmount<Fixed>) -> Result<Self, Self::Error> {
        Ok(match v {
            SwapAmount::WithDesiredInput {
                desired_amount_in,
                min_amount_out,
            } => SwapAmount::WithDesiredInput {
                desired_amount_in: desired_amount_in
                    .into_bits()
                    .try_into()
                    .map_err(|_| TryFromSwapAmountError)?,
                min_amount_out: min_amount_out
                    .into_bits()
                    .try_into()
                    .map_err(|_| TryFromSwapAmountError)?,
            },
            SwapAmount::WithDesiredOutput {
                desired_amount_out,
                max_amount_in,
            } => SwapAmount::WithDesiredOutput {
                desired_amount_out: desired_amount_out
                    .into_bits()
                    .try_into()
                    .map_err(|_| TryFromSwapAmountError)?,
                max_amount_in: max_amount_in
                    .into_bits()
                    .try_into()
                    .map_err(|_| TryFromSwapAmountError)?,
            },
        })
    }
}

impl UniqueSaturatedFrom<SwapAmount<Fixed>> for SwapAmount<Balance> {
    fn unique_saturated_from(v: SwapAmount<Fixed>) -> Self {
        match v {
            SwapAmount::WithDesiredInput {
                desired_amount_in,
                min_amount_out,
            } => SwapAmount::WithDesiredInput {
                desired_amount_in: desired_amount_in.into_bits().unique_saturated_into(),
                min_amount_out: min_amount_out.into_bits().unique_saturated_into(),
            },
            SwapAmount::WithDesiredOutput {
                desired_amount_out,
                max_amount_in,
            } => SwapAmount::WithDesiredOutput {
                desired_amount_out: desired_amount_out.into_bits().unique_saturated_into(),
                max_amount_in: max_amount_in.into_bits().unique_saturated_into(),
            },
        }
    }
}

impl TryFrom<SwapAmount<Balance>> for SwapAmount<Fixed> {
    type Error = TryFromSwapAmountError;

    fn try_from(v: SwapAmount<Balance>) -> Result<Self, Self::Error> {
        Ok(match v {
            SwapAmount::WithDesiredInput {
                desired_amount_in,
                min_amount_out,
            } => SwapAmount::WithDesiredInput {
                desired_amount_in: Fixed::from_bits(
                    desired_amount_in
                        .try_into()
                        .map_err(|_| TryFromSwapAmountError)?,
                ),
                min_amount_out: Fixed::from_bits(
                    min_amount_out
                        .try_into()
                        .map_err(|_| TryFromSwapAmountError)?,
                ),
            },
            SwapAmount::WithDesiredOutput {
                desired_amount_out,
                max_amount_in,
            } => SwapAmount::WithDesiredOutput {
                desired_amount_out: Fixed::from_bits(
                    desired_amount_out
                        .try_into()
                        .map_err(|_| TryFromSwapAmountError)?,
                ),
                max_amount_in: Fixed::from_bits(
                    max_amount_in
                        .try_into()
                        .map_err(|_| TryFromSwapAmountError)?,
                ),
            },
        })
    }
}

impl UniqueSaturatedFrom<SwapAmount<Balance>> for SwapAmount<Fixed> {
    fn unique_saturated_from(v: SwapAmount<Balance>) -> Self {
        match v {
            SwapAmount::WithDesiredInput {
                desired_amount_in,
                min_amount_out,
            } => SwapAmount::WithDesiredInput {
                desired_amount_in: Fixed::from_bits(desired_amount_in.unique_saturated_into()),
                min_amount_out: Fixed::from_bits(min_amount_out.unique_saturated_into()),
            },
            SwapAmount::WithDesiredOutput {
                desired_amount_out,
                max_amount_in,
            } => SwapAmount::WithDesiredOutput {
                desired_amount_out: Fixed::from_bits(desired_amount_out.unique_saturated_into()),
                max_amount_in: Fixed::from_bits(max_amount_in.unique_saturated_into()),
            },
        }
    }
}

impl<T> From<SwapAmount<T>> for SwapVariant {
    fn from(v: SwapAmount<T>) -> Self {
        match v {
            SwapAmount::WithDesiredInput { .. } => SwapVariant::WithDesiredInput,
            _ => SwapVariant::WithDesiredOutput,
        }
    }
}

// TODO: use macros for impl generation
impl<T> Mul<Fixed> for SwapAmount<T>
where
    T: Copy + Into<Fixed> + From<Fixed>,
{
    type Output = Self;

    fn mul(self, rhs: Fixed) -> Self::Output {
        match self {
            SwapAmount::WithDesiredInput {
                desired_amount_in,
                min_amount_out,
            } => SwapAmount::with_desired_input(
                rhs.rmul(desired_amount_in.into(), Floor).unwrap().into(),
                rhs.rmul(min_amount_out.into(), Floor).unwrap().into(),
            ),
            SwapAmount::WithDesiredOutput {
                desired_amount_out,
                max_amount_in,
            } => SwapAmount::with_desired_output(
                rhs.rmul(desired_amount_out.into(), Floor).unwrap().into(),
                rhs.rmul(max_amount_in.into(), Floor).unwrap().into(),
            ),
        }
    }
}

impl<T> MulAssign<Fixed> for SwapAmount<T>
where
    T: Copy + Into<Fixed> + From<Fixed>,
{
    fn mul_assign(&mut self, rhs: Fixed) {
        match *self {
            SwapAmount::WithDesiredInput {
                desired_amount_in,
                min_amount_out,
            } => mem::replace(
                self,
                SwapAmount::with_desired_input(
                    rhs.rmul(desired_amount_in.into(), Floor).unwrap().into(),
                    rhs.rmul(min_amount_out.into(), Floor).unwrap().into(),
                ),
            ),
            SwapAmount::WithDesiredOutput {
                desired_amount_out,
                max_amount_in,
            } => mem::replace(
                self,
                SwapAmount::with_desired_output(
                    rhs.rmul(desired_amount_out.into(), Floor).unwrap().into(),
                    rhs.rmul(max_amount_in.into(), Floor).unwrap().into(),
                ),
            ),
        };
    }
}

impl<T> Mul<SwapAmount<T>> for Fixed
where
    T: Copy + RoundingMul<Output = T> + Into<Fixed> + From<Fixed>,
{
    type Output = SwapAmount<T>;

    fn mul(self, rhs: SwapAmount<T>) -> Self::Output {
        match rhs {
            SwapAmount::WithDesiredInput {
                desired_amount_in,
                min_amount_out,
            } => SwapAmount::with_desired_input(
                self.rmul(desired_amount_in.into(), Floor).unwrap().into(),
                self.rmul(min_amount_out.into(), Floor).unwrap().into(),
            ),
            SwapAmount::WithDesiredOutput {
                desired_amount_out,
                max_amount_in,
            } => SwapAmount::with_desired_output(
                self.rmul(desired_amount_out.into(), Floor).unwrap().into(),
                self.rmul(max_amount_in.into(), Floor).unwrap().into(),
            ),
        }
    }
}

#[derive(
    Encode,
    Decode,
    Default,
    Eq,
    PartialEq,
    Clone,
    Ord,
    PartialOrd,
    RuntimeDebug,
    scale_info::TypeInfo,
)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct OutcomeFee<AssetId: Ord, AmountType>(
    pub sp_std::collections::btree_map::BTreeMap<AssetId, AmountType>,
);

impl<AssetId: Ord, AmountType> OutcomeFee<AssetId, AmountType> {
    pub fn new() -> Self {
        Self(BTreeMap::new())
    }
}

// Most used fee assets
impl<AssetId, AmountType> OutcomeFee<AssetId, AmountType>
where
    AssetId: Ord + From<crate::AssetId32<crate::PredefinedAssetId>>,
    AmountType: Default + Copy,
{
    pub fn xor(amount: AmountType) -> Self {
        Self(BTreeMap::from([(crate::XOR.into(), amount)]))
    }

    pub fn xst(amount: AmountType) -> Self {
        Self(BTreeMap::from([(crate::XST.into(), amount)]))
    }

    pub fn xstusd(amount: AmountType) -> Self {
        Self(BTreeMap::from([(crate::XSTUSD.into(), amount)]))
    }

    pub fn get_xor(&self) -> AmountType {
        self.0.get(&crate::XOR.into()).copied().unwrap_or_default()
    }

    pub fn get_xst(&self) -> AmountType {
        self.0.get(&crate::XST.into()).copied().unwrap_or_default()
    }

    pub fn get_xstusd(&self) -> AmountType {
        self.0
            .get(&crate::XSTUSD.into())
            .copied()
            .unwrap_or_default()
    }
}

impl<AssetId, AmountType> OutcomeFee<AssetId, AmountType>
where
    AssetId: Ord + From<crate::AssetId32<crate::PredefinedAssetId>>,
    AmountType: Copy + sp_runtime::traits::Saturating,
{
    pub fn add_xor(&mut self, amount: AmountType) {
        self.0
            .entry(crate::XOR.into())
            .and_modify(|xor_amount| *xor_amount = xor_amount.saturating_add(amount))
            .or_insert(amount);
    }

    pub fn add_xst(&mut self, amount: AmountType) {
        self.0
            .entry(crate::XST.into())
            .and_modify(|xst_amount| *xst_amount = xst_amount.saturating_add(amount))
            .or_insert(amount);
    }

    pub fn add_xstusd(&mut self, amount: AmountType) {
        self.0
            .entry(crate::XSTUSD.into())
            .and_modify(|xstusd_amount| *xstusd_amount = xstusd_amount.saturating_add(amount))
            .or_insert(amount);
    }

    pub fn merge(mut self, other: Self) -> Self {
        for (asset, other_amount) in other.0 {
            self.0
                .entry(asset)
                .and_modify(|amount| *amount = amount.saturating_add(other_amount))
                .or_insert(other_amount);
        }
        self
    }
}

/// Amount of output tokens from either price request or actual exchange.
#[derive(
    Encode, Decode, Clone, RuntimeDebug, PartialEq, Eq, PartialOrd, Ord, scale_info::TypeInfo,
)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct SwapOutcome<AmountType, AssetId: Ord> {
    /// Actual swap output/input amount including deduced fee.
    pub amount: AmountType,
    /// Accumulated fee amount.
    pub fee: OutcomeFee<AssetId, AmountType>,
}

impl<AmountType, AssetId: Ord> SwapOutcome<AmountType, AssetId> {
    pub fn new(amount: AmountType, fee: OutcomeFee<AssetId, AmountType>) -> Self {
        Self { amount, fee }
    }
}

#[derive(Clone, Copy)]
#[cfg_attr(feature = "std", derive(Debug))]
pub struct TryFromSwapOutcomeError;

impl<AssetId: Ord> TryFrom<SwapOutcome<Balance, AssetId>> for SwapOutcome<Fixed, AssetId> {
    type Error = TryFromSwapOutcomeError;

    fn try_from(value: SwapOutcome<Balance, AssetId>) -> Result<Self, Self::Error> {
        let amount = Fixed::from_bits(
            value
                .amount
                .try_into()
                .map_err(|_| TryFromSwapOutcomeError)?,
        );

        let mut fee = OutcomeFee::new();
        for (asset, fee_amount) in value.fee.0 {
            let fee_fixed =
                Fixed::from_bits(fee_amount.try_into().map_err(|_| TryFromSwapOutcomeError)?);
            fee.0.insert(asset, fee_fixed);
        }

        Ok(Self { amount, fee })
    }
}

impl<AssetId: Ord> TryFrom<SwapOutcome<Fixed, AssetId>> for SwapOutcome<Balance, AssetId> {
    type Error = TryFromSwapOutcomeError;

    fn try_from(value: SwapOutcome<Fixed, AssetId>) -> Result<Self, Self::Error> {
        let amount = value
            .amount
            .into_bits()
            .try_into()
            .map_err(|_| TryFromSwapOutcomeError)?;

        let mut fee = OutcomeFee::new();
        for (asset, fee_amount) in value.fee.0 {
            let fee_balance = fee_amount
                .into_bits()
                .try_into()
                .map_err(|_| TryFromSwapOutcomeError)?;
            fee.0.insert(asset, fee_balance);
        }

        Ok(Self { amount, fee })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::fixed;

    #[test]
    fn test_mul_amount_should_pass() {
        let swap_amount: SwapAmount<Fixed> =
            SwapAmount::with_desired_input(fixed!(100), fixed!(50));
        assert_eq!(
            swap_amount * fixed!(2),
            SwapAmount::with_desired_input(fixed!(200), fixed!(100))
        );
    }

    #[test]
    fn test_mul_assign_amount_should_pass() {
        let mut swap_amount: SwapAmount<Fixed> =
            SwapAmount::with_desired_input(fixed!(100), fixed!(50));
        swap_amount *= fixed!(2);
        assert_eq!(
            swap_amount,
            SwapAmount::with_desired_input(fixed!(200), fixed!(100))
        );
    }
}
