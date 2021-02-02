use crate::contract::{MethodId, FUNCTIONS};
use crate::{
    types, Address, AssetIdOf, AssetKind, Decoder, Error, Module, OutgoingRequest, PswapOwners,
    RequestStatus, SignatureParams, Timepoint, Trait,
};
use alloc::{collections::BTreeSet, string::String};
use codec::{Decode, Encode};
use common::prelude::Balance;
use common::{fixed, AssetSymbol, BalancePrecision, PSWAP};
use ethabi::{FixedBytes, Token};
#[allow(unused_imports)]
use frame_support::debug;
use frame_support::sp_runtime::app_crypto::sp_core;
use frame_support::{dispatch::DispatchError, ensure, RuntimeDebug, StorageMap, StorageValue};
use frame_system::RawOrigin;
#[cfg(feature = "std")]
use serde::{Deserialize, Serialize};
use sp_core::{H256, U256};
use sp_std::prelude::*;

pub const MIN_PEERS: usize = 4;
pub const MAX_PEERS: usize = 100;

#[derive(Clone, Encode, Decode, PartialEq, Eq, RuntimeDebug)]
#[cfg_attr(feature = "std", derive(Serialize))]
pub struct IncomingAddToken<T: Trait> {
    pub token_address: Address,
    pub asset_id: T::AssetId,
    pub precision: BalancePrecision,
    pub symbol: AssetSymbol,
    pub tx_hash: H256,
    pub at_height: u64,
    pub timepoint: Timepoint<T>,
}

impl<T: Trait> IncomingAddToken<T> {
    pub fn finalize(&self) -> Result<H256, DispatchError> {
        crate::Module::<T>::register_sidechain_asset(
            self.token_address,
            self.precision,
            self.symbol.clone(),
        )?;
        Ok(self.tx_hash)
    }

    pub fn timepoint(&self) -> Timepoint<T> {
        self.timepoint
    }
}

#[derive(Clone, Encode, Decode, PartialEq, Eq, RuntimeDebug)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct IncomingChangePeers<T: Trait> {
    pub peer_account_id: T::AccountId,
    pub peer_address: Address,
    pub added: bool,
    pub tx_hash: H256,
    pub at_height: u64,
    pub timepoint: Timepoint<T>,
}

impl<T: Trait> IncomingChangePeers<T> {
    pub fn finalize(&self) -> Result<H256, DispatchError> {
        let pending_peer = crate::PendingPeer::<T>::get().ok_or(Error::<T>::NoPendingPeer)?;
        ensure!(
            pending_peer == self.peer_account_id,
            Error::<T>::WrongPendingPeer
        );
        if self.added {
            let account_id = self.peer_account_id.clone();
            bridge_multisig::Module::<T>::add_signatory(
                RawOrigin::Signed(crate::BridgeAccount::<T>::get()).into(),
                account_id.clone(),
            )?;
            crate::Peers::<T>::mutate(|set| set.insert(account_id));
        }
        crate::PendingPeer::<T>::set(None);
        Ok(self.tx_hash)
    }

    pub fn timepoint(&self) -> Timepoint<T> {
        self.timepoint
    }
}

#[derive(Clone, Encode, Decode, PartialEq, Eq, RuntimeDebug)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct IncomingTransfer<T: Trait> {
    pub from: Address,
    pub to: T::AccountId,
    pub asset_id: AssetIdOf<T>,
    pub asset_kind: AssetKind,
    pub amount: Balance,
    pub tx_hash: H256,
    pub at_height: u64,
    pub timepoint: Timepoint<T>,
}

impl<T: Trait> IncomingTransfer<T> {
    pub fn prepare(&self) -> Result<(), DispatchError> {
        if self.asset_kind.is_owned() {
            let bridge_account = crate::BridgeAccount::<T>::get();
            assets::Module::<T>::reserve(self.asset_id, &bridge_account, self.amount)?;
        }
        Ok(())
    }

