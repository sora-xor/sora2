#![cfg_attr(not(feature = "std"), no_std)]

#[macro_use]
extern crate alloc;

use codec::{Decode, Encode};
use common::{
    fixed, linspace, prelude::SwapAmount, prelude::SwapOutcome, prelude::SwapVariant, FilterMode,
    Fixed, IntervalEndpoints, LiquidityRegistry, LiquiditySource, LiquiditySourceFilter,
    LiquiditySourceId, LiquiditySourceType,
};
use frame_support::{
    decl_error, decl_event, decl_module, dispatch::DispatchResult, ensure, traits::Get,
    weights::Weight, RuntimeDebug,
};
use frame_system::ensure_signed;
use sp_arithmetic::{traits::Bounded, FixedPointNumber};
use sp_runtime::DispatchError;
use sp_std::vec::Vec;

mod weights;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

pub mod algo;

/// Output of the aggregated LiquidityProxy::quote() price.
#[derive(Encode, Decode, Clone, RuntimeDebug, PartialEq, Eq, PartialOrd, Ord)]
pub struct AggregatedSwapOutcome<LiquiditySourceType, AmountType> {
    /// A distribution of shares each liquidity sources gets to swap in the entire trade
    pub distribution: Vec<(LiquiditySourceType, Fixed)>,
    /// The best possible output/input amount for a given trade and a set of liquidity sources
    pub amount: AmountType,
    /// Total fee amount, nominated in XOR
    pub fee: AmountType,
}

impl<LiquiditySourceIdType, AmountType> AggregatedSwapOutcome<LiquiditySourceIdType, AmountType> {
    pub fn new(
        distribution: Vec<(LiquiditySourceIdType, Fixed)>,
        amount: AmountType,
        fee: AmountType,
    ) -> Self {
        Self {
            distribution,
            amount,
            fee,
        }
    }
}

pub trait WeightInfo {
    fn swap(amount: SwapVariant) -> Weight;
}

pub trait Trait: common::Trait + assets::Trait {
    type Event: From<Event<Self>> + Into<<Self as frame_system::Trait>::Event>;
    type LiquidityRegistry: LiquidityRegistry<
        Self::DEXId,
        Self::AccountId,
        Self::AssetId,
        LiquiditySourceType,
        Fixed,
        DispatchError,
    >;
    type GetNumSamples: Get<usize>;

    /// Weight information for the extrinsics in this pallet.
    type WeightInfo: WeightInfo;
}

decl_event!(
    pub enum Event<T>
    where
        AccountId = <T as frame_system::Trait>::AccountId,
        AssetId = <T as assets::Trait>::AssetId,
        DEXId = <T as common::Trait>::DEXId,
    {
        /// Exchange of tokens has been performed
        /// [Caller Account, DEX Id, Input Asset Id, Output Asset Id, Input Amount, Output Amount, Fee Amount]
        Exchange(AccountId, DEXId, AssetId, AssetId, Fixed, Fixed, Fixed),
    }
);

decl_error! {
    pub enum Error for Module<T: Trait> {
        /// No route exists in a given DEX for given parameters to carry out the swap
        UnavailableExchangePath,
        /// Max fee exceeded
        MaxFeeExceeded,
        /// Fee value outside of the basis points range [0..10000]
        InvalidFeeValue,
        /// None of the sources has enough reserves to execute a trade
        InsufficientLiquidity,
    }
}

