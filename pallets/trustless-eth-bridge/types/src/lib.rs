#![cfg_attr(not(feature = "std"), no_std)]

pub mod channel_abi;
pub mod difficulty;
pub mod ethashdata;
pub mod ethashproof;
pub mod header;
pub mod log;
mod mpt;
pub mod network_config;
pub mod receipt;
pub mod traits;
pub mod types;

#[cfg(any(feature = "test", test))]
pub mod test_utils;

use codec::Encode;
pub use ethereum_types::{Address, H160, H256, H64, U256};
use sp_std::vec;
use sp_std::vec::Vec;

pub use header::{Header, HeaderId};
pub use log::Log;
pub use receipt::Receipt;

#[derive(Debug)]
pub enum DecodeError {
    // Unexpected RLP data
    InvalidRLP(rlp::DecoderError),
    // Data does not match expected ABI
    InvalidABI(ethabi::Error),
    // Invalid message payload
    InvalidPayload,
}

impl From<rlp::DecoderError> for DecodeError {
    fn from(err: rlp::DecoderError) -> Self {
        DecodeError::InvalidRLP(err)
    }
}

impl From<ethabi::Error> for DecodeError {
    fn from(err: ethabi::Error) -> Self {
        DecodeError::InvalidABI(err)
    }
}

pub type EthNetworkId = U256;

pub const CHANNEL_INDEXING_PREFIX: &'static [u8] = b"commitment";

pub fn import_digest(network_id: &EthNetworkId, header: &Header) -> Vec<u8>
where
    EthNetworkId: Encode,
    Header: Encode,
{
    let mut digest = vec![];
    network_id.encode_to(&mut digest);
    header.encode_to(&mut digest);
    digest
}