    pub fn unreserve(&self) {
        if self.asset_kind.is_owned() {
            let bridge_acc = &crate::Module::<T>::bridge_account();
            if let Err(e) = assets::Module::<T>::unreserve(self.asset_id, bridge_acc, self.amount) {
                debug::error!("Unexpected error: {:?}", e);
            }
        }
    }

    pub fn cancel(&self) -> Result<(), DispatchError> {
        self.unreserve();
        Ok(())
    }

    pub fn finalize(&self) -> Result<H256, DispatchError> {
        let bridge_account_id = crate::Module::<T>::bridge_account();
        if self.asset_kind.is_owned() {
            self.unreserve();
            assets::Module::<T>::ensure_can_withdraw(
                &self.asset_id,
                &bridge_account_id,
                self.amount,
            )?;
            assets::Module::<T>::transfer_from(
                &self.asset_id,
                &bridge_account_id,
                &self.to,
                self.amount,
            )?;
        } else {
            assets::Module::<T>::mint_to(
                &self.asset_id,
                &bridge_account_id,
                &self.to,
                self.amount,
            )?;
        }
        Ok(self.tx_hash)
    }

    pub fn timepoint(&self) -> Timepoint<T> {
        self.timepoint
    }
}

#[derive(Clone, Encode, Decode, PartialEq, Eq, RuntimeDebug)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct IncomingClaimPswap<T: Trait> {
    pub account_id: T::AccountId,
    pub eth_address: Address,
    pub tx_hash: H256,
    pub at_height: u64,
    pub timepoint: Timepoint<T>,
}

impl<T: Trait> IncomingClaimPswap<T> {
    pub fn finalize(&self) -> Result<H256, DispatchError> {
        let bridge_account_id = Module::<T>::bridge_account();
        let amount = PswapOwners::get(&self.eth_address).ok_or(Error::<T>::AccountNotFound)?;
        ensure!(amount != fixed!(0), Error::<T>::AlreadyClaimed);
        let empty_balance: Balance = fixed!(0);
        PswapOwners::insert(&self.eth_address, empty_balance);
        assets::Module::<T>::mint_to(&PSWAP.into(), &bridge_account_id, &self.account_id, amount)?;
        Ok(self.tx_hash.clone())
    }

    pub fn timepoint(&self) -> Timepoint<T> {
        self.timepoint
    }
}

pub fn encode_outgoing_request_eth_call<T: Trait>(
    method_id: MethodId,
    request: &OutgoingRequest<T>,
) -> Result<Vec<u8>, Error<T>> {
    let fun_metas = &*FUNCTIONS;
    let fun_meta = fun_metas.get(&method_id).ok_or(Error::UnknownMethodId)?;
    let request_hash = request.hash();
    let request_encoded = request.to_eth_abi(request_hash)?;
    let approves: BTreeSet<SignatureParams> = crate::RequestApproves::get(&request_hash);
    let input_tokens = request_encoded.input_tokens(Some(approves.into_iter().collect()));
    fun_meta
        .function
        .encode_input(&input_tokens)
        .map_err(|_| Error::EthAbiEncodingError)
}

#[derive(Clone, Encode, Decode, PartialEq, Eq, RuntimeDebug)]
#[cfg_attr(feature = "std", derive(Serialize))]
pub struct CancelOutgoingRequest<T: Trait> {
    pub request: OutgoingRequest<T>,
    pub tx_input: Vec<u8>,
    pub tx_hash: H256,
    pub at_height: u64,
    pub timepoint: Timepoint<T>,
}

impl<T: Trait> CancelOutgoingRequest<T> {
    pub fn prepare(&self) -> Result<(), DispatchError> {
        let request_hash = self.request.hash();
        let req_status =
            crate::RequestStatuses::get(&request_hash).ok_or(crate::Error::<T>::Other)?;
        ensure!(
            req_status == RequestStatus::Ready,
            crate::Error::<T>::RequestIsNotReady
        );
        let mut method_id = [0u8; 4];
        method_id.clone_from_slice(&self.tx_input[..4]);
        let expected_input = encode_outgoing_request_eth_call(method_id, &self.request)?;
        ensure!(
            expected_input == self.tx_input,
            crate::Error::<T>::InvalidContractInput
        );
        crate::RequestStatuses::insert(&request_hash, RequestStatus::Frozen);
        Ok(())
    }

