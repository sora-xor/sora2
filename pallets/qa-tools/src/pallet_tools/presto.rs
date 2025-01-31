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

use crate::Config;

use common::{
    AccountIdOf, AssetManager, AssetName, AssetSymbol, AssetType, DEXId, DEXInfo, DexIdOf,
    ExtendedAssetsManager, FromGenericPair, DEFAULT_BALANCE_PRECISION, PRUSD, SBT_PRACS,
    SBT_PRCRDT, SBT_PRINVST, XST,
};
use frame_support::sp_runtime::{DispatchError, DispatchResult};
use permissions::{Scope, BURN, MANAGE_DEX, MINT};

fn system_asset_account_id<T: Config>() -> Result<AccountIdOf<T>, DispatchError> {
    let assets_and_permissions_tech_account_id = T::TechAccountId::from_generic_pair(
        b"SYSTEM_ACCOUNT".to_vec(),
        b"ASSETS_PERMISSIONS".to_vec(),
    );

    technical::Pallet::<T>::tech_account_id_to_account_id(&assets_and_permissions_tech_account_id)
}

fn presto_main_account_id<T: Config>() -> Result<AccountIdOf<T>, DispatchError> {
    let tech_account_id = T::TechAccountId::from_generic_pair(
        presto::TECH_ACCOUNT_PREFIX.to_vec(),
        presto::TECH_ACCOUNT_MAIN.to_vec(),
    );

    technical::Pallet::<T>::tech_account_id_to_account_id(&tech_account_id)
}

fn presto_buffer_account_id<T: Config>() -> Result<AccountIdOf<T>, DispatchError> {
    let tech_account_id = T::TechAccountId::from_generic_pair(
        presto::TECH_ACCOUNT_PREFIX.to_vec(),
        presto::TECH_ACCOUNT_BUFFER.to_vec(),
    );

    technical::Pallet::<T>::tech_account_id_to_account_id(&tech_account_id)
}

pub fn register_presto_assets<T: Config>() -> DispatchResult {
    let system_account_id = system_asset_account_id::<T>()?;
    let presto_account_id = presto_main_account_id::<T>()?;
    let presto_buffer_account_id = presto_buffer_account_id::<T>()?;

    let now = pallet_timestamp::Pallet::<T>::now();

    frame_system::Pallet::<T>::inc_providers(&presto_account_id);

    T::AssetManager::register_asset_id(
        system_account_id.clone(),
        PRUSD.into(),
        AssetSymbol(b"PRUSD".to_vec()),
        AssetName(b"Presto USD".to_vec()),
        DEFAULT_BALANCE_PRECISION,
        0,
        true,
        AssetType::Regulated,
        None,
        None,
    )?;

    T::AssetManager::register_asset_id(
        system_account_id.clone(),
        SBT_PRACS.into_predefined().into(),
        AssetSymbol(b"PRACS".to_vec()),
        AssetName(b"Presto Access".to_vec()),
        0,
        0,
        true,
        AssetType::Soulbound,
        None,
        None,
    )?;
    T::ExtendedAssetsManager::set_metadata(&SBT_PRACS.into_predefined().into(), None, now);
    T::ExtendedAssetsManager::bind_regulated_asset_to_sbt_asset(
        &SBT_PRACS.into_predefined().into(),
        &PRUSD.into(),
    )?;

    T::AssetManager::mint_to(
        &SBT_PRACS.into_predefined().into(),
        &system_account_id,
        &presto_account_id,
        1,
    )?;
    T::AssetManager::mint_to(
        &SBT_PRACS.into_predefined().into(),
        &system_account_id,
        &presto_buffer_account_id,
        1,
    )?;

    T::AssetManager::register_asset_id(
        system_account_id.clone(),
        SBT_PRINVST.into_predefined().into(),
        AssetSymbol(b"PRINVST".to_vec()),
        AssetName(b"Presto Investor".to_vec()),
        0,
        0,
        true,
        AssetType::Soulbound,
        None,
        None,
    )?;
    T::ExtendedAssetsManager::set_metadata(&SBT_PRINVST.into_predefined().into(), None, now);

    T::AssetManager::register_asset_id(
        system_account_id.clone(),
        SBT_PRCRDT.into_predefined().into(),
        AssetSymbol(b"PRCRDT".to_vec()),
        AssetName(b"Presto Creditor".to_vec()),
        0,
        0,
        true,
        AssetType::Soulbound,
        None,
        None,
    )?;
    T::ExtendedAssetsManager::set_metadata(&SBT_PRCRDT.into_predefined().into(), None, now);

    let scopes = [
        Scope::Limited(common::hash(&PRUSD)),
        Scope::Limited(common::hash(&SBT_PRACS)),
        Scope::Limited(common::hash(&SBT_PRINVST)),
        Scope::Limited(common::hash(&SBT_PRCRDT)),
    ];

    let permission_ids = [MINT, BURN];

    for scope in scopes {
        for permission_id in &permission_ids {
            let permission_owner = permissions::Owners::<T>::get(permission_id, scope)
                .pop()
                .unwrap_or(system_account_id.clone());

            permissions::Pallet::<T>::grant_permission_with_scope(
                permission_owner,
                presto_account_id.clone(),
                *permission_id,
                scope,
            )?;
        }
    }

    let dex_id: DexIdOf<T> = DEXId::PolkaswapPresto.into();

    if !dex_manager::DEXInfos::<T>::contains_key(dex_id) {
        dex_manager::DEXInfos::<T>::insert(
            dex_id,
            DEXInfo {
                base_asset_id: PRUSD.into(),
                synthetic_base_asset_id: XST.into(),
                is_public: false,
            },
        );
    }

    permissions::Pallet::<T>::assign_permission(
        system_account_id,
        &presto_account_id,
        MANAGE_DEX,
        Scope::Limited(common::hash(&DEXId::PolkaswapPresto)),
    )?;

    Ok(())
}
