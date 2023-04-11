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

use common::weights::constants::EXTRINSIC_FIXED_WEIGHT;
use core::marker::PhantomData;
use frame_support::weights::Weight;

pub struct WeightInfo<T>(PhantomData<T>);

impl<T: frame_system::Config> crate::WeightInfo for WeightInfo<T> {
    fn transfer_to_sidechain() -> Weight {
        Weight::zero()
    }
    fn request_from_sidechain() -> Weight {
        Weight::zero()
    }
    fn register_incoming_request() -> Weight {
        Weight::zero()
    }
    fn finalize_incoming_request() -> Weight {
        Weight::zero()
    }
    fn approve_request() -> Weight {
        Weight::zero()
    }
    fn approve_request_finalize() -> Weight {
        Weight::zero()
    }
    fn abort_request() -> Weight {
        Weight::zero()
    }
    fn register_bridge() -> Weight {
        Default::default()
    }
    fn add_asset() -> Weight {
        Default::default()
    }
    fn add_sidechain_token() -> Weight {
        Default::default()
    }
    fn add_peer() -> Weight {
        Default::default()
    }
    fn remove_peer() -> Weight {
        Default::default()
    }
    fn force_add_peer() -> Weight {
        Default::default()
    }
    fn prepare_for_migration() -> Weight {
        Default::default()
    }
    fn migrate() -> Weight {
        Default::default()
    }
    fn remove_sidechain_asset() -> Weight {
        Default::default()
    }
    fn register_existing_sidechain_asset() -> Weight {
        Default::default()
    }
}

impl crate::WeightInfo for () {
    fn register_bridge() -> Weight {
        EXTRINSIC_FIXED_WEIGHT
    }
    fn add_asset() -> Weight {
        EXTRINSIC_FIXED_WEIGHT
    }
    fn add_sidechain_token() -> Weight {
        EXTRINSIC_FIXED_WEIGHT
    }
    fn transfer_to_sidechain() -> Weight {
        EXTRINSIC_FIXED_WEIGHT.mul(10)
    }
    fn request_from_sidechain() -> Weight {
        EXTRINSIC_FIXED_WEIGHT
    }
    fn add_peer() -> Weight {
        EXTRINSIC_FIXED_WEIGHT
    }
    fn remove_peer() -> Weight {
        EXTRINSIC_FIXED_WEIGHT
    }
    fn force_add_peer() -> Weight {
        EXTRINSIC_FIXED_WEIGHT
    }
    fn prepare_for_migration() -> Weight {
        EXTRINSIC_FIXED_WEIGHT
    }
    fn migrate() -> Weight {
        EXTRINSIC_FIXED_WEIGHT
    }
    fn register_incoming_request() -> Weight {
        EXTRINSIC_FIXED_WEIGHT
    }
    fn finalize_incoming_request() -> Weight {
        EXTRINSIC_FIXED_WEIGHT
    }
    fn approve_request() -> Weight {
        EXTRINSIC_FIXED_WEIGHT
    }
    fn approve_request_finalize() -> Weight {
        EXTRINSIC_FIXED_WEIGHT
    }
    fn abort_request() -> Weight {
        EXTRINSIC_FIXED_WEIGHT
    }
    fn remove_sidechain_asset() -> Weight {
        EXTRINSIC_FIXED_WEIGHT
    }
    fn register_existing_sidechain_asset() -> Weight {
        EXTRINSIC_FIXED_WEIGHT
    }
}
