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

#![cfg(feature = "wip")] // order-book

use assets::AssetIdOf;
use common::{
    balance, AssetInfoProvider, AssetName, AssetSymbol, Balance, PriceVariant, TradingPair,
    TradingPairSelector, VAL, XOR,
};
use frame_support::{assert_err, assert_ok};
use framenode_chain_spec::ext;
use framenode_runtime::order_book::{CurrencyLocker, LimitOrder, OrderBookId, Pallet};
use framenode_runtime::{order_book, Runtime, RuntimeOrigin};
use sp_std::collections::btree_map::BTreeMap;

type OrderBook = Pallet<Runtime>;

fn alice() -> <Runtime as frame_system::Config>::AccountId {
    <Runtime as frame_system::Config>::AccountId::new([1u8; 32])
}

fn bob() -> <Runtime as frame_system::Config>::AccountId {
    <Runtime as frame_system::Config>::AccountId::new([2u8; 32])
}

type E = order_book::Error<Runtime>;

#[test]
fn should_insert_limit_order() {
    ext().execute_with(|| {
        let order_book_id = OrderBookId::<Runtime> {
            base_asset_id: XOR.into(),
            target_asset_id: VAL.into(),
        };

        let order_buy_id = 1;
        let order_sell_id = 2;
        let owner = alice();
        let price = balance!(12);
        let amount = balance!(10);

        let order_buy = LimitOrder::<Runtime> {
            id: order_buy_id,
            owner: owner.clone(),
            side: PriceVariant::Buy,
            price: price,
            original_amount: amount,
            amount: amount,
            time: 10,
            lifespan: 1000,
        };

        let order_sell = LimitOrder::<Runtime> {
            id: order_sell_id,
            owner: owner.clone(),
            side: PriceVariant::Sell,
            price: price,
            original_amount: amount,
            amount: amount,
            time: 10,
            lifespan: 1000,
        };

        assert_ok!(OrderBook::insert_limit_order(&order_book_id, &order_buy));
        assert_eq!(
            OrderBook::limit_orders(order_book_id, order_buy_id).unwrap(),
            order_buy
        );
        assert_eq!(
            OrderBook::bids(order_book_id, price).unwrap(),
            vec![order_buy_id]
        );
        assert_eq!(
            OrderBook::aggregated_bids(order_book_id),
            BTreeMap::from([(price, amount)])
        );
        assert_eq!(OrderBook::asks(order_book_id, price), None);
        assert_eq!(
            OrderBook::aggregated_asks(order_book_id),
            BTreeMap::from([])
        );
        assert_eq!(
            OrderBook::user_limit_orders(&owner, order_book_id).unwrap(),
            vec![order_buy_id]
        );

        assert_ok!(OrderBook::insert_limit_order(&order_book_id, &order_sell));
        assert_eq!(
            OrderBook::limit_orders(order_book_id, order_sell_id).unwrap(),
            order_sell
        );
        assert_eq!(
            OrderBook::bids(order_book_id, price).unwrap(),
            vec![order_buy_id]
        );
        assert_eq!(
            OrderBook::aggregated_bids(order_book_id),
            BTreeMap::from([(price, amount)])
        );
        assert_eq!(
            OrderBook::asks(order_book_id, price).unwrap(),
            vec![order_sell_id]
        );
        assert_eq!(
            OrderBook::aggregated_asks(order_book_id),
            BTreeMap::from([(price, amount)])
        );
        assert_eq!(
            OrderBook::user_limit_orders(&owner, order_book_id).unwrap(),
            vec![order_buy_id, order_sell_id]
        );
    });
}

