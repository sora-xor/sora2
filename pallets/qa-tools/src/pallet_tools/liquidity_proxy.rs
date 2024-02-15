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

pub mod source_initialization {
    use crate::pallet_tools::order_book::settings;
    use crate::{Config, Error};
    use assets::AssetIdOf;
    use codec::{Decode, Encode};
    use common::fixnum::ops::{CheckedSub, Zero};
    use common::prelude::{BalanceUnit, FixedWrapper, QuoteAmount};
    use common::{
        balance, fixed, AssetInfoProvider, AssetName, AssetSymbol, Balance, DEXInfo, DexIdOf,
        DexInfoProvider, Fixed, Oracle, PriceToolsPallet, PriceVariant, TradingPair,
        TradingPairSourceManager, XOR,
    };
    use frame_support::dispatch::{
        DispatchError, DispatchResult, DispatchResultWithPostInfo, RawOrigin,
    };
    use frame_support::ensure;
    use frame_support::traits::Get;
    use frame_support::weights::Weight;
    use frame_system::pallet_prelude::BlockNumberFor;
    use order_book::{MomentOf, OrderBookId};
    use sp_arithmetic::traits::CheckedMul;
    use sp_std::fmt::Debug;
    use sp_std::vec;
    use sp_std::vec::Vec;

    // todo: group by source or domain or smth
    // todo: rename 'CAPS' to 'Caps'

    #[derive(Clone, PartialEq, Eq, Encode, Decode, scale_info::TypeInfo, Debug)]
    pub struct XYKPair<DEXId, AssetId> {
        pub dex_id: DEXId,
        pub asset_a: AssetId,
        pub asset_b: AssetId,
        /// Price of `asset_a` in terms of `asset_b` (how much `asset_b` is needed to buy 1 `asset_a`)
        pub price: Balance,
    }

    impl<DEXId, AssetId> XYKPair<DEXId, AssetId> {
        // `price` - Price of `asset_a` in terms of `asset_b` (how much `asset_b` is needed to buy 1
        // `asset_a`)
        pub fn new(dex_id: DEXId, asset_a: AssetId, asset_b: AssetId, price: Balance) -> Self {
            Self {
                dex_id,
                asset_a,
                asset_b,
                price,
            }
        }
    }

    /// `None` if neither of the assets is base
    fn trading_pair_from_asset_ids<T: Config>(
        dex_info: DEXInfo<AssetIdOf<T>>,
        asset_a: AssetIdOf<T>,
        asset_b: AssetIdOf<T>,
    ) -> Option<TradingPair<AssetIdOf<T>>> {
        if asset_a == dex_info.base_asset_id {
            Some(TradingPair {
                base_asset_id: asset_a,
                target_asset_id: asset_b,
            })
        } else if asset_b == dex_info.base_asset_id {
            Some(TradingPair {
                base_asset_id: asset_b,
                target_asset_id: asset_a,
            })
        } else {
            None
        }
    }

    /// Initialize xyk liquidity source for multiple asset pairs at once.
    ///
    /// ## Return
    ///
    /// Due to limited precision of fixed-point numbers, the requested price might not be precisely
    /// obtainable. Therefore, actual resulting price is returned.
    ///
    /// Note: with current implementation the prices should always be equal
    pub fn xyk<T: Config + pool_xyk::Config>(
        caller: T::AccountId,
        pairs: Vec<XYKPair<DexIdOf<T>, AssetIdOf<T>>>,
    ) -> Result<Vec<XYKPair<DexIdOf<T>, AssetIdOf<T>>>, DispatchError> {
        let mut actual_prices = pairs.clone();
        for (
            XYKPair {
                dex_id,
                asset_a,
                asset_b,
                price: expected_price,
            },
            XYKPair {
                price: actual_price,
                ..
            },
        ) in pairs.into_iter().zip(actual_prices.iter_mut())
        {
            if <T as Config>::AssetInfoProvider::is_non_divisible(&asset_a)
                || <T as Config>::AssetInfoProvider::is_non_divisible(&asset_b)
            {
                return Err(Error::<T>::AssetsMustBeDivisible.into());
            }

            let dex_info = <T as Config>::DexInfoProvider::get_dex_info(&dex_id)?;
            let trading_pair = trading_pair_from_asset_ids::<T>(dex_info, asset_a, asset_b)
                .ok_or(pool_xyk::Error::<T>::BaseAssetIsNotMatchedWithAnyAssetArguments)?;

            if !<T as Config>::TradingPairSourceManager::is_trading_pair_enabled(
                &dex_id,
                &trading_pair.base_asset_id,
                &trading_pair.target_asset_id,
            )? {
                <T as Config>::TradingPairSourceManager::register_pair(
                    dex_id,
                    trading_pair.base_asset_id,
                    trading_pair.target_asset_id,
                )?
            }

            pool_xyk::Pallet::<T>::initialize_pool(
                RawOrigin::Signed(caller.clone()).into(),
                dex_id,
                asset_a,
                asset_b,
            )
            .map_err(|e| e.error)?;

            // Some magic numbers taken from existing init code
            // https://github.com/soramitsu/sora2-api-tests/blob/f590995abbd3b191a57b988ba3c10607a89d6f89/tests/testAccount/mintTokensForPairs.test.ts#L136
            let value_a: BalanceUnit = if asset_a == XOR.into() {
                balance!(1000000).into()
            } else {
                balance!(10000).into()
            };
            let price = BalanceUnit::divisible(expected_price);
            let value_b = value_a
                .checked_mul(&price)
                .ok_or(Error::<T>::ArithmeticError)?;

            assets::Pallet::<T>::mint_unchecked(&asset_a, &caller, *value_a.balance())?;
            assets::Pallet::<T>::mint_unchecked(&asset_b, &caller, *value_b.balance())?;

            *actual_price = *(value_b / value_a).balance();
            pool_xyk::Pallet::<T>::deposit_liquidity(
                RawOrigin::Signed(caller.clone()).into(),
                dex_id,
                asset_a,
                asset_b,
                *value_a.balance(),
                *value_b.balance(),
                // no need for range when the pool is empty
                *value_a.balance(),
                *value_b.balance(),
            )
            .map_err(|e| e.error)?;
        }
        Ok(actual_prices)
    }

