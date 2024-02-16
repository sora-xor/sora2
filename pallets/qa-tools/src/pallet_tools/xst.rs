use crate::pallet_tools;
use crate::{Config, Error};
use codec::{Decode, Encode};
use common::fixnum::ops::CheckedSub;
use common::prelude::{BalanceUnit, QuoteAmount};
use common::{fixed, AssetName, AssetSymbol, Balance, Fixed, Oracle, PriceVariant};
use frame_support::dispatch::{
    DispatchError, DispatchResult, DispatchResultWithPostInfo, RawOrigin,
};
use frame_support::ensure;
use frame_support::traits::Get;
use frame_support::weights::Weight;
use pallet_tools::price_tools::AssetPrices;
use sp_std::fmt::Debug;
use sp_std::vec;
use sp_std::vec::Vec;

/// Prices with 10^18 precision. Amount of the asset per 1 XOR. The same format as used
/// in price tools.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct XSTBaseXorPrices {
    pub synthetic_base: AssetPrices,
    pub reference: AssetPrices,
}

/// Price initialization parameters of `xst`'s synthetic base asset (in terms of reference asset)
#[derive(Clone, PartialEq, Eq, Encode, Decode, scale_info::TypeInfo, Debug)]
pub struct XSTBaseInput {
    pub reference_per_synthetic_base_buy: Balance,
    pub reference_per_synthetic_base_sell: Balance,
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

/// Adapter for [`pallet_tools::price_tools::calculate_xor_prices`] to avoid confusion with
/// assets.
fn calculate_xor_prices<T: Config>(
    input_prices: XSTBaseInput,
    synthetic_base_asset_id: &T::AssetId,
    reference_asset_id: &T::AssetId,
) -> Result<XSTBaseXorPrices, DispatchError> {
    // B = reference
    // A = synthetic base
    let xor_prices = pallet_tools::price_tools::calculate_xor_prices::<T>(
        synthetic_base_asset_id,
        reference_asset_id,
        input_prices.reference_per_synthetic_base_buy,
        input_prices.reference_per_synthetic_base_sell,
    )?;
    Ok(XSTBaseXorPrices {
        synthetic_base: xor_prices.asset_a,
        reference: xor_prices.asset_b,
    })
}

/// Feed `band` pallet the price for the symbol.
///
/// Tries to remove (decay) the dynamic fee occurring from the price change.
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
    ref_per_synthetic_base: &AssetPrices,
) -> Result<u64, DispatchError> {
    // band price is `ref_per_synthetic`.
    // we need to get it from formulae in xst pallet.
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
            let ref_per_synthetic_base_sell = BalanceUnit::divisible(ref_per_synthetic_base.sell);
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
            let ref_per_synthetic_base_buy = BalanceUnit::divisible(ref_per_synthetic_base.buy);
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
    ref_per_synthetic_base: &AssetPrices,
) -> XSTSyntheticOutput<T::AssetId> {
    let ref_per_synthetic = synthetic_band_price as Balance * 10_u128.pow(9);
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
            BalanceUnit::divisible(*amount_in) * BalanceUnit::divisible(ref_per_synthetic_base.sell)
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
                / BalanceUnit::divisible(ref_per_synthetic_base.sell)
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
                / BalanceUnit::divisible(ref_per_synthetic_base.buy)
        }
        (
            XSTSyntheticQuoteDirection::SyntheticToSyntheticBase,
            QuoteAmount::WithDesiredOutput {
                desired_amount_out: amount_out,
            },
        ) => {
            // amount_in = amount_out * ref_per_synthetic_base (buy) / ref_per_synthetic
            BalanceUnit::divisible(*amount_out) * BalanceUnit::divisible(ref_per_synthetic_base.buy)
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

pub(crate) fn xst_base_assets<T: Config>(input: XSTBaseInput) -> DispatchResult {
    let synthetic_base_asset_id = <T as xst::Config>::GetSyntheticBaseAssetId::get();
    let reference_asset_id = xst::ReferenceAssetId::<T>::get();

    let xor_prices =
        calculate_xor_prices::<T>(input, &synthetic_base_asset_id, &reference_asset_id)?;
    ensure!(
        xor_prices.synthetic_base.buy >= xor_prices.synthetic_base.sell
            && xor_prices.reference.buy >= xor_prices.reference.sell,
        Error::<T>::BuyLessThanSell
    );
    pallet_tools::price_tools::set_price::<T>(
        &synthetic_base_asset_id,
        xor_prices.synthetic_base.buy,
        PriceVariant::Buy,
    )?;
    pallet_tools::price_tools::set_price::<T>(
        &synthetic_base_asset_id,
        xor_prices.synthetic_base.sell,
        PriceVariant::Sell,
    )?;
    Ok(())
}

fn xst_single_synthetic<T: Config>(
    input: XSTSyntheticInput<T::AssetId, <T as Config>::Symbol>,
    relayer: T::AccountId,
) -> Result<XSTSyntheticOutput<T::AssetId>, DispatchError> {
    let synthetic_base_asset_id = <T as xst::Config>::GetSyntheticBaseAssetId::get();
    let ref_per_synthetic_base = AssetPrices {
        buy: xst::Pallet::<T>::reference_price(&synthetic_base_asset_id, PriceVariant::Buy)
            .unwrap(),
        sell: xst::Pallet::<T>::reference_price(&synthetic_base_asset_id, PriceVariant::Sell)
            .unwrap(),
    };
    let band_price = calculate_band_price::<T>(&input.expected_quote, &ref_per_synthetic_base)?;
    let resulting_quote = calculate_actual_quote::<T>(
        input.asset_id,
        input.expected_quote,
        band_price,
        &ref_per_synthetic_base,
    );
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

pub(crate) fn xst_synthetics<T: Config>(
    inputs: Vec<XSTSyntheticInput<T::AssetId, <T as Config>::Symbol>>,
    relayer: T::AccountId,
) -> Result<Vec<XSTSyntheticOutput<T::AssetId>>, DispatchError> {
    if !inputs.is_empty() {
        if !band::Pallet::<T>::trusted_relayers().is_some_and(|t| t.contains(&relayer)) {
            band::Pallet::<T>::add_relayers(RawOrigin::Root.into(), vec![relayer.clone()])
                .map_err(|e| e.error)?;
        };
        if !oracle_proxy::Pallet::<T>::enabled_oracles().contains(&Oracle::BandChainFeed) {
            oracle_proxy::Pallet::<T>::enable_oracle(RawOrigin::Root.into(), Oracle::BandChainFeed)
                .map_err(|e| e.error)?;
        }
    }
    let mut synthetic_init_results = vec![];
    for synthetic in inputs {
        synthetic_init_results.push(xst_single_synthetic::<T>(synthetic, relayer.clone())?)
    }
    Ok(synthetic_init_results)
}
