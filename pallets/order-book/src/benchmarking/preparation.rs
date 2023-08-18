#[allow(unused)]
#[cfg(not(test))]
use crate::{
    cache_data_layer::CacheDataLayer, traits::DataLayer, Config, Event, ExpirationScheduler,
    ExpirationsAgenda, LimitOrder, MarketRole, MomentOf, OrderAmount, OrderBook, OrderBookId,
    OrderBookStatus, OrderBooks, OrderVolume, Pallet, Payment,
};
use frame_benchmarking::log::info;
#[allow(unused)]
#[cfg(test)]
use framenode_runtime::order_book::{
    cache_data_layer::CacheDataLayer, traits::DataLayer, Config, Event, ExpirationScheduler,
    ExpirationsAgenda, LimitOrder, MarketRole, MomentOf, OrderAmount, OrderBook, OrderBookId,
    OrderBookStatus, OrderBooks, OrderVolume, Pallet, Payment,
};

use assets::AssetIdOf;
use common::prelude::FixedWrapper;
use common::{balance, Balance, PriceVariant, VAL, XOR};
#[allow(unused)]
use frame_support::traits::{Get, Time};
use frame_system::RawOrigin;
use sp_runtime::traits::SaturatedConversion;
use sp_std::collections::btree_map::BTreeMap;
use sp_std::iter::repeat;
use sp_std::vec::Vec;

use crate::benchmarking::{bob, DEX};

use crate::OrderPrice;
use assets::Pallet as Assets;
use Pallet as OrderBookPallet;

// Creates and populates the order book with the following orders:
// price | volume | orders
//          Asks
//  11.5 |  255.8 | sell4, sell5, sell6
//  11.2 |  178.6 | sell2, sell3
//  11.0 |  176.3 | sell1
//  spread
//  10.0 |  168.5 | buy1
//   9.8 |  139.9 | buy2, buy3
//   9.5 |  261.3 | buy4, buy5, buy6
//          Bids
pub fn create_and_populate_order_book<T: Config>(
    order_book_id: OrderBookId<AssetIdOf<T>, T::DEXId>,
) {
    OrderBookPallet::<T>::create_orderbook(RawOrigin::Signed(bob::<T>()).into(), order_book_id)
        .unwrap();

    Assets::<T>::update_balance(
        RawOrigin::Root.into(),
        bob::<T>(),
        order_book_id.quote,
        balance!(1000000).try_into().unwrap(),
    )
    .unwrap();

    Assets::<T>::update_balance(
        RawOrigin::Root.into(),
        bob::<T>(),
        order_book_id.base,
        balance!(1000000).try_into().unwrap(),
    )
    .unwrap();

    let lifespan: Option<MomentOf<T>> = Some(10000u32.into());

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

    OrderBookPallet::<T>::place_limit_order(
        RawOrigin::Signed(bob::<T>()).into(),
        order_book_id,
        bp1,
        amount1,
        PriceVariant::Buy,
        lifespan,
    )
    .unwrap();
    OrderBookPallet::<T>::place_limit_order(
        RawOrigin::Signed(bob::<T>()).into(),
        order_book_id,
        bp2,
        amount2,
        PriceVariant::Buy,
        lifespan,
    )
    .unwrap();
    OrderBookPallet::<T>::place_limit_order(
        RawOrigin::Signed(bob::<T>()).into(),
        order_book_id,
        bp2,
        amount3,
        PriceVariant::Buy,
        lifespan,
    )
    .unwrap();
    OrderBookPallet::<T>::place_limit_order(
        RawOrigin::Signed(bob::<T>()).into(),
        order_book_id,
        bp3,
        amount4,
        PriceVariant::Buy,
        lifespan,
    )
    .unwrap();
    OrderBookPallet::<T>::place_limit_order(
        RawOrigin::Signed(bob::<T>()).into(),
        order_book_id,
        bp3,
        amount5,
        PriceVariant::Buy,
        lifespan,
    )
    .unwrap();
    OrderBookPallet::<T>::place_limit_order(
        RawOrigin::Signed(bob::<T>()).into(),
        order_book_id,
        bp3,
        amount6,
        PriceVariant::Buy,
        lifespan,
    )
    .unwrap();

    OrderBookPallet::<T>::place_limit_order(
        RawOrigin::Signed(bob::<T>()).into(),
        order_book_id,
        sp1,
        amount7,
        PriceVariant::Sell,
        lifespan,
    )
    .unwrap();
    OrderBookPallet::<T>::place_limit_order(
        RawOrigin::Signed(bob::<T>()).into(),
        order_book_id,
        sp2,
        amount8,
        PriceVariant::Sell,
        lifespan,
    )
    .unwrap();
    OrderBookPallet::<T>::place_limit_order(
        RawOrigin::Signed(bob::<T>()).into(),
        order_book_id,
        sp2,
        amount9,
        PriceVariant::Sell,
        lifespan,
    )
    .unwrap();
    OrderBookPallet::<T>::place_limit_order(
        RawOrigin::Signed(bob::<T>()).into(),
        order_book_id,
        sp3,
        amount10,
        PriceVariant::Sell,
        lifespan,
    )
    .unwrap();
    OrderBookPallet::<T>::place_limit_order(
        RawOrigin::Signed(bob::<T>()).into(),
        order_book_id,
        sp3,
        amount11,
        PriceVariant::Sell,
        lifespan,
    )
    .unwrap();
    OrderBookPallet::<T>::place_limit_order(
        RawOrigin::Signed(bob::<T>()).into(),
        order_book_id,
        sp3,
        amount12,
        PriceVariant::Sell,
        lifespan,
    )
    .unwrap();
}