    /// Create multiple order books with parameters and fill them according to given parameters.
    ///
    /// Balance for placing the orders is minted automatically, trading pairs are created if needed.
    ///
    /// Parameters:
    /// - `bids_owner`: Creator of the buy orders placed on the order books,
    /// - `asks_owner`: Creator of the sell orders placed on the order books,
    /// - `settings`: Parameters for creation of the order book and placing the orders in each
    /// order book.
    pub fn order_book_create_and_fill<T: Config>(
        bids_owner: T::AccountId,
        asks_owner: T::AccountId,
        settings: Vec<(
            OrderBookId<T::AssetId, T::DEXId>,
            settings::OrderBookAttributes,
            settings::OrderBookFill<MomentOf<T>, BlockNumberFor<T>>,
        )>,
    ) -> DispatchResult {
        let creation_settings: Vec<_> = settings
            .iter()
            .map(|(id, attributes, _)| (*id, *attributes))
            .collect();
        for (order_book_id, _) in creation_settings.iter() {
            ensure!(
                !order_book::OrderBooks::<T>::contains_key(order_book_id),
                crate::Error::<T>::OrderBookAlreadyExists
            );
        }
        crate::pallet_tools::order_book::create_multiple_empty_unchecked::<T>(creation_settings)?;

        let orders_settings: Vec<_> = settings
            .into_iter()
            .map(|(id, _, fill_settings)| (id, fill_settings))
            .collect();
        crate::pallet_tools::order_book::fill_multiple_empty_unchecked::<T>(
            bids_owner,
            asks_owner,
            orders_settings,
        )?;
        Ok(())
    }

    /// Fill the order books according to given parameters.
    ///
    /// Balance for placing the orders is minted automatically.
    ///
    /// Parameters:
    /// - `bids_owner`: Creator of the buy orders placed on the order books,
    /// - `asks_owner`: Creator of the sell orders placed on the order books,
    /// - `settings`: Parameters for placing the orders in each order book.
    pub fn order_book_only_fill<T: Config>(
        bids_owner: T::AccountId,
        asks_owner: T::AccountId,
        settings: Vec<(
            OrderBookId<T::AssetId, T::DEXId>,
            settings::OrderBookFill<MomentOf<T>, BlockNumberFor<T>>,
        )>,
    ) -> DispatchResult {
        for (order_book_id, _) in settings.iter() {
            ensure!(
                order_book::OrderBooks::<T>::contains_key(order_book_id),
                crate::Error::<T>::CannotFillUnknownOrderBook
            );
        }
        crate::pallet_tools::order_book::fill_multiple_empty_unchecked::<T>(
            bids_owner, asks_owner, settings,
        )?;
        Ok(())
    }

    /// Prices with 10^18 precision. Amount of the asset per 1 XOR. The same format as used
    /// in price tools.
    #[derive(Clone, PartialEq, Eq, Encode, Decode, scale_info::TypeInfo, Debug)]
    pub struct XSTBaseXorSidePrices {
        /// Amount of synthetic base asset per XOR
        pub synthetic_base: Balance,
        /// Amount of reference asset per XOR
        pub reference: Balance,
    }

