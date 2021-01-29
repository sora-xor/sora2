use codec::Codec;
use common::InvokeRPCError;
pub use farming_runtime_api::FarmingRuntimeApi;
use jsonrpc_core::{Error as RpcError, ErrorCode, Result};
use jsonrpc_derive::rpc;
use sp_api::ProvideRuntimeApi;
use sp_blockchain::HeaderBackend;
use sp_runtime::{generic::BlockId, traits::Block as BlockT};
use std::sync::Arc;

#[rpc]
pub trait FarmingApi<BlockHash, AccountId, FarmName, FarmInfo, FarmerInfo> {
    #[rpc(name = "farming_getFarmInfo")]
    fn get_farm_info(
        &self,
        who: AccountId,
        name: FarmName,
        at: Option<BlockHash>,
    ) -> Result<Option<FarmInfo>>;

    #[rpc(name = "farming_getFarmerInfo")]
    fn get_farmer_info(
        &self,
        who: AccountId,
        name: FarmName,
        at: Option<BlockHash>,
    ) -> Result<Option<FarmerInfo>>;
}

pub struct FarmingRpc<C, M> {
    client: Arc<C>,
    _marker: std::marker::PhantomData<M>,
}

impl<C, M> FarmingRpc<C, M> {
    pub fn new(client: Arc<C>) -> Self {
        Self {
            client,
            _marker: Default::default(),
        }
    }
}

impl<C, Block, AccountId, FarmName, FarmInfo, FarmerInfo>
    FarmingApi<<Block as BlockT>::Hash, AccountId, FarmName, FarmInfo, FarmerInfo>
    for FarmingRpc<C, Block>
where
    Block: BlockT,
    C: Send + Sync + 'static,
    C: ProvideRuntimeApi<Block> + HeaderBackend<Block>,
    C::Api: FarmingRuntimeApi<Block, AccountId, FarmName, FarmInfo, FarmerInfo>,
    AccountId: Codec,
    FarmName: Codec,
    FarmInfo: Codec,
    FarmerInfo: Codec,
{
    fn get_farm_info(
        &self,
        who: AccountId,
        name: FarmName,
        at: Option<<Block as BlockT>::Hash>,
    ) -> Result<Option<FarmInfo>> {
        let api = self.client.runtime_api();
        let at = BlockId::hash(at.unwrap_or_else(|| self.client.info().best_hash));
        let runtime_api_result = api.get_farm_info(&at, who, name);
        runtime_api_result.map_err(|e| RpcError {
            code: ErrorCode::ServerError(InvokeRPCError::RuntimeError.into()),
            message: "Failed to get Farm Info".into(),
            data: Some(format!("{:?}", e).into()),
        })
    }

    fn get_farmer_info(
        &self,
        who: AccountId,
        name: FarmName,
        at: Option<<Block as BlockT>::Hash>,
    ) -> Result<Option<FarmerInfo>> {
        let api = self.client.runtime_api();
        let at = BlockId::hash(at.unwrap_or_else(|| self.client.info().best_hash));
        let runtime_api_result = api.get_farmer_info(&at, who, name);
        runtime_api_result.map_err(|e| RpcError {
            code: ErrorCode::ServerError(InvokeRPCError::RuntimeError.into()),
            message: "Failed to get Farmer Info".into(),
            data: Some(format!("{:?}", e).into()),
        })
    }
}

pub enum Error {
    RuntimeError,
}

impl Into<i64> for Error {
    fn into(self) -> i64 {
        match self {
            Error::RuntimeError => 1,
        }
    }
}