decl_module! {
    pub struct Module<T: Trait> for enum Call where origin: T::Origin {
        type Error = Error<T>;

        fn deposit_event() = default;

        /// Perform swap of tokens (input/output defined via SwapAmount direction).
        ///
        /// - `origin`: the account on whose behalf the transaction is being executed,
        /// - `dex_id`: DEX ID for which liquidity sources aggregation is being done,
        /// - `input_asset_id`: ID of the asset being sold,
        /// - `output_asset_id`: ID of the asset being bought,
        /// - `swap_amount`: the exact amount to be sold (either in input_assed_id or output_asset_id units with corresponding slippage tolerance absolute bound),
        /// - `selected_source_types`: list of selected LiquiditySource types, selection effect is determined by filter_mode,
        /// - `filter_mode`: indicate either to allow or forbid selected types only, or disable filtering.
        #[weight = <T as Trait>::WeightInfo::swap((*swap_amount).into())]
        pub fn swap(
            origin,
            dex_id: T::DEXId,
            input_asset_id: T::AssetId,
            output_asset_id: T::AssetId,
            swap_amount: SwapAmount<Fixed>,
            selected_source_types: Vec<LiquiditySourceType>,
            filter_mode: FilterMode,
        ) -> DispatchResult {
            let who = ensure_signed(origin)?;
            let outcome = Self::perform_swap(
                &who,
                &input_asset_id,
                &output_asset_id,
                swap_amount.clone(),
                LiquiditySourceFilter::with_mode(dex_id, filter_mode, selected_source_types),
            )?;
            let (input_amount, output_amount, fee_amount) = match swap_amount {
                SwapAmount::WithDesiredInput{desired_amount_in, ..} => (desired_amount_in, outcome.amount, outcome.fee),
                SwapAmount::WithDesiredOutput{desired_amount_out, ..} => (outcome.amount, desired_amount_out, outcome.fee),
            };
            Self::deposit_event(
                RawEvent::Exchange(
                    who,
                    dex_id,
                    input_asset_id,
                    output_asset_id,
                    input_amount,
                    output_amount,
                    fee_amount,
                )
            );
            Ok(())
        }
    }
}

impl<T: Trait> Module<T> {
    /// Sample a single liquidity source with a range of swap amounts to get respective prices for the exchange.
    fn sample_liquidity_source(
        liquidity_source_id: &LiquiditySourceId<T::DEXId, LiquiditySourceType>,
        input_asset_id: &T::AssetId,
        output_asset_id: &T::AssetId,
        amount: SwapAmount<Fixed>,
    ) -> Vec<SwapOutcome<Fixed>> {
        let num_samples = T::GetNumSamples::get();
        match amount {
            SwapAmount::WithDesiredInput {
                desired_amount_in: amount,
                min_amount_out: min_out,
            } => {
                let inputs: Vec<_> =
                    linspace(fixed!(0), amount, num_samples, IntervalEndpoints::Right)
                        .iter()
                        .zip(
                            linspace(fixed!(0), min_out, num_samples, IntervalEndpoints::Right)
                                .iter(),
                        )
                        .map(|(x, y)| {
                            T::LiquidityRegistry::quote(
                                liquidity_source_id,
                                input_asset_id,
                                output_asset_id,
                                SwapAmount::with_desired_input(*x, *y),
                            )
                            .unwrap_or_else(|_| SwapOutcome::new(fixed!(0), fixed!(0)))
                        })
                        .collect();
                inputs
            }
            SwapAmount::WithDesiredOutput {
                desired_amount_out: amount,
                max_amount_in: max_in,
            } => {
                let outputs: Vec<_> =
                    linspace(fixed!(0), amount, num_samples, IntervalEndpoints::Right)
                        .iter()
                        .zip(
                            linspace(fixed!(0), max_in, num_samples, IntervalEndpoints::Right)
                                .iter(),
                        )
                        .map(|(x, y)| {
                            T::LiquidityRegistry::quote(
                                liquidity_source_id,
                                input_asset_id,
                                output_asset_id,
                                SwapAmount::with_desired_output(*x, *y),
                            )
                            .unwrap_or_else(|_| SwapOutcome::new(Fixed::max_value(), fixed!(0)))
                        })
                        .collect();
                outputs
            }
        }
    }