    /// Price initialization parameters of `xst`'s synthetic base asset (in terms of reference asset)
    #[derive(Clone, PartialEq, Eq, Encode, Decode, scale_info::TypeInfo, Debug)]
    pub struct XSTBaseXorPrices {
        pub buy: XSTBaseXorSidePrices,
        pub sell: XSTBaseXorSidePrices,
    }

    /// Input for setting prices for xst base assets
    #[derive(Clone, PartialEq, Eq, Encode, Decode, scale_info::TypeInfo, Debug)]
    pub struct XSTBaseSideInput {
        /// Dictates price of synthetic base asset.
        pub reference_per_synthetic_base: Balance,
        /// Dictates price of reference asset.
        /// `None` - get existing price
        pub reference_per_xor: Option<Balance>,
    }

    impl XSTBaseSideInput {
        fn should_update_reference(&self) -> bool {
            self.reference_per_xor.is_some()
        }
    }

    /// Price initialization parameters of `xst`'s synthetic base asset (in terms of reference asset)
    #[derive(Clone, PartialEq, Eq, Encode, Decode, scale_info::TypeInfo, Debug)]
    pub struct XSTBaseInput {
        pub buy: XSTBaseSideInput,
        pub sell: XSTBaseSideInput,
    }

    #[derive(Clone, PartialEq, Eq, Encode, Decode, scale_info::TypeInfo, Debug)]
    pub enum XSTSyntheticExistence<Symbol> {
        AlreadyExists,
        RegisterNewAsset {
            symbol: AssetSymbol,
            name: AssetName,
            reference_symbol: Symbol,
            fee_ratio: common::Fixed,
        },
    }

    #[derive(Clone, PartialEq, Eq, Encode, Decode, scale_info::TypeInfo, Debug)]
    pub enum XSTSyntheticQuoteDirection {
        SyntheticBaseToSynthetic,
        SyntheticToSyntheticBase,
    }

    #[derive(Clone, PartialEq, Eq, Encode, Decode, scale_info::TypeInfo, Debug)]
    pub struct XSTSyntheticQuote {
        pub direction: XSTSyntheticQuoteDirection,
        pub amount: QuoteAmount<Balance>,
        pub result: Balance,
    }

    /// Buy/sell price discrepancy is determined for all synthetics in `xst` pallet by synthetic
    /// base (XST) asset prices;
    ///
    /// We can't control it granularly for each asset, so we just deduce it from the existing
    /// pricing and price provided for the given variant
    #[derive(Clone, PartialEq, Eq, Encode, Decode, scale_info::TypeInfo, Debug)]
    pub struct XSTSyntheticInput<AssetId, Symbol> {
        pub asset_id: AssetId,
        /// Quote call with expected output.
        /// The initialization tries to set up pallets to achieve these values
        pub expected_quote: XSTSyntheticQuote,
        pub existence: XSTSyntheticExistence<Symbol>,
    }

    /// Resulting of initialization for `asset_id`.
    #[derive(Clone, PartialEq, Eq, Encode, Decode, scale_info::TypeInfo, Debug)]
    pub struct XSTSyntheticOutput<AssetId> {
        pub asset_id: AssetId,
        /// Quote call with output.
        /// Sometimes, due to fixed-point precision limitations the exact value cannot be
        /// reproduced exactly. This provides a way to get the actual result for further usage.
        pub quote_achieved: XSTSyntheticQuote,
    }

    fn set_prices_in_price_tools<T: Config>(
        asset_id: &T::AssetId,
        price: Balance,
        variant: PriceVariant,
    ) -> DispatchResult {
        let _ = price_tools::Pallet::<T>::register_asset(asset_id);

        for _ in 0..price_tools::AVG_BLOCK_SPAN {
            price_tools::Pallet::<T>::incoming_spot_price_failure(asset_id, variant);
        }
        for _ in 0..31 {
            price_tools::Pallet::<T>::incoming_spot_price(asset_id, price, variant)?;
        }
        Ok(())
    }

