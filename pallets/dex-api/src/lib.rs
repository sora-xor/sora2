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

use common::prelude::{Balance, QuoteAmount, SwapAmount, SwapOutcome};
use common::{
    DexInfoProvider, LiquidityRegistry, LiquiditySource, LiquiditySourceFilter, LiquiditySourceId,
    LiquiditySourceType, RewardReason,
};
use frame_support::sp_runtime::DispatchError;
use frame_support::weights::Weight;
use sp_std::vec::Vec;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

type DEXInfo<T> = common::prelude::DEXInfo<<T as assets::Config>::AssetId>;

impl<T: Config>
    LiquiditySource<
        LiquiditySourceId<T::DEXId, LiquiditySourceType>,
        T::AccountId,
        T::AssetId,
        Balance,
        DispatchError,
    > for Pallet<T>
{
    fn can_exchange(
        liquidity_source_id: &LiquiditySourceId<T::DEXId, LiquiditySourceType>,
        input_asset_id: &T::AssetId,
        output_asset_id: &T::AssetId,
    ) -> bool {
        use LiquiditySourceType::*;
        macro_rules! can_exchange {
            ($source_type:ident) => {
                T::$source_type::can_exchange(
                    &liquidity_source_id.dex_id,
                    input_asset_id,
                    output_asset_id,
                )
            };
        }
        match liquidity_source_id.liquidity_source_index {
            XYKPool => can_exchange!(XYKPool),
            MulticollateralBondingCurvePool => can_exchange!(MulticollateralBondingCurvePool),
            XSTPool => can_exchange!(XSTPool),

            #[cfg(feature = "ready-to-test")] // order-book
            OrderBook => can_exchange!(OrderBook),

            MockPool => can_exchange!(MockLiquiditySource),
            MockPool2 => can_exchange!(MockLiquiditySource2),
            MockPool3 => can_exchange!(MockLiquiditySource3),
            MockPool4 => can_exchange!(MockLiquiditySource4),
            BondingCurvePool => unreachable!(),
        }
    }

    fn quote(
        liquidity_source_id: &LiquiditySourceId<T::DEXId, LiquiditySourceType>,
        input_asset_id: &T::AssetId,
        output_asset_id: &T::AssetId,
        amount: QuoteAmount<Balance>,
        deduce_fee: bool,
    ) -> Result<(SwapOutcome<Balance>, Weight), DispatchError> {
        use LiquiditySourceType::*;
        macro_rules! quote {
            ($source_type:ident) => {
                T::$source_type::quote(
                    &liquidity_source_id.dex_id,
                    input_asset_id,
                    output_asset_id,
                    amount,
                    deduce_fee,
                )
            };
        }
        match liquidity_source_id.liquidity_source_index {
            LiquiditySourceType::XYKPool => quote!(XYKPool),
            MulticollateralBondingCurvePool => quote!(MulticollateralBondingCurvePool),
            XSTPool => quote!(XSTPool),

            #[cfg(feature = "ready-to-test")] // order-book
            OrderBook => quote!(OrderBook),

            MockPool => quote!(MockLiquiditySource),
            MockPool2 => quote!(MockLiquiditySource2),
            MockPool3 => quote!(MockLiquiditySource3),
            MockPool4 => quote!(MockLiquiditySource4),
            BondingCurvePool => unreachable!(),
        }
    }

    fn exchange(
        sender: &T::AccountId,
        receiver: &T::AccountId,
        liquidity_source_id: &LiquiditySourceId<T::DEXId, LiquiditySourceType>,
        input_asset_id: &T::AssetId,
        output_asset_id: &T::AssetId,
        swap_amount: SwapAmount<Balance>,
    ) -> Result<(SwapOutcome<Balance>, Weight), DispatchError> {
        use LiquiditySourceType::*;
        macro_rules! exchange {
            ($source_type:ident) => {
                T::$source_type::exchange(
                    sender,
                    receiver,
                    &liquidity_source_id.dex_id,
                    input_asset_id,
                    output_asset_id,
                    swap_amount,
                )
            };
        }
        match liquidity_source_id.liquidity_source_index {
            XYKPool => exchange!(XYKPool),
            MulticollateralBondingCurvePool => exchange!(MulticollateralBondingCurvePool),
            XSTPool => exchange!(XSTPool),

            #[cfg(feature = "ready-to-test")] // order-book
            OrderBook => exchange!(OrderBook),

            MockPool => exchange!(MockLiquiditySource),
            MockPool2 => exchange!(MockLiquiditySource2),
            MockPool3 => exchange!(MockLiquiditySource3),
            MockPool4 => exchange!(MockLiquiditySource4),
            BondingCurvePool => unreachable!(),
        }
    }

    fn check_rewards(
        liquidity_source_id: &LiquiditySourceId<T::DEXId, LiquiditySourceType>,
        input_asset_id: &T::AssetId,
        output_asset_id: &T::AssetId,
        input_amount: Balance,
        output_amount: Balance,
    ) -> Result<(Vec<(Balance, T::AssetId, RewardReason)>, Weight), DispatchError> {
        use LiquiditySourceType::*;
        macro_rules! check_rewards {
            ($source_type:ident) => {
                T::$source_type::check_rewards(
                    &liquidity_source_id.dex_id,
                    input_asset_id,
                    output_asset_id,
                    input_amount,
                    output_amount,
                )
            };
        }
        match liquidity_source_id.liquidity_source_index {
            XYKPool => check_rewards!(XYKPool),
            MulticollateralBondingCurvePool => check_rewards!(MulticollateralBondingCurvePool),
            XSTPool => check_rewards!(XSTPool),

            #[cfg(feature = "ready-to-test")] // order-book
            OrderBook => check_rewards!(OrderBook),

            MockPool => check_rewards!(MockLiquiditySource),
            MockPool2 => check_rewards!(MockLiquiditySource2),
            MockPool3 => check_rewards!(MockLiquiditySource3),
            MockPool4 => check_rewards!(MockLiquiditySource4),
            BondingCurvePool => unreachable!(),
        }
    }

    fn quote_without_impact(
        liquidity_source_id: &LiquiditySourceId<T::DEXId, LiquiditySourceType>,
        input_asset_id: &T::AssetId,
        output_asset_id: &T::AssetId,
        amount: QuoteAmount<Balance>,
        deduce_fee: bool,
    ) -> Result<SwapOutcome<Balance>, DispatchError> {
        use LiquiditySourceType::*;
        macro_rules! quote_without_impact {
            ($source_type:ident) => {
                T::$source_type::quote_without_impact(
                    &liquidity_source_id.dex_id,
                    input_asset_id,
                    output_asset_id,
                    amount,
                    deduce_fee,
                )
            };
        }
        match liquidity_source_id.liquidity_source_index {
            XYKPool => quote_without_impact!(XYKPool),
            MulticollateralBondingCurvePool => {
                quote_without_impact!(MulticollateralBondingCurvePool)
            }
            XSTPool => quote_without_impact!(XSTPool),

            #[cfg(feature = "ready-to-test")] // order-book
            OrderBook => quote_without_impact!(OrderBook),

            MockPool => quote_without_impact!(MockLiquiditySource),
            MockPool2 => quote_without_impact!(MockLiquiditySource2),
            MockPool3 => quote_without_impact!(MockLiquiditySource3),
            MockPool4 => quote_without_impact!(MockLiquiditySource4),
            BondingCurvePool => unreachable!(),
        }
    }

    fn quote_weight() -> Weight {
        #[allow(unused_mut)] // order-book
        #[allow(unused_assignments)] // order-book
        let mut weight = Weight::zero();

        #[cfg(feature = "ready-to-test")] // order-book
        {
            weight = T::OrderBook::quote_weight();
        }

        weight
            .max(T::XSTPool::quote_weight())
            .max(T::XYKPool::quote_weight())
            .max(T::MulticollateralBondingCurvePool::quote_weight())
    }

    fn exchange_weight() -> Weight {
        #[allow(unused_mut)] // order-book
        #[allow(unused_assignments)] // order-book
        let mut weight = Weight::zero();

        #[cfg(feature = "ready-to-test")] // order-book
        {
            weight = T::OrderBook::exchange_weight();
        }

        weight
            .max(T::XSTPool::exchange_weight())
            .max(T::XYKPool::exchange_weight())
            .max(T::MulticollateralBondingCurvePool::exchange_weight())
    }

    fn check_rewards_weight() -> Weight {
        #[allow(unused_mut)] // order-book
        #[allow(unused_assignments)] // order-book
        let mut weight = Weight::zero();

        #[cfg(feature = "ready-to-test")] // order-book
        {
            weight = T::OrderBook::check_rewards_weight();
        }

        weight
            .max(T::XSTPool::check_rewards_weight())
            .max(T::XYKPool::check_rewards_weight())
            .max(T::MulticollateralBondingCurvePool::check_rewards_weight())
    }
}