    pub fn cancel(&self) -> Result<(), DispatchError> {
        crate::RequestStatuses::insert(&self.request.hash(), RequestStatus::Ready);
        Ok(())
    }

    pub fn finalize(&self) -> Result<H256, DispatchError> {
        self.request.cancel()?;
        let hash = &self.request.hash();
        crate::RequestStatuses::insert(hash, RequestStatus::Failed);
        crate::RequestApproves::take(hash);
        Ok(self.tx_hash)
    }

    pub fn timepoint(&self) -> Timepoint<T> {
        self.timepoint
    }
}

#[derive(Clone, Encode, Decode, PartialEq, Eq, RuntimeDebug)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct OutgoingTransfer<T: Trait> {
    pub from: T::AccountId,
    pub to: Address,
    pub asset_id: AssetIdOf<T>,
    #[cfg_attr(serde, serde(skip))]
    pub amount: Balance,
    pub nonce: T::Index,
}

impl<T: Trait> OutgoingTransfer<T> {
    pub fn to_eth_abi(&self, tx_hash: H256) -> Result<OutgoingTransferEthEncoded, Error<T>> {
        // TODO: Incorrect type (Address != AccountId).
        let from = Address::from_slice(&self.from.encode()[..20]);
        let to = self.to;
        let currency_id;
        if let Some(token_address) = Module::<T>::registered_sidechain_token(&self.asset_id) {
            currency_id = CurrencyIdEncoded::TokenAddress(token_address);
        } else {
            let x = <T::AssetId as Into<H256>>::into(self.asset_id);
            currency_id = CurrencyIdEncoded::AssetId(H256(x.0));
        }
        let amount = U256::from(*self.amount.0.as_bits());
        let tx_hash = H256(tx_hash.0);
        let raw = ethabi::encode_packed(&[
            currency_id.to_token(),
            Token::Uint(types::U256(amount.0)),
            Token::Address(types::H160(to.0)),
            Token::Address(types::H160(from.0)),
            Token::FixedBytes(tx_hash.0.to_vec()),
        ]);
        Ok(OutgoingTransferEthEncoded {
            from,
            to,
            currency_id,
            amount,
            tx_hash,
            raw,
        })
    }

    pub fn prepare(&mut self) -> Result<(), DispatchError> {
        assets::Module::<T>::ensure_can_withdraw(&self.asset_id, &self.from, self.amount)?;
        let bridge_account = crate::BridgeAccount::<T>::get();
        assets::Module::<T>::transfer_from(
            &self.asset_id,
            &self.from,
            &bridge_account,
            self.amount,
        )?;
        assets::Module::<T>::reserve(self.asset_id, &bridge_account, self.amount)?;
        Ok(())
    }

    pub fn validate(&self) -> Result<(), DispatchError> {
        ensure!(
            crate::RegisteredAsset::<T>::get(&self.asset_id).is_some(),
            Error::<T>::UnsupportedToken
        );
        Ok(())
    }

    pub fn finalize(&self) -> Result<(), DispatchError> {
        self.validate()?;
        if let Some(AssetKind::Sidechain) = Module::<T>::registered_asset(&self.asset_id) {
            let bridge_acc = &Module::<T>::bridge_account();
            assets::Module::<T>::unreserve(self.asset_id, bridge_acc, self.amount)?;
            assets::Module::<T>::burn_from(&self.asset_id, bridge_acc, bridge_acc, self.amount)?;
        }
        Ok(())
    }