    /// Performs a swap given a numbebr of liquidity sources and a distribuition of the swap amount across the sources.
    pub fn perform_swap(
        origin: &T::AccountId,
        input_asset_id: &T::AssetId,
        output_asset_id: &T::AssetId,
        amount: SwapAmount<Fixed>,
        filter: LiquiditySourceFilter<T::DEXId, LiquiditySourceType>,
    ) -> Result<SwapOutcome<Fixed>, DispatchError> {
        let res = Self::quote(
            input_asset_id,
            output_asset_id,
            amount.clone(),
            filter.clone(),
        )?
        .distribution
        .into_iter()
        .filter(|(_src, share)| *share > fixed!(0))
        .map(|(src, share)| {
            T::LiquidityRegistry::exchange(
                origin,
                origin,
                &src,
                input_asset_id,
                output_asset_id,
                amount.clone() * share,
            )
        })
        .collect::<Result<Vec<SwapOutcome<Fixed>>, DispatchError>>()?;

        Ok(res
            .into_iter()
            .fold(SwapOutcome::new(fixed!(0), fixed!(0)), |acc, x| {
                SwapOutcome::new(acc.amount + x.amount, acc.fee + x.fee)
            }))
    }

    /// Computes the optimal distribution across available liquidity sources to exectute the requested trade
    /// given the input and output assets, the trade amount and a liquidity sources filter.
    ///
    /// - 'input_asset_id' - ID of the asset to sell,
    /// - 'output_asset_id' - ID of the asset to buy,
    /// - 'amount' - the amount with "direction" (sell or buy) together with the maximum price impact (slippage),
    /// - 'filter' - a filter composed of a list of liquidity sources IDs to accept or ban for this trade.
    ///
    pub fn quote(
        input_asset_id: &T::AssetId,
        output_asset_id: &T::AssetId,
        amount: SwapAmount<Fixed>,
        filter: LiquiditySourceFilter<T::DEXId, LiquiditySourceType>,
    ) -> Result<
        AggregatedSwapOutcome<LiquiditySourceId<T::DEXId, LiquiditySourceType>, Fixed>,
        DispatchError,
    > {
        let num_samples = T::GetNumSamples::get();
        let sources = T::LiquidityRegistry::list_liquidity_sources(
            input_asset_id,
            output_asset_id,
            filter.clone(),
        )?;

        ensure!(!sources.is_empty(), Error::<T>::UnavailableExchangePath);

        let sampling_outcome = sources
            .iter()
            .map(|src| {
                Self::sample_liquidity_source(src, input_asset_id, output_asset_id, amount.clone())
            })
            .collect::<Vec<_>>();

        let (sample_data, sample_fees): (Vec<_>, Vec<_>) = sampling_outcome
            .into_iter()
            .map(|row| {
                let data = row.iter().map(|x| x.amount).collect::<Vec<_>>();
                let fees = row.iter().map(|x| x.fee).collect::<Vec<_>>();
                (data, fees)
            })
            .unzip();

        let (distr, best) = match amount {
            SwapAmount::WithDesiredInput { .. } => algo::find_distribution(sample_data, false),
            _ => algo::find_distribution(sample_data, true),
        };

        match amount {
            SwapAmount::WithDesiredInput {
                desired_amount_in: _,
                min_amount_out: threshold,
            } => {
                ensure!(
                    best > fixed!(0) && best >= threshold,
                    Error::<T>::UnavailableExchangePath
                );
            }
            SwapAmount::WithDesiredOutput {
                desired_amount_out: _,
                max_amount_in: threshold,
            } => {
                ensure!(
                    best <= threshold && best < Fixed::max_value(),
                    Error::<T>::UnavailableExchangePath
                );
            }
        }

        let total_fee = (0..distr.len()).fold(fixed!(0), |acc, i| {
            let idx = match (Fixed::from(num_samples as u128) * distr[i]).saturating_mul_int(1u32) {
                0 => 0,
                k => k - 1,
            };
            acc + *sample_fees[i].get(idx as usize).unwrap_or(&fixed!(0))
        });

        Ok(AggregatedSwapOutcome::<
            LiquiditySourceId<T::DEXId, LiquiditySourceType>,
            Fixed,
        >::new(
            sources
                .into_iter()
                .zip(distr.into_iter())
                .collect::<Vec<_>>(),
            best,
            total_fee,
        ))
    }
}