    /// Returns prices for setting in `price_tools`
    fn calculate_xor_prices<T: Config>(
        input_prices: XSTBaseInput,
    ) -> Result<XSTBaseXorPrices, DispatchError> {
        // To obtain xor prices, these formulae should be followed:
        //
        // B:
        // (synthetic -buy-> reference) = (synthetic -buy-> xor) * (xor -buy-> reference) =
        // = (1 / (xor -sell-> synthetic)) * (xor -buy-> reference)
        //
        // S:
        // (synthetic -sell-> reference) = (synthetic -sell-> xor) * (xor -sell-> reference) =
        // = (1 / (xor -buy-> synthetic)) * (xor -sell-> reference)

        // Get known values from the formula:
        let synthetic_buy_reference =
            BalanceUnit::divisible(input_prices.buy.reference_per_synthetic_base);
        let xor_buy_reference = match input_prices.buy.reference_per_xor {
            Some(p) => p,
            None => price_tools::Pallet::<T>::get_average_price(
                &XOR.into(),
                &xst::ReferenceAssetId::<T>::get(),
                PriceVariant::Buy,
            )
            .map_err(|_| Error::<T>::ReferenceAssetPriceNotFound)?,
        };
        let xor_buy_reference = BalanceUnit::divisible(xor_buy_reference);
        let synthetic_sell_reference =
            BalanceUnit::divisible(input_prices.sell.reference_per_synthetic_base);
        let xor_sell_reference = match input_prices.sell.reference_per_xor {
            Some(p) => p,
            None => price_tools::Pallet::<T>::get_average_price(
                &XOR.into(),
                &xst::ReferenceAssetId::<T>::get(),
                PriceVariant::Sell,
            )
            .map_err(|_| Error::<T>::ReferenceAssetPriceNotFound)?,
        };
        let xor_sell_reference = BalanceUnit::divisible(xor_sell_reference);

        // B:
        // (synthetic -buy-> reference) = (xor -buy-> reference) / (xor -sell-> synthetic)
        //
        // known:
        // (synthetic -buy-> reference), (xor -buy-> reference)
        //
        // solving for unknown:
        // (xor -sell-> synthetic) = (xor -buy-> reference) / (synthetic -buy-> reference)
        let xor_sell_synthetic = xor_buy_reference / synthetic_buy_reference;

        // S:
        // (synthetic -sell-> reference) = (xor -sell-> reference) / (xor -buy-> synthetic)
        //
        // known:
        // (synthetic -sell-> reference), (xor -sell-> reference)
        //
        // solving for unknown:
        // (xor -buy-> synthetic) = (xor -sell-> reference) / (synthetic -sell-> reference)
        let xor_buy_synthetic = xor_sell_reference / synthetic_sell_reference;
        Ok(XSTBaseXorPrices {
            buy: XSTBaseXorSidePrices {
                synthetic_base: *xor_buy_synthetic.balance(),
                reference: *xor_buy_reference.balance(),
            },
            sell: XSTBaseXorSidePrices {
                synthetic_base: *xor_sell_synthetic.balance(),
                reference: *xor_sell_reference.balance(),
            },
        })
    }

    fn relay_symbol<T: Config>(
        symbol: <T as Config>::Symbol,
        relayer: T::AccountId,
        price_band: u64,
    ) -> DispatchResultWithPostInfo {
        let symbol: <T as band::Config>::Symbol = symbol.into();
        let latest_rate = band::Pallet::<T>::rates(&symbol);
        let mut resolve_time = latest_rate.map_or(0, |rate| rate.last_updated + 1);
        let mut request_id = latest_rate.map_or(0, |rate| rate.request_id + 1);
        let mut post_info = band::Pallet::<T>::relay(
            RawOrigin::Signed(relayer.clone()).into(),
            vec![(symbol.clone(), price_band)].try_into().unwrap(),
            resolve_time,
            request_id,
        )?;
        resolve_time += 1;
        request_id += 1;
        let mut previous_fee: Fixed = fixed!(2);
        for _ in 0..30 {
            if let Some(new_rate) = band::Pallet::<T>::rates(&symbol) {
                if previous_fee.saturating_sub(new_rate.dynamic_fee) == fixed!(0) {
                    break;
                }
                previous_fee = new_rate.dynamic_fee;
                if new_rate.dynamic_fee > fixed!(0) {
                    let next_post_info = band::Pallet::<T>::relay(
                        RawOrigin::Signed(relayer.clone()).into(),
                        vec![(symbol.clone(), price_band)].try_into().unwrap(),
                        resolve_time,
                        request_id,
                    )?;
                    resolve_time += 1;
                    request_id += 1;
                    post_info.actual_weight = post_info
                        .actual_weight
                        .map(|w| {
                            w.saturating_add(next_post_info.actual_weight.unwrap_or(Weight::zero()))
                        })
                        .or(next_post_info.actual_weight);
                } else {
                    break;
                }
            }
        }
        Ok(post_info)
    }

