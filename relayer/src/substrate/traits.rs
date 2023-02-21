use std::time::Duration;

use crate::prelude::*;
use beefy_light_client::ProvedSubstrateBridgeMessage;
use bridge_common::{
    beefy_types::{BeefyMMRLeaf, Commitment, ValidatorProof, ValidatorSet},
    simplified_mmr_proof::SimplifiedMMRProof,
};
use bridge_types::{types::ParachainMessage, GenericNetworkId, SubNetworkId, H256};
use common::Balance;
use sp_runtime::{
    traits::{AtLeast32BitUnsigned, Member},
    DeserializeOwned,
};
use subxt::{
    config::Parameter,
    metadata::DecodeStaticType,
    storage::{address::Yes, StaticStorageAddress},
    tx::{Signer, StaticTxPayload},
};

use super::BlockNumberOrHash;

pub type KeyPair = sp_core::sr25519::Pair;

#[derive(Clone, Copy, Debug)]
pub struct ParachainConfig;

#[derive(Clone, Copy, Debug)]
pub struct MainnetConfig;

pub trait ConfigExt: Clone + core::fmt::Debug {
    type Config: subxt::Config + Clone;
    type Event: Decode + core::fmt::Debug + Send + Sync + 'static;
    type BlockNumber: AtLeast32BitUnsigned
        + Parameter
        + Member
        + Copy
        + Into<u64>
        + Into<BlockNumberOrHash>
        + Into<<Self::Config as subxt::Config>::BlockNumber>
        + From<<Self::Config as subxt::Config>::BlockNumber>
        + Serialize
        // + Deserialize
        + DeserializeOwned;
    type Hash: Parameter
        + Member
        + Copy
        + Serialize
        // + Deserialize
        + DeserializeOwned
        + AsRef<[u8]>
        + AsMut<[u8]>
        + Into<BlockNumberOrHash>
        + From<H256>
        + Into<<Self::Config as subxt::Config>::Hash>
        + From<<Self::Config as subxt::Config>::Hash>;
    type Signer: Signer<Self::Config> + Clone + Sync + Send + 'static;

    fn average_block_time() -> Duration;
}

pub trait SenderConfig: ConfigExt + 'static {
    fn current_validator_set() -> StaticStorageAddress<DecodeStaticType<ValidatorSet>, Yes, Yes, ()>;

    fn next_validator_set() -> StaticStorageAddress<DecodeStaticType<ValidatorSet>, Yes, Yes, ()>;

    fn current_session_index() -> StaticStorageAddress<DecodeStaticType<u32>, Yes, Yes, ()>;

    fn network_id() -> StaticStorageAddress<DecodeStaticType<SubNetworkId>, Yes, Yes, ()>;

    fn bridge_outbound_nonce(
        network_id: GenericNetworkId,
    ) -> StaticStorageAddress<DecodeStaticType<u64>, Yes, Yes, Yes>;
}

pub trait ReceiverConfig: ConfigExt {
    type SubmitSignatureCommitment: Encode;
    type SubmitMessagesCommitment: Encode;

    fn submit_signature_commitment(
        network_id: SubNetworkId,
        commitment: Commitment,
        validator_proof: ValidatorProof,
        latest_mmr_leaf: BeefyMMRLeaf,
        proof: SimplifiedMMRProof,
    ) -> StaticTxPayload<Self::SubmitSignatureCommitment>;

    fn submit_messages_commitment(
        network_id: SubNetworkId,
        message: ProvedSubstrateBridgeMessage<Vec<ParachainMessage<Balance>>>,
    ) -> StaticTxPayload<Self::SubmitMessagesCommitment>;

    fn current_validator_set(
        network_id: SubNetworkId,
    ) -> StaticStorageAddress<DecodeStaticType<ValidatorSet>, Yes, (), Yes>;

    fn next_validator_set(
        network_id: SubNetworkId,
    ) -> StaticStorageAddress<DecodeStaticType<ValidatorSet>, Yes, (), Yes>;

    fn latest_beefy_block(
        network_id: SubNetworkId,
    ) -> StaticStorageAddress<DecodeStaticType<u64>, Yes, Yes, Yes>;

    fn substrate_bridge_inbound_nonce(
        network_id: SubNetworkId,
    ) -> StaticStorageAddress<DecodeStaticType<u64>, Yes, Yes, Yes>;

    fn network_id() -> StaticStorageAddress<DecodeStaticType<SubNetworkId>, Yes, Yes, ()>;
}

