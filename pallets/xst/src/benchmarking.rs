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

//! XST pool module benchmarking.

#![cfg(feature = "runtime-benchmarks")]

use super::*;

use crate::Pallet as XSTPool;
use common::{fixed, DAI};
use frame_benchmarking::benchmarks;
use frame_system::{EventRecord, RawOrigin};
use sp_std::prelude::*;

fn assert_last_event<T: Config>(generic_event: <T as Config>::Event) {
    let events = frame_system::Pallet::<T>::events();
    let system_event: <T as frame_system::Config>::Event = generic_event.into();
    // compare to the last event record
    let EventRecord { event, .. } = &events[events.len() - 1];
    assert_eq!(event, &system_event);
}

benchmarks! {
    set_reference_asset {
    }: _(
        RawOrigin::Root,
        DAI.into()
    )
    verify {
        assert_last_event::<T>(Event::ReferenceAssetChanged(DAI.into()).into())
    }

    enable_synthetic_asset {
        let reference_symbol = "EURO";
    }: _(
        RawOrigin::Root,
        AssetSymbol(b"XSTEURO".to_vec()),
        AssetName(b"Sora Synthetic EURO".to_vec()),
        reference_symbol.into(),
        fixed!(0)
    )
    verify {
        assert!(XSTPool::<T>::enabled_symbols(T::Symbol::from(reference_symbol)).is_some());
    }

    disable_synthetic_asset {
        let reference_symbol = "EURO";
        XSTPool::<T>::enable_synthetic_asset(
            RawOrigin::Root.into(),
            AssetSymbol(b"XSTEURO".to_vec()),
            AssetName(b"Sora Synthetic EURO".to_vec()),
            reference_symbol.into(),
            fixed!(0),
        )?;
        let asset_id = XSTPool::<T>::enabled_symbols(T::Symbol::from(reference_symbol))
            .expect("Expected enabled synthetic");
    }: _(
        RawOrigin::Root,
        asset_id
    )
    verify {
        assert_last_event::<T>(Event::SyntheticAssetDisabled(asset_id).into())
    }

    set_synthetic_asset_fee {
        let reference_symbol = "EURO";
        XSTPool::<T>::enable_synthetic_asset(
            RawOrigin::Root.into(),
            AssetSymbol(b"XSTEURO".to_vec()),
            AssetName(b"Sora Synthetic EURO".to_vec()),
            reference_symbol.into(),
            fixed!(0),
        )?;
        let asset_id = XSTPool::<T>::enabled_symbols(T::Symbol::from(reference_symbol))
            .expect("Expected enabled synthetic");
        let fee_ratio = fixed!(0.06);
    }: _(
        RawOrigin::Root,
        asset_id.clone(),
        fee_ratio
    )
    verify {
        assert_last_event::<T>(Event::SyntheticAssetFeeChanged(asset_id, fee_ratio).into())
    }

    set_synthetic_base_asset_floor_price {
    }: _(RawOrigin::Root, balance!(200))
    verify {
        assert_last_event::<T>(Event::SyntheticBaseAssetFloorPriceChanged(balance!(200)).into())
    }

    impl_benchmark_test_suite!(Pallet, crate::mock::ExtBuilder::default().build(), crate::mock::Runtime);
}
