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

pub mod beefy_subscription;
pub mod traits;
pub mod types;

use std::ops::{Deref, DerefMut};
use std::sync::Arc;

use crate::prelude::*;
use bridge_types::types::AuxiliaryDigest;
use bridge_types::H256;
use common::{AssetName, AssetSymbol, Balance, ContentSource, Description};
use pallet_mmr_rpc::MmrApiClient;
use sp_core::Bytes;
use sp_mmr_primitives::{EncodableOpaqueLeaf, Proof};
use sp_runtime::traits::AtLeast32BitUnsigned;
use std::sync::RwLock;
pub use substrate_gen::{runtime, DefaultConfig};
use subxt::events::EventDetails;
use subxt::metadata::DecodeWithMetadata;
pub use subxt::rpc::Subscription;
use subxt::rpc::{rpc_params, RpcClientT};
use subxt::storage::address::Yes;
use subxt::storage::StorageAddress;
use subxt::tx::{Signer, TxEvents};
pub use types::*;

// Find first occurence of value in storage with increasing values
pub async fn binary_search_first_occurence<N: AtLeast32BitUnsigned, T: PartialOrd, F, Fut>(
    low: N,
    high: N,
    value: T,
    f: F,
) -> AnyResult<Option<N>>
where
    F: Fn(N) -> Fut,
    Fut: futures::Future<Output = AnyResult<Option<T>>>,
{
    let mut low = low;
    let mut high = high;
    while low < high {
        let mid = (high.clone() + low.clone()) / 2u32.into();
        let found_value = f(mid.clone()).await?;
        match found_value {
            None => low = mid + 1u32.into(),
            Some(found_value) if found_value < value => low = mid + 1u32.into(),
            _ => high = mid,
        }
    }
    // If value between blocks can increase more than by 1
    if f(low.clone()).await? >= Some(value) {
        Ok(Some(low))
    } else {
        Ok(None)
    }
}

pub fn event_to_string<T: ConfigExt>(ev: EventDetails) -> String {
    let input = &mut ev.bytes();
    let phase = subxt::events::Phase::decode(input);
    let event = T::Event::decode(input);
    format!("(Phase: {:?}, Event: {:?})", phase, event)
}

pub fn log_tx_events<T: ConfigExt>(events: TxEvents<T::Config>) {
    for ev in events.iter() {
        match ev {
            Ok(ev) => {
                debug!("{}", event_to_string::<T>(ev));
            }
            Err(err) => {
                warn!("Failed to decode event: {:?}", err);
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct ClonableClient(Arc<jsonrpsee::async_client::Client>);

impl RpcClientT for ClonableClient {
    fn request_raw<'a>(
        &'a self,
        method: &'a str,
        params: Option<Box<jsonrpsee::core::JsonRawValue>>,
    ) -> subxt::rpc::RpcFuture<'a, Box<jsonrpsee::core::JsonRawValue>> {
        self.0.request_raw(method, params)
    }

    fn subscribe_raw<'a>(
        &'a self,
        sub: &'a str,
        params: Option<Box<jsonrpsee::core::JsonRawValue>>,
        unsub: &'a str,
    ) -> subxt::rpc::RpcFuture<'a, subxt::rpc::RpcSubscription> {
        self.0.subscribe_raw(sub, params, unsub)
    }
}

#[derive(Debug, Clone)]
pub struct UnsignedClient<T: ConfigExt> {
    api: ApiInner<T>,
    client: ClonableClient,
}

impl<T: ConfigExt> UnsignedClient<T> {
    pub async fn new(url: impl Into<String>) -> AnyResult<Self> {
        let url: Uri = url.into().parse()?;
        let (sender, receiver) =
            jsonrpsee::client_transport::ws::WsTransportClientBuilder::default()
                .build(url)
                .await?;
        let client = jsonrpsee::async_client::ClientBuilder::default()
            .max_notifs_per_subscription(4096)
            .build_with_tokio(sender, receiver);
        let client = ClonableClient(Arc::new(client));
        let api = ApiInner::<T>::from_rpc_client(client.clone()).await?;
        Ok(Self { api, client })
    }

    pub fn rpc(&self) -> &jsonrpsee::async_client::Client {
        &self.client.0
    }

    pub fn mmr(&self) -> &impl pallet_mmr_rpc::MmrApiClient<BlockHash<T>, BlockNumber<T>> {
        self.rpc()
    }

    pub fn beefy(
        &self,
    ) -> &impl beefy_gadget_rpc::BeefyApiClient<types::EncodedBeefyCommitment, BlockHash<T>> {
        self.rpc()
    }