    /// Calculate the band price needed to achieve the expected quote values (closely enough).
    fn calculate_band_price<T: Config>(
        target_quote: &XSTSyntheticQuote,
    ) -> Result<u64, DispatchError> {
        // band price is `ref_per_synthetic`.
        // we need to get it from formulae in xst pallet.
        let synthetic_base_asset_id = <T as xst::Config>::GetSyntheticBaseAssetId::get();

        let ref_per_synthetic: BalanceUnit = match (
            &target_quote.direction,
            target_quote.amount,
            target_quote.result,
        ) {
            // sell:
            // synthetic base (xst) -> synthetic (xst***)
            // synthetic base (also called main) - sell price, synthetic - no diff between buy/sell
            // (all prices in reference assets per this asset)
            (
                XSTSyntheticQuoteDirection::SyntheticBaseToSynthetic,
                QuoteAmount::WithDesiredInput {
                    desired_amount_in: amount_in,
                },
                amount_out,
            )
            | (
                XSTSyntheticQuoteDirection::SyntheticBaseToSynthetic,
                QuoteAmount::WithDesiredOutput {
                    desired_amount_out: amount_out,
                },
                amount_in,
            ) => {
                // equivalent formulae for desired input/output:
                //
                // amount_out = amount_in * ref_per_synthetic_base (sell) / ref_per_synthetic
                // amount_in = amount_out * ref_per_synthetic / ref_per_synthetic_base (sell)

                // from this,
                // ref_per_synthetic = ref_per_synthetic_base (sell) * amount_in / amount_out
                let ref_per_synthetic_base_sell =
                    BalanceUnit::divisible(xst::Pallet::<T>::reference_price(
                        &synthetic_base_asset_id,
                        PriceVariant::Sell,
                    )?);
                ref_per_synthetic_base_sell * BalanceUnit::divisible(amount_in)
                    / BalanceUnit::divisible(amount_out)
            }
            // buy
            // synthetic (xst***) -> synthetic base (xst)
            // synthetic base (also called main) - buy price, synthetic - no diff between buy/sell
            // (all prices in reference assets per this asset)
            (
                XSTSyntheticQuoteDirection::SyntheticToSyntheticBase,
                QuoteAmount::WithDesiredInput {
                    desired_amount_in: amount_in,
                },
                amount_out,
            )
            | (
                XSTSyntheticQuoteDirection::SyntheticToSyntheticBase,
                QuoteAmount::WithDesiredOutput {
                    desired_amount_out: amount_out,
                },
                amount_in,
            ) => {
                // equivalent formulae for desired input/output:
                //
                // amount_out = amount_in * ref_per_synthetic / ref_per_synthetic_base (buy)
                // amount_in = amount_out * ref_per_synthetic_base (buy) / ref_per_synthetic

                // from this,
                // ref_per_synthetic = ref_per_synthetic_base (buy) * amount_out / amount_in
                let ref_per_synthetic_base_buy = BalanceUnit::divisible(
                    xst::Pallet::<T>::reference_price(&synthetic_base_asset_id, PriceVariant::Buy)?,
                );
                ref_per_synthetic_base_buy * BalanceUnit::divisible(amount_out)
                    / BalanceUnit::divisible(amount_in)
            }
        };
        // band price
        (*ref_per_synthetic.balance() / 10u128.pow(9))
            .try_into()
            .map_err(|_| Error::<T>::ArithmeticError.into())
    }

    fn calculate_actual_quote<T: Config>(
        asset_id: T::AssetId,
        expected_quote: XSTSyntheticQuote,
        synthetic_band_price: u64,
    ) -> XSTSyntheticOutput<T::AssetId> {
        let ref_per_synthetic = synthetic_band_price as Balance * 10_u128.pow(9);
        let synthetic_base_asset_id = <T as xst::Config>::GetSyntheticBaseAssetId::get();
        // todo: pass as args
        let ref_per_synthetic_base_sell =
            xst::Pallet::<T>::reference_price(&synthetic_base_asset_id, PriceVariant::Sell)
                .unwrap();
        let ref_per_synthetic_base_buy =
            xst::Pallet::<T>::reference_price(&synthetic_base_asset_id, PriceVariant::Buy).unwrap();
        let actual_quote_result = match (&expected_quote.direction, &expected_quote.amount) {
            // sell:
            // synthetic base (xst) -> synthetic (xst***)
            // synthetic base (also called main) - sell price, synthetic - no diff between buy/sell
            // (all prices in reference assets per this asset)
            (
                XSTSyntheticQuoteDirection::SyntheticBaseToSynthetic,
                QuoteAmount::WithDesiredInput {
                    desired_amount_in: amount_in,
                },
            ) => {
                // amount_out = amount_in * ref_per_synthetic_base (sell) / ref_per_synthetic
                BalanceUnit::divisible(*amount_in)
                    * BalanceUnit::divisible(ref_per_synthetic_base_sell)
                    / BalanceUnit::divisible(ref_per_synthetic)
            }
            (
                XSTSyntheticQuoteDirection::SyntheticBaseToSynthetic,
                QuoteAmount::WithDesiredOutput {
                    desired_amount_out: amount_out,
                },
            ) => {
                // amount_in = amount_out * ref_per_synthetic / ref_per_synthetic_base (sell)
                BalanceUnit::divisible(*amount_out) * BalanceUnit::divisible(ref_per_synthetic)
                    / BalanceUnit::divisible(ref_per_synthetic_base_sell)
            }
            // buy
            // synthetic (xst***) -> synthetic base (xst)
            // synthetic base (also called main) - buy price, synthetic - no diff between buy/sell
            // (all prices in reference assets per this asset)
            (
                XSTSyntheticQuoteDirection::SyntheticToSyntheticBase,
                QuoteAmount::WithDesiredInput {
                    desired_amount_in: amount_in,
                },
            ) => {
                // amount_out = amount_in * ref_per_synthetic / ref_per_synthetic_base (buy)
                BalanceUnit::divisible(*amount_in) * BalanceUnit::divisible(ref_per_synthetic)
                    / BalanceUnit::divisible(ref_per_synthetic_base_buy)
            }
            (
                XSTSyntheticQuoteDirection::SyntheticToSyntheticBase,
                QuoteAmount::WithDesiredOutput {
                    desired_amount_out: amount_out,
                },
            ) => {
                // amount_in = amount_out * ref_per_synthetic_base (buy) / ref_per_synthetic
                BalanceUnit::divisible(*amount_out)
                    * BalanceUnit::divisible(ref_per_synthetic_base_buy)
                    / BalanceUnit::divisible(ref_per_synthetic)
            }
        };
        let actual_quote = XSTSyntheticQuote {
            result: *actual_quote_result.balance(),
            ..expected_quote
        };
        XSTSyntheticOutput {
            asset_id,
            quote_achieved: actual_quote,
        }
    }

