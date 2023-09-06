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
use common::prelude::{QuoteAmount, SwapVariant};
use common::{fixed, Balance, SwapChunk};
use frame_support::RuntimeDebug;
use itertools::Itertools;
use sp_runtime::traits::Zero;
use sp_std::collections::btree_map::BTreeMap;
use sp_std::collections::vec_deque::VecDeque;

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
#[derive(Clone)]
pub struct LiquidityAggregator<LiquiditySourceType> {
    liquidity_chunks: BTreeMap<LiquiditySourceType, VecDeque<SwapChunk<Balance>>>,
    variant: SwapVariant,
}

impl<LiquiditySourceType> LiquidityAggregator<LiquiditySourceType>
where
    LiquiditySourceType: Copy + Clone + Ord,
{
    pub fn new(variant: SwapVariant) -> Self {
        Self {
            liquidity_chunks: BTreeMap::new(),
            variant,
        }
    }

    pub fn add_source(
        &mut self,
        source: LiquiditySourceType,
        chunks: VecDeque<SwapChunk<Balance>>,
    ) {
        self.liquidity_chunks.insert(source, chunks);
    }

    pub fn aggregate_swap_outcome(
        mut self,
        amount: Balance,
    ) -> Option<AggregatedSwapOutcome<LiquiditySourceType, Balance>> {
        if self.liquidity_chunks.is_empty() {
            return None;
        }

        let mut remaining_amount = amount;
        let mut result_amount = Balance::zero();

        let mut distribution: BTreeMap<LiquiditySourceType, Balance> = BTreeMap::new();

        // just take the first value because all such values must be equal
        let chunk_size = match self.variant {
            SwapVariant::WithDesiredInput => {
                self.liquidity_chunks.first_key_value()?.1.front()?.input
            }
            SwapVariant::WithDesiredOutput => {
                self.liquidity_chunks.first_key_value()?.1.front()?.output
            }
        };

        while remaining_amount >= chunk_size {
            let candidates = self.find_best_price_candidates();

            let mut source = candidates.first()?;

            // if there are several candidates with the same best price,
            // then we need to select the source that already been selected
            for candidate in candidates.iter() {
                if distribution.keys().contains(candidate) {
                    source = candidate;
                    break;
                }
            }

            let chunk = self.liquidity_chunks.get_mut(source)?.pop_front()?;

            let (remaining_delta, result_delta) = match self.variant {
                SwapVariant::WithDesiredInput => (chunk.input, chunk.output),
                SwapVariant::WithDesiredOutput => (chunk.output, chunk.input),
            };

            distribution
                .entry(*source)
                .and_modify(|amount| *amount = amount.saturating_add(remaining_delta))
                .or_insert(remaining_delta);
            result_amount = result_amount.checked_add(result_delta)?;
            remaining_amount = remaining_amount.checked_sub(remaining_delta)?;
        }

        Some(AggregatedSwapOutcome {
            distribution: distribution
                .into_iter()
                .map(|(source, amount)| (source, QuoteAmount::with_variant(self.variant, amount)))
                .collect(),
            amount: result_amount,
            fee: Balance::zero(), // todo (m.tagirov) 447 fee
        })
    }

    /// Find liquidity sources where the top chunk has the best price.
    fn find_best_price_candidates(&self) -> Vec<LiquiditySourceType> {
        let mut candidates = Vec::new();
        let mut max = fixed!(0);
        for (source, chunks) in self.liquidity_chunks.iter() {
            let Some(front) = chunks.front() else {
                continue;
            };
            let Some(price) = front.price() else {
                continue;
            };

            if price == max {
                candidates.push(*source)
            }

            if price > max {
                candidates.clear();
                max = price;
                candidates.push(*source);
            }
        }
        candidates
    }
}

#[cfg(test)]
mod tests {
    use crate::liquidity_aggregator::*;
    use common::prelude::{QuoteAmount, SwapVariant};
    use common::{balance, LiquiditySourceType, SwapChunk};
    use sp_std::collections::vec_deque::VecDeque;

    fn get_liquidity_aggregator_with_desired_input() -> LiquidityAggregator<LiquiditySourceType> {
        let mut aggregator = LiquidityAggregator::new(SwapVariant::WithDesiredInput);
        aggregator.add_source(
            LiquiditySourceType::XYKPool,
            VecDeque::from([
                SwapChunk::new(balance!(10), balance!(100)),
                SwapChunk::new(balance!(10), balance!(90)),
                SwapChunk::new(balance!(10), balance!(80)),
                SwapChunk::new(balance!(10), balance!(70)),
                SwapChunk::new(balance!(10), balance!(60)),
            ]),
        );

        aggregator.add_source(
            LiquiditySourceType::XSTPool,
            VecDeque::from([
                SwapChunk::new(balance!(10), balance!(85)),
                SwapChunk::new(balance!(10), balance!(85)),
                SwapChunk::new(balance!(10), balance!(85)),
                SwapChunk::new(balance!(10), balance!(85)),
                SwapChunk::new(balance!(10), balance!(85)),
            ]),
        );

        aggregator.add_source(
            LiquiditySourceType::OrderBook,
            VecDeque::from([
                SwapChunk::new(balance!(10), balance!(120)),
                SwapChunk::new(balance!(10), balance!(100)),
                SwapChunk::new(balance!(10), balance!(80)),
            ]),
        );

        aggregator
    }

