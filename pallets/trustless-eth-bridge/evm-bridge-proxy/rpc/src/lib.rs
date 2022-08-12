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

use bridge_types::{types::AppKind, EthNetworkId, H160};
use codec::{Codec, Decode, Encode};

use jsonrpsee::{
    core::{Error as RpcError, RpcResult as Result},
    proc_macros::rpc,
    types::error::CallError,
};
use serde::{Deserialize, Serialize};
use sp_api::ProvideRuntimeApi;
use sp_blockchain::HeaderBackend;
use sp_runtime::generic::BlockId;
use sp_runtime::traits::Block as BlockT;

use std::sync::Arc;

pub use evm_bridge_proxy_runtime_api::EvmBridgeProxyAPI as EvmBridgeProxyRuntimeAPI;

#[derive(Eq, PartialEq, Encode, Decode, Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "std", serde(rename_all = "camelCase"))]
pub struct AppWithSupportedAssets<AssetId> {
    kind: AppKind,
    address: H160,
    supported_assets: Vec<AssetId>,
}

#[rpc(server, client)]
pub trait EvmBridgeProxyAPI<BlockHash, AssetId>
where
    BlockHash: Codec,
    AssetId: Codec + Serialize,
{
    #[method(name = "evmBridgeProxy_listApps")]
    fn list_apps(
        &self,
        network_id: EthNetworkId,
        at: Option<BlockHash>,
    ) -> Result<Vec<(AppKind, H160)>>;

    #[method(name = "evmBridgeProxy_listAssets")]
    fn list_supported_assets(
        &self,
        network_id: EthNetworkId,
        at: Option<BlockHash>,
    ) -> Result<Vec<(AppKind, AssetId)>>;

    #[method(name = "evmBridgeProxy_listAppsWithSupportedAssets")]
    fn list_apps_with_supported_assets(
        &self,
        network_id: EthNetworkId,
        at: Option<BlockHash>,
    ) -> Result<Vec<AppWithSupportedAssets<AssetId>>>;
}

pub struct EvmBridgeProxyClient<C, B> {
    client: Arc<C>,
    _marker: std::marker::PhantomData<B>,
}

impl<C, B> EvmBridgeProxyClient<C, B> {
    /// Construct default `Template`.
    pub fn new(client: Arc<C>) -> Self {
        Self {
            client,
            _marker: Default::default(),
        }
    }
}

impl<C, Block, AssetId> EvmBridgeProxyAPIServer<<Block as BlockT>::Hash, AssetId>
    for EvmBridgeProxyClient<C, Block>
where
    Block: BlockT,
    AssetId: Codec + Serialize + Clone,
    C: Send + Sync + 'static,
    C: ProvideRuntimeApi<Block> + HeaderBackend<Block>,
    C::Api: EvmBridgeProxyRuntimeAPI<Block, AssetId>,
{
    fn list_apps(
        &self,
        network_id: EthNetworkId,
        at: Option<<Block as BlockT>::Hash>,
    ) -> Result<Vec<(AppKind, H160)>> {
        let at = BlockId::hash(at.unwrap_or_else(|| self.client.info().best_hash));
        let api = self.client.runtime_api();
        api.list_apps(&at, network_id)
            .map_err(|e| RpcError::Call(CallError::Failed(e.into())))
    }

    fn list_supported_assets(
        &self,
        network_id: EthNetworkId,
        at: Option<<Block as BlockT>::Hash>,
    ) -> Result<Vec<(AppKind, AssetId)>> {
        let at = BlockId::hash(at.unwrap_or_else(|| self.client.info().best_hash));
        let api = self.client.runtime_api();
        api.list_supported_assets(&at, network_id)
            .map_err(|e| RpcError::Call(CallError::Failed(e.into())))
    }

    fn list_apps_with_supported_assets(
        &self,
        network_id: EthNetworkId,
        at: Option<<Block as BlockT>::Hash>,
    ) -> Result<Vec<AppWithSupportedAssets<AssetId>>> {
        let at = BlockId::hash(at.unwrap_or_else(|| self.client.info().best_hash));
        let api = self.client.runtime_api();
        let assets = api
            .list_supported_assets(&at, network_id)
            .map_err(|e| RpcError::Call(CallError::Failed(e.into())))?;
        let apps = api
            .list_apps(&at, network_id)
            .map_err(|e| RpcError::Call(CallError::Failed(e.into())))?
            .into_iter()
            .map(|(kind, address)| {
                let supported_assets = assets
                    .iter()
                    .filter(|(app_kind, _)| *app_kind == kind)
                    .map(|(_, asset_id)| asset_id.clone())
                    .collect();
                AppWithSupportedAssets {
                    kind,
                    address,
                    supported_assets,
                }
            })
            .collect::<Vec<_>>();
        Ok(apps)
    }
}