    fn xst_base_assets<T: Config>(input: XSTBaseInput) -> DispatchResult {
        let synthetic_base_asset_id = <T as xst::Config>::GetSyntheticBaseAssetId::get();
        let reference_asset_id = xst::ReferenceAssetId::<T>::get();

        let should_update_reference_buy = input.buy.should_update_reference();
        let should_update_reference_sell = input.sell.should_update_reference();
        let xor_prices = calculate_xor_prices::<T>(input)?;
        ensure!(
            xor_prices.buy.synthetic_base >= xor_prices.sell.synthetic_base
                && xor_prices.buy.reference >= xor_prices.sell.reference,
            Error::<T>::BuyLessThanSell
        );
        set_prices_in_price_tools::<T>(
            &synthetic_base_asset_id,
            xor_prices.buy.synthetic_base,
            PriceVariant::Buy,
        )?;
        set_prices_in_price_tools::<T>(
            &synthetic_base_asset_id,
            xor_prices.sell.synthetic_base,
            PriceVariant::Sell,
        )?;
        if should_update_reference_buy {
            set_prices_in_price_tools::<T>(
                &reference_asset_id,
                xor_prices.buy.reference,
                PriceVariant::Buy,
            )?;
        }
        if should_update_reference_sell {
            set_prices_in_price_tools::<T>(
                &reference_asset_id,
                xor_prices.sell.reference,
                PriceVariant::Sell,
            )?;
        }
        Ok(())
    }

    fn xst_single_synthetic<T: Config>(
        input: XSTSyntheticInput<T::AssetId, <T as Config>::Symbol>,
        relayer: T::AccountId,
    ) -> Result<XSTSyntheticOutput<T::AssetId>, DispatchError> {
        let band_price = calculate_band_price::<T>(&input.expected_quote)?;
        let resulting_quote =
            calculate_actual_quote::<T>(input.asset_id, input.expected_quote, band_price);
        match (
            xst::Pallet::<T>::enabled_synthetics(input.asset_id),
            input.existence,
        ) {
            (Some(info), XSTSyntheticExistence::AlreadyExists) => {
                relay_symbol::<T>(info.reference_symbol.into(), relayer, band_price)
                    .map_err(|e| e.error)?;
            }
            (
                None,
                XSTSyntheticExistence::RegisterNewAsset {
                    symbol,
                    name,
                    reference_symbol,
                    fee_ratio,
                },
            ) => {
                relay_symbol::<T>(reference_symbol.clone(), relayer, band_price)
                    .map_err(|e| e.error)?;
                xst::Pallet::<T>::register_synthetic_asset(
                    RawOrigin::Root.into(),
                    symbol,
                    name,
                    reference_symbol.into(),
                    fee_ratio,
                )
                .map_err(|e| e.error)?;
            }
            (Some(_), XSTSyntheticExistence::RegisterNewAsset { .. }) => {
                return Err(Error::<T>::AssetAlreadyExists.into())
            }
            (None, XSTSyntheticExistence::AlreadyExists) => {
                return Err(Error::<T>::UnknownSynthetic.into())
            }
        }
        Ok(resulting_quote)
    }