    fn get_liquidity_aggregator_with_desired_output() -> LiquidityAggregator<LiquiditySourceType> {
        let mut aggregator = LiquidityAggregator::new(SwapVariant::WithDesiredOutput);

        aggregator.add_source(
            LiquiditySourceType::XYKPool,
            VecDeque::from([
                SwapChunk::new(balance!(10), balance!(100)),
                SwapChunk::new(balance!(11), balance!(100)),
                SwapChunk::new(balance!(12), balance!(100)),
                SwapChunk::new(balance!(13), balance!(100)),
                SwapChunk::new(balance!(14), balance!(100)),
            ]),
        );

        aggregator.add_source(
            LiquiditySourceType::XSTPool,
            VecDeque::from([
                SwapChunk::new(balance!(12.5), balance!(100)),
                SwapChunk::new(balance!(12.5), balance!(100)),
                SwapChunk::new(balance!(12.5), balance!(100)),
                SwapChunk::new(balance!(12.5), balance!(100)),
                SwapChunk::new(balance!(12.5), balance!(100)),
            ]),
        );

        aggregator.add_source(
            LiquiditySourceType::OrderBook,
            VecDeque::from([
                SwapChunk::new(balance!(8), balance!(100)),
                SwapChunk::new(balance!(10), balance!(100)),
                SwapChunk::new(balance!(13), balance!(100)),
            ]),
        );

        aggregator
    }

    #[test]
    fn check_find_best_price_candidates_with_desired_input() {
        let mut aggregator = get_liquidity_aggregator_with_desired_input();

        let candidates = aggregator.find_best_price_candidates();
        assert_eq!(candidates, vec![LiquiditySourceType::OrderBook]);

        // remove order book chunk 1
        aggregator
            .liquidity_chunks
            .get_mut(&LiquiditySourceType::OrderBook)
            .unwrap()
            .pop_front();

        let candidates = aggregator.find_best_price_candidates();
        assert_eq!(
            candidates,
            vec![LiquiditySourceType::XYKPool, LiquiditySourceType::OrderBook]
        );

        // remove order book chunk 2
        aggregator
            .liquidity_chunks
            .get_mut(&LiquiditySourceType::OrderBook)
            .unwrap()
            .pop_front();

        let candidates = aggregator.find_best_price_candidates();
        assert_eq!(candidates, vec![LiquiditySourceType::XYKPool]);

        // remove xyk pool chunk 1
        aggregator
            .liquidity_chunks
            .get_mut(&LiquiditySourceType::XYKPool)
            .unwrap()
            .pop_front();

        let candidates = aggregator.find_best_price_candidates();
        assert_eq!(candidates, vec![LiquiditySourceType::XYKPool]);

        // remove xyk pool chunk 2
        aggregator
            .liquidity_chunks
            .get_mut(&LiquiditySourceType::XYKPool)
            .unwrap()
            .pop_front();

        let candidates = aggregator.find_best_price_candidates();
        assert_eq!(candidates, vec![LiquiditySourceType::XSTPool]);

        // remove xst pool chunk 1
        aggregator
            .liquidity_chunks
            .get_mut(&LiquiditySourceType::XSTPool)
            .unwrap()
            .pop_front();

        let candidates = aggregator.find_best_price_candidates();
        assert_eq!(candidates, vec![LiquiditySourceType::XSTPool]);

        // remove xst pool chunk 2
        aggregator
            .liquidity_chunks
            .get_mut(&LiquiditySourceType::XSTPool)
            .unwrap()
            .pop_front();

        let candidates = aggregator.find_best_price_candidates();
        assert_eq!(candidates, vec![LiquiditySourceType::XSTPool]);
    }

