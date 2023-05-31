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

use crate::{
    Config, LimitOrder, MarketSide, OrderBookId, OrderPrice, OrderVolume, PriceOrders, UserOrders,
};
use assets::AssetIdOf;
use common::PriceVariant;
use frame_support::sp_runtime::DispatchError;
use sp_std::collections::btree_map::BTreeMap;
use sp_std::vec::Vec;

/// This trait is used by Order Book as a storage to get limit orders and their derived data and to change them
pub trait DataLayer<T>
where
    T: Config,
{
    /// Returns the limit order if exists, otherwise returns error.
    fn get_limit_order(
        &mut self,
        order_book_id: &OrderBookId<AssetIdOf<T>>,
        order_id: T::OrderId,
    ) -> Result<LimitOrder<T>, DispatchError>;

    /// Returns all limit orders of order book
    fn get_all_limit_orders(
        &mut self,
        order_book_id: &OrderBookId<AssetIdOf<T>>,
    ) -> Vec<LimitOrder<T>>;

    /// Inserts limit order consistently in all necessary storages.
    /// Must check before call. If returns error, it means we have problems with data consistency.
    /// If order_id already exists - returns error. Use `update_limit_order` to update the existing order.
    fn insert_limit_order(
        &mut self,
        order_book_id: &OrderBookId<AssetIdOf<T>>,
        order: LimitOrder<T>,
    ) -> Result<(), DispatchError>;

    /// Updates the amount of the limit order.
    /// If order doesn't exist - return error
    fn update_limit_order_amount(
        &mut self,
        order_book_id: &OrderBookId<AssetIdOf<T>>,
        order_id: T::OrderId,
        new_amount: OrderVolume,
    ) -> Result<(), DispatchError>;

    /// Deletes limit order consistently from all necessary storages.
    /// If order doesn't exist - return error
    /// Must check before call. If returns error, it means we have problems with data consistency.
    fn delete_limit_order(
        &mut self,
        order_book_id: &OrderBookId<AssetIdOf<T>>,
        order_id: T::OrderId,
    ) -> Result<(), DispatchError>;

    /// Returns order ids of orders inside the bid or ask price
    fn get_limit_orders_by_price(
        &mut self,
        order_book_id: &OrderBookId<AssetIdOf<T>>,
        side: PriceVariant,
        price: &OrderPrice,
    ) -> Option<PriceOrders<T::OrderId, T::MaxLimitOrdersForPrice>> {
        match side {
            PriceVariant::Buy => self.get_bids(order_book_id, price),
            PriceVariant::Sell => self.get_asks(order_book_id, price),
        }
    }

    /// Returns order ids of orders inside the bid price
    fn get_bids(
        &mut self,
        order_book_id: &OrderBookId<AssetIdOf<T>>,
        price: &OrderPrice,
    ) -> Option<PriceOrders<T::OrderId, T::MaxLimitOrdersForPrice>>;

    /// Returns order ids of orders inside the ask price
    fn get_asks(
        &mut self,
        order_book_id: &OrderBookId<AssetIdOf<T>>,
        price: &OrderPrice,
    ) -> Option<PriceOrders<T::OrderId, T::MaxLimitOrdersForPrice>>;

    /// Returns all bid prices with their volumes
    fn get_aggregated_bids(
        &mut self,
        order_book_id: &OrderBookId<AssetIdOf<T>>,
    ) -> MarketSide<T::MaxSidePriceCount>;

    /// Returns all ask prices with their volumes
    fn get_aggregated_asks(
        &mut self,
        order_book_id: &OrderBookId<AssetIdOf<T>>,
    ) -> MarketSide<T::MaxSidePriceCount>;

    /// Returns order ids of user
    fn get_user_limit_orders(
        &mut self,
        account: &T::AccountId,
        order_book_id: &OrderBookId<AssetIdOf<T>>,
    ) -> Option<UserOrders<T::OrderId, T::MaxOpenedLimitOrdersPerUser>>;
}

// todo: make pub(tests) (k.ivanov)
pub trait CurrencyLocker<AccountId, AssetId, DEXId> {
    /// Lock `amount` of liquidity in `order_book_id`'s asset chosen by `asset`.
    /// The assets are taken from `account`.
    fn lock_liquidity(
        dex_id: DEXId,
        account: &AccountId,
        order_book_id: OrderBookId<AssetId>,
        asset_id: &AssetId,
        amount: OrderVolume,
    ) -> Result<(), DispatchError>;
}

// todo: make pub(tests) (k.ivanov)
pub trait CurrencyUnlocker<AccountId, AssetId, DEXId> {
    /// Unlock `amount` of liquidity in `order_book_id`'s asset chosen by `asset`.
    /// The assets are taken from `account`.
    fn unlock_liquidity(
        dex_id: DEXId,
        account: &AccountId,
        order_book_id: OrderBookId<AssetId>,
        asset_id: &AssetId,
        amount: OrderVolume,
    ) -> Result<(), DispatchError>;

    fn unlock_liquidity_batch(
        dex_id: DEXId,
        order_book_id: OrderBookId<AssetId>,
        asset_id: &AssetId,
        receivers: &BTreeMap<AccountId, OrderVolume>,
    ) -> Result<(), DispatchError>;
}
