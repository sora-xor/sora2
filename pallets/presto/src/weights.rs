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

use core::marker::PhantomData;
use frame_support::weights::Weight;

// TODO

/// Weight functions needed for presto.
pub trait WeightInfo {
    fn add_presto_manager() -> Weight;
    fn remove_presto_manager() -> Weight;
    fn add_presto_auditor() -> Weight;
    fn remove_presto_auditor() -> Weight;
    fn mint_presto_usd() -> Weight;
    fn burn_presto_usd() -> Weight;
    fn send_presto_usd() -> Weight;
    fn create_deposit_request() -> Weight;
    fn create_withdraw_request() -> Weight;
    fn cancel_request() -> Weight;
    fn approve_deposit_request() -> Weight;
    fn approve_withdraw_request() -> Weight;
    fn decline_request() -> Weight;
    fn create_crop_receipt() -> Weight;
    fn rate_crop_receipt() -> Weight;
    fn decline_crop_receipt() -> Weight;
    fn publish_crop_receipt() -> Weight;
}

pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
    fn add_presto_manager() -> Weight {
        Weight::zero()
    }

    fn remove_presto_manager() -> Weight {
        Weight::zero()
    }

    fn add_presto_auditor() -> Weight {
        Weight::zero()
    }

    fn remove_presto_auditor() -> Weight {
        Weight::zero()
    }

    fn mint_presto_usd() -> Weight {
        Weight::zero()
    }

    fn burn_presto_usd() -> Weight {
        Weight::zero()
    }

    fn send_presto_usd() -> Weight {
        Weight::zero()
    }

    fn create_deposit_request() -> Weight {
        Weight::zero()
    }

    fn create_withdraw_request() -> Weight {
        Weight::zero()
    }

    fn cancel_request() -> Weight {
        Weight::zero()
    }

    fn approve_deposit_request() -> Weight {
        Weight::zero()
    }

    fn approve_withdraw_request() -> Weight {
        Weight::zero()
    }

    fn decline_request() -> Weight {
        Weight::zero()
    }

    fn create_crop_receipt() -> Weight {
        Weight::zero()
    }

    fn rate_crop_receipt() -> Weight {
        Weight::zero()
    }

    fn decline_crop_receipt() -> Weight {
        Weight::zero()
    }

    fn publish_crop_receipt() -> Weight {
        Weight::zero()
    }
}

impl WeightInfo for () {
    fn add_presto_manager() -> Weight {
        Weight::zero()
    }

    fn remove_presto_manager() -> Weight {
        Weight::zero()
    }

    fn add_presto_auditor() -> Weight {
        Weight::zero()
    }

    fn remove_presto_auditor() -> Weight {
        Weight::zero()
    }

    fn mint_presto_usd() -> Weight {
        Weight::zero()
    }

    fn burn_presto_usd() -> Weight {
        Weight::zero()
    }

    fn send_presto_usd() -> Weight {
        Weight::zero()
    }

    fn create_deposit_request() -> Weight {
        Weight::zero()
    }

    fn create_withdraw_request() -> Weight {
        Weight::zero()
    }

    fn cancel_request() -> Weight {
        Weight::zero()
    }

    fn approve_deposit_request() -> Weight {
        Weight::zero()
    }

    fn approve_withdraw_request() -> Weight {
        Weight::zero()
    }

    fn decline_request() -> Weight {
        Weight::zero()
    }

    fn create_crop_receipt() -> Weight {
        Weight::zero()
    }

    fn rate_crop_receipt() -> Weight {
        Weight::zero()
    }

    fn decline_crop_receipt() -> Weight {
        Weight::zero()
    }

    fn publish_crop_receipt() -> Weight {
        Weight::zero()
    }
}