#[test]
fn should_not_insert_limit_order() {
    ext().execute_with(|| {
        let order_book_id = OrderBookId::<Runtime> {
            base_asset_id: XOR.into(),
            target_asset_id: VAL.into(),
        };

        let order_id = 1;
        let owner = alice();
        let price = balance!(12);
        let amount = balance!(10);

        let order = LimitOrder::<Runtime> {
            id: order_id,
            owner: owner.clone(),
            side: PriceVariant::Sell,
            price: price,
            original_amount: amount,
            amount: amount,
            time: 10,
            lifespan: 1000,
        };

        // Take actual values from `impl order_book::Config for Runtime`
        // =min(MaxOpenedLimitOrdersForAllOrderBooksPerUser, MaxLimitOrdersForPrice)
        let max = 10000;

        for _ in 0..max {
            assert_ok!(OrderBook::insert_limit_order(&order_book_id, &order));
        }

        // Error if storage overflow
        assert_err!(
            OrderBook::insert_limit_order(&order_book_id, &order),
            E::LimitOrderStorageOverflow
        );
    });
}

#[test]
fn should_delete_limit_order_success() {
    ext().execute_with(|| {
        let order_book_id = OrderBookId::<Runtime> {
            base_asset_id: XOR.into(),
            target_asset_id: VAL.into(),
        };

        let order_buy_id1 = 1;
        let order_buy_id2 = 2;
        let order_sell_id1 = 3;
        let order_sell_id2 = 4;
        let order_sell_id3 = 5;
        let owner = alice();
        let price1 = balance!(12);
        let price2 = balance!(13);
        let amount = balance!(10);

        let order_buy1 = LimitOrder::<Runtime> {
            id: order_buy_id1,
            owner: owner.clone(),
            side: PriceVariant::Buy,
            price: price1,
            original_amount: amount,
            amount: amount,
            time: 10,
            lifespan: 1000,
        };

        let order_buy2 = LimitOrder::<Runtime> {
            id: order_buy_id2,
            owner: owner.clone(),
            side: PriceVariant::Buy,
            price: price1,
            original_amount: amount,
            amount: amount,
            time: 10,
            lifespan: 1000,
        };

        let order_sell1 = LimitOrder::<Runtime> {
            id: order_sell_id1,
            owner: owner.clone(),
            side: PriceVariant::Sell,
            price: price1,
            original_amount: amount,
            amount: amount,
            time: 10,
            lifespan: 1000,
        };

        let order_sell2 = LimitOrder::<Runtime> {
            id: order_sell_id2,
            owner: owner.clone(),
            side: PriceVariant::Sell,
            price: price1,
            original_amount: amount,
            amount: amount,
            time: 10,
            lifespan: 1000,
        };

        let order_sell3 = LimitOrder::<Runtime> {
            id: order_sell_id3,
            owner: owner.clone(),
            side: PriceVariant::Sell,
            price: price2,
            original_amount: amount,
            amount: amount,
            time: 10,
            lifespan: 1000,
        };

        // add orders
        assert_ok!(OrderBook::insert_limit_order(&order_book_id, &order_buy1));
        assert_ok!(OrderBook::insert_limit_order(&order_book_id, &order_buy2));
        assert_ok!(OrderBook::insert_limit_order(&order_book_id, &order_sell1));
        assert_ok!(OrderBook::insert_limit_order(&order_book_id, &order_sell2));
        assert_ok!(OrderBook::insert_limit_order(&order_book_id, &order_sell3));

        // check they added
        assert_eq!(
            OrderBook::limit_orders(order_book_id, order_buy_id1).unwrap(),
            order_buy1
        );
        assert_eq!(
            OrderBook::limit_orders(order_book_id, order_buy_id2).unwrap(),
            order_buy2
        );
        assert_eq!(
            OrderBook::limit_orders(order_book_id, order_sell_id1).unwrap(),
            order_sell1
        );
        assert_eq!(
            OrderBook::limit_orders(order_book_id, order_sell_id2).unwrap(),
            order_sell2
        );
        assert_eq!(
            OrderBook::limit_orders(order_book_id, order_sell_id3).unwrap(),
            order_sell3
        );
        assert_eq!(
            OrderBook::bids(order_book_id, price1).unwrap(),
            vec![order_buy_id1, order_buy_id2]
        );
        assert_eq!(
            OrderBook::aggregated_bids(order_book_id),
            BTreeMap::from([(price1, 2 * amount)])
        );
        assert_eq!(
            OrderBook::asks(order_book_id, price1).unwrap(),
            vec![order_sell_id1, order_sell_id2]
        );
        assert_eq!(
            OrderBook::asks(order_book_id, price2).unwrap(),
            vec![order_sell_id3]
        );
        assert_eq!(
            OrderBook::aggregated_asks(order_book_id),
            BTreeMap::from([(price1, 2 * amount), (price2, amount)])
        );
        assert_eq!(
            OrderBook::user_limit_orders(&owner, &order_book_id).unwrap(),
            vec![
                order_buy_id1,
                order_buy_id2,
                order_sell_id1,
                order_sell_id2,
                order_sell_id3
            ]
        );

        // delete order sell 1
        assert_ok!(OrderBook::delete_limit_order(
            &order_book_id,
            order_sell_id1
        ));
        assert_eq!(OrderBook::limit_orders(order_book_id, order_sell_id1), None);
        assert_eq!(
            OrderBook::bids(order_book_id, price1).unwrap(),
            vec![order_buy_id1, order_buy_id2]
        );
        assert_eq!(
            OrderBook::aggregated_bids(order_book_id),
            BTreeMap::from([(price1, 2 * amount)])
        );
        assert_eq!(
            OrderBook::asks(order_book_id, price1).unwrap(),
            vec![order_sell_id2]
        );
        assert_eq!(
            OrderBook::asks(order_book_id, price2).unwrap(),
            vec![order_sell_id3]
        );
        assert_eq!(
            OrderBook::aggregated_asks(order_book_id),
            BTreeMap::from([(price1, amount), (price2, amount)])
        );
        assert_eq!(
            OrderBook::user_limit_orders(&owner, &order_book_id).unwrap(),
            vec![order_buy_id1, order_buy_id2, order_sell_id2, order_sell_id3]
        );

        // delete order buy 1
        assert_ok!(OrderBook::delete_limit_order(&order_book_id, order_buy_id1));
        assert_eq!(OrderBook::limit_orders(order_book_id, order_buy_id1), None);
        assert_eq!(
            OrderBook::bids(order_book_id, price1).unwrap(),
            vec![order_buy_id2]
        );
        assert_eq!(
            OrderBook::aggregated_bids(order_book_id),
            BTreeMap::from([(price1, amount)])
        );
        assert_eq!(
            OrderBook::asks(order_book_id, price1).unwrap(),
            vec![order_sell_id2]
        );
        assert_eq!(
            OrderBook::asks(order_book_id, price2).unwrap(),
            vec![order_sell_id3]
        );
        assert_eq!(
            OrderBook::aggregated_asks(order_book_id),
            BTreeMap::from([(price1, amount), (price2, amount)])
        );
        assert_eq!(
            OrderBook::user_limit_orders(&owner, &order_book_id).unwrap(),
            vec![order_buy_id2, order_sell_id2, order_sell_id3]
        );

        // delete order buy 2
        assert_ok!(OrderBook::delete_limit_order(&order_book_id, order_buy_id2));
        assert_eq!(OrderBook::limit_orders(order_book_id, order_buy_id2), None);
        assert_eq!(OrderBook::bids(order_book_id, price1), None);
        assert_eq!(
            OrderBook::aggregated_bids(order_book_id),
            BTreeMap::from([])
        );
        assert_eq!(
            OrderBook::asks(order_book_id, price1).unwrap(),
            vec![order_sell_id2]
        );
        assert_eq!(
            OrderBook::asks(order_book_id, price2).unwrap(),
            vec![order_sell_id3]
        );
        assert_eq!(
            OrderBook::aggregated_asks(order_book_id),
            BTreeMap::from([(price1, amount), (price2, amount)])
        );
        assert_eq!(
            OrderBook::user_limit_orders(&owner, &order_book_id).unwrap(),
            vec![order_sell_id2, order_sell_id3]
        );

        // delete order sell 3
        assert_ok!(OrderBook::delete_limit_order(
            &order_book_id,
            order_sell_id3
        ));
        assert_eq!(OrderBook::limit_orders(order_book_id, order_sell_id3), None);
        assert_eq!(OrderBook::bids(order_book_id, price1), None);
        assert_eq!(
            OrderBook::aggregated_bids(order_book_id),
            BTreeMap::from([])
        );
        assert_eq!(
            OrderBook::asks(order_book_id, price1).unwrap(),
            vec![order_sell_id2]
        );
        assert_eq!(OrderBook::asks(order_book_id, price2), None);
        assert_eq!(
            OrderBook::aggregated_asks(order_book_id),
            BTreeMap::from([(price1, amount)])
        );
        assert_eq!(
            OrderBook::user_limit_orders(&owner, &order_book_id).unwrap(),
            vec![order_sell_id2]
        );

        // delete order sell 2
        assert_ok!(OrderBook::delete_limit_order(
            &order_book_id,
            order_sell_id2
        ));
        assert_eq!(OrderBook::limit_orders(order_book_id, order_sell_id2), None);
        assert_eq!(OrderBook::bids(order_book_id, price1), None);
        assert_eq!(
            OrderBook::aggregated_bids(order_book_id),
            BTreeMap::from([])
        );
        assert_eq!(OrderBook::asks(order_book_id, price1), None);
        assert_eq!(OrderBook::asks(order_book_id, price2), None);
        assert_eq!(
            OrderBook::aggregated_asks(order_book_id),
            BTreeMap::from([])
        );
        assert_eq!(OrderBook::user_limit_orders(&owner, &order_book_id), None);
    });
}