    pub fn assets(
        &self,
    ) -> &impl assets_rpc::AssetsAPIClient<
        BlockHash<T>,
        AccountId<T>,
        AssetId,
        Balance,
        Option<assets_runtime_api::BalanceInfo<Balance>>,
        Option<
            assets_runtime_api::AssetInfo<
                AssetId,
                AssetSymbol,
                AssetName,
                u8,
                ContentSource,
                Description,
            >,
        >,
        Vec<
            assets_runtime_api::AssetInfo<
                AssetId,
                AssetSymbol,
                AssetName,
                u8,
                ContentSource,
                Description,
            >,
        >,
        Vec<AssetId>,
    > {
        self.rpc()
    }

    pub async fn bridge_commitments(
        &self,
        hash: H256,
    ) -> AnyResult<bridge_channel_rpc::Commitment> {
        Ok(
            bridge_channel_rpc::BridgeChannelAPIClient::commitment(self.rpc(), hash)
                .await?
                .ok_or(anyhow!(
                    "Connect to substrate server with enabled offhcain indexing"
                ))?,
        )
    }

    pub async fn auxiliary_digest(&self, at: Option<BlockHash<T>>) -> AnyResult<AuxiliaryDigest> {
        let res = leaf_provider_rpc::LeafProviderAPIClient::latest_digest(self.rpc(), at).await?;
        Ok(res.unwrap_or_default())
    }

    pub async fn substrate_bridge_commitments(
        &self,
        hash: H256,
    ) -> AnyResult<substrate_bridge_channel_rpc::Commitment<Balance>> {
        Ok(
            substrate_bridge_channel_rpc::BridgeChannelAPIClient::commitment(self.rpc(), hash)
                .await?
                .ok_or(anyhow!(
                    "Connect to substrate server with enabled offhcain indexing"
                ))?,
        )
    }

    pub async fn beefy_start_block(&self) -> AnyResult<u64> {
        let latest_finalized_hash = self.api().rpc().finalized_head().await?;
        let latest_finalized_number = self
            .api()
            .rpc()
            .block(Some(latest_finalized_hash))
            .await?
            .expect("should exist")
            .block
            .header
            .number()
            .clone();
        let mmr_leaves = self
            .storage_fetch_or_default(&runtime::storage().mmr().number_of_leaves(), ())
            .await?;
        let beefy_start_block = latest_finalized_number.into().saturating_sub(mmr_leaves);
        debug!("Beefy started at: {}", beefy_start_block);
        Ok(beefy_start_block)
    }

    pub async fn offchain_local_get(
        &self,
        storage: StorageKind,
        key: Vec<u8>,
    ) -> AnyResult<Option<Vec<u8>>> {
        let res = self
            .api()
            .rpc()
            .request::<Option<Bytes>>(
                "offchain_localStorageGet",
                rpc_params![storage.as_string(), Bytes(key)],
            )
            .await?;
        Ok(res.map(|x| x.0))
    }

    pub async fn mmr_generate_proof(
        &self,
        block_number: BlockNumber<T>,
        at: Option<BlockHash<T>>,
    ) -> AnyResult<LeafProof<T>>
    where
        BlockNumber<T>: Serialize,
    {
        let res = self.mmr().generate_proof(block_number, at).await?;
        let leaf = MmrLeaf::<T>::decode(
            &mut &*EncodableOpaqueLeaf::decode(&mut res.leaf.as_ref())?
                .into_opaque_leaf()
                .0,
        )?;
        let proof = Proof::<MmrHash>::decode(&mut res.proof.as_ref())?;
        Ok(LeafProof {
            leaf,
            proof,
            block_hash: res.block_hash,
        })
    }

    pub fn api(&self) -> &ApiInner<T> {
        &self.api
    }

    pub async fn header<N: Into<BlockNumberOrHash>>(&self, at: N) -> AnyResult<Header<T>> {
        let hash = self.block_hash(at).await?;
        let header = self
            .api()
            .rpc()
            .header(Some(hash.into()))
            .await?
            .ok_or(anyhow::anyhow!("Header not found"))?;
        Ok(header)
    }

    pub async fn block_number<N: Into<BlockNumberOrHash>>(
        &self,
        at: N,
    ) -> AnyResult<BlockNumber<T>> {
        let header = self.header(at).await?;
        Ok(BlockNumber::<T>::from(header.number().clone()))
    }

    pub async fn finalized_head(&self) -> AnyResult<BlockHash<T>> {
        let hash = self.api().rpc().finalized_head().await?;
        Ok(hash.into())
    }

    pub async fn block_hash<N: Into<BlockNumberOrHash>>(&self, at: N) -> AnyResult<BlockHash<T>> {
        let block_number = match at.into() {
            BlockNumberOrHash::Number(n) => Some(n),
            BlockNumberOrHash::Hash(h) => return Ok(h.into()),
            BlockNumberOrHash::Best => None,
        };
        let res = self
            .api()
            .rpc()
            .block_hash(block_number.map(Into::into))
            .await?
            .ok_or(anyhow::anyhow!("Block not found"))?;
        Ok(res.into())
    }

