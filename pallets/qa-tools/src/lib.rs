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

pub use pallet::*;

#[cfg(test)]
mod tests;

pub mod weights;
use common::AssetInfoProvider;
use order_book::OrderBookId;
pub use weights::*;

#[frame_support::pallet]
pub mod pallet {
    use super::*;
    use common::prelude::FixedWrapper;
    use common::{balance, Balance, PriceVariant};
    use frame_support::traits::{Get, Time};
    use frame_support::{dispatch::PostDispatchInfo, pallet_prelude::*};
    use frame_system::pallet_prelude::*;
    use order_book::cache_data_layer::CacheDataLayer;
    use order_book::{MomentOf, OrderBook};
    use sp_std::prelude::*;

    #[pallet::pallet]
    pub struct Pallet<T>(_);

    #[pallet::config]
    pub trait Config: frame_system::Config + order_book::Config + trading_pair::Config {
        type WeightInfo: WeightInfo;
        type OrderBookOrderLifespan: Get<MomentOf<Self>>;
    }

    // Errors inform users that something went wrong.
    #[pallet::error]
    pub enum Error<T> {
        /// Error names should be descriptive.
        NoneValue,
        /// Errors should have helpful documentation associated with them.
        StorageOverflow,
        /// Order book does not exist for this trading pair
        OrderBookUnkonwnBook,
        /// Could not place limit order
        OrderBookFailedToPlaceOrders,
    }