#[test]
fn should_not_delete_limit_order() {
    ext().execute_with(|| {
        let order_book_id = OrderBookId::<Runtime> {
            base_asset_id: XOR.into(),
            target_asset_id: VAL.into(),
        };

        let order_id = 1;

        assert_err!(
            OrderBook::delete_limit_order(&order_book_id, order_id),
            E::DeleteLimitOrderError
        );
    });
}

#[test]
fn should_register_technical_account() {
    ext().execute_with(|| {
        framenode_runtime::frame_system::Pallet::<Runtime>::inc_providers(&alice());
        let nft = assets::Pallet::<Runtime>::register_from(
            &alice(),
            AssetSymbol(b"NFT".to_vec()),
            AssetName(b"Nft".to_vec()),
            0,
            balance!(1),
            false,
            None,
            None,
        )
        .unwrap();

        let accounts = [
            (
                common::DEXId::Polkaswap,
                TradingPair {
                    base_asset_id: XOR,
                    target_asset_id: VAL,
                },
            ),
            (
                common::DEXId::Polkaswap,
                TradingPair {
                    base_asset_id: XOR,
                    target_asset_id: nft,
                },
            ),
        ];

        // register (on order book creation)
        for (dex_id, trading_pair) in accounts {
            OrderBook::register_tech_account(dex_id.into(), trading_pair).expect(&format!(
                "Could not register account for dex_id: {:?}, pair: {:?}",
                dex_id, trading_pair,
            ));
        }

        // deregister (on order book removal)
        for (dex_id, trading_pair) in accounts {
            OrderBook::deregister_tech_account(dex_id.into(), trading_pair).expect(&format!(
                "Could not deregister account for dex_id: {:?}, pair: {:?}",
                dex_id, trading_pair,
            ));
        }
    });
}