impl ConfigExt for ParachainConfig {
    type Config = parachain_gen::DefaultConfig;
    type Event = parachain_runtime::Event;
    type BlockNumber = u32;
    type Hash = H256;
    type Signer = subxt::tx::PairSigner<Self::Config, KeyPair>;

    fn average_block_time() -> Duration {
        Duration::from_secs(12)
    }
}

impl ConfigExt for MainnetConfig {
    type Config = substrate_gen::DefaultConfig;
    type Event = mainnet_runtime::Event;
    type BlockNumber = u32;
    type Hash = H256;
    type Signer = subxt::tx::PairSigner<Self::Config, KeyPair>;

    fn average_block_time() -> Duration {
        Duration::from_secs(6)
    }
}

impl SenderConfig for ParachainConfig {
    fn current_session_index() -> StaticStorageAddress<DecodeStaticType<u32>, Yes, Yes, ()> {
        parachain_runtime::storage().session().current_index()
    }

    fn network_id() -> StaticStorageAddress<DecodeStaticType<SubNetworkId>, Yes, Yes, ()> {
        parachain_runtime::storage()
            .beefy_light_client()
            .this_network_id()
    }

    fn bridge_outbound_nonce(
        network_id: GenericNetworkId,
    ) -> StaticStorageAddress<DecodeStaticType<u64>, Yes, Yes, Yes> {
        match network_id {
            GenericNetworkId::Sub(network_id) => parachain_runtime::storage()
                .substrate_bridge_outbound_channel()
                .channel_nonces(network_id),
            GenericNetworkId::EVM(_chain_id) => {
                unimplemented!("Bridge from parachain to EVM network is supported")
            }
        }
    }

    fn current_validator_set() -> StaticStorageAddress<DecodeStaticType<ValidatorSet>, Yes, Yes, ()>
    {
        parachain_runtime::storage().beefy_mmr().beefy_authorities()
    }

    fn next_validator_set() -> StaticStorageAddress<DecodeStaticType<ValidatorSet>, Yes, Yes, ()> {
        parachain_runtime::storage()
            .beefy_mmr()
            .beefy_next_authorities()
    }
}

impl SenderConfig for MainnetConfig {
    fn current_session_index() -> StaticStorageAddress<DecodeStaticType<u32>, Yes, Yes, ()> {
        mainnet_runtime::storage().session().current_index()
    }

    fn network_id() -> StaticStorageAddress<DecodeStaticType<SubNetworkId>, Yes, Yes, ()> {
        mainnet_runtime::storage()
            .beefy_light_client()
            .this_network_id()
    }

    fn bridge_outbound_nonce(
        network_id: GenericNetworkId,
    ) -> StaticStorageAddress<DecodeStaticType<u64>, Yes, Yes, Yes> {
        match network_id {
            GenericNetworkId::Sub(network_id) => mainnet_runtime::storage()
                .substrate_bridge_outbound_channel()
                .channel_nonces(network_id),
            GenericNetworkId::EVM(chain_id) => mainnet_runtime::storage()
                .bridge_outbound_channel()
                .channel_nonces(chain_id),
        }
    }

    fn current_validator_set() -> StaticStorageAddress<DecodeStaticType<ValidatorSet>, Yes, Yes, ()>
    {
        mainnet_runtime::storage().mmr_leaf().beefy_authorities()
    }

    fn next_validator_set() -> StaticStorageAddress<DecodeStaticType<ValidatorSet>, Yes, Yes, ()> {
        mainnet_runtime::storage()
            .mmr_leaf()
            .beefy_next_authorities()
    }
}

impl ReceiverConfig for MainnetConfig {
    type SubmitSignatureCommitment =
        mainnet_runtime::beefy_light_client::calls::SubmitSignatureCommitment;

    type SubmitMessagesCommitment =
        mainnet_runtime::substrate_bridge_inbound_channel::calls::Submit;

    fn submit_signature_commitment(
        network_id: SubNetworkId,
        commitment: Commitment,
        validator_proof: ValidatorProof,
        latest_mmr_leaf: BeefyMMRLeaf,
        proof: SimplifiedMMRProof,
    ) -> StaticTxPayload<Self::SubmitSignatureCommitment> {
        mainnet_runtime::tx()
            .beefy_light_client()
            .submit_signature_commitment(
                network_id,
                commitment,
                validator_proof,
                latest_mmr_leaf,
                proof,
            )
    }