    fn xst_synthetics<T: Config>(
        inputs: Vec<XSTSyntheticInput<T::AssetId, <T as Config>::Symbol>>,
        relayer: T::AccountId,
    ) -> Result<Vec<XSTSyntheticOutput<T::AssetId>>, DispatchError> {
        if !inputs.is_empty() {
            if !band::Pallet::<T>::trusted_relayers().is_some_and(|t| t.contains(&relayer)) {
                band::Pallet::<T>::add_relayers(RawOrigin::Root.into(), vec![relayer.clone()])
                    .map_err(|e| e.error)?;
            };
            if !oracle_proxy::Pallet::<T>::enabled_oracles().contains(&Oracle::BandChainFeed) {
                oracle_proxy::Pallet::<T>::enable_oracle(
                    RawOrigin::Root.into(),
                    Oracle::BandChainFeed,
                )
                .map_err(|e| e.error)?;
            }
        }
        let mut synthetic_init_results = vec![];
        for synthetic in inputs {
            synthetic_init_results.push(xst_single_synthetic::<T>(synthetic, relayer.clone())?)
        }
        Ok(synthetic_init_results)
    }

    /// Initialize xst liquidity source. Can both update prices of base assets and synthetics.
    ///
    /// ## Return
    ///
    /// Due to limited precision of fixed-point numbers, the requested price might not be precisely
    /// obtainable. Therefore, actual resulting price of synthetics is returned.
    ///
    /// `quote` in `xst` pallet requires swap to involve synthetic base asset, as well as
    pub fn xst<T: Config>(
        base: Option<XSTBaseInput>,
        synthetics: Vec<XSTSyntheticInput<T::AssetId, <T as Config>::Symbol>>,
        relayer: T::AccountId,
    ) -> Result<Vec<XSTSyntheticOutput<T::AssetId>>, DispatchError> {
        if let Some(base_prices) = base {
            xst_base_assets::<T>(base_prices)?;
        }
        xst_synthetics::<T>(synthetics, relayer)
    }

    #[derive(Clone, PartialEq, Eq, Encode, Decode, scale_info::TypeInfo, Debug)]
    pub struct MCBCPriceToolsPrice {
        pub buy: Option<Balance>,
        pub sell: Option<Balance>,
    }

    /// Input for initializing collateral assets except TBCD.
    #[derive(Clone, PartialEq, Eq, Encode, Decode, scale_info::TypeInfo, Debug)]
    pub struct MCBCCollateralInput<AssetId> {
        /// Collateral asset id
        pub asset: AssetId,
        /// Price of collateral in terms of reference asset. Linearly affects the exchange amounts.
        /// (if collateral costs 10x more sell output should be 10x smaller)
        pub ref_prices: MCBCPriceToolsPrice,
        /// Desired amount of collateral asset in the MCBC reserve account. Affects actual sell
        /// price according to formulae.
        pub reserves: Balance,
    }

    /// Input for initializing TBCD collateral.
    #[derive(Clone, PartialEq, Eq, Encode, Decode, scale_info::TypeInfo, Debug)]
    pub struct MCBCTBCDInput<AssetId> {
        /// Collateral asset id
        pub asset: AssetId,
        /// Price of collateral in terms of reference asset. Linearly affects the exchange amounts.
        /// (if collateral costs 10x more sell output should be 10x smaller)
        pub ref_prices: MCBCPriceToolsPrice,
        /// Desired amount of collateral asset in the MCBC reserve account. Affects actual sell
        /// price according to formulae.
        pub reserves: Balance,
        pub xor_ref_prices: MCBCPriceToolsPrice,
    }

    pub struct MCBCBaseSupply<AccountId> {
        pub base_supply_collector: AccountId,
        pub new_base_supply: Balance,
    }

