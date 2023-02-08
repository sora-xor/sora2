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
use frame_support::traits::Get;
use frame_support::weights::Weight;
use sp_std::marker::PhantomData;

pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> crate::WeightInfo for WeightInfo<T> {
    // Weights generated by the benchmark
    // Maxim will integrate those later himself
    /*
     fn claim_rewards() -> Weight {
        (159_135_000 as Weight)
            .saturating_add(T::DbWeight::get().reads(7 as Weight))
            .saturating_add(T::DbWeight::get().writes(6 as Weight))
    }
    fn distribute_limits(n: u32, ) -> Weight {
        (0 as Weight)
            // Standard Error: 47_000
            .saturating_add((17_729_000 as Weight).saturating_mul(n as Weight))
            .saturating_add(T::DbWeight::get().reads(2 as Weight))
            .saturating_add(T::DbWeight::get().reads((1 as Weight).saturating_mul(n as Weight)))
            .saturating_add(T::DbWeight::get().writes((1 as Weight).saturating_mul(n as Weight)))
    }
    fn distribute_market_maker_rewards(n: u32, m: u32, ) -> Weight {
        (0 as Weight)
            // Standard Error: 108_000
            .saturating_add((43_264_000 as Weight).saturating_mul(n as Weight))
            // Standard Error: 108_000
            .saturating_add((16_423_000 as Weight).saturating_mul(m as Weight))
            .saturating_add(T::DbWeight::get().reads(2 as Weight))
            .saturating_add(T::DbWeight::get().reads((3 as Weight).saturating_mul(n as Weight)))
            .saturating_add(T::DbWeight::get().reads((1 as Weight).saturating_mul(m as Weight)))
            .saturating_add(T::DbWeight::get().writes(1 as Weight))
            .saturating_add(T::DbWeight::get().writes((3 as Weight).saturating_mul(n as Weight)))
            .saturating_add(T::DbWeight::get().writes((1 as Weight).saturating_mul(m as Weight)))
    }
    */
    fn claim_incentives() -> Weight {
        (740_250_000 as Weight)
            .saturating_add(T::DbWeight::get().reads(10 as Weight))
            .saturating_add(T::DbWeight::get().writes(5 as Weight))
    }

    fn on_initialize(_n: u32) -> Weight {
        100_000_000 as Weight // TODO: benchmark
    }

    fn claim_crowdloan_rewards() -> Weight {
        EXTRINSIC_FIXED_WEIGHT
    }

    // Storage: VestedRewards TotalRewards (r:1 w:1)
    // Storage: VestedRewards Rewards (r:2 w:2)
    /// The range of component `n` is `[0, 100]`.
    fn update_rewards(n: u32) -> Weight {
        (6_689_000 as Weight)
            // Standard Error: 2_000
            .saturating_add((4_609_000 as Weight).saturating_mul(n as Weight))
            .saturating_add(T::DbWeight::get().reads(1 as Weight))
            .saturating_add(T::DbWeight::get().reads((1 as Weight).saturating_mul(n as Weight)))
            .saturating_add(T::DbWeight::get().writes(1 as Weight))
            .saturating_add(T::DbWeight::get().writes((1 as Weight).saturating_mul(n as Weight)))
    }
}

impl crate::WeightInfo for () {
    fn claim_incentives() -> Weight {
        EXTRINSIC_FIXED_WEIGHT
    }

    fn on_initialize(_n: u32) -> Weight {
        EXTRINSIC_FIXED_WEIGHT
    }

    fn claim_crowdloan_rewards() -> Weight {
        EXTRINSIC_FIXED_WEIGHT
    }

    fn update_rewards(_: u32) -> Weight {
        EXTRINSIC_FIXED_WEIGHT
    }
}