    pub fn cancel(&self) -> Result<(), DispatchError> {
        let bridge_account = crate::BridgeAccount::<T>::get();
        assets::Module::<T>::unreserve(self.asset_id, &bridge_account, self.amount)?;
        assets::Module::<T>::transfer_from(
            &self.asset_id,
            &crate::Module::<T>::bridge_account(),
            &self.from,
            self.amount,
        )?;
        Ok(())
    }
}

#[derive(Clone, PartialEq, Eq, Encode, Decode, RuntimeDebug)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub enum CurrencyIdEncoded {
    AssetId(H256),
    TokenAddress(Address),
}

impl CurrencyIdEncoded {
    pub fn to_token(&self) -> Token {
        match self {
            CurrencyIdEncoded::AssetId(asset_id) => Token::FixedBytes(asset_id.encode()),
            CurrencyIdEncoded::TokenAddress(address) => {
                Token::Address(types::H160(address.0.clone()))
            }
        }
    }
}

#[derive(Clone, PartialEq, Eq, Encode, Decode, RuntimeDebug)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct OutgoingTransferEthEncoded {
    pub currency_id: CurrencyIdEncoded,
    pub amount: U256,
    pub to: Address,
    pub from: Address,
    pub tx_hash: H256,
    /// EABI-encoded data to be signed.
    pub raw: Vec<u8>,
}

impl OutgoingTransferEthEncoded {
    pub fn input_tokens(&self, signatures: Option<Vec<SignatureParams>>) -> Vec<Token> {
        let mut tokens = vec![
            self.currency_id.to_token(),
            Token::Uint(types::U256(self.amount.0)),
            Token::Address(types::H160(self.to.0)),
            Token::Address(types::H160(self.from.0)),
            Token::FixedBytes(self.tx_hash.0.to_vec()),
        ];

        if let Some(sigs) = signatures {
            let sig_tokens = signature_params_to_tokens(sigs);
            tokens.extend(sig_tokens);
        }
        tokens
    }
}

// TODO: lock the adding token to prevent double-adding.
#[derive(Clone, Encode, Decode, PartialEq, Eq, RuntimeDebug)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct AddAssetOutgoingRequest<T: Trait> {
    pub author: T::AccountId,
    pub asset_id: AssetIdOf<T>,
    pub supply: Balance,
    pub nonce: T::Index,
}

impl<T: Trait> AddAssetOutgoingRequest<T> {
    pub fn to_eth_abi(&self, tx_hash: H256) -> Result<AddAssetRequestEncoded, Error<T>> {
        let hash = H256(tx_hash.0);
        let (symbol, precision, _) = assets::Module::<T>::get_asset_info(&self.asset_id);
        let symbol: String = String::from_utf8_lossy(&symbol.0).into();
        let name = symbol.clone();
        let asset_id_code = <AssetIdOf<T> as Into<H256>>::into(self.asset_id);
        let supply: U256 = U256::from(*self.supply.0.as_bits());
        let sidechain_asset_id = asset_id_code.0.to_vec();
        let raw = ethabi::encode_packed(&[
            Token::String(name.clone()),
            Token::String(symbol.clone()),
            Token::UintSized(precision.into(), 8),
            Token::Uint(types::U256(supply.clone().0)),
            Token::FixedBytes(sidechain_asset_id.clone()),
            Token::FixedBytes(tx_hash.0.to_vec()),
        ]);

        Ok(AddAssetRequestEncoded {
            name,
            symbol,
            decimal: precision,
            supply, // TODO: supply
            sidechain_asset_id,
            hash,
            raw,
        })
    }

    pub fn validate(&self) -> Result<(), DispatchError> {
        ensure!(
            assets::Module::<T>::is_asset_owner(&self.asset_id, &self.author),
            Error::<T>::TokenIsNotOwnedByTheAuthor
        );
        ensure!(
            crate::RegisteredAsset::<T>::get(&self.asset_id).is_none(),
            Error::<T>::TokenIsAlreadyAdded
        );
        Ok(())
    }

    pub fn prepare(&mut self, _validated_state: ()) -> Result<(), DispatchError> {
        Ok(())
    }

