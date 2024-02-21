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

use codec::{Decode, Encode};
use common::prelude::QuoteAmount;
use frame_support::RuntimeDebug;
use sp_std::vec::Vec;

#[cfg(feature = "wip")] // ALT
use {
    common::prelude::{SwapChunk, SwapVariant},
    common::{fixed, Balance, DiscreteQuotation},
    itertools::Itertools,
    sp_runtime::traits::Zero,
    sp_std::collections::btree_map::BTreeMap,
};

/// Info with input & output amounts for liquidity source
#[cfg(feature = "wip")] // ALT
type SwapInfo<LiquiditySourceType, AmountType> =
    BTreeMap<LiquiditySourceType, (AmountType, AmountType)>;

/// Output of the aggregated LiquidityProxy::quote() price.
#[derive(
    Encode, Decode, Clone, RuntimeDebug, PartialEq, Eq, PartialOrd, Ord, scale_info::TypeInfo,
)]
pub struct AggregatedSwapOutcome<LiquiditySourceType, AmountType> {
    /// A distribution of amounts each liquidity sources gets to swap in the entire trade
    pub distribution: Vec<(LiquiditySourceType, QuoteAmount<AmountType>)>,
    /// The best possible output/input amount for a given trade and a set of liquidity sources
    pub amount: AmountType,
    /// Total fee amount, nominated in XOR
    pub fee: AmountType,
}

