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

use crate::assets::AssetsCall;
use crate::order_book::OrderBookCall;
use common::{AssetId32, PredefinedAssetId};
use frame_support::pallet_prelude::{MaybeSerializeDeserialize, Member};
use frame_support::sp_runtime::testing::H256;
use frame_support::sp_runtime::traits::{AtLeast32BitUnsigned, MaybeDisplay};
use frame_support::Parameter;
use scale::{Encode, MaxEncodedLen};
use sp_std::fmt::Debug;
pub mod assets;

pub mod order_book;

/// It is a part of the runtime dispatchables API.
/// `Ink!` doesn't expose the real enum, so we need a partial definition matching our targets.
/// You should get or count index of the pallet, using `construct_runtime!`, it is zero based
#[derive(Encode)]
pub enum RuntimeCall<AssetId: AssetIdBounds, AccountId: AccountIdBounds, OrderId: OrderIdBounds> {
    #[codec(index = 21)]
    Assets(AssetsCall<AssetId, AccountId>),
    #[codec(index = 57)]
    OrderBook(OrderBookCall<AssetId, OrderId>),
}

pub trait AssetIdBounds:
    Parameter
    + Member
    + Copy
    + MaybeSerializeDeserialize
    + Ord
    + Default
    + Into<AssetId32<PredefinedAssetId>>
    + From<AssetId32<PredefinedAssetId>>
    + From<H256>
    + Into<H256>
    + MaxEncodedLen
{
}

impl<T> AssetIdBounds for T where
    T: Parameter
        + Member
        + Copy
        + MaybeSerializeDeserialize
        + Ord
        + Default
        + Into<AssetId32<PredefinedAssetId>>
        + From<AssetId32<PredefinedAssetId>>
        + From<H256>
        + Into<H256>
        + MaxEncodedLen
{
}

pub trait AccountIdBounds:
    Parameter + Member + MaybeSerializeDeserialize + Debug + MaybeDisplay + Ord + MaxEncodedLen
{
}

impl<T> AccountIdBounds for T where
    T: Parameter + Member + MaybeSerializeDeserialize + Debug + MaybeDisplay + Ord + MaxEncodedLen
{
}

pub trait OrderIdBounds:
    Parameter
    + Member
    + MaybeSerializeDeserialize
    + Debug
    + MaybeDisplay
    + AtLeast32BitUnsigned
    + Copy
    + Ord
    + PartialEq
    + Eq
    + MaxEncodedLen
    + scale_info::TypeInfo
{
}

impl<T> OrderIdBounds for T where
    T: Parameter
        + Member
        + MaybeSerializeDeserialize
        + Debug
        + MaybeDisplay
        + AtLeast32BitUnsigned
        + Copy
        + Ord
        + PartialEq
        + Eq
        + MaxEncodedLen
        + scale_info::TypeInfo
{
}
