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

pub mod init {
    use crate::*;
    use common::{KEN, KUSD};
    use core::marker::PhantomData;
    use frame_support::log::error;
    use frame_support::pallet_prelude::Weight;
    use frame_support::traits::OnRuntimeUpgrade;
    use permissions::{Scope, BURN, MINT};
    use sp_core::Get;

    pub struct RegisterTreasuryTechAccount<T>(PhantomData<T>);

    /// Registers Kensetsu Treasury technical account
    impl<T: Config + permissions::Config + technical::Config> OnRuntimeUpgrade
        for RegisterTreasuryTechAccount<T>
    {
        fn on_runtime_upgrade() -> Weight {
            let tech_account = <T>::TreasuryTechAccount::get();
            match technical::Pallet::<T>::register_tech_account_id_if_not_exist(&tech_account) {
                Ok(()) => <T as frame_system::Config>::DbWeight::get().writes(1),
                Err(err) => {
                    error!(
                        "Failed to register technical account: {:?}, error: {:?}",
                        tech_account, err
                    );
                    <T as frame_system::Config>::DbWeight::get().reads(1)
                }
            }
        }
    }

    pub struct GrantPermissionsTreasuryTechAccount<T>(PhantomData<T>);

    impl<T: Config + permissions::Config + technical::Config> OnRuntimeUpgrade
        for GrantPermissionsTreasuryTechAccount<T>
    {
        fn on_runtime_upgrade() -> Weight {
            let mut weight = <T as frame_system::Config>::DbWeight::get().reads(1);
            if let Ok(technical_account_id) = technical::Pallet::<T>::tech_account_id_to_account_id(
                &T::TreasuryTechAccount::get(),
            ) {
                for token in &[KEN, KUSD] {
                    let scope = Scope::Limited(common::hash(token));
                    for permission_id in &[MINT, BURN] {
                        match permissions::Pallet::<T>::assign_permission(
                            technical_account_id.clone(),
                            &technical_account_id,
                            *permission_id,
                            scope,
                        ) {
                            Ok(()) => {
                                weight += <T as frame_system::Config>::DbWeight::get().writes(1)
                            }
                            Err(err) => {
                                error!(
                                "Failed to grant permission to technical account id: {:?}, error: {:?}",
                                technical_account_id, err
                            );
                                weight += <T as frame_system::Config>::DbWeight::get().reads(1);
                            }
                        }
                    }
                }
            }

            weight
        }
    }
}

/// Due to bug in stability fee update some extra KUSD were minted, this migration burns and sets
/// correct amounts.
pub mod stage_correction {
    use crate::{CDPDepository, CollateralInfos, Config};
    use assets::AssetIdOf;
    use common::AssetInfoProvider;
    use common::Balance;
    use frame_support::dispatch::Weight;
    use frame_support::traits::OnRuntimeUpgrade;
    use sp_arithmetic::traits::Zero;
    use sp_core::Get;
    use std::collections::BTreeMap;
    use std::marker::PhantomData;

    pub struct CorrectKusdBalances<T>(PhantomData<T>);

    impl<T: Config + permissions::Config + technical::Config> OnRuntimeUpgrade
        for CorrectKusdBalances<T>
    {
        fn on_runtime_upgrade() -> Weight {
            let mut weight = Weight::zero();

            let mut collateral_debts: BTreeMap<AssetIdOf<T>, Balance> = BTreeMap::new();
            let mut total_debt = Balance::zero();

            for (_, cdp) in CDPDepository::<T>::iter() {
                *collateral_debts.entry(cdp.collateral_asset_id).or_default() += cdp.debt;
                total_debt += cdp.debt;
                weight += <T as frame_system::Config>::DbWeight::get().reads(1);
            }

            for asset_id in CollateralInfos::<T>::iter_keys() {
                CollateralInfos::<T>::mutate(asset_id, |collateral_info| {
                    let collateral_info = collateral_info.as_mut().unwrap();
                    collateral_info.kusd_supply =
                        *collateral_debts.get(&asset_id).unwrap_or(&Balance::zero());
                });
                weight += <T as frame_system::Config>::DbWeight::get().writes(1);
            }

            // burn KUSD on tech account
            let treasury_account_id = technical::Pallet::<T>::tech_account_id_to_account_id(
                &T::TreasuryTechAccount::get(),
            )
            .unwrap();
            let balance =
                T::AssetInfoProvider::free_balance(&T::KusdAssetId::get(), &treasury_account_id)
                    .unwrap();
            let to_burn = balance - total_debt;
            assets::Pallet::<T>::burn_from(
                &T::KusdAssetId::get(),
                &treasury_account_id,
                &treasury_account_id,
                to_burn,
            )
            .unwrap();

            weight += <T as frame_system::Config>::DbWeight::get().writes(1);
            weight
        }
    }
}