    fn submit_messages_commitment(
        network_id: SubNetworkId,
        message: ProvedSubstrateBridgeMessage<Vec<ParachainMessage<Balance>>>,
    ) -> subxt::tx::StaticTxPayload<Self::SubmitMessagesCommitment> {
        mainnet_runtime::tx()
            .substrate_bridge_inbound_channel()
            .submit(network_id, message)
    }

    fn current_validator_set(
        network_id: SubNetworkId,
    ) -> StaticStorageAddress<DecodeStaticType<ValidatorSet>, Yes, (), Yes> {
        mainnet_runtime::storage()
            .beefy_light_client()
            .current_validator_set(network_id)
    }

    fn next_validator_set(
        network_id: SubNetworkId,
    ) -> StaticStorageAddress<DecodeStaticType<ValidatorSet>, Yes, (), Yes> {
        mainnet_runtime::storage()
            .beefy_light_client()
            .next_validator_set(network_id)
    }

    fn latest_beefy_block(
        network_id: SubNetworkId,
    ) -> StaticStorageAddress<DecodeStaticType<u64>, Yes, Yes, Yes> {
        mainnet_runtime::storage()
            .beefy_light_client()
            .latest_beefy_block(network_id)
    }

    fn substrate_bridge_inbound_nonce(
        network_id: SubNetworkId,
    ) -> StaticStorageAddress<DecodeStaticType<u64>, Yes, Yes, Yes> {
        mainnet_runtime::storage()
            .substrate_bridge_inbound_channel()
            .channel_nonces(network_id)
    }

    fn network_id() -> StaticStorageAddress<DecodeStaticType<SubNetworkId>, Yes, Yes, ()> {
        mainnet_runtime::storage()
            .beefy_light_client()
            .this_network_id()
    }
}

impl ReceiverConfig for ParachainConfig {
    type SubmitSignatureCommitment =
        parachain_runtime::beefy_light_client::calls::SubmitSignatureCommitment;

    type SubmitMessagesCommitment =
        parachain_runtime::substrate_bridge_inbound_channel::calls::Submit;

    fn submit_signature_commitment(
        network_id: SubNetworkId,
        commitment: Commitment,
        validator_proof: ValidatorProof,
        latest_mmr_leaf: BeefyMMRLeaf,
        proof: SimplifiedMMRProof,
    ) -> StaticTxPayload<Self::SubmitSignatureCommitment> {
        parachain_runtime::tx()
            .beefy_light_client()
            .submit_signature_commitment(
                network_id,
                commitment,
                validator_proof,
                latest_mmr_leaf,
                proof,
            )
    }

    fn submit_messages_commitment(
        network_id: SubNetworkId,
        message: ProvedSubstrateBridgeMessage<Vec<ParachainMessage<Balance>>>,
    ) -> subxt::tx::StaticTxPayload<Self::SubmitMessagesCommitment> {
        parachain_runtime::tx()
            .substrate_bridge_inbound_channel()
            .submit(network_id, message)
    }

    fn current_validator_set(
        network_id: SubNetworkId,
    ) -> StaticStorageAddress<DecodeStaticType<ValidatorSet>, Yes, (), Yes> {
        parachain_runtime::storage()
            .beefy_light_client()
            .current_validator_set(network_id)
    }

    fn next_validator_set(
        network_id: SubNetworkId,
    ) -> StaticStorageAddress<DecodeStaticType<ValidatorSet>, Yes, (), Yes> {
        parachain_runtime::storage()
            .beefy_light_client()
            .next_validator_set(network_id)
    }

    fn latest_beefy_block(
        network_id: SubNetworkId,
    ) -> StaticStorageAddress<DecodeStaticType<u64>, Yes, Yes, Yes> {
        parachain_runtime::storage()
            .beefy_light_client()
            .latest_beefy_block(network_id)
    }

    fn substrate_bridge_inbound_nonce(
        network_id: SubNetworkId,
    ) -> StaticStorageAddress<DecodeStaticType<u64>, Yes, Yes, Yes> {
        parachain_runtime::storage()
            .substrate_bridge_inbound_channel()
            .channel_nonces(network_id)
    }

    fn network_id() -> StaticStorageAddress<DecodeStaticType<SubNetworkId>, Yes, Yes, ()> {
        parachain_runtime::storage()
            .beefy_light_client()
            .this_network_id()
    }
}