impl<LiquiditySourceIdType, AmountType> AggregatedSwapOutcome<LiquiditySourceIdType, AmountType> {
    pub fn new(
        distribution: Vec<(LiquiditySourceIdType, QuoteAmount<AmountType>)>,
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

/// Aggregates the liquidity from the provided liquidity sources.
/// Liquidity sources provide discretized liquidity curve by chunks and then Liquidity Aggregator selects the best chunks from different sources to gain the best swap amount.
#[cfg(feature = "wip")] // ALT
#[derive(Clone)]
pub struct LiquidityAggregator<LiquiditySourceType> {
    liquidity_quotations: BTreeMap<LiquiditySourceType, DiscreteQuotation<Balance>>,
    variant: SwapVariant,
}

#[cfg(feature = "wip")] // ALT
impl<LiquiditySourceType> LiquidityAggregator<LiquiditySourceType>
where
    LiquiditySourceType: Clone + Ord,
{
    pub fn new(variant: SwapVariant) -> Self {
        Self {
            liquidity_quotations: BTreeMap::new(),
            variant,
        }
    }

    pub fn add_source(
        &mut self,
        source: LiquiditySourceType,
        discrete_quotation: DiscreteQuotation<Balance>,
    ) {
        self.liquidity_quotations.insert(source, discrete_quotation);
    }

    pub fn aggregate_swap_outcome(
        mut self,
        amount: Balance,
    ) -> Option<(
        SwapInfo<LiquiditySourceType, Balance>,
        AggregatedSwapOutcome<LiquiditySourceType, Balance>,
    )> {
        if self.liquidity_quotations.is_empty() {
            return None;
        }

        let mut remaining_amount = amount;
        let mut locked_sources = Vec::new();
        let mut selected = BTreeMap::new();

        while remaining_amount > Balance::zero() {
            let candidates = self.find_best_price_candidates(&locked_sources);

            let mut source = candidates.first()?;

            // if there are several candidates with the same best price,
            // then we need to select the source that already been selected
            for candidate in candidates.iter() {
                if selected.keys().contains(candidate) {
                    source = candidate;
                    break;
                }
            }

            let discrete_quotation = self.liquidity_quotations.get_mut(source)?;
            let mut chunk = discrete_quotation.chunks.pop_front()?;

            // modify chunk if needed
            let remaining_delta = match self.variant {
                SwapVariant::WithDesiredInput => {
                    if remaining_amount < chunk.input {
                        chunk = chunk.rescale_by_input(remaining_amount)?;
                    }

                    let (input, remainder) = discrete_quotation.limits.align_max(chunk.input);
                    if !remainder.is_zero() {
                        // max amount exceeded
                        chunk = chunk.rescale_by_input(input)?;
                        locked_sources.push(source.clone());
                    }

                    let (input, remainder) = discrete_quotation.limits.align_precision(chunk.input);
                    if !remainder.is_zero() {
                        chunk = chunk.rescale_by_input(input)?;
                    }

                    chunk.input
                }
                SwapVariant::WithDesiredOutput => {
                    if remaining_amount < chunk.output {
                        chunk = chunk.rescale_by_output(remaining_amount)?;
                    }

                    let (output, remainder) = discrete_quotation.limits.align_max(chunk.output);
                    if !remainder.is_zero() {
                        // max amount exceeded
                        chunk = chunk.rescale_by_output(output)?;
                        locked_sources.push(source.clone());
                    }

                    let (output, remainder) =
                        discrete_quotation.limits.align_precision(chunk.output);
                    if !remainder.is_zero() {
                        chunk = chunk.rescale_by_output(output)?;
                    }

                    chunk.output
                }
            };

            selected
                .entry(source.clone())
                .and_modify(|total: &mut SwapChunk<Balance>| {
                    *total = total.saturating_add(chunk);
                })
                .or_insert(chunk);
            remaining_amount = remaining_amount.checked_sub(remaining_delta)?;

            if remaining_amount.is_zero() {
                let mut to_delete = Vec::new();
                for (source, total) in &mut selected {
                    match self.variant {
                        SwapVariant::WithDesiredInput => {
                            let (input, remainder) = self
                                .liquidity_quotations
                                .get(source)?
                                .limits
                                .align(total.input);
                            if !remainder.is_zero() {
                                remaining_amount = remaining_amount.checked_add(remainder)?;

                                if input.is_zero() {
                                    // liquidity is not enough even for the min amount
                                    to_delete.push(source.clone());
                                    locked_sources.push(source.clone());
                                } else {
                                    *total = total.rescale_by_input(input)?;
                                }
                            }
                        }
                        SwapVariant::WithDesiredOutput => {
                            let (output, remainder) = self
                                .liquidity_quotations
                                .get(source)?
                                .limits
                                .align(total.output);
                            if !remainder.is_zero() {
                                remaining_amount = remaining_amount.checked_add(remainder)?;

                                if output.is_zero() {
                                    // liquidity is not enough even for the min amount
                                    to_delete.push(source.clone());
                                    locked_sources.push(source.clone());
                                } else {
                                    *total = total.rescale_by_output(output)?;
                                }
                            }
                        }
                    }
                }
                selected.retain(|source, _| !to_delete.contains(source));
            }
        }

        let mut distribution = Vec::new();
        let mut swap_info: SwapInfo<LiquiditySourceType, Balance> = SwapInfo::new();
        let mut result_amount = Balance::zero();
        let mut fee = Balance::zero();

        for (source, total) in &selected {
            swap_info.insert(source.clone(), (total.input, total.output));

            let (desired_part, result_part) = match self.variant {
                SwapVariant::WithDesiredInput => (total.input, total.output),
                SwapVariant::WithDesiredOutput => (total.output, total.input),
            };
            distribution.push((
                source.clone(),
                QuoteAmount::with_variant(self.variant, desired_part),
            ));
            result_amount = result_amount.checked_add(result_part)?;
            fee = fee.checked_add(total.fee)?;
        }

        Some((
            swap_info,
            AggregatedSwapOutcome {
                distribution,
                amount: result_amount,
                fee,
            },
        ))
    }

    /// Find liquidity sources where the top chunk has the best price.
    fn find_best_price_candidates(
        &self,
        locked: &Vec<LiquiditySourceType>,
    ) -> Vec<LiquiditySourceType> {
        let mut candidates = Vec::new();
        let mut max = fixed!(0);
        for (source, discrete_quotation) in self.liquidity_quotations.iter() {
            // skip the locked source
            if locked.contains(source) {
                continue;
            }

            let Some(front) = discrete_quotation.chunks.front() else {
                continue;
            };
            let Some(price) = front.price() else {
                continue;
            };

            if price == max {
                candidates.push(source.clone())
            }

            if price > max {
                candidates.clear();
                max = price;
                candidates.push(source.clone());
            }
        }
        candidates
    }
}

#[cfg(feature = "wip")] // ALT
#[cfg(test)]
mod tests {
    use crate::liquidity_aggregator::*;
    use common::prelude::{QuoteAmount, SwapVariant};
    use common::{balance, DiscreteQuotation, LiquiditySourceType, SwapChunk, SwapLimits};
    use sp_std::collections::vec_deque::VecDeque;

    fn get_liquidity_aggregator_with_desired_input_and_equal_chunks(
    ) -> LiquidityAggregator<LiquiditySourceType> {
        let mut aggregator = LiquidityAggregator::new(SwapVariant::WithDesiredInput);
        aggregator.add_source(
            LiquiditySourceType::XYKPool,
            DiscreteQuotation {
                chunks: VecDeque::from([
                    SwapChunk::new(balance!(10), balance!(100), balance!(1)),
                    SwapChunk::new(balance!(10), balance!(90), balance!(0.9)),
                    SwapChunk::new(balance!(10), balance!(80), balance!(0.8)),
                    SwapChunk::new(balance!(10), balance!(70), balance!(0.7)),
                    SwapChunk::new(balance!(10), balance!(60), balance!(0.6)),
                ]),
                limits: Default::default(),
            },
        );

        aggregator.add_source(
            LiquiditySourceType::XSTPool,
            DiscreteQuotation {
                chunks: VecDeque::from([
                    SwapChunk::new(balance!(10), balance!(85), balance!(0.85)),
                    SwapChunk::new(balance!(10), balance!(85), balance!(0.85)),
                    SwapChunk::new(balance!(10), balance!(85), balance!(0.85)),
                    SwapChunk::new(balance!(10), balance!(85), balance!(0.85)),
                    SwapChunk::new(balance!(10), balance!(85), balance!(0.85)),
                ]),
                limits: SwapLimits::new(None, Some(balance!(1000000)), None),
            },
        );

        aggregator.add_source(
            LiquiditySourceType::OrderBook,
            DiscreteQuotation {
                chunks: VecDeque::from([
                    SwapChunk::new(balance!(10), balance!(120), balance!(0)),
                    SwapChunk::new(balance!(10), balance!(100), balance!(0)),
                    SwapChunk::new(balance!(10), balance!(80), balance!(0)),
                ]),
                limits: SwapLimits::new(
                    Some(balance!(1)),
                    Some(balance!(1000)),
                    Some(balance!(0.00001)),
                ),
            },
        );

        aggregator
    }

    fn get_liquidity_aggregator_with_desired_output_and_equal_chunks(
    ) -> LiquidityAggregator<LiquiditySourceType> {
        let mut aggregator = LiquidityAggregator::new(SwapVariant::WithDesiredOutput);

        aggregator.add_source(
            LiquiditySourceType::XYKPool,
            DiscreteQuotation {
                chunks: VecDeque::from([
                    SwapChunk::new(balance!(10), balance!(100), balance!(1)),
                    SwapChunk::new(balance!(11), balance!(100), balance!(1)),
                    SwapChunk::new(balance!(12), balance!(100), balance!(1)),
                    SwapChunk::new(balance!(13), balance!(100), balance!(1)),
                    SwapChunk::new(balance!(14), balance!(100), balance!(1)),
                ]),
                limits: Default::default(),
            },
        );

        aggregator.add_source(
            LiquiditySourceType::XSTPool,
            DiscreteQuotation {
                chunks: VecDeque::from([
                    SwapChunk::new(balance!(12.5), balance!(100), balance!(1)),
                    SwapChunk::new(balance!(12.5), balance!(100), balance!(1)),
                    SwapChunk::new(balance!(12.5), balance!(100), balance!(1)),
                    SwapChunk::new(balance!(12.5), balance!(100), balance!(1)),
                    SwapChunk::new(balance!(12.5), balance!(100), balance!(1)),
                ]),
                limits: SwapLimits::new(None, Some(balance!(1000000)), None),
            },
        );

        aggregator.add_source(
            LiquiditySourceType::OrderBook,
            DiscreteQuotation {
                chunks: VecDeque::from([
                    SwapChunk::new(balance!(8), balance!(100), balance!(0)),
                    SwapChunk::new(balance!(10), balance!(100), balance!(0)),
                    SwapChunk::new(balance!(13), balance!(100), balance!(0)),
                ]),
                limits: SwapLimits::new(
                    Some(balance!(1)),
                    Some(balance!(1000)),
                    Some(balance!(0.00001)),
                ),
            },
        );

        aggregator
    }

    fn get_liquidity_aggregator_with_desired_input_and_different_chunks(
    ) -> LiquidityAggregator<LiquiditySourceType> {
        let mut aggregator = LiquidityAggregator::new(SwapVariant::WithDesiredInput);
        aggregator.add_source(
            LiquiditySourceType::XYKPool,
            DiscreteQuotation {
                chunks: VecDeque::from([
                    SwapChunk::new(balance!(10), balance!(100), balance!(1)),
                    SwapChunk::new(balance!(12), balance!(108), balance!(1.08)),
                    SwapChunk::new(balance!(14), balance!(112), balance!(1.12)),
                    SwapChunk::new(balance!(16), balance!(112), balance!(1.12)),
                    SwapChunk::new(balance!(18), balance!(108), balance!(1.08)),
                ]),
                limits: Default::default(),
            },
        );

        aggregator.add_source(
            LiquiditySourceType::XSTPool,
            DiscreteQuotation {
                chunks: VecDeque::from([
                    SwapChunk::new(balance!(10), balance!(85), balance!(0.85)),
                    SwapChunk::new(balance!(11), balance!(93.5), balance!(0.935)),
                    SwapChunk::new(balance!(12), balance!(102), balance!(1.02)),
                    SwapChunk::new(balance!(13), balance!(110.5), balance!(1.105)),
                    SwapChunk::new(balance!(14), balance!(119), balance!(1.19)),
                ]),
                limits: SwapLimits::new(None, Some(balance!(1000000)), None),
            },
        );

        aggregator.add_source(
            LiquiditySourceType::OrderBook,
            DiscreteQuotation {
                chunks: VecDeque::from([
                    SwapChunk::new(balance!(12), balance!(144), balance!(0)),
                    SwapChunk::new(balance!(10), balance!(100), balance!(0)),
                    SwapChunk::new(balance!(14), balance!(112), balance!(0)),
                ]),
                limits: SwapLimits::new(
                    Some(balance!(1)),
                    Some(balance!(1000)),
                    Some(balance!(0.00001)),
                ),
            },
        );

        aggregator
    }

    fn get_liquidity_aggregator_with_desired_output_and_different_chunks(
    ) -> LiquidityAggregator<LiquiditySourceType> {
        let mut aggregator = LiquidityAggregator::new(SwapVariant::WithDesiredOutput);

        aggregator.add_source(
            LiquiditySourceType::XYKPool,
            DiscreteQuotation {
                chunks: VecDeque::from([
                    SwapChunk::new(balance!(10), balance!(100), balance!(0)),
                    SwapChunk::new(balance!(5.5), balance!(50), balance!(0)),
                    SwapChunk::new(balance!(3), balance!(25), balance!(0)),
                    SwapChunk::new(balance!(26), balance!(200), balance!(0)),
                    SwapChunk::new(balance!(7), balance!(50), balance!(0)),
                ]),
                limits: Default::default(),
            },
        );

        aggregator.add_source(
            LiquiditySourceType::XSTPool,
            DiscreteQuotation {
                chunks: VecDeque::from([
                    SwapChunk::new(balance!(12.5), balance!(100), balance!(0)),
                    SwapChunk::new(balance!(10), balance!(80), balance!(0)),
                    SwapChunk::new(balance!(9), balance!(72), balance!(0)),
                    SwapChunk::new(balance!(8), balance!(64), balance!(0)),
                    SwapChunk::new(balance!(7), balance!(56), balance!(0)),
                ]),
                limits: SwapLimits::new(None, Some(balance!(1000000)), None),
            },
        );

        aggregator.add_source(
            LiquiditySourceType::OrderBook,
            DiscreteQuotation {
                chunks: VecDeque::from([
                    SwapChunk::new(balance!(8), balance!(100), balance!(0)),
                    SwapChunk::new(balance!(9), balance!(90), balance!(0)),
                    SwapChunk::new(balance!(13), balance!(100), balance!(0)),
                ]),
                limits: SwapLimits::new(
                    Some(balance!(1)),
                    Some(balance!(1000)),
                    Some(balance!(0.00001)),
                ),
            },
        );

        aggregator
    }

    #[test]
    fn check_find_best_price_candidates_with_desired_input_and_equal_chunks() {
        let mut aggregator = get_liquidity_aggregator_with_desired_input_and_equal_chunks();

        let candidates = aggregator.find_best_price_candidates(&Vec::new());
        assert_eq!(candidates, vec![LiquiditySourceType::OrderBook]);

        // remove order book chunk 1
        aggregator
            .liquidity_quotations
            .get_mut(&LiquiditySourceType::OrderBook)
            .unwrap()
            .chunks
            .pop_front();

        let candidates = aggregator.find_best_price_candidates(&Vec::new());
        assert_eq!(
            candidates,
            vec![LiquiditySourceType::XYKPool, LiquiditySourceType::OrderBook]
        );

        // remove order book chunk 2
        aggregator
            .liquidity_quotations
            .get_mut(&LiquiditySourceType::OrderBook)
            .unwrap()
            .chunks
            .pop_front();

        let candidates = aggregator.find_best_price_candidates(&Vec::new());
        assert_eq!(candidates, vec![LiquiditySourceType::XYKPool]);

        // remove xyk pool chunk 1
        aggregator
            .liquidity_quotations
            .get_mut(&LiquiditySourceType::XYKPool)
            .unwrap()
            .chunks
            .pop_front();

        let candidates = aggregator.find_best_price_candidates(&Vec::new());
        assert_eq!(candidates, vec![LiquiditySourceType::XYKPool]);

        // remove xyk pool chunk 2
        aggregator
            .liquidity_quotations
            .get_mut(&LiquiditySourceType::XYKPool)
            .unwrap()
            .chunks
            .pop_front();

        let candidates = aggregator.find_best_price_candidates(&Vec::new());
        assert_eq!(candidates, vec![LiquiditySourceType::XSTPool]);

        // remove xst pool chunk 1
        aggregator
            .liquidity_quotations
            .get_mut(&LiquiditySourceType::XSTPool)
            .unwrap()
            .chunks
            .pop_front();

        let candidates = aggregator.find_best_price_candidates(&Vec::new());
        assert_eq!(candidates, vec![LiquiditySourceType::XSTPool]);

        // remove xst pool chunk 2
        aggregator
            .liquidity_quotations
            .get_mut(&LiquiditySourceType::XSTPool)
            .unwrap()
            .chunks
            .pop_front();

        let candidates = aggregator.find_best_price_candidates(&Vec::new());
        assert_eq!(candidates, vec![LiquiditySourceType::XSTPool]);
    }

    #[test]
    fn check_find_best_price_candidates_with_desired_output_and_equal_chunks() {
        let mut aggregator = get_liquidity_aggregator_with_desired_output_and_equal_chunks();

        let candidates = aggregator.find_best_price_candidates(&Vec::new());
        assert_eq!(candidates, vec![LiquiditySourceType::OrderBook]);

        // remove order book chunk 1
        aggregator
            .liquidity_quotations
            .get_mut(&LiquiditySourceType::OrderBook)
            .unwrap()
            .chunks
            .pop_front();

        let candidates = aggregator.find_best_price_candidates(&Vec::new());
        assert_eq!(
            candidates,
            vec![LiquiditySourceType::XYKPool, LiquiditySourceType::OrderBook]
        );

        // remove order book chunk 2
        aggregator
            .liquidity_quotations
            .get_mut(&LiquiditySourceType::OrderBook)
            .unwrap()
            .chunks
            .pop_front();

        let candidates = aggregator.find_best_price_candidates(&Vec::new());
        assert_eq!(candidates, vec![LiquiditySourceType::XYKPool]);

        // remove xyk pool chunk 1
        aggregator
            .liquidity_quotations
            .get_mut(&LiquiditySourceType::XYKPool)
            .unwrap()
            .chunks
            .pop_front();

        let candidates = aggregator.find_best_price_candidates(&Vec::new());
        assert_eq!(candidates, vec![LiquiditySourceType::XYKPool]);

        // remove xyk pool chunk 2
        aggregator
            .liquidity_quotations
            .get_mut(&LiquiditySourceType::XYKPool)
            .unwrap()
            .chunks
            .pop_front();

        let candidates = aggregator.find_best_price_candidates(&Vec::new());
        assert_eq!(candidates, vec![LiquiditySourceType::XYKPool]);

        // remove xyk pool chunk 3
        aggregator
            .liquidity_quotations
            .get_mut(&LiquiditySourceType::XYKPool)
            .unwrap()
            .chunks
            .pop_front();

        let candidates = aggregator.find_best_price_candidates(&Vec::new());
        assert_eq!(candidates, vec![LiquiditySourceType::XSTPool]);

        // remove xst pool chunk 1
        aggregator
            .liquidity_quotations
            .get_mut(&LiquiditySourceType::XSTPool)
            .unwrap()
            .chunks
            .pop_front();

        let candidates = aggregator.find_best_price_candidates(&Vec::new());
        assert_eq!(candidates, vec![LiquiditySourceType::XSTPool]);

        // remove xst pool chunk 2
        aggregator
            .liquidity_quotations
            .get_mut(&LiquiditySourceType::XSTPool)
            .unwrap()
            .chunks
            .pop_front();

        let candidates = aggregator.find_best_price_candidates(&Vec::new());
        assert_eq!(candidates, vec![LiquiditySourceType::XSTPool]);
    }

    #[test]
    fn check_aggregate_swap_outcome_with_desired_input_and_equal_chunks() {
        let aggregator = get_liquidity_aggregator_with_desired_input_and_equal_chunks();
        assert_eq!(
            aggregator.aggregate_swap_outcome(balance!(10)).unwrap(),
            (
                SwapInfo::from([(
                    LiquiditySourceType::OrderBook,
                    (balance!(10), balance!(120))
                )]),
                AggregatedSwapOutcome::new(
                    vec![(
                        LiquiditySourceType::OrderBook,
                        QuoteAmount::with_desired_input(balance!(10))
                    )],
                    balance!(120),
                    0
                )
            )
        );

        let aggregator = get_liquidity_aggregator_with_desired_input_and_equal_chunks();
        assert_eq!(
            aggregator.aggregate_swap_outcome(balance!(20)).unwrap(),
            (
                SwapInfo::from([(
                    LiquiditySourceType::OrderBook,
                    (balance!(20), balance!(220))
                )]),
                AggregatedSwapOutcome::new(
                    vec![(
                        LiquiditySourceType::OrderBook,
                        QuoteAmount::with_desired_input(balance!(20))
                    )],
                    balance!(220),
                    0
                )
            )
        );

        let aggregator = get_liquidity_aggregator_with_desired_input_and_equal_chunks();
        assert_eq!(
            aggregator.aggregate_swap_outcome(balance!(30)).unwrap(),
            (
                SwapInfo::from([
                    (LiquiditySourceType::XYKPool, (balance!(10), balance!(100))),
                    (
                        LiquiditySourceType::OrderBook,
                        (balance!(20), balance!(220))
                    )
                ]),
                AggregatedSwapOutcome::new(
                    vec![
                        (
                            LiquiditySourceType::XYKPool,
                            QuoteAmount::with_desired_input(balance!(10))
                        ),
                        (
                            LiquiditySourceType::OrderBook,
                            QuoteAmount::with_desired_input(balance!(20))
                        )
                    ],
                    balance!(320),
                    balance!(1)
                )
            )
        );

        let aggregator = get_liquidity_aggregator_with_desired_input_and_equal_chunks();
        assert_eq!(
            aggregator.aggregate_swap_outcome(balance!(40)).unwrap(),
            (
                SwapInfo::from([
                    (LiquiditySourceType::XYKPool, (balance!(20), balance!(190))),
                    (
                        LiquiditySourceType::OrderBook,
                        (balance!(20), balance!(220))
                    )
                ]),
                AggregatedSwapOutcome::new(
                    vec![
                        (
                            LiquiditySourceType::XYKPool,
                            QuoteAmount::with_desired_input(balance!(20))
                        ),
                        (
                            LiquiditySourceType::OrderBook,
                            QuoteAmount::with_desired_input(balance!(20))
                        )
                    ],
                    balance!(410),
                    balance!(1.9)
                )
            )
        );

        let aggregator = get_liquidity_aggregator_with_desired_input_and_equal_chunks();
        assert_eq!(
            aggregator.aggregate_swap_outcome(balance!(50)).unwrap(),
            (
                SwapInfo::from([
                    (LiquiditySourceType::XYKPool, (balance!(20), balance!(190))),
                    (LiquiditySourceType::XSTPool, (balance!(10), balance!(85))),
                    (
                        LiquiditySourceType::OrderBook,
                        (balance!(20), balance!(220))
                    )
                ]),
                AggregatedSwapOutcome::new(
                    vec![
                        (
                            LiquiditySourceType::XYKPool,
                            QuoteAmount::with_desired_input(balance!(20))
                        ),
                        (
                            LiquiditySourceType::XSTPool,
                            QuoteAmount::with_desired_input(balance!(10))
                        ),
                        (
                            LiquiditySourceType::OrderBook,
                            QuoteAmount::with_desired_input(balance!(20))
                        )
                    ],
                    balance!(495),
                    balance!(2.75)
                )
            )
        );

        let aggregator = get_liquidity_aggregator_with_desired_input_and_equal_chunks();
        assert_eq!(
            aggregator.aggregate_swap_outcome(balance!(60)).unwrap(),
            (
                SwapInfo::from([
                    (LiquiditySourceType::XYKPool, (balance!(20), balance!(190))),
                    (LiquiditySourceType::XSTPool, (balance!(20), balance!(170))),
                    (
                        LiquiditySourceType::OrderBook,
                        (balance!(20), balance!(220))
                    )
                ]),
                AggregatedSwapOutcome::new(
                    vec![
                        (
                            LiquiditySourceType::XYKPool,
                            QuoteAmount::with_desired_input(balance!(20))
                        ),
                        (
                            LiquiditySourceType::XSTPool,
                            QuoteAmount::with_desired_input(balance!(20))
                        ),
                        (
                            LiquiditySourceType::OrderBook,
                            QuoteAmount::with_desired_input(balance!(20))
                        )
                    ],
                    balance!(580),
                    balance!(3.6)
                )
            )
        );
    }

    #[test]
    fn check_aggregate_swap_outcome_with_desired_output_and_equal_chunks() {
        let aggregator = get_liquidity_aggregator_with_desired_output_and_equal_chunks();
        assert_eq!(
            aggregator.aggregate_swap_outcome(balance!(100)).unwrap(),
            (
                SwapInfo::from([(LiquiditySourceType::OrderBook, (balance!(8), balance!(100)))]),
                AggregatedSwapOutcome::new(
                    vec![(
                        LiquiditySourceType::OrderBook,
                        QuoteAmount::with_desired_output(balance!(100))
                    )],
                    balance!(8),
                    0
                )
            )
        );

        let aggregator = get_liquidity_aggregator_with_desired_output_and_equal_chunks();
        assert_eq!(
            aggregator.aggregate_swap_outcome(balance!(200)).unwrap(),
            (
                SwapInfo::from([(
                    LiquiditySourceType::OrderBook,
                    (balance!(18), balance!(200))
                )]),
                AggregatedSwapOutcome::new(
                    vec![(
                        LiquiditySourceType::OrderBook,
                        QuoteAmount::with_desired_output(balance!(200))
                    )],
                    balance!(18),
                    0
                )
            )
        );

        let aggregator = get_liquidity_aggregator_with_desired_output_and_equal_chunks();
        assert_eq!(
            aggregator.aggregate_swap_outcome(balance!(300)).unwrap(),
            (
                SwapInfo::from([
                    (LiquiditySourceType::XYKPool, (balance!(10), balance!(100))),
                    (
                        LiquiditySourceType::OrderBook,
                        (balance!(18), balance!(200))
                    )
                ]),
                AggregatedSwapOutcome::new(
                    vec![
                        (
                            LiquiditySourceType::XYKPool,
                            QuoteAmount::with_desired_output(balance!(100))
                        ),
                        (
                            LiquiditySourceType::OrderBook,
                            QuoteAmount::with_desired_output(balance!(200))
                        )
                    ],
                    balance!(28),
                    balance!(1)
                )
            )
        );

        let aggregator = get_liquidity_aggregator_with_desired_output_and_equal_chunks();
        assert_eq!(
            aggregator.aggregate_swap_outcome(balance!(400)).unwrap(),
            (
                SwapInfo::from([
                    (LiquiditySourceType::XYKPool, (balance!(21), balance!(200))),
                    (
                        LiquiditySourceType::OrderBook,
                        (balance!(18), balance!(200))
                    )
                ]),
                AggregatedSwapOutcome::new(
                    vec![
                        (
                            LiquiditySourceType::XYKPool,
                            QuoteAmount::with_desired_output(balance!(200))
                        ),
                        (
                            LiquiditySourceType::OrderBook,
                            QuoteAmount::with_desired_output(balance!(200))
                        )
                    ],
                    balance!(39),
                    balance!(2)
                )
            )
        );

        let aggregator = get_liquidity_aggregator_with_desired_output_and_equal_chunks();
        assert_eq!(
            aggregator.aggregate_swap_outcome(balance!(500)).unwrap(),
            (
                SwapInfo::from([
                    (LiquiditySourceType::XYKPool, (balance!(33), balance!(300))),
                    (
                        LiquiditySourceType::OrderBook,
                        (balance!(18), balance!(200))
                    )
                ]),
                AggregatedSwapOutcome::new(
                    vec![
                        (
                            LiquiditySourceType::XYKPool,
                            QuoteAmount::with_desired_output(balance!(300))
                        ),
                        (
                            LiquiditySourceType::OrderBook,
                            QuoteAmount::with_desired_output(balance!(200))
                        )
                    ],
                    balance!(51),
                    balance!(3)
                )
            )
        );

        let aggregator = get_liquidity_aggregator_with_desired_output_and_equal_chunks();
        assert_eq!(
            aggregator.aggregate_swap_outcome(balance!(600)).unwrap(),
            (
                SwapInfo::from([
                    (LiquiditySourceType::XYKPool, (balance!(33), balance!(300))),
                    (
                        LiquiditySourceType::XSTPool,
                        (balance!(12.5), balance!(100))
                    ),
                    (
                        LiquiditySourceType::OrderBook,
                        (balance!(18), balance!(200))
                    )
                ]),
                AggregatedSwapOutcome::new(
                    vec![
                        (
                            LiquiditySourceType::XYKPool,
                            QuoteAmount::with_desired_output(balance!(300))
                        ),
                        (
                            LiquiditySourceType::XSTPool,
                            QuoteAmount::with_desired_output(balance!(100))
                        ),
                        (
                            LiquiditySourceType::OrderBook,
                            QuoteAmount::with_desired_output(balance!(200))
                        )
                    ],
                    balance!(63.5),
                    balance!(4)
                )
            )
        );

        let aggregator = get_liquidity_aggregator_with_desired_output_and_equal_chunks();
        assert_eq!(
            aggregator.aggregate_swap_outcome(balance!(700)).unwrap(),
            (
                SwapInfo::from([
                    (LiquiditySourceType::XYKPool, (balance!(33), balance!(300))),
                    (LiquiditySourceType::XSTPool, (balance!(25), balance!(200))),
                    (
                        LiquiditySourceType::OrderBook,
                        (balance!(18), balance!(200))
                    )
                ]),
                AggregatedSwapOutcome::new(
                    vec![
                        (
                            LiquiditySourceType::XYKPool,
                            QuoteAmount::with_desired_output(balance!(300))
                        ),
                        (
                            LiquiditySourceType::XSTPool,
                            QuoteAmount::with_desired_output(balance!(200))
                        ),
                        (
                            LiquiditySourceType::OrderBook,
                            QuoteAmount::with_desired_output(balance!(200))
                        )
                    ],
                    balance!(76),
                    balance!(5)
                )
            )
        );
    }

    #[test]
    fn check_find_best_price_candidates_with_desired_input_and_different_chunks() {
        let mut aggregator = get_liquidity_aggregator_with_desired_input_and_different_chunks();

        let candidates = aggregator.find_best_price_candidates(&Vec::new());
        assert_eq!(candidates, vec![LiquiditySourceType::OrderBook]);

        // remove order book chunk 1
        aggregator
            .liquidity_quotations
            .get_mut(&LiquiditySourceType::OrderBook)
            .unwrap()
            .chunks
            .pop_front();

        let candidates = aggregator.find_best_price_candidates(&Vec::new());
        assert_eq!(
            candidates,
            vec![LiquiditySourceType::XYKPool, LiquiditySourceType::OrderBook]
        );

        // remove order book chunk 2
        aggregator
            .liquidity_quotations
            .get_mut(&LiquiditySourceType::OrderBook)
            .unwrap()
            .chunks
            .pop_front();

        let candidates = aggregator.find_best_price_candidates(&Vec::new());
        assert_eq!(candidates, vec![LiquiditySourceType::XYKPool]);

        // remove xyk pool chunk 1
        aggregator
            .liquidity_quotations
            .get_mut(&LiquiditySourceType::XYKPool)
            .unwrap()
            .chunks
            .pop_front();

        let candidates = aggregator.find_best_price_candidates(&Vec::new());
        assert_eq!(candidates, vec![LiquiditySourceType::XYKPool]);

        // remove xyk pool chunk 2
        aggregator
            .liquidity_quotations
            .get_mut(&LiquiditySourceType::XYKPool)
            .unwrap()
            .chunks
            .pop_front();

        let candidates = aggregator.find_best_price_candidates(&Vec::new());
        assert_eq!(candidates, vec![LiquiditySourceType::XSTPool]);

        // remove xst pool chunk 1
        aggregator
            .liquidity_quotations
            .get_mut(&LiquiditySourceType::XSTPool)
            .unwrap()
            .chunks
            .pop_front();

        let candidates = aggregator.find_best_price_candidates(&Vec::new());
        assert_eq!(candidates, vec![LiquiditySourceType::XSTPool]);

        // remove xst pool chunk 2
        aggregator
            .liquidity_quotations
            .get_mut(&LiquiditySourceType::XSTPool)
            .unwrap()
            .chunks
            .pop_front();

        let candidates = aggregator.find_best_price_candidates(&Vec::new());
        assert_eq!(candidates, vec![LiquiditySourceType::XSTPool]);
    }

    #[test]
    fn check_find_best_price_candidates_with_desired_output_and_different_chunks() {
        let mut aggregator = get_liquidity_aggregator_with_desired_output_and_different_chunks();

        let candidates = aggregator.find_best_price_candidates(&Vec::new());
        assert_eq!(candidates, vec![LiquiditySourceType::OrderBook]);

        // remove order book chunk 1
        aggregator
            .liquidity_quotations
            .get_mut(&LiquiditySourceType::OrderBook)
            .unwrap()
            .chunks
            .pop_front();

        let candidates = aggregator.find_best_price_candidates(&Vec::new());
        assert_eq!(
            candidates,
            vec![LiquiditySourceType::XYKPool, LiquiditySourceType::OrderBook]
        );

        // remove order book chunk 2
        aggregator
            .liquidity_quotations
            .get_mut(&LiquiditySourceType::OrderBook)
            .unwrap()
            .chunks
            .pop_front();

        let candidates = aggregator.find_best_price_candidates(&Vec::new());
        assert_eq!(candidates, vec![LiquiditySourceType::XYKPool]);

        // remove xyk pool chunk 1
        aggregator
            .liquidity_quotations
            .get_mut(&LiquiditySourceType::XYKPool)
            .unwrap()
            .chunks
            .pop_front();

        let candidates = aggregator.find_best_price_candidates(&Vec::new());
        assert_eq!(candidates, vec![LiquiditySourceType::XYKPool]);

        // remove xyk pool chunk 2
        aggregator
            .liquidity_quotations
            .get_mut(&LiquiditySourceType::XYKPool)
            .unwrap()
            .chunks
            .pop_front();

        let candidates = aggregator.find_best_price_candidates(&Vec::new());
        assert_eq!(candidates, vec![LiquiditySourceType::XYKPool]);

        // remove xyk pool chunk 3
        aggregator
            .liquidity_quotations
            .get_mut(&LiquiditySourceType::XYKPool)
            .unwrap()
            .chunks
            .pop_front();

        let candidates = aggregator.find_best_price_candidates(&Vec::new());
        assert_eq!(candidates, vec![LiquiditySourceType::XSTPool]);

        // remove xst pool chunk 1
        aggregator
            .liquidity_quotations
            .get_mut(&LiquiditySourceType::XSTPool)
            .unwrap()
            .chunks
            .pop_front();

        let candidates = aggregator.find_best_price_candidates(&Vec::new());
        assert_eq!(candidates, vec![LiquiditySourceType::XSTPool]);

        // remove xst pool chunk 2
        aggregator
            .liquidity_quotations
            .get_mut(&LiquiditySourceType::XSTPool)
            .unwrap()
            .chunks
            .pop_front();

        let candidates = aggregator.find_best_price_candidates(&Vec::new());
        assert_eq!(candidates, vec![LiquiditySourceType::XSTPool]);
    }

    #[test]
    fn check_aggregate_swap_outcome_with_desired_input_and_different_chunks() {
        let aggregator = get_liquidity_aggregator_with_desired_input_and_different_chunks();
        assert_eq!(
            aggregator.aggregate_swap_outcome(balance!(10)).unwrap(),
            (
                SwapInfo::from([(
                    LiquiditySourceType::OrderBook,
                    (balance!(10), balance!(120))
                )]),
                AggregatedSwapOutcome::new(
                    vec![(
                        LiquiditySourceType::OrderBook,
                        QuoteAmount::with_desired_input(balance!(10))
                    )],
                    balance!(120),
                    0
                )
            )
        );

        let aggregator = get_liquidity_aggregator_with_desired_input_and_different_chunks();
        assert_eq!(
            aggregator.aggregate_swap_outcome(balance!(20)).unwrap(),
            (
                SwapInfo::from([(
                    LiquiditySourceType::OrderBook,
                    (balance!(20), balance!(224))
                )]),
                AggregatedSwapOutcome::new(
                    vec![(
                        LiquiditySourceType::OrderBook,
                        QuoteAmount::with_desired_input(balance!(20))
                    )],
                    balance!(224),
                    0
                )
            )
        );

        let aggregator = get_liquidity_aggregator_with_desired_input_and_different_chunks();
        assert_eq!(
            aggregator.aggregate_swap_outcome(balance!(30)).unwrap(),
            (
                SwapInfo::from([
                    (LiquiditySourceType::XYKPool, (balance!(8), balance!(80))),
                    (
                        LiquiditySourceType::OrderBook,
                        (balance!(22), balance!(244))
                    )
                ]),
                AggregatedSwapOutcome::new(
                    vec![
                        (
                            LiquiditySourceType::XYKPool,
                            QuoteAmount::with_desired_input(balance!(8))
                        ),
                        (
                            LiquiditySourceType::OrderBook,
                            QuoteAmount::with_desired_input(balance!(22))
                        )
                    ],
                    balance!(324),
                    balance!(0.8)
                )
            )
        );

        let aggregator = get_liquidity_aggregator_with_desired_input_and_different_chunks();
        assert_eq!(
            aggregator.aggregate_swap_outcome(balance!(40)).unwrap(),
            (
                SwapInfo::from([
                    (LiquiditySourceType::XYKPool, (balance!(18), balance!(172))),
                    (
                        LiquiditySourceType::OrderBook,
                        (balance!(22), balance!(244))
                    )
                ]),
                AggregatedSwapOutcome::new(
                    vec![
                        (
                            LiquiditySourceType::XYKPool,
                            QuoteAmount::with_desired_input(balance!(18))
                        ),
                        (
                            LiquiditySourceType::OrderBook,
                            QuoteAmount::with_desired_input(balance!(22))
                        )
                    ],
                    balance!(416),
                    balance!(1.719999999999999999)
                )
            )
        );

        let aggregator = get_liquidity_aggregator_with_desired_input_and_different_chunks();
        assert_eq!(
            aggregator.aggregate_swap_outcome(balance!(50)).unwrap(),
            (
                SwapInfo::from([
                    (LiquiditySourceType::XYKPool, (balance!(22), balance!(208))),
                    (LiquiditySourceType::XSTPool, (balance!(6), balance!(51))),
                    (
                        LiquiditySourceType::OrderBook,
                        (balance!(22), balance!(244))
                    )
                ]),
                AggregatedSwapOutcome::new(
                    vec![
                        (
                            LiquiditySourceType::XYKPool,
                            QuoteAmount::with_desired_input(balance!(22))
                        ),
                        (
                            LiquiditySourceType::XSTPool,
                            QuoteAmount::with_desired_input(balance!(6))
                        ),
                        (
                            LiquiditySourceType::OrderBook,
                            QuoteAmount::with_desired_input(balance!(22))
                        )
                    ],
                    balance!(503),
                    balance!(2.59)
                )
            )
        );

        let aggregator = get_liquidity_aggregator_with_desired_input_and_different_chunks();
        assert_eq!(
            aggregator.aggregate_swap_outcome(balance!(60)).unwrap(),
            (
                SwapInfo::from([
                    (LiquiditySourceType::XYKPool, (balance!(22), balance!(208))),
                    (LiquiditySourceType::XSTPool, (balance!(16), balance!(136))),
                    (
                        LiquiditySourceType::OrderBook,
                        (balance!(22), balance!(244))
                    )
                ]),
                AggregatedSwapOutcome::new(
                    vec![
                        (
                            LiquiditySourceType::XYKPool,
                            QuoteAmount::with_desired_input(balance!(22))
                        ),
                        (
                            LiquiditySourceType::XSTPool,
                            QuoteAmount::with_desired_input(balance!(16))
                        ),
                        (
                            LiquiditySourceType::OrderBook,
                            QuoteAmount::with_desired_input(balance!(22))
                        )
                    ],
                    balance!(588),
                    balance!(3.439999999999999999)
                )
            )
        );
    }

    #[test]
    fn check_aggregate_swap_outcome_with_desired_output_and_different_chunks() {
        let aggregator = get_liquidity_aggregator_with_desired_output_and_different_chunks();
        assert_eq!(
            aggregator.aggregate_swap_outcome(balance!(100)).unwrap(),
            (
                SwapInfo::from([(LiquiditySourceType::OrderBook, (balance!(8), balance!(100)))]),
                AggregatedSwapOutcome::new(
                    vec![(
                        LiquiditySourceType::OrderBook,
                        QuoteAmount::with_desired_output(balance!(100))
                    )],
                    balance!(8),
                    0
                )
            )
        );

        let aggregator = get_liquidity_aggregator_with_desired_output_and_different_chunks();
        assert_eq!(
            aggregator.aggregate_swap_outcome(balance!(150)).unwrap(),
            (
                SwapInfo::from([(
                    LiquiditySourceType::OrderBook,
                    (balance!(13), balance!(150))
                )]),
                AggregatedSwapOutcome::new(
                    vec![(
                        LiquiditySourceType::OrderBook,
                        QuoteAmount::with_desired_output(balance!(150))
                    )],
                    balance!(13),
                    0
                )
            )
        );

        let aggregator = get_liquidity_aggregator_with_desired_output_and_different_chunks();
        assert_eq!(
            aggregator.aggregate_swap_outcome(balance!(250)).unwrap(),
            (
                SwapInfo::from([
                    (LiquiditySourceType::XYKPool, (balance!(6), balance!(60))),
                    (
                        LiquiditySourceType::OrderBook,
                        (balance!(17), balance!(190))
                    )
                ]),
                AggregatedSwapOutcome::new(
                    vec![
                        (
                            LiquiditySourceType::XYKPool,
                            QuoteAmount::with_desired_output(balance!(60))
                        ),
                        (
                            LiquiditySourceType::OrderBook,
                            QuoteAmount::with_desired_output(balance!(190))
                        )
                    ],
                    balance!(23),
                    0
                )
            )
        );

        let aggregator = get_liquidity_aggregator_with_desired_output_and_different_chunks();
        assert_eq!(
            aggregator.aggregate_swap_outcome(balance!(340)).unwrap(),
            (
                SwapInfo::from([
                    (
                        LiquiditySourceType::XYKPool,
                        (balance!(15.5), balance!(150))
                    ),
                    (
                        LiquiditySourceType::OrderBook,
                        (balance!(17), balance!(190))
                    )
                ]),
                AggregatedSwapOutcome::new(
                    vec![
                        (
                            LiquiditySourceType::XYKPool,
                            QuoteAmount::with_desired_output(balance!(150))
                        ),
                        (
                            LiquiditySourceType::OrderBook,
                            QuoteAmount::with_desired_output(balance!(190))
                        )
                    ],
                    balance!(32.5),
                    0
                )
            )
        );

        let aggregator = get_liquidity_aggregator_with_desired_output_and_different_chunks();
        assert_eq!(
            aggregator.aggregate_swap_outcome(balance!(405)).unwrap(),
            (
                SwapInfo::from([
                    (
                        LiquiditySourceType::XYKPool,
                        (balance!(18.5), balance!(175))
                    ),
                    (LiquiditySourceType::XSTPool, (balance!(5), balance!(40))),
                    (
                        LiquiditySourceType::OrderBook,
                        (balance!(17), balance!(190))
                    )
                ]),
                AggregatedSwapOutcome::new(
                    vec![
                        (
                            LiquiditySourceType::XYKPool,
                            QuoteAmount::with_desired_output(balance!(175))
                        ),
                        (
                            LiquiditySourceType::XSTPool,
                            QuoteAmount::with_desired_output(balance!(40))
                        ),
                        (
                            LiquiditySourceType::OrderBook,
                            QuoteAmount::with_desired_output(balance!(190))
                        )
                    ],
                    balance!(40.5),
                    0
                )
            )
        );

        let aggregator = get_liquidity_aggregator_with_desired_output_and_different_chunks();
        assert_eq!(
            aggregator.aggregate_swap_outcome(balance!(505)).unwrap(),
            (
                SwapInfo::from([
                    (
                        LiquiditySourceType::XYKPool,
                        (balance!(18.5), balance!(175))
                    ),
                    (
                        LiquiditySourceType::XSTPool,
                        (balance!(17.5), balance!(140))
                    ),
                    (
                        LiquiditySourceType::OrderBook,
                        (balance!(17), balance!(190))
                    )
                ]),
                AggregatedSwapOutcome::new(
                    vec![
                        (
                            LiquiditySourceType::XYKPool,
                            QuoteAmount::with_desired_output(balance!(175))
                        ),
                        (
                            LiquiditySourceType::XSTPool,
                            QuoteAmount::with_desired_output(balance!(140))
                        ),
                        (
                            LiquiditySourceType::OrderBook,
                            QuoteAmount::with_desired_output(balance!(190))
                        )
                    ],
                    balance!(53),
                    0
                )
            )
        );
    }
}