    pub fn finalize(&self) -> Result<(), DispatchError> {
        self.validate()?;
        crate::RegisteredAsset::<T>::insert(&self.asset_id, AssetKind::Thischain);
        Ok(())
    }

    pub fn cancel(&self) -> Result<(), DispatchError> {
        Ok(())
    }
}

#[derive(Clone, PartialEq, Eq, Encode, Decode, RuntimeDebug)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct AddAssetRequestEncoded {
    pub name: String,
    pub symbol: String,
    pub decimal: u8,
    pub supply: U256,
    pub sidechain_asset_id: FixedBytes,
    pub hash: H256,
    /// EABI-encoded data to be signed.
    pub raw: Vec<u8>,
}

impl AddAssetRequestEncoded {
    pub fn input_tokens(&self, signatures: Option<Vec<SignatureParams>>) -> Vec<Token> {
        let mut tokens = vec![
            Token::String(self.name.clone()),
            Token::String(self.symbol.clone()),
            Token::Uint(self.decimal.into()),
            Token::Uint(types::U256(self.supply.clone().0)),
            Token::FixedBytes(self.sidechain_asset_id.clone()),
        ];
        if let Some(sigs) = signatures {
            let sig_tokens = signature_params_to_tokens(sigs);
            tokens.extend(sig_tokens);
        }
        tokens
    }
}

#[derive(Clone, Encode, Decode, PartialEq, Eq, RuntimeDebug)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct AddTokenOutgoingRequest<T: Trait> {
    pub author: T::AccountId,
    pub token_address: Address,
    pub ticker: String,
    pub name: String,
    pub decimals: u8,
    pub nonce: T::Index,
}

pub struct Encoder {
    tokens: Vec<Token>,
}

impl Encoder {
    pub fn new() -> Self {
        Encoder { tokens: Vec::new() }
    }

    pub fn write_address(&mut self, val: &Address) {
        self.tokens.push(Token::Address(types::H160(val.0)));
    }

    pub fn write_string(&mut self, val: String) {
        self.tokens.push(Token::String(val));
    }

    pub fn write_u8(&mut self, val: u8) {
        self.tokens.push(Token::Uint(types::U256::from(val)));
    }

    pub fn into_inner(self) -> Vec<Token> {
        self.tokens
    }
}

pub fn signature_params_to_tokens(sig_params: Vec<SignatureParams>) -> Vec<Token> {
    let mut vs = Vec::new();
    let mut rs = Vec::new();
    let mut ss = Vec::new();
    for sig_param in sig_params {
        vs.push(Token::Uint(types::U256::from(sig_param.v)));
        rs.push(Token::FixedBytes(sig_param.r.to_vec()));
        ss.push(Token::FixedBytes(sig_param.s.to_vec()));
    }
    vec![Token::Array(vs), Token::Array(rs), Token::Array(ss)]
}

impl<T: Trait> AddTokenOutgoingRequest<T> {
    pub fn to_eth_abi(&self, tx_hash: H256) -> Result<AddTokenRequestEncoded, Error<T>> {
        let hash = H256(tx_hash.0);
        let token_address = self.token_address.clone();
        let ticker = self.ticker.clone();
        let name = self.name.clone();
        let decimals = self.decimals;
        let raw = ethabi::encode_packed(&[
            Token::Address(types::H160(token_address.0)),
            Token::String(ticker.clone()),
            Token::String(name.clone()),
            Token::UintSized(decimals.into(), 8),
            Token::FixedBytes(tx_hash.0.to_vec()),
        ]);
        Ok(AddTokenRequestEncoded {
            token_address,
            name,
            ticker,
            decimals,
            hash,
            raw,
        })
    }

    pub fn validate(&self) -> Result<AssetSymbol, DispatchError> {
        ensure!(
            crate::RegisteredSidechainAsset::<T>::get(&self.token_address).is_none(),
            Error::<T>::Other
        );
        let symbol = AssetSymbol(self.ticker.as_bytes().to_vec());
        ensure!(
            assets::is_symbol_valid(&symbol),
            assets::Error::<T>::InvalidAssetSymbol
        );
        Ok(symbol)
    }