fn test_lock_unlock_same_account(
    dex_id: common::DEXId,
    trading_pair: TradingPair<AssetIdOf<Runtime>>,
    asset: TradingPairSelector,
    amount_to_lock: Balance,
    account: &<Runtime as frame_system::Config>::AccountId,
) {
    let asset_id = trading_pair.select(asset.clone());
    let balance_before =
        assets::Pallet::<Runtime>::free_balance(asset_id, account).expect("Asset must exist");

    assert_ok!(OrderBook::lock_liquidity(
        dex_id.into(),
        account,
        trading_pair,
        asset.clone(),
        amount_to_lock
    ));

    let balance_after_lock =
        assets::Pallet::<Runtime>::free_balance(asset_id, account).expect("Asset must exist");
    assert_eq!(balance_after_lock, balance_before - amount_to_lock);

    assert_ok!(OrderBook::unlock_liquidity(
        dex_id.into(),
        account,
        trading_pair,
        asset,
        amount_to_lock
    ));

    let balance_after_unlock =
        assets::Pallet::<Runtime>::free_balance(asset_id, account).expect("Asset must exist");
    assert_eq!(balance_before, balance_after_unlock);
}

fn test_lock_unlock_other_account(
    dex_id: common::DEXId,
    trading_pair: TradingPair<AssetIdOf<Runtime>>,
    asset: TradingPairSelector,
    amount_to_lock: Balance,
    lock_account: &<Runtime as frame_system::Config>::AccountId,
    unlock_account: &<Runtime as frame_system::Config>::AccountId,
) {
    let asset_id = trading_pair.select(asset.clone());
    let lock_account_balance_before =
        assets::Pallet::<Runtime>::free_balance(asset_id, lock_account).expect("Asset must exist");
    let unlock_account_balance_before =
        assets::Pallet::<Runtime>::free_balance(asset_id, unlock_account)
            .expect("Asset must exist");

    assert_ok!(OrderBook::lock_liquidity(
        dex_id.into(),
        lock_account,
        trading_pair,
        asset.clone(),
        amount_to_lock
    ));

    let lock_account_balance_after_lock =
        assets::Pallet::<Runtime>::free_balance(asset_id, lock_account).expect("Asset must exist");
    assert_eq!(
        lock_account_balance_after_lock,
        lock_account_balance_before - amount_to_lock
    );

    assert_ok!(OrderBook::unlock_liquidity(
        dex_id.into(),
        unlock_account,
        trading_pair,
        asset,
        amount_to_lock
    ));

    let unlock_account_balance_after_unlock =
        assets::Pallet::<Runtime>::free_balance(asset_id, unlock_account)
            .expect("Asset must exist");
    assert_eq!(
        unlock_account_balance_after_unlock,
        unlock_account_balance_before + amount_to_lock
    );
}