    fn init_single_mcbc_collateral<T: Config>(
        input: MCBCCollateralInput<T::AssetId>,
    ) -> DispatchResult {
        // initialize price???

        // initialize reserves

        // let base_asset = T::GetBaseAssetId::get();
        // let reference_asset = multicollateral_bonding_curve_pool::Pallet::<T>::reference_asset_id();
        // let total_issuance = assets::Pallet::<T>::total_issuance(&base_asset)?;
        // todo: register TP if not exist
        // TradingPair::register(
        //     RuntimeOrigin::signed(alice()),
        //     DEXId::Polkaswap.into(),
        //     XOR,
        //     VAL,
        // )
        // .expect("Failed to register trading pair.");
        // TradingPair::register(
        //     RuntimeOrigin::signed(alice()),
        //     DEXId::Polkaswap.into(),
        //     XOR,
        //     XSTUSD,
        // )
        // .expect("Failed to register trading pair.");

        // todo: initialize pool if not already
        // MBCPool::initialize_pool_unchecked(VAL, false).expect("Failed to initialize pool.");

        // todo: register account if not present???
        // let bonding_curve_tech_account_id = TechAccountId::Pure(
        //     DEXId::Polkaswap,
        //     TechPurpose::Identifier(b"bonding_curve_tech_account_id".to_vec()),
        // );
        // Technical::register_tech_account_id(bonding_curve_tech_account_id.clone())?;
        // MBCPool::set_reserves_account_id(bonding_curve_tech_account_id.clone())?;

        // set price_tools prices if needed
        if let Some(price) = input.ref_prices.buy {
            set_prices_in_price_tools::<T>(&input.asset, price, PriceVariant::Buy)?;
        }
        if let Some(price) = input.ref_prices.sell {
            set_prices_in_price_tools::<T>(&input.asset, price, PriceVariant::Sell)?;
        }

        // todo: use traits where possible (not only here, in whole pallet)
        // let reserve_amount_expected = FixedWrapper::from(total_issuance)
        //     * multicollateral_bonding_curve_pool::Pallet::<T>::sell_function(
        //         &base_asset,
        //         &input.asset,
        //         Fixed::ZERO,
        //     )?;

        // let pool_reference_amount = reserve_amount_expected * ratio;
        // let pool_reference_amount = pool_reference_amount
        //     .try_into_balance()
        //     .map_err(|_| Error::<T>::ArithmeticError)?;
        // let pool_val_amount = <T as Config>::LiquidityProxy::quote(
        //     DEXId::Polkaswap.into(),
        //     &reference_asset,
        //     &input.asset,
        //     QuoteAmount::with_desired_input(pool_reference_amount),
        //     LiquiditySourceFilter::empty(DEXId::Polkaswap.into()),
        //     true,
        // )?;

        // let reserves_account =
        //     multicollateral_bonding_curve_pool::Pallet::<T>::reserves_account_id();
        // technical::Pallet::<T>::mint(&input.asset, &reserves_account, pool_val_amount.amount)?;

        Ok(())
    }

    fn init_tbcd_mcbc_collateral<T: Config>(input: MCBCTBCDInput<T::AssetId>) -> DispatchResult {
        // handle xor ref price
        // input.xor_ref_prices

        init_single_mcbc_collateral::<T>(MCBCCollateralInput {
            asset: input.asset,
            ref_prices: input.ref_prices,
            reserves: input.reserves,
        })
    }

    fn init_mcbc_base_supply<T: Config>(input: MCBCBaseSupply<T::AccountId>) -> DispatchResult {
        let base_asset_id = &T::GetBaseAssetId::get();
        let current_base_supply: FixedWrapper =
            assets::Pallet::<T>::total_issuance(base_asset_id)?.into();
        let supply_delta = input.new_base_supply - current_base_supply;
        let supply_delta = supply_delta
            .get()
            .map_err(|_| Error::<T>::ArithmeticError)?
            .into_bits();

        // realistically the error should never be triggered
        let owner =
            assets::Pallet::<T>::asset_owner(&base_asset_id).ok_or(Error::<T>::UnknownMCBCAsset)?;
        if supply_delta > 0 {
            let mint_amount = supply_delta
                .try_into()
                .map_err(|_| Error::<T>::ArithmeticError)?;
            assets::Pallet::<T>::mint_to(
                base_asset_id,
                &owner,
                &input.base_supply_collector,
                mint_amount,
            )?;
        } else if supply_delta < 0 {
            let burn_amount = supply_delta
                .abs()
                .try_into()
                .map_err(|_| Error::<T>::ArithmeticError)?;
            assets::Pallet::<T>::burn_from(
                base_asset_id,
                &owner,
                &input.base_supply_collector,
                burn_amount,
            )?;
        }
        Ok(())
    }

    pub fn mcbc<T: Config>(
        base_supply: Option<MCBCBaseSupply<T::AccountId>>,
        other_collaterals: Vec<MCBCCollateralInput<T::AssetId>>,
        tbcd_collateral: Option<MCBCTBCDInput<T::AssetId>>,
    ) -> DispatchResult {
        if let Some(base_supply) = base_supply {
            init_mcbc_base_supply::<T>(base_supply)?;
        }

        for collateral_input in other_collaterals {
            init_single_mcbc_collateral::<T>(collateral_input)?;
        }
        if let Some(tbcd_collateral) = tbcd_collateral {
            init_tbcd_mcbc_collateral::<T>(tbcd_collateral)?;
        }
        Ok(())
    }
}