    pub fn prepare(&mut self, _validated_state: ()) -> Result<(), DispatchError> {
        Ok(())
    }

    pub fn finalize(&self) -> Result<(), DispatchError> {
        let symbol = self.validate()?;
        crate::Module::<T>::register_sidechain_asset(self.token_address, self.decimals, symbol)?;
        Ok(())
    }

    pub fn cancel(&self) -> Result<(), DispatchError> {
        Ok(())
    }
}

#[derive(Clone, PartialEq, Eq, Encode, Decode, RuntimeDebug)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct AddTokenRequestEncoded {
    pub token_address: Address,
    pub ticker: String,
    pub name: String,
    pub decimals: u8,
    pub hash: H256,
    /// EABI-encoded data to be signed.
    pub raw: Vec<u8>,
}

impl AddTokenRequestEncoded {
    pub fn input_tokens(&self, signatures: Option<Vec<SignatureParams>>) -> Vec<Token> {
        let mut tokens = vec![
            Token::Address(types::H160(self.token_address.0)),
            Token::String(self.ticker.clone()),
            Token::String(self.name.clone()),
            Token::Uint(self.decimals.into()),
        ];
        if let Some(sigs) = signatures {
            let sig_tokens = signature_params_to_tokens(sigs);
            tokens.extend(sig_tokens);
        }
        tokens
    }
}

#[derive(Clone, Encode, Decode, PartialEq, Eq, RuntimeDebug)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct AddPeerOutgoingRequest<T: Trait> {
    pub author: T::AccountId,
    pub peer_address: Address,
    pub peer_account_id: T::AccountId,
    pub nonce: T::Index,
}

impl<T: Trait> AddPeerOutgoingRequest<T> {
    pub fn to_eth_abi(&self, tx_hash: H256) -> Result<AddPeerOutgoingRequestEncoded, Error<T>> {
        let tx_hash = H256(tx_hash.0);
        let peer_address = self.peer_address;
        let raw = ethabi::encode_packed(&[
            Token::Address(types::H160(peer_address.clone().0)),
            Token::FixedBytes(tx_hash.0.to_vec()),
        ]);
        Ok(AddPeerOutgoingRequestEncoded {
            peer_address,
            tx_hash,
            raw,
        })
    }

    pub fn validate(&self) -> Result<BTreeSet<T::AccountId>, DispatchError> {
        let peers = crate::Peers::<T>::get();
        ensure!(peers.len() <= MAX_PEERS, Error::<T>::CantAddMorePeers);
        ensure!(
            !peers.contains(&self.peer_account_id),
            Error::<T>::PeerIsAlreadyAdded
        );
        Ok(peers)
    }

    pub fn prepare(&mut self, _validated_state: ()) -> Result<(), DispatchError> {
        let pending_peer = crate::PendingPeer::<T>::get();
        ensure!(pending_peer.is_none(), Error::<T>::TooManyPendingPeers);
        crate::PendingPeer::<T>::set(Some(self.peer_account_id.clone()));
        Ok(())
    }

    pub fn finalize(&self) -> Result<(), DispatchError> {
        let _peers = self.validate()?;
        crate::PeerAccountId::<T>::insert(self.peer_address, self.peer_account_id.clone());
        crate::PeerAddress::<T>::insert(&self.peer_account_id, self.peer_address.clone());
        Ok(())
    }

    pub fn cancel(&self) -> Result<(), DispatchError> {
        crate::PendingPeer::<T>::set(None);
        Ok(())
    }
}

#[derive(Clone, Encode, Decode, PartialEq, Eq, RuntimeDebug)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct RemovePeerOutgoingRequest<T: Trait> {
    pub author: T::AccountId,
    pub peer_account_id: T::AccountId,
    pub peer_address: Address,
    pub nonce: T::Index,
}

