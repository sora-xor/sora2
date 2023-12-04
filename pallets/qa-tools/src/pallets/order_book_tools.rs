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

use crate::Config;
use common::{balance, Balance, PriceVariant, TradingPairSourceManager};
use frame_support::pallet_prelude::*;
use frame_support::traits::Time;
use frame_system::pallet_prelude::*;
use order_book::DataLayer;
use order_book::{MomentOf, OrderBook, OrderBookId};
use order_book::{OrderPrice, OrderVolume};
use sp_std::prelude::*;

#[derive(Encode, Decode, Clone, Copy, PartialEq, Eq, scale_info::TypeInfo)]
#[cfg_attr(feature = "std", derive(Debug))]
pub struct OrderBookAttributes {
    pub tick_size: Balance,
    pub step_lot_size: Balance,
    pub min_lot_size: Balance,
    pub max_lot_size: Balance,
}

// default attributes for regular assets (not NFT)
impl Default for OrderBookAttributes {
    fn default() -> Self {
        Self {
            tick_size: balance!(0.00001),
            step_lot_size: balance!(0.00001),
            min_lot_size: balance!(1),
            max_lot_size: balance!(1000),
        }
    }
}

#[derive(Encode, Decode, Clone, PartialEq, Eq, scale_info::TypeInfo)]
#[cfg_attr(feature = "std", derive(Debug))]
pub struct OrderBookFillSettings<Moment> {
    /// Best (highest) price for placed buy orders
    pub best_bid_price: order_book::types::OrderPrice,
    /// Best (lowest) price for placed sell orders
    pub best_ask_price: order_book::types::OrderPrice,
    /// Lifespan of inserted orders, max by default
    pub lifespan: Option<Moment>,
}

/// Does not create an order book if it already exists
pub fn create_multiple_empty_unchecked<T: Config>(
    order_book_settings: Vec<(OrderBookId<T::AssetId, T::DEXId>, OrderBookAttributes)>,
) -> Result<(), DispatchError> {
    let to_create_ids: Vec<_> = order_book_settings
        .into_iter()
        .filter(|(id, _)| !<order_book::OrderBooks<T>>::contains_key(id))
        .collect();
    for (order_book_id, _) in &to_create_ids {
        if !T::TradingPairSourceManager::is_trading_pair_enabled(
            &order_book_id.dex_id,
            &order_book_id.quote.into(),
            &order_book_id.base.into(),
        )? {
            T::TradingPairSourceManager::register_pair(
                order_book_id.dex_id,
                order_book_id.quote.into(),
                order_book_id.base.into(),
            )?;
        }
        order_book::Pallet::<T>::verify_create_orderbook_params(&order_book_id)?;
    }

    for (order_book_id, attributes) in to_create_ids {
        order_book::Pallet::<T>::create_orderbook_unchecked(
            &order_book_id,
            attributes.tick_size,
            attributes.step_lot_size,
            attributes.min_lot_size,
            attributes.max_lot_size,
        )?;
    }
    Ok(())
}

/// Place orders into the orderbook.
///
/// In fill settings, `best_bid_price` should be at least 3 price steps from the
/// lowest accepted price, and `best_ask_price` - at least 3 steps below
/// maximum price.
pub fn fill_multiple_empty_unchecked<T: Config>(
    bids_owner: T::AccountId,
    asks_owner: T::AccountId,
    settings: Vec<(
        OrderBookId<T::AssetId, T::DEXId>,
        OrderBookAttributes,
        OrderBookFillSettings<MomentOf<T>>,
    )>,
) -> Result<(), DispatchError> {
    let now = <T as order_book::Config>::Time::now();

    // Prices are specified as price steps from the specified best ask price.
    // Amounts are added to min_lot and aligned with lot(amount) step.

    // (price_steps_from_best_bid, amount)
    let buy_orders_steps = [
        (0, OrderVolume::divisible(balance!(168.5))),
        (1, OrderVolume::divisible(balance!(95.2))),
        (1, OrderVolume::divisible(balance!(44.7))),
        (3, OrderVolume::divisible(balance!(56.4))),
        (3, OrderVolume::divisible(balance!(89.9))),
        (3, OrderVolume::divisible(balance!(115))),
    ];

    // (price_steps_from_best_ask, amount)
    let sell_orders_steps = [
        (0, OrderVolume::divisible(balance!(176.3))),
        (1, OrderVolume::divisible(balance!(85.4))),
        (1, OrderVolume::divisible(balance!(93.2))),
        (3, OrderVolume::divisible(balance!(36.6))),
        (3, OrderVolume::divisible(balance!(205.5))),
        (3, OrderVolume::divisible(balance!(13.7))),
    ];

    let mut data = order_book::cache_data_layer::CacheDataLayer::<T>::new();

    for (order_book_id, _, fill_settings) in settings {
        fill_order_book(
            &mut data,
            order_book_id,
            asks_owner.clone(),
            bids_owner.clone(),
            buy_orders_steps.into_iter(),
            sell_orders_steps.into_iter(),
            fill_settings,
            now,
        )?;
    }
    data.commit();
    Ok(())
}