pub fn prepare_delete_orderbook_benchmark<T: Config>(
    fill_settings: FillSettings<T>,
) -> OrderBookId<AssetIdOf<T>, T::DEXId> {
    let order_book_id = OrderBookId::<AssetIdOf<T>, T::DEXId> {
        dex_id: DEX.into(),
        base: VAL.into(),
        quote: XOR.into(),
    };
    OrderBookPallet::<T>::create_orderbook(RawOrigin::Signed(bob::<T>()).into(), order_book_id)
        .expect("failed to create an order book");
    let mut data_layer = CacheDataLayer::<T>::new();
    fill_order_book_worst_case::<T>(fill_settings.clone(), &order_book_id, &mut data_layer, None);
    data_layer.commit();
    order_book_id
}

pub fn prepare_place_orderbook_benchmark<T: Config>(
    fill_settings: FillSettings<T>,
    author: T::AccountId,
) -> OrderBookId<AssetIdOf<T>, T::DEXId> {
    let order_book_id = OrderBookId::<AssetIdOf<T>, T::DEXId> {
        dex_id: DEX.into(),
        base: VAL.into(),
        quote: XOR.into(),
    };
    OrderBookPallet::<T>::create_orderbook(RawOrigin::Signed(bob::<T>()).into(), order_book_id)
        .expect("failed to create an order book");
    let mut data_layer = CacheDataLayer::<T>::new();
    // Place only buy orders
    fill_order_book_worst_case::<T>(
        fill_settings.clone(),
        &order_book_id,
        &mut data_layer,
        Some(PriceVariant::Sell),
    );
    // fill user orders of the author

    data_layer.commit();
    order_book_id
}

#[cfg(not(test))]
pub mod presets {
    use crate::benchmarking::preparation::FillSettings;
    use crate::Config;

    pub fn preset_1<T: Config>() -> FillSettings<T> {
        FillSettings::<T>::new(16, 16, 16, 128)
    }

    pub fn preset_2<T: Config>() -> FillSettings<T> {
        FillSettings::<T>::new(32, 32, 32, 256)
    }

    pub fn preset_3<T: Config>() -> FillSettings<T> {
        FillSettings::<T>::new(64, 64, 64, 512)
    }

    pub fn preset_4<T: Config>() -> FillSettings<T> {
        FillSettings::<T>::new(128, 128, 128, 1024)
    }

    pub fn preset_5<T: Config>() -> FillSettings<T> {
        FillSettings::<T>::new(256, 256, 256, 2048)
    }

    pub fn preset_6<T: Config>() -> FillSettings<T> {
        FillSettings::<T>::new(512, 512, 512, 4096)
    }

    pub fn preset_7<T: Config>() -> FillSettings<T> {
        FillSettings::<T>::new(1024, 1024, 1024, 8192)
    }
}

#[derive(Clone, Debug)]
pub struct FillSettings<T: Config> {
    pub now: <<T as Config>::Time as Time>::Moment,
    pub max_side_price_count: u32,
    pub max_orders_per_price: u32,
    pub max_orders_per_user: u32,
    pub max_expiring_orders_per_block: u32,
}