impl<T: Trait> RemovePeerOutgoingRequest<T> {
    pub fn to_eth_abi(&self, tx_hash: H256) -> Result<RemovePeerOutgoingRequestEncoded, Error<T>> {
        let tx_hash = H256(tx_hash.0);
        let peer_address = self.peer_address;
        let raw = ethabi::encode_packed(&[
            Token::Address(types::H160(peer_address.clone().0)),
            Token::FixedBytes(tx_hash.0.to_vec()),
        ]);
        Ok(RemovePeerOutgoingRequestEncoded {
            peer_address,
            tx_hash,
            raw,
        })
    }

    pub fn validate(&self) -> Result<BTreeSet<T::AccountId>, DispatchError> {
        let peers = crate::Peers::<T>::get();
        ensure!(peers.len() >= MIN_PEERS, Error::<T>::CantRemoveMorePeers);
        ensure!(
            peers.contains(&self.peer_account_id),
            Error::<T>::UnknownPeerId
        );
        Ok(peers)
    }

    pub fn prepare(&mut self, _validated_state: ()) -> Result<(), DispatchError> {
        let pending_peer = crate::PendingPeer::<T>::get();
        ensure!(pending_peer.is_none(), Error::<T>::TooManyPendingPeers);
        crate::PendingPeer::<T>::set(Some(self.peer_account_id.clone()));
        Ok(())
    }

    pub fn finalize(&self) -> Result<(), DispatchError> {
        let mut peers = self.validate()?;
        bridge_multisig::Module::<T>::remove_signatory(
            RawOrigin::Signed(crate::BridgeAccount::<T>::get()).into(),
            self.peer_account_id.clone(),
        )?;
        peers.remove(&self.peer_account_id);
        crate::Peers::<T>::set(peers);
        Ok(())
    }

    pub fn cancel(&self) -> Result<(), DispatchError> {
        crate::PendingPeer::<T>::set(None);
        Ok(())
    }
}

#[derive(Clone, PartialEq, Eq, Encode, Decode, RuntimeDebug)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct AddPeerOutgoingRequestEncoded {
    pub peer_address: Address,
    pub tx_hash: H256,
    /// EABI-encoded data to be signed.
    pub raw: Vec<u8>,
}

impl AddPeerOutgoingRequestEncoded {
    pub fn input_tokens(&self, signatures: Option<Vec<SignatureParams>>) -> Vec<Token> {
        let mut tokens = vec![
            Token::Address(types::H160(self.peer_address.clone().0)),
            Token::FixedBytes(self.tx_hash.0.to_vec()),
        ];
        if let Some(sigs) = signatures {
            let sig_tokens = signature_params_to_tokens(sigs);
            tokens.extend(sig_tokens);
        }
        tokens
    }
}

#[derive(Clone, PartialEq, Eq, Encode, Decode, RuntimeDebug)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct RemovePeerOutgoingRequestEncoded {
    pub peer_address: Address,
    pub tx_hash: H256,
    /// EABI-encoded data to be signed.
    pub raw: Vec<u8>,
}

impl RemovePeerOutgoingRequestEncoded {
    pub fn input_tokens(&self, signatures: Option<Vec<SignatureParams>>) -> Vec<Token> {
        let mut tokens = vec![
            Token::Address(types::H160(self.peer_address.clone().0)),
            Token::FixedBytes(self.tx_hash.0.to_vec()),
        ];
        if let Some(sigs) = signatures {
            let sig_tokens = signature_params_to_tokens(sigs);
            tokens.extend(sig_tokens);
        }
        tokens
    }
}

pub fn parse_hash_from_call<T: Trait>(
    tokens: Vec<Token>,
    tx_hash_arg_pos: usize,
) -> Result<H256, DispatchError> {
    tokens
        .get(tx_hash_arg_pos)
        .cloned()
        .and_then(Decoder::<T>::parse_h256)
        .ok_or(Error::<T>::FailedToParseTxHashInCall.into())
}