impl<T: Config> Pallet<T> {
    /// List liquidity source types which are enabled on chain, this applies to all DEX'es.
    /// Used in aggregation pallets, such as liquidity-proxy.
    pub fn get_supported_types() -> Vec<LiquiditySourceType> {
        EnabledSourceTypes::<T>::get()
    }
}

impl<T: Config>
    LiquidityRegistry<
        T::DEXId,
        T::AccountId,
        T::AssetId,
        LiquiditySourceType,
        Balance,
        DispatchError,
    > for Pallet<T>
{
    fn list_liquidity_sources(
        input_asset_id: &T::AssetId,
        output_asset_id: &T::AssetId,
        filter: LiquiditySourceFilter<T::DEXId, LiquiditySourceType>,
    ) -> Result<Vec<LiquiditySourceId<T::DEXId, LiquiditySourceType>>, DispatchError> {
        let supported_types = Self::get_supported_types();
        T::DexInfoProvider::ensure_dex_exists(&filter.dex_id)?;
        Ok(supported_types
            .iter()
            .filter_map(|source_type| {
                if filter.matches_index(*source_type)
                    && Self::can_exchange(
                        &LiquiditySourceId::new(filter.dex_id, *source_type),
                        input_asset_id,
                        output_asset_id,
                    )
                {
                    Some(LiquiditySourceId::new(
                        filter.dex_id.clone(),
                        source_type.clone(),
                    ))
                } else {
                    None
                }
            })
            .collect())
    }
}
pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
    use super::*;
    use frame_support::pallet_prelude::*;
    use frame_support::traits::StorageVersion;
    use frame_system::pallet_prelude::*;

    #[pallet::config]
    pub trait Config:
        frame_system::Config + common::Config + trading_pair::Config + assets::Config
    {
        type MockLiquiditySource: LiquiditySource<
            Self::DEXId,
            Self::AccountId,
            Self::AssetId,
            Balance,
            DispatchError,
        >;
        type MockLiquiditySource2: LiquiditySource<
            Self::DEXId,
            Self::AccountId,
            Self::AssetId,
            Balance,
            DispatchError,
        >;
        type MockLiquiditySource3: LiquiditySource<
            Self::DEXId,
            Self::AccountId,
            Self::AssetId,
            Balance,
            DispatchError,
        >;
        type MockLiquiditySource4: LiquiditySource<
            Self::DEXId,
            Self::AccountId,
            Self::AssetId,
            Balance,
            DispatchError,
        >;
        type MulticollateralBondingCurvePool: LiquiditySource<
            Self::DEXId,
            Self::AccountId,
            Self::AssetId,
            Balance,
            DispatchError,
        >;
        type XSTPool: LiquiditySource<
            Self::DEXId,
            Self::AccountId,
            Self::AssetId,
            Balance,
            DispatchError,
        >;
        type XYKPool: LiquiditySource<
            Self::DEXId,
            Self::AccountId,
            Self::AssetId,
            Balance,
            DispatchError,
        >;
        type DexInfoProvider: DexInfoProvider<
            Self::DEXId,
            DEXInfo<Self>,
        >;

        #[cfg(feature = "ready-to-test")] // order-book
        type OrderBook: LiquiditySource<
            Self::DEXId,
            Self::AccountId,
            Self::AssetId,
            Balance,
            DispatchError,
        >;
    }

    /// The current storage version.
    const STORAGE_VERSION: StorageVersion = StorageVersion::new(1);

    #[pallet::pallet]
    #[pallet::generate_store(pub(super) trait Store)]
    #[pallet::storage_version(STORAGE_VERSION)]
    #[pallet::without_storage_info]
    pub struct Pallet<T>(PhantomData<T>);

    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

    #[pallet::call]
    impl<T: Config> Pallet<T> {}

    #[pallet::storage]
    pub type EnabledSourceTypes<T: Config> = StorageValue<_, Vec<LiquiditySourceType>, ValueQuery>;

    #[pallet::genesis_config]
    pub struct GenesisConfig {
        pub source_types: Vec<LiquiditySourceType>,
    }

    #[cfg(feature = "std")]
    impl Default for GenesisConfig {
        fn default() -> Self {
            Self {
                source_types: Default::default(),
            }
        }
    }

    #[pallet::genesis_build]
    impl<T: Config> GenesisBuild<T> for GenesisConfig {
        fn build(&self) {
            EnabledSourceTypes::<T>::put(&self.source_types);
        }
    }
}