impl<T: Config> FillSettings<T> {
    pub fn new(
        max_side_price_count: u32,
        max_orders_per_price: u32,
        max_orders_per_user: u32,
        max_expiring_orders_per_block: u32,
    ) -> Self {
        Self {
            now: T::Time::now(),
            max_side_price_count,
            max_orders_per_price,
            max_orders_per_user,
            max_expiring_orders_per_block,
        }
    }
}

fn fill_order_book_side<T: Config>(
    data: &mut impl DataLayer<T>,
    settings: FillSettings<T>,
    order_book: &mut OrderBook<T>,
    side: PriceVariant,
    orders_amount: OrderVolume,
    prices: &mut impl Iterator<Item = Balance>,
    users: &mut impl Iterator<Item = T::AccountId>,
    lifespans: &mut impl Iterator<Item = u64>,
) {
    #[cfg(feature = "std")]
    use std::io::Write;

    let current_block = frame_system::Pallet::<T>::block_number();
    let mut total_payment = Payment::new(order_book.order_book_id);
    let mut to_expire = BTreeMap::<_, Vec<_>>::new();
    #[cfg(feature = "std")]
    println!("inserting orders");
    for (i, price) in prices.enumerate() {
        #[cfg(feature = "std")]
        {
            print!(
                "\r{}/{} ({}%)",
                i,
                settings.max_side_price_count,
                100.0 * (i as f32) / (settings.max_side_price_count as f32)
            );
            std::io::stdout().flush().unwrap();
        }
        for _ in 0..settings.max_orders_per_price {
            let user = users.next().expect("infinite iterator");
            let order = LimitOrder::<T>::new(
                order_book.next_order_id(),
                user.clone(),
                side,
                price,
                orders_amount,
                settings.now.clone(),
                lifespans
                    .next()
                    .expect("infinite iterator")
                    .saturated_into(),
                current_block,
            );
            // Instead of `order_book.place_limit_order(order, data)` we do the same steps manually
            // in order to avoid overhead on checking various restrictions and other unnecessary
            // stuff

            let order_id = order.id;
            let expires_at = order.expires_at;
            // lock corresponding currency
            let lock_amount = order.deal_amount(MarketRole::Taker, None).unwrap();
            let lock_asset = lock_amount.associated_asset(&order_book.order_book_id);
            total_payment
                .to_lock
                .entry(*lock_asset)
                .or_default()
                .entry(order.owner.clone())
                .and_modify(|amount| *amount += *lock_amount.value())
                .or_insert(*lock_amount.value());
            // insert the order in storages
            data.insert_limit_order(&order_book.order_book_id, order)
                .unwrap();
            // schedule its expiration
            to_expire.entry(expires_at).or_default().push(order_id);
        }
    }
    #[cfg(feature = "std")]
    println!("\nlocking payments");
    total_payment
        .execute_all::<OrderBookPallet<T>, OrderBookPallet<T>>()
        .unwrap();
    #[cfg(feature = "std")]
    println!("scheduling expirations");
    #[cfg(feature = "std")]
    let total_expirations = to_expire.len();
    for (i, (expires_at, orders)) in to_expire.into_iter().enumerate() {
        #[cfg(feature = "std")]
        {
            print!(
                "\r{}/{} ({}%)",
                i,
                total_expirations,
                100.0 * (i as f32) / (total_expirations as f32)
            );
            std::io::stdout().flush().unwrap();
        }
        <ExpirationsAgenda<T>>::try_mutate(expires_at, |block_expirations| {
            block_expirations.try_extend(
                orders
                    .into_iter()
                    .map(|order_id| (order_book.order_book_id, order_id)),
            )
        })
        .expect("Failed to schedule orders for expiration");
    }
    #[cfg(feature = "std")]
    println!();
}

fn bid_prices_iterator(
    tick_size: OrderPrice,
    max_side_price_count: u32,
) -> impl Iterator<Item = Balance> {
    (1..=max_side_price_count).map(move |i| (i as u128) * tick_size)
}

fn ask_prices_iterator(
    tick_size: OrderPrice,
    max_side_price_count: u32,
) -> impl Iterator<Item = Balance> {
    (max_side_price_count + 1..=2 * max_side_price_count)
        .rev()
        .map(move |i| (i as u128) * tick_size)
}

