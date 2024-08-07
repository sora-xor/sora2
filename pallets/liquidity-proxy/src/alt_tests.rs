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

#![cfg(feature = "wip")] // ALT

use crate::alt_test_utils::*;
use common::prelude::{OutcomeFee, QuoteAmount, SwapAmount, SwapOutcome};
use common::{
    balance, FilterMode, LiquiditySourceFilter, LiquiditySourceId, LiquiditySourceType, DAI, TBCD,
    VAL, XOR,
};
use frame_support::{assert_err, assert_ok};
use framenode_chain_spec::ext;
use framenode_runtime::liquidity_proxy::liquidity_aggregator::AggregatedSwapOutcome;
use framenode_runtime::liquidity_proxy::{Error, Pallet};
use framenode_runtime::{Runtime, RuntimeOrigin};
use qa_tools::pallet_tools::liquidity_proxy::liquidity_sources;
use qa_tools::pallet_tools::mcbc::{CollateralCommonParameters, TbcdCollateralInput};
use qa_tools::pallet_tools::price_tools::AssetPrices;
use sp_std::vec;
use sp_std::vec::Vec;

type LiquidityProxyPallet = Pallet<Runtime>;
type E = Error<Runtime>;

#[test]
fn check_xyk_pool_small_reserves() {
    ext().execute_with(|| {
        framenode_runtime::frame_system::Pallet::<Runtime>::inc_providers(&bob());
        let asset = assets::Pallet::<Runtime>::register_from(
            &bob(),
            common::AssetSymbol(b"TEST".to_vec()),
            common::AssetName(b"Test".to_vec()),
            common::DEFAULT_BALANCE_PRECISION,
            balance!(1000000),
            false,
            common::AssetType::Regular,
            None,
            None,
        )
        .unwrap();

        init_xyk_pool(asset, XOR, balance!(10), Some(balance!(100)), bob());
        init_order_book(
            asset,
            balance!(9),
            balance!(10),
            balance!(100),
            1,
            0,
            alice(),
        );

        let (info, _) = LiquidityProxyPallet::inner_quote(
            DEX.into(),
            &XOR,
            &asset,
            QuoteAmount::with_desired_output(balance!(101)),
            LiquiditySourceFilter::empty(DEX.into()),
            true,
            true,
        )
        .unwrap();

        assert_eq!(
            info.outcome,
            SwapOutcome::new(
                balance!(1011.13217566127906472),
                OutcomeFee::xor(balance!(0.033396526983837194))
            )
        );
    });
}

#[test]
fn check_tbc_pool_small_reserves() {
    ext().execute_with(|| {
        framenode_runtime::frame_system::Pallet::<Runtime>::inc_providers(&bob());
        let asset = assets::Pallet::<Runtime>::register_from(
            &bob(),
            common::AssetSymbol(b"TEST".to_vec()),
            common::AssetName(b"Test".to_vec()),
            common::DEFAULT_BALANCE_PRECISION,
            balance!(1000000),
            true,
            common::AssetType::Regular,
            None,
            None,
        )
        .unwrap();

        init_mcbc_pool(asset, balance!(1000000000), balance!(100));
        init_order_book(
            asset,
            balance!(9),
            balance!(10),
            balance!(100),
            1,
            0,
            alice(),
        );

        let (info, _) = LiquidityProxyPallet::inner_quote(
            DEX.into(),
            &XOR,
            &asset,
            QuoteAmount::with_desired_output(balance!(101)),
            LiquiditySourceFilter::empty(DEX.into()),
            true,
            true,
        )
        .unwrap();

        assert_eq!(
            info.outcome,
            SwapOutcome::new(
                balance!(1088.910339125839073864),
                OutcomeFee::xor(balance!(8.268661538703033861))
            )
        );
    });
}

#[test]
fn check_not_enough_liquidity() {
    ext().execute_with(|| {
        framenode_runtime::frame_system::Pallet::<Runtime>::inc_providers(&bob());
        let asset = assets::Pallet::<Runtime>::register_from(
            &bob(),
            common::AssetSymbol(b"TEST".to_vec()),
            common::AssetName(b"Test".to_vec()),
            common::DEFAULT_BALANCE_PRECISION,
            balance!(1000000),
            true,
            common::AssetType::Regular,
            None,
            None,
        )
        .unwrap();

        init_xyk_pool(asset, XOR, balance!(10), Some(balance!(100)), bob());
        init_mcbc_pool(asset, balance!(1000000000), balance!(100));
        init_order_book(
            asset,
            balance!(9),
            balance!(10),
            balance!(100),
            1,
            0,
            alice(),
        );

        assert_err!(
            LiquidityProxyPallet::inner_quote(
                DEX.into(),
                &XOR,
                &asset,
                QuoteAmount::with_desired_output(balance!(1000)),
                LiquiditySourceFilter::empty(DEX.into()),
                true,
                true,
            ),
            E::InsufficientLiquidity
        );
    });
}