    #[derive(
        Encode,
        Decode,
        Eq,
        PartialEq,
        Copy,
        Clone,
        PartialOrd,
        Ord,
        RuntimeDebug,
        Hash,
        scale_info::TypeInfo,
        MaxEncodedLen,
    )]
    pub struct OrderBookFillSettings {
        pub best_bid_price: order_book::types::OrderPrice,
        pub best_ask_price: order_book::types::OrderPrice,
    }

    // Dispatchable functions allows users to interact with the pallet and invoke state changes.
    // These functions materialize as "extrinsics", which are often compared to transactions.
    // Dispatchable functions must be annotated with a weight and must return a DispatchResult.
    #[pallet::call]
    impl<T: Config> Pallet<T> {
        /// Create multiple many order books with default parameters if do not exist.
        #[pallet::call_index(0)]
        #[pallet::weight(<T as Config>::WeightInfo::do_something())]
        pub fn order_book_create_empty_many(
            origin: OriginFor<T>,
            dex_id: T::DEXId,
            order_book_ids: Vec<OrderBookId<T::AssetId>>,
        ) -> DispatchResultWithPostInfo {
            let _ = ensure_signed(origin)?;

            Self::create_multiple_empty_unchecked(dex_id, order_book_ids)?;

            // Extrinsic is only for testing, so we return all fees
            // for simplicity.
            Ok(PostDispatchInfo {
                actual_weight: Some(Weight::zero()),
                pays_fee: Pays::No,
            })
        }

        /// Create multiple many order books with default parameters if do not exist and
        /// fill them according to given parameters
        #[pallet::call_index(1)]
        #[pallet::weight(<T as Config>::WeightInfo::cause_error())]
        pub fn order_book_create_and_fill_many(
            origin: OriginFor<T>,
            dex_id: T::DEXId,
            bids_owner: T::AccountId,
            asks_owner: T::AccountId,
            fill_settings: Vec<(OrderBookId<T::AssetId>, OrderBookFillSettings)>,
        ) -> DispatchResultWithPostInfo {
            let _ = ensure_signed(origin)?;

            let order_book_ids: Vec<_> = fill_settings.iter().map(|(id, _)| id).cloned().collect();
            Self::create_multiple_empty_unchecked(dex_id, order_book_ids)?;
            Self::fill_multiple_empty_unchecked(bids_owner, asks_owner, fill_settings)?;

            // Extrinsic is only for testing, so we return all fees
            // for simplicity.
            Ok(PostDispatchInfo {
                actual_weight: Some(Weight::zero()),
                pays_fee: Pays::No,
            })
        }
    }

    impl<T: Config> Pallet<T> {
        /// Does not create an order book if already exists
        fn create_multiple_empty_unchecked(
            dex_id: T::DEXId,
            order_book_ids: Vec<OrderBookId<T::AssetId>>,
        ) -> Result<(), DispatchError> {
            let to_create_ids: Vec<_> = order_book_ids
                .into_iter()
                .filter(|id| !<order_book::OrderBooks<T>>::contains_key(id))
                .collect();
            for order_book_id in &to_create_ids {
                if !trading_pair::Pallet::<T>::is_trading_pair_enabled(
                    &dex_id,
                    &order_book_id.quote.into(),
                    &order_book_id.base.into(),
                )? {
                    trading_pair::Pallet::<T>::register_pair(
                        dex_id,
                        order_book_id.quote.into(),
                        order_book_id.base.into(),
                    )?;
                }
                order_book::Pallet::<T>::verify_create_orderbook_params(&dex_id, order_book_id)?;
            }

            for order_book_id in to_create_ids {
                let order_book = if T::AssetInfoProvider::is_non_divisible(&order_book_id.base) {
                    order_book::OrderBook::<T>::default_nft(order_book_id, dex_id)
                } else {
                    order_book::OrderBook::<T>::default(order_book_id, dex_id)
                };

                #[cfg(feature = "wip")] // order-book
                {
                    T::TradingPairSourceManager::enable_source_for_trading_pair(
                        &dex_id,
                        &order_book_id.quote,
                        &order_book_id.base,
                        LiquiditySourceType::OrderBook,
                    )?;
                }

                <order_book::OrderBooks<T>>::insert(order_book_id, order_book);
                order_book::Pallet::<T>::register_tech_account(dex_id, order_book_id)?;
            }
            Ok(())
        }

        fn fill_multiple_empty_unchecked(
            bids_owner: T::AccountId,
            asks_owner: T::AccountId,
            fill_settings: Vec<(OrderBookId<T::AssetId>, OrderBookFillSettings)>,
        ) -> Result<(), DispatchError> {
            let now = <T as order_book::Config>::Time::now();

            // (price_steps_from_best_ask, amount)
            let buy_orders_steps = [
                (0u128, balance!(168.5)),
                (1, balance!(95.2)),
                (1, balance!(44.7)),
                (3, balance!(56.4)),
                (3, balance!(89.9)),
                (3, balance!(115)),
            ];

            // (price_steps_from_best_bid, amount)
            let sell_orders_steps = [
                (0u128, balance!(176.3)),
                (1, balance!(85.4)),
                (1, balance!(93.2)),
                (3, balance!(36.6)),
                (3, balance!(205.5)),
                (3, balance!(13.7)),
            ];

            let mut data = order_book::cache_data_layer::CacheDataLayer::<T>::new();

            for (order_book_id, settings) in fill_settings {
                Self::fill_order_book(
                    &mut data,
                    order_book_id,
                    asks_owner.clone(),
                    bids_owner.clone(),
                    buy_orders_steps.into_iter(),
                    sell_orders_steps.into_iter(),
                    settings,
                    now,
                )?;
            }
            data.commit();
            Ok(())
        }

        fn fill_order_book(
            data: &mut CacheDataLayer<T>,
            book_id: OrderBookId<T::AssetId>,
            asks_owner: T::AccountId,
            bids_owner: T::AccountId,
            buy_orders_steps: impl Iterator<Item = (u128, Balance)>,
            sell_orders_steps: impl Iterator<Item = (u128, Balance)>,
            settings: OrderBookFillSettings,
            now: MomentOf<T>,
        ) -> Result<(), DispatchError> {
            let current_block = frame_system::Pallet::<T>::block_number();
            let mut order_book = <order_book::OrderBooks<T>>::get(book_id)
                .ok_or(Error::<T>::OrderBookUnkonwnBook)?;

            // Convert price steps and best ask to prices
            let buy_orders: Vec<_> = buy_orders_steps
                .map(|(price_steps, base)| {
                    (
                        settings.best_ask_price - price_steps * order_book.tick_size,
                        base,
                    )
                })
                .collect();
            let sell_orders: Vec<_> = sell_orders_steps
                .map(|(price_steps, base)| {
                    (
                        settings.best_bid_price + price_steps * order_book.tick_size,
                        base,
                    )
                })
                .collect();
            // Total amount of quote asset to be locked from `asks_owner`
            let buy_quote_locked: Balance = buy_orders
                .iter()
                .map(|(quote, base)| {
                    let quote_amount_fixed = FixedWrapper::from(*quote) * FixedWrapper::from(*base);
                    quote_amount_fixed.into_balance()
                })
                .sum();
            let sell_base_locked: Balance = sell_orders.iter().map(|(_, base)| base).sum();

            // mint required amount to make this extrinsic self-sufficient
            assets::Pallet::<T>::mint_unchecked(&book_id.base, &bids_owner, sell_base_locked)?;
            assets::Pallet::<T>::mint_unchecked(&book_id.quote, &asks_owner, buy_quote_locked)?;

            // place buy orders
            Self::place_multiple_orders(
                data,
                &mut order_book,
                bids_owner.clone(),
                PriceVariant::Buy,
                buy_orders.into_iter(),
                now,
                T::OrderBookOrderLifespan::get(),
                current_block,
            )?;

            // place sell orders
            Self::place_multiple_orders(
                data,
                &mut order_book,
                asks_owner.clone(),
                PriceVariant::Sell,
                sell_orders.into_iter(),
                now,
                T::OrderBookOrderLifespan::get(),
                current_block,
            )?;
            Ok(())
        }

        fn place_multiple_orders(
            data: &mut CacheDataLayer<T>,
            book: &mut OrderBook<T>,
            owner: T::AccountId,
            side: PriceVariant,
            orders: impl Iterator<Item = (Balance, Balance)>,
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
                let (market_input, deal_input) =
                    book.place_limit_order::<order_book::Pallet<T>, order_book::Pallet<T>, order_book::Pallet<T>>(order, data)?;
                if let (None, None) = (market_input, deal_input) {
                    // should never happen
                    return Err(Error::<T>::OrderBookFailedToPlaceOrders.into());
                }
            }
            Ok(())
        }
    }
}