#[test]
fn should_lock_unlock_base_asset() {
    ext().execute_with(|| {
        let amount_to_lock = balance!(10);
        let amount_to_mint = amount_to_lock;
        assert_ok!(assets::Pallet::<Runtime>::update_balance(
            RuntimeOrigin::root(),
            alice(),
            XOR,
            amount_to_mint.try_into().unwrap()
        ));
        let trading_pair = TradingPair {
            base_asset_id: XOR,
            target_asset_id: VAL,
        };
        let asset = TradingPairSelector::Base;
        OrderBook::register_tech_account(common::DEXId::Polkaswap.into(), trading_pair).unwrap();

        // Alice -> Alice (expected on order cancellation)
        test_lock_unlock_same_account(
            common::DEXId::Polkaswap,
            trading_pair,
            asset.clone(),
            amount_to_lock,
            &alice(),
        );

        // Alice -> Bob (expected exchange mechanism)
        test_lock_unlock_other_account(
            common::DEXId::Polkaswap,
            trading_pair,
            asset,
            amount_to_lock,
            &alice(),
            &bob(),
        );
    });
}

#[test]
fn should_lock_unlock_other_asset() {
    ext().execute_with(|| {
        let amount_to_lock = balance!(10);
        let amount_to_mint = amount_to_lock;
        assert_ok!(assets::Pallet::<Runtime>::update_balance(
            RuntimeOrigin::root(),
            alice(),
            VAL,
            amount_to_mint.try_into().unwrap()
        ));
        let trading_pair = TradingPair {
            base_asset_id: XOR,
            target_asset_id: VAL,
        };
        let asset = TradingPairSelector::Target;
        OrderBook::register_tech_account(common::DEXId::Polkaswap.into(), trading_pair).unwrap();

        // Alice -> Alice (expected on order cancellation)
        test_lock_unlock_same_account(
            common::DEXId::Polkaswap,
            trading_pair,
            asset.clone(),
            amount_to_lock,
            &alice(),
        );

        // Alice -> Bob (expected exchange mechanism)
        test_lock_unlock_other_account(
            common::DEXId::Polkaswap,
            trading_pair,
            asset,
            amount_to_lock,
            &alice(),
            &bob(),
        );
    });
}