#[test]
fn check_rounding() {
    ext().execute_with(|| {
        init_order_book(
            VAL,
            balance!(3500),
            balance!(3600),
            balance!(910),
            1,
            0,
            alice(),
        );

        // before the fix it was balance!(36000.0000000001008),
        // because for desired output: input = output / price
        // price = chunk.output / chunk.input = 1 / 3600 = 0.0002(7)
        // input = 10 / 0.0002(7) = 36000.0000000001008
        assert_eq!(
            LiquidityProxyPallet::inner_quote(
                DEX.into(),
                &XOR,
                &VAL,
                QuoteAmount::with_desired_output(balance!(10)),
                LiquiditySourceFilter::empty(DEX.into()),
                true,
                true,
            )
            .unwrap()
            .0
            .outcome,
            SwapOutcome::new(balance!(36000), Default::default())
        );

        // before the fix it was balance!(0.99999) - aligned by precision,
        // because for desired input: output = input * price
        // price = chunk.output / chunk.input = 1 / 3600 = 0.0002(7)
        // output = 3600 * 0.0002(7) = 0.(9)
        assert_eq!(
            LiquidityProxyPallet::inner_quote(
                DEX.into(),
                &XOR,
                &VAL,
                QuoteAmount::with_desired_input(balance!(3600)),
                LiquiditySourceFilter::empty(DEX.into()),
                true,
                true,
            )
            .unwrap()
            .0
            .outcome,
            SwapOutcome::new(balance!(1), Default::default())
        );
    });
}

#[test]
fn check_tbcd_swap_smooth_quote() {
    ext().execute_with(|| {
        init_xyk_pool(TBCD, XOR, balance!(0.3), None, bob());

        assert_ok!(liquidity_sources::initialize_mcbc::<Runtime>(
            None,
            Vec::new(),
            Some(TbcdCollateralInput {
                parameters: CollateralCommonParameters {
                    ref_prices: Some(AssetPrices {
                        buy: balance!(1),
                        sell: balance!(1)
                    }),
                    reserves: Some(balance!(10000))
                },
                ref_xor_prices: Some(AssetPrices {
                    buy: balance!(0.000020960663069257),
                    sell: balance!(0.000020960663069257)
                })
            }),
        ));

        add_balance(alice(), TBCD, balance!(1000));

        let amount = SwapAmount::WithDesiredInput {
            desired_amount_in: balance!(1),
            min_amount_out: balance!(0),
        };

        assert_ok!(LiquidityProxyPallet::swap(
            RuntimeOrigin::signed(alice()),
            DEX.into(),
            TBCD,
            XOR,
            amount,
            Vec::new(),
            FilterMode::Disabled
        ));
    });
}

#[test]
fn check_xyk_swap_small_quote_fluctuation() {
    ext().execute_with(|| {
        init_order_book(
            DAI,
            balance!(77000),
            balance!(78000),
            balance!(1000),
            1,
            0,
            bob(),
        );

        assert_ok!(pool_xyk::Pallet::<Runtime>::initialize_pool(
            RuntimeOrigin::signed(bob()),
            DEX.into(),
            XOR,
            DAI,
        ));

        add_balance(alice(), XOR, balance!(100000));
        add_balance(alice(), DAI, balance!(100000));

        add_balance(bob(), XOR, balance!(1000000000));
        add_balance(bob(), DAI, balance!(1000000000));

        assert_ok!(pool_xyk::Pallet::<Runtime>::deposit_liquidity(
            RuntimeOrigin::signed(bob()),
            DEX.into(),
            XOR,
            DAI,
            balance!(99536258.840678562847701235),
            balance!(1293.714132065792292136),
            balance!(1),
            balance!(1),
        ));

        let amount = balance!(0.00001);

        // xyk pool returns the chunks in not descending price order, but in the scope of accepted slippage

        assert_ok!(LiquidityProxyPallet::swap(
            RuntimeOrigin::signed(alice()),
            DEX.into(),
            XOR,
            DAI,
            SwapAmount::with_desired_input(amount, balance!(0)),
            Vec::new(),
            FilterMode::Disabled
        ));

        assert_ok!(LiquidityProxyPallet::swap(
            RuntimeOrigin::signed(alice()),
            DEX.into(),
            DAI,
            XOR,
            SwapAmount::with_desired_output(amount, balance!(10000000000)),
            Vec::new(),
            FilterMode::Disabled
        ));
    });
}