/// Fill a single order book.
fn fill_order_book<T: Config>(
    data: &mut impl DataLayer<T>,
    book_id: OrderBookId<T::AssetId, T::DEXId>,
    asks_owner: T::AccountId,
    bids_owner: T::AccountId,
    buy_orders_steps: impl Iterator<Item = (u128, OrderVolume)>,
    sell_orders_steps: impl Iterator<Item = (u128, OrderVolume)>,
    settings: OrderBookFillSettings<MomentOf<T>>,
    now: MomentOf<T>,
) -> Result<(), DispatchError> {
    let current_block = frame_system::Pallet::<T>::block_number();
    let lifespan = settings
        .lifespan
        .unwrap_or(<T as order_book::Config>::MAX_ORDER_LIFESPAN);
    let mut order_book = <order_book::OrderBooks<T>>::get(book_id)
        .ok_or(crate::Error::<T>::CannotFillUnknownOrderBook)?;

    // Convert price steps and best price to actual prices
    let buy_orders: Vec<_> = buy_orders_steps
        .map(|(price_steps, base)| {
            (
                settings.best_bid_price
                    - OrderPrice::divisible(price_steps * (*order_book.tick_size.balance())),
                order_book.align_amount(base + order_book.step_lot_size),
            )
        })
        .collect();
    let sell_orders: Vec<_> = sell_orders_steps
        .map(|(price_steps, base)| {
            (
                settings.best_ask_price
                    + OrderPrice::divisible(price_steps * (*order_book.tick_size.balance())),
                order_book.align_amount(base + order_book.step_lot_size),
            )
        })
        .collect();
    // Total amount of quote asset to be locked from `bids_owner`
    let buy_quote_locked: Balance = buy_orders
        .iter()
        .map(|(quote, base)| *(*quote * (*base)).balance())
        .sum();
    let sell_base_locked: Balance = sell_orders.iter().map(|(_, base)| *base.balance()).sum();

    // mint required amount to make this extrinsic self-sufficient
    assets::Pallet::<T>::mint_unchecked(&book_id.quote, &bids_owner, buy_quote_locked)?;
    assets::Pallet::<T>::mint_unchecked(&book_id.base, &asks_owner, sell_base_locked)?;

    // place buy orders
    place_multiple_orders(
        data,
        &mut order_book,
        bids_owner.clone(),
        PriceVariant::Buy,
        buy_orders.into_iter(),
        now,
        lifespan,
        current_block,
    )?;

    // place sell orders
    place_multiple_orders(
        data,
        &mut order_book,
        asks_owner.clone(),
        PriceVariant::Sell,
        sell_orders.into_iter(),
        now,
        lifespan,
        current_block,
    )?;

    <order_book::OrderBooks<T>>::set(book_id, Some(order_book));
    Ok(())
}

fn place_multiple_orders<T: Config>(
    data: &mut impl DataLayer<T>,
    book: &mut OrderBook<T>,
    owner: T::AccountId,
    side: PriceVariant,
    orders: impl Iterator<Item = (OrderPrice, OrderVolume)>,
    time: MomentOf<T>,
    lifespan: MomentOf<T>,
    current_block: BlockNumberFor<T>,
) -> Result<(), DispatchError> {
    for (price, amount) in orders {
        let order_id = book.next_order_id();
        let order = order_book::LimitOrder::<T>::new(
            order_id,
            owner.clone(),
            side,
            price,
            amount,
            time,
            lifespan,
            current_block,
        );
        book.place_limit_order(order, data)?;
    }
    Ok(())
}