#[test]
fn should_lock_unlock_indivisible_nft() {
    ext().execute_with(|| {
        framenode_runtime::frame_system::Pallet::<Runtime>::inc_providers(&alice());

        let nft = assets::Pallet::<Runtime>::register_from(
            &alice(),
            AssetSymbol(b"NFT".to_vec()),
            AssetName(b"Nft".to_vec()),
            0,
            balance!(1),
            false,
            None,
            None,
        )
        .unwrap();

        let trading_pair = TradingPair {
            base_asset_id: XOR,
            target_asset_id: nft.clone(),
        };
        let asset = TradingPairSelector::Target;
        OrderBook::register_tech_account(common::DEXId::Polkaswap.into(), trading_pair).unwrap();

        // Alice -> Alice (expected on order cancellation)
        test_lock_unlock_same_account(
            common::DEXId::Polkaswap,
            trading_pair,
            asset.clone(),
            balance!(1),
            &alice(),
        );

        // Alice -> Bob (expected exchange mechanism)
        test_lock_unlock_other_account(
            common::DEXId::Polkaswap,
            trading_pair,
            asset,
            balance!(1),
            &alice(),
            &bob(),
        );
    });
}

#[test]
fn should_not_lock_insufficient_base_asset() {
    ext().execute_with(|| {
        let amount_to_lock = balance!(10);
        let amount_to_mint = balance!(9.9);
        assert_ok!(assets::Pallet::<Runtime>::update_balance(
            RuntimeOrigin::root(),
            alice(),
            XOR,
            amount_to_mint.try_into().unwrap()
        ));
        let trading_pair = TradingPair {
            base_asset_id: XOR,
            target_asset_id: VAL,
        };
        let asset = TradingPairSelector::Base;
        OrderBook::register_tech_account(common::DEXId::Polkaswap.into(), trading_pair).unwrap();

        assert_err!(
            OrderBook::lock_liquidity(
                common::DEXId::Polkaswap.into(),
                &alice(),
                trading_pair,
                asset,
                amount_to_lock
            ),
            pallet_balances::Error::<Runtime>::InsufficientBalance
        );
    });
}

#[test]
fn should_not_lock_insufficient_other_asset() {
    ext().execute_with(|| {
        let amount_to_lock = balance!(10);
        let amount_to_mint = balance!(9.9);
        assert_ok!(assets::Pallet::<Runtime>::update_balance(
            RuntimeOrigin::root(),
            alice(),
            VAL,
            amount_to_mint.try_into().unwrap()
        ));
        let trading_pair = TradingPair {
            base_asset_id: XOR,
            target_asset_id: VAL,
        };
        let asset = TradingPairSelector::Target;
        OrderBook::register_tech_account(common::DEXId::Polkaswap.into(), trading_pair).unwrap();

        assert_err!(
            OrderBook::lock_liquidity(
                common::DEXId::Polkaswap.into(),
                &alice(),
                trading_pair,
                asset,
                amount_to_lock
            ),
            tokens::Error::<Runtime>::BalanceTooLow
        );
    });
}

