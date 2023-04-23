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

use assets::AssetIdOf;
use common::{balance, DEXId, PriceVariant};
use frame_support::assert_ok;
use frame_system::RawOrigin;
use framenode_runtime::order_book::{self, Config, OrderBookId, Pallet};
use framenode_runtime::Runtime;
use sp_std::collections::btree_map::BTreeMap;

pub type E = order_book::Error<Runtime>;
pub const DEX: DEXId = DEXId::Polkaswap;

pub type OrderBookPallet = Pallet<Runtime>;

pub fn alice() -> <Runtime as frame_system::Config>::AccountId {
    <Runtime as frame_system::Config>::AccountId::new([1u8; 32])
}

pub fn bob() -> <Runtime as frame_system::Config>::AccountId {
    <Runtime as frame_system::Config>::AccountId::new([2u8; 32])
}

pub fn generate_account(seed: u32) -> <Runtime as frame_system::Config>::AccountId {
    let mut adr = [0u8; 32];

    let mut value = seed;
    let mut id = 0;
    while value != 0 {
        adr[31 - id] = (value % 256) as u8;
        value = value / 256;
        id += 1;
    }

    <Runtime as frame_system::Config>::AccountId::new(adr)
}

// Fills the order book
// price | volume | orders
//          Asks
//  11.5 |  255.8 | sell4, sell5, sell6
//  11.2 |  178.6 | sell2, sell3
//  11.0 |  176.3 | sell1
//  spread
//  10.0 |  168.5 | buy1
//   9.8 |  139.9 | buy2, buy3
//   9.5 |  264.3 | buy4, buy5, buy6
//          Bids
pub fn create_and_fill_order_book(order_book_id: OrderBookId<AssetIdOf<Runtime>>) {
    assert_ok!(OrderBookPallet::create_orderbook(
        RawOrigin::Signed(bob()).into(),
        DEX.into(),
        order_book_id
    ));

    let lifespan = 10000;

    // prices
    let bp1 = balance!(10);
    let bp2 = balance!(9.8);
    let bp3 = balance!(9.5);
    let sp1 = balance!(11);
    let sp2 = balance!(11.2);
    let sp3 = balance!(11.5);

    // amounts
    let amount1 = balance!(168.5);
    let amount2 = balance!(95.2);
    let amount3 = balance!(44.7);
    let amount4 = balance!(56.4);
    let amount5 = balance!(89.9);
    let amount6 = balance!(115);
    let amount7 = balance!(176.3);
    let amount8 = balance!(85.4);
    let amount9 = balance!(93.2);
    let amount10 = balance!(36.6);
    let amount11 = balance!(205.5);
    let amount12 = balance!(13.7);

    assert_ok!(OrderBookPallet::place_limit_order(
        RawOrigin::Signed(bob()).into(),
        order_book_id,
        bp1,
        amount1,
        PriceVariant::Buy,
        lifespan
    ));
    assert_ok!(OrderBookPallet::place_limit_order(
        RawOrigin::Signed(bob()).into(),
        order_book_id,
        bp2,
        amount2,
        PriceVariant::Buy,
        lifespan
    ));
    assert_ok!(OrderBookPallet::place_limit_order(
        RawOrigin::Signed(bob()).into(),
        order_book_id,
        bp2,
        amount3,
        PriceVariant::Buy,
        lifespan
    ));
    assert_ok!(OrderBookPallet::place_limit_order(
        RawOrigin::Signed(bob()).into(),
        order_book_id,
        bp3,
        amount4,
        PriceVariant::Buy,
        lifespan
    ));
    assert_ok!(OrderBookPallet::place_limit_order(
        RawOrigin::Signed(bob()).into(),
        order_book_id,
        bp3,
        amount5,
        PriceVariant::Buy,
        lifespan
    ));
    assert_ok!(OrderBookPallet::place_limit_order(
        RawOrigin::Signed(bob()).into(),
        order_book_id,
        bp3,
        amount6,
        PriceVariant::Buy,
        lifespan
    ));

    assert_ok!(OrderBookPallet::place_limit_order(
        RawOrigin::Signed(bob()).into(),
        order_book_id,
        sp1,
        amount7,
        PriceVariant::Sell,
        lifespan
    ));
    assert_ok!(OrderBookPallet::place_limit_order(
        RawOrigin::Signed(bob()).into(),
        order_book_id,
        sp2,
        amount8,
        PriceVariant::Sell,
        lifespan
    ));
    assert_ok!(OrderBookPallet::place_limit_order(
        RawOrigin::Signed(bob()).into(),
        order_book_id,
        sp2,
        amount9,
        PriceVariant::Sell,
        lifespan
    ));
    assert_ok!(OrderBookPallet::place_limit_order(
        RawOrigin::Signed(bob()).into(),
        order_book_id,
        sp3,
        amount10,
        PriceVariant::Sell,
        lifespan
    ));
    assert_ok!(OrderBookPallet::place_limit_order(
        RawOrigin::Signed(bob()).into(),
        order_book_id,
        sp3,
        amount11,
        PriceVariant::Sell,
        lifespan
    ));
    assert_ok!(OrderBookPallet::place_limit_order(
        RawOrigin::Signed(bob()).into(),
        order_book_id,
        sp3,
        amount12,
        PriceVariant::Sell,
        lifespan
    ));

    // check
    assert_eq!(
        OrderBookPallet::bids(&order_book_id, &bp1).unwrap(),
        vec![1]
    );
    assert_eq!(
        OrderBookPallet::bids(&order_book_id, &bp2).unwrap(),
        vec![2, 3]
    );
    assert_eq!(
        OrderBookPallet::bids(&order_book_id, &bp3).unwrap(),
        vec![4, 5, 6]
    );

    assert_eq!(
        OrderBookPallet::asks(&order_book_id, &sp1).unwrap(),
        vec![7]
    );
    assert_eq!(
        OrderBookPallet::asks(&order_book_id, &sp2).unwrap(),
        vec![8, 9]
    );
    assert_eq!(
        OrderBookPallet::asks(&order_book_id, &sp3).unwrap(),
        vec![10, 11, 12]
    );

    assert_eq!(
        OrderBookPallet::aggregated_bids(&order_book_id),
        BTreeMap::from([
            (bp1, amount1),
            (bp2, amount2 + amount3),
            (bp3, amount4 + amount5 + amount6)
        ])
    );
    assert_eq!(
        OrderBookPallet::aggregated_asks(&order_book_id),
        BTreeMap::from([
            (sp1, amount7),
            (sp2, amount8 + amount9),
            (sp3, amount10 + amount11 + amount12)
        ])
    );
}

pub fn get_last_order_id(
    order_book_id: OrderBookId<AssetIdOf<Runtime>>,
) -> Option<<Runtime as Config>::OrderId> {
    if let Some(order_book) = OrderBookPallet::order_books(order_book_id) {
        Some(order_book.last_order_id)
    } else {
        None
    }
}