fn users_iterator<T: Config>(
    order_book_id: OrderBookId<AssetIdOf<T>, T::DEXId>,
    mint_per_user: Balance,
    max_price: Balance,
    max_orders_per_user: u32,
) -> impl Iterator<Item = T::AccountId> {
    (1..)
        .map(crate::test_utils::generate_account::<T>)
        // each user receives assets that should be enough for placing their orders
        .inspect(move |user| {
            assets::Pallet::<T>::mint_unchecked(&order_book_id.base, &user, mint_per_user).unwrap();
            assets::Pallet::<T>::mint_unchecked(
                &order_book_id.quote,
                &user,
                (FixedWrapper::from(max_price) * FixedWrapper::from(mint_per_user))
                    .try_into_balance()
                    .unwrap(),
            )
            .unwrap();
        })
        // yield same user for `max_orders_per_user` orders.
        // `inspect` is still called only once for each user.
        .flat_map(move |user| repeat(user).take(max_orders_per_user.try_into().unwrap()))
}

fn lifespans_iterator<T: Config>(max_expiring_orders_per_block: u32) -> impl Iterator<Item = u64> {
    (1..)
        .map(|i| {
            i * T::MILLISECS_PER_BLOCK.saturated_into::<u64>()
                + T::MIN_ORDER_LIFETIME.saturated_into::<u64>()
        })
        // same lifespan should be yielded for `max_expiring_orders_per_block` orders
        .flat_map(move |lifespan| {
            repeat(lifespan).take(max_expiring_orders_per_block.try_into().unwrap())
        })
}

pub fn fill_order_book_worst_case<T: Config + assets::Config>(
    settings: FillSettings<T>,
    order_book_id: &OrderBookId<AssetIdOf<T>, T::DEXId>,
    data: &mut impl DataLayer<T>,
    skip_side: Option<PriceVariant>,
) {
    let FillSettings {
        now: _,
        max_side_price_count,
        max_orders_per_price,
        max_orders_per_user,
        max_expiring_orders_per_block,
    } = settings;

    let mut order_book = <OrderBooks<T>>::get(order_book_id).unwrap();
    let order_amount = sp_std::cmp::max(order_book.step_lot_size, order_book.min_lot_size);
    let max_price = (2 * max_side_price_count) as u128 * order_book.tick_size;

    // Owners for each placed order
    let mut users = users_iterator::<T>(
        order_book.order_book_id,
        max_orders_per_user as u128 * order_amount,
        max_price,
        max_orders_per_user,
    );
    // Lifespans for each placed order
    let mut lifespans = lifespans_iterator::<T>(max_expiring_orders_per_block);

    if !matches!(skip_side, Some(PriceVariant::Buy)) {
        let mut bid_prices = bid_prices_iterator(order_book.tick_size, max_side_price_count);
        #[cfg(feature = "std")]
        let start_time = std::time::Instant::now();
        #[cfg(feature = "std")]
        println!(
            "Starting placement of bid orders, {} orders per price",
            max_orders_per_price
        );
        info!("Placing bids...");
        fill_order_book_side(
            data,
            settings.clone(),
            &mut order_book,
            PriceVariant::Buy,
            order_amount,
            &mut bid_prices,
            &mut users,
            &mut lifespans,
        );

        info!("Placed all bids");
        #[cfg(feature = "std")]
        println!("\nprocessed all bid prices in {:?}", start_time.elapsed());
    }

    if !matches!(skip_side, Some(PriceVariant::Sell)) {
        let mut ask_prices = ask_prices_iterator(order_book.tick_size, max_side_price_count);
        #[cfg(feature = "std")]
        let start_time = std::time::Instant::now();
        #[cfg(feature = "std")]
        println!(
            "Starting placement of ask orders, {} orders per price",
            max_orders_per_price
        );
        info!("Placing asks...");
        fill_order_book_side(
            data,
            settings,
            &mut order_book,
            PriceVariant::Sell,
            order_amount,
            &mut ask_prices,
            &mut users,
            &mut lifespans,
        );
        info!("Placed all asks");
        #[cfg(feature = "std")]
        println!("\nprocessed all ask prices in {:?}", start_time.elapsed());
    }
    <OrderBooks<T>>::insert(order_book_id, order_book);
}