    pub async fn block<N: Into<BlockNumberOrHash>>(
        &self,
        at: N,
    ) -> AnyResult<ChainBlock<T::Config>> {
        let hash = self.block_hash(at).await?;
        let block = self
            .api()
            .rpc()
            .block(Some(hash.into()))
            .await?
            .ok_or(anyhow::anyhow!("Block not found"))?;
        Ok(block)
    }

    pub async fn storage_fetch<N, Address>(
        &self,
        address: &Address,
        hash: N,
    ) -> AnyResult<Option<<Address::Target as DecodeWithMetadata>::Target>>
    where
        Address: StorageAddress<IsFetchable = Yes>,
        N: Into<BlockNumberOrHash>,
    {
        let hash = self.block_hash(hash).await?;
        let res = self
            .api()
            .storage()
            .fetch(address, Some(hash.into()))
            .await?;
        Ok(res)
    }

    pub async fn storage_fetch_or_default<N, Address>(
        &self,
        address: &Address,
        hash: N,
    ) -> AnyResult<<Address::Target as DecodeWithMetadata>::Target>
    where
        Address: StorageAddress<IsFetchable = Yes, IsDefaultable = Yes>,
        N: Into<BlockNumberOrHash>,
    {
        let hash = self.block_hash(hash).await?;
        let res = self
            .api()
            .storage()
            .fetch_or_default(address, Some(hash.into()))
            .await?;
        Ok(res)
    }

    pub async fn signed(self, signer: PairSigner<T>) -> AnyResult<SignedClient<T>> {
        SignedClient::<T>::new(self, signer).await
    }
}

#[derive(Clone)]
pub struct SignedClient<T: ConfigExt> {
    inner: UnsignedClient<T>,
    key: PairSigner<T>,
    nonce: Arc<RwLock<Option<Index<T>>>>,
}

impl<T: ConfigExt> SignedClient<T> {
    pub async fn new(client: UnsignedClient<T>, key: PairSigner<T>) -> AnyResult<Self> {
        let res = Self {
            inner: client,
            key,
            nonce: Arc::new(RwLock::new(None)),
        };
        res.load_nonce().await?;
        Ok(res)
    }

    pub fn account_id(&self) -> AccountId<T> {
        self.key.account_id().clone()
    }

    pub async fn submit_extrinsic<P: subxt::tx::TxPayload>(&self, xt: &P) -> AnyResult<()>
    where
        <<<T as ConfigExt>::Config as subxt::Config>::ExtrinsicParams as subxt::tx::ExtrinsicParams<
            <<T as ConfigExt>::Config as subxt::Config>::Index,
            <<T as ConfigExt>::Config as subxt::Config>::Hash,
        >>::OtherParams: Default,
    {
        if let Some(validation) = xt.validation_details() {
            debug!(
                "Submitting extrinsic: {}::{}",
                validation.pallet_name, validation.call_name
            );
        } else {
            debug!("Submitting extrinsic without validation data");
        }
        let res = self
            .api()
            .tx()
            .sign_and_submit_then_watch_default(xt, self)
            .await?
            .wait_for_in_block()
            .await?
            .wait_for_success()
            .await?;
        log_tx_events::<T>(res);
        Ok(())
    }

    pub async fn load_nonce(&self) -> AnyResult<()> {
        let nonce = self
            .inner
            .api()
            .rpc()
            .system_account_next_index(&self.key.account_id())
            .await?;
        self.set_nonce(nonce);
        Ok(())
    }

    pub fn unsigned(self) -> UnsignedClient<T> {
        self.inner
    }

    pub fn api(&self) -> &ApiInner<T> {
        &self.inner.api()
    }

    pub fn set_nonce(&self, index: Index<T>) {
        let mut nonce = self.nonce.write().expect("poisoned");
        *nonce = Some(index);
    }
}

impl<T: ConfigExt> Deref for SignedClient<T> {
    type Target = UnsignedClient<T>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<T: ConfigExt> DerefMut for SignedClient<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl<T: ConfigExt> Signer<T::Config> for SignedClient<T> {
    fn account_id(&self) -> &AccountId<T> {
        self.key.account_id()
    }

    fn nonce(&self) -> Option<Index<T>> {
        let res = *self.nonce.read().expect("poisoned");
        self.nonce
            .write()
            .expect("poisoned")
            .as_mut()
            .map(|nonce| *nonce += 1u32.into());
        res
    }

    fn sign(&self, extrinsic: &[u8]) -> Signature<T> {
        self.key.sign(extrinsic)
    }

    fn address(&self) -> Address<T> {
        self.key.address()
    }
}
