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

use super::pallet::{Config, Pallet};
use common::generate_storage_instance;
use common::{fixed, Fixed, XSTUSD};
use frame_support::pallet_prelude::{Get, StorageVersion};
use frame_support::pallet_prelude::{StorageValue, ValueQuery};
use frame_support::{log::info, traits::GetStorageVersion as _, weights::Weight};

use crate::{EnabledSymbols, EnabledSynthetics, SyntheticInfo};

generate_storage_instance!(PoolXST, BaseFee);
type OldBaseFee = StorageValue<BaseFeeOldInstance, Fixed, ValueQuery>;

/// Migration which migrates `XSTUSD` synthetic to the new format.
pub fn migrate<T: Config>() -> Weight {
    if Pallet::<T>::on_chain_storage_version() >= 2 {
        info!("Migration to version 2 has already been applied");
        return 0;
    }

    if OldBaseFee::exists() {
        OldBaseFee::kill();
    }

    let xstusd_symbol = T::Symbol::from(common::SymbolName::usd());

    EnabledSynthetics::<T>::insert(
        T::AssetId::from(XSTUSD),
        Some(SyntheticInfo {
            reference_symbol: xstusd_symbol.clone(),
            fee_ratio: fixed!(0.00666),
        }),
    );
    EnabledSymbols::<T>::insert(xstusd_symbol, Some(T::AssetId::from(XSTUSD)));

    StorageVersion::new(2).put::<Pallet<T>>();
    T::DbWeight::get().reads_writes(0, 2)
}

#[cfg(test)]
mod tests;