    #[test]
    fn check_find_best_price_candidates_with_desired_output() {
        let mut aggregator = get_liquidity_aggregator_with_desired_output();

        let candidates = aggregator.find_best_price_candidates();
        assert_eq!(candidates, vec![LiquiditySourceType::OrderBook]);

        // remove order book chunk 1
        aggregator
            .liquidity_chunks
            .get_mut(&LiquiditySourceType::OrderBook)
            .unwrap()
            .pop_front();

        let candidates = aggregator.find_best_price_candidates();
        assert_eq!(
            candidates,
            vec![LiquiditySourceType::XYKPool, LiquiditySourceType::OrderBook]
        );

        // remove order book chunk 2
        aggregator
            .liquidity_chunks
            .get_mut(&LiquiditySourceType::OrderBook)
            .unwrap()
            .pop_front();

        let candidates = aggregator.find_best_price_candidates();
        assert_eq!(candidates, vec![LiquiditySourceType::XYKPool]);

        // remove xyk pool chunk 1
        aggregator
            .liquidity_chunks
            .get_mut(&LiquiditySourceType::XYKPool)
            .unwrap()
            .pop_front();

        let candidates = aggregator.find_best_price_candidates();
        assert_eq!(candidates, vec![LiquiditySourceType::XYKPool]);

        // remove xyk pool chunk 2
        aggregator
            .liquidity_chunks
            .get_mut(&LiquiditySourceType::XYKPool)
            .unwrap()
            .pop_front();

        let candidates = aggregator.find_best_price_candidates();
        assert_eq!(candidates, vec![LiquiditySourceType::XYKPool]);

        // remove xyk pool chunk 3
        aggregator
            .liquidity_chunks
            .get_mut(&LiquiditySourceType::XYKPool)
            .unwrap()
            .pop_front();

        let candidates = aggregator.find_best_price_candidates();
        assert_eq!(candidates, vec![LiquiditySourceType::XSTPool]);

        // remove xst pool chunk 1
        aggregator
            .liquidity_chunks
            .get_mut(&LiquiditySourceType::XSTPool)
            .unwrap()
            .pop_front();

        let candidates = aggregator.find_best_price_candidates();
        assert_eq!(candidates, vec![LiquiditySourceType::XSTPool]);

        // remove xst pool chunk 2
        aggregator
            .liquidity_chunks
            .get_mut(&LiquiditySourceType::XSTPool)
            .unwrap()
            .pop_front();

        let candidates = aggregator.find_best_price_candidates();
        assert_eq!(candidates, vec![LiquiditySourceType::XSTPool]);
    }

    #[test]
    fn check_aggregate_swap_outcome_with_desired_input() {
        let aggregator = get_liquidity_aggregator_with_desired_input();
        assert_eq!(
            aggregator.aggregate_swap_outcome(balance!(10)).unwrap(),
            AggregatedSwapOutcome::new(
                vec![(
                    LiquiditySourceType::OrderBook,
                    QuoteAmount::with_desired_input(balance!(10))
                )],
                balance!(120),
                0
            )
        );

        let aggregator = get_liquidity_aggregator_with_desired_input();
        assert_eq!(
            aggregator.aggregate_swap_outcome(balance!(20)).unwrap(),
            AggregatedSwapOutcome::new(
                vec![(
                    LiquiditySourceType::OrderBook,
                    QuoteAmount::with_desired_input(balance!(20))
                )],
                balance!(220),
                0
            )
        );

        let aggregator = get_liquidity_aggregator_with_desired_input();
        assert_eq!(
            aggregator.aggregate_swap_outcome(balance!(30)).unwrap(),
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
                0
            )
        );

        let aggregator = get_liquidity_aggregator_with_desired_input();
        assert_eq!(
            aggregator.aggregate_swap_outcome(balance!(40)).unwrap(),
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
                0
            )
        );

        let aggregator = get_liquidity_aggregator_with_desired_input();
        assert_eq!(
            aggregator.aggregate_swap_outcome(balance!(50)).unwrap(),
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
                0
            )
        );

        let aggregator = get_liquidity_aggregator_with_desired_input();
        assert_eq!(
            aggregator.aggregate_swap_outcome(balance!(60)).unwrap(),
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
                0
            )
        );
    }

    #[test]
    fn check_aggregate_swap_outcome_with_desired_output() {
        let aggregator = get_liquidity_aggregator_with_desired_output();
        assert_eq!(
            aggregator.aggregate_swap_outcome(balance!(100)).unwrap(),
            AggregatedSwapOutcome::new(
                vec![(
                    LiquiditySourceType::OrderBook,
                    QuoteAmount::with_desired_output(balance!(100))
                )],
                balance!(8),
                0
            )
        );

        let aggregator = get_liquidity_aggregator_with_desired_output();
        assert_eq!(
            aggregator.aggregate_swap_outcome(balance!(200)).unwrap(),
            AggregatedSwapOutcome::new(
                vec![(
                    LiquiditySourceType::OrderBook,
                    QuoteAmount::with_desired_output(balance!(200))
                )],
                balance!(18),
                0
            )
        );

        let aggregator = get_liquidity_aggregator_with_desired_output();
        assert_eq!(
            aggregator.aggregate_swap_outcome(balance!(300)).unwrap(),
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
                0
            )
        );

        let aggregator = get_liquidity_aggregator_with_desired_output();
        assert_eq!(
            aggregator.aggregate_swap_outcome(balance!(400)).unwrap(),
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
                0
            )
        );

        let aggregator = get_liquidity_aggregator_with_desired_output();
        assert_eq!(
            aggregator.aggregate_swap_outcome(balance!(500)).unwrap(),
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
                0
            )
        );

        let aggregator = get_liquidity_aggregator_with_desired_output();
        assert_eq!(
            aggregator.aggregate_swap_outcome(balance!(600)).unwrap(),
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
                0
            )
        );

        let aggregator = get_liquidity_aggregator_with_desired_output();
        assert_eq!(
            aggregator.aggregate_swap_outcome(balance!(700)).unwrap(),
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
                0
            )
        );
    }
}