#[test]
fn should_not_lock_insufficient_nft() {
    ext().execute_with(|| {
        let caller = alice();
        let creator = bob();
        framenode_runtime::frame_system::Pallet::<Runtime>::inc_providers(&creator);

        let nft = assets::Pallet::<Runtime>::register_from(
            &creator,
            AssetSymbol(b"NFT".to_vec()),
            AssetName(b"Nft".to_vec()),
            0,
            balance!(1),
            false,
            None,
            None,
        )
        .unwrap();

        let trading_pair = TradingPair {
            base_asset_id: XOR,
            target_asset_id: nft.clone(),
        };
        let asset = TradingPairSelector::Target;
        OrderBook::register_tech_account(common::DEXId::Polkaswap.into(), trading_pair).unwrap();

        assert_err!(
            OrderBook::lock_liquidity(
                common::DEXId::Polkaswap.into(),
                &caller,
                trading_pair,
                asset,
                balance!(1)
            ),
            tokens::Error::<Runtime>::BalanceTooLow
        );
    });
}

#[test]
fn should_not_unlock_more_base_that_tech_account_has() {
    ext().execute_with(|| {
        let amount_to_lock = balance!(10);
        let amount_to_mint = amount_to_lock;
        let amount_to_try_unlock = balance!(10.1);
        assert_ok!(assets::Pallet::<Runtime>::update_balance(
            RuntimeOrigin::root(),
            alice(),
            XOR,
            amount_to_mint.try_into().unwrap()
        ));

        let trading_pair = TradingPair {
            base_asset_id: XOR,
            target_asset_id: VAL,
        };
        let asset = TradingPairSelector::Base;
        OrderBook::register_tech_account(common::DEXId::Polkaswap.into(), trading_pair).unwrap();

        assert_ok!(OrderBook::lock_liquidity(
            common::DEXId::Polkaswap.into(),
            &alice(),
            trading_pair,
            asset.clone(),
            amount_to_lock
        ));

        assert_err!(
            OrderBook::unlock_liquidity(
                common::DEXId::Polkaswap.into(),
                &alice(),
                trading_pair,
                asset,
                amount_to_try_unlock
            ),
            pallet_balances::Error::<Runtime>::InsufficientBalance
        );
    });
}

#[test]
fn should_not_unlock_more_other_that_tech_account_has() {
    ext().execute_with(|| {
        let amount_to_lock = balance!(10);
        let amount_to_mint = amount_to_lock;
        let amount_to_try_unlock = balance!(10.1);
        assert_ok!(assets::Pallet::<Runtime>::update_balance(
            RuntimeOrigin::root(),
            alice(),
            VAL,
            amount_to_mint.try_into().unwrap()
        ));
        let trading_pair = TradingPair {
            base_asset_id: XOR,
            target_asset_id: VAL,
        };
        let asset = TradingPairSelector::Target;
        OrderBook::register_tech_account(common::DEXId::Polkaswap.into(), trading_pair).unwrap();

        assert_ok!(OrderBook::lock_liquidity(
            common::DEXId::Polkaswap.into(),
            &alice(),
            trading_pair,
            asset.clone(),
            amount_to_lock
        ));

        assert_err!(
            OrderBook::unlock_liquidity(
                common::DEXId::Polkaswap.into(),
                &alice(),
                trading_pair,
                asset,
                amount_to_try_unlock
            ),
            tokens::Error::<Runtime>::BalanceTooLow
        );
    });
}

#[test]
fn should_not_unlock_more_nft_that_tech_account_has() {
    ext().execute_with(|| {
        framenode_runtime::frame_system::Pallet::<Runtime>::inc_providers(&alice());

        let nft = assets::Pallet::<Runtime>::register_from(
            &alice(),
            AssetSymbol(b"NFT".to_vec()),
            AssetName(b"Nft".to_vec()),
            0,
            balance!(1),
            false,
            None,
            None,
        )
        .unwrap();

        let trading_pair = TradingPair {
            base_asset_id: XOR,
            target_asset_id: nft.clone(),
        };
        let asset = TradingPairSelector::Target;
        OrderBook::register_tech_account(common::DEXId::Polkaswap.into(), trading_pair).unwrap();

        assert_err!(
            OrderBook::unlock_liquidity(
                common::DEXId::Polkaswap.into(),
                &alice(),
                trading_pair,
                asset,
                balance!(1)
            ),
            tokens::Error::<Runtime>::BalanceTooLow
        );
    });
}
