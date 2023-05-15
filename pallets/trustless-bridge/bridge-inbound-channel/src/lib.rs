//! Channel for passing messages from ethereum to substrate.

#![cfg_attr(not(feature = "std"), no_std)]

use bridge_types::traits::{MessageDispatch, Verifier};
use bridge_types::types::{AdditionalEVMInboundData, AdditionalEVMOutboundData, MessageId};
use bridge_types::EVMChainId;
use frame_support::dispatch::DispatchResult;
use frame_support::traits::Get;
use frame_system::ensure_signed;
use sp_core::{H160, U256};
use sp_std::convert::TryFrom;

use events::Envelope;

use sp_runtime::traits::{Convert, Zero};
use sp_runtime::Perbill;
use traits::MultiCurrency;

mod benchmarking;

pub mod weights;
pub use weights::WeightInfo;

#[cfg(test)]
mod test;

mod events;

type BalanceOf<T> = <<T as assets::Config>::Currency as MultiCurrency<
    <T as frame_system::Config>::AccountId,
>>::Balance;

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
    use super::*;
    use crate::events::MessageDispatched;
    use bridge_types::traits::{AppRegistry, MessageStatusNotifier, OutboundChannel};
    use bridge_types::types::MessageStatus;
    use bridge_types::{GenericNetworkId, Log, H256};
    use frame_support::log::{debug, warn};
    use frame_support::pallet_prelude::*;
    use frame_support::traits::StorageVersion;
    use frame_system::pallet_prelude::*;
    use frame_system::RawOrigin;
    use sp_runtime::traits::{Hash, Keccak256};

    #[pallet::config]
    pub trait Config: frame_system::Config + assets::Config + technical::Config {
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

        /// Verifier module for message verification.
        type Verifier: Verifier;

        /// Verifier module for message verification.
        type MessageDispatch: MessageDispatch<Self, EVMChainId, MessageId, AdditionalEVMInboundData>;

        type Hashing: Hash<Output = H256>;

        type MessageStatusNotifier: MessageStatusNotifier<
            Self::AssetId,
            Self::AccountId,
            BalanceOf<Self>,
        >;

        type FeeConverter: Convert<U256, BalanceOf<Self>>;

        /// The base asset as the core asset in all trading pairs
        type FeeAssetId: Get<Self::AssetId>;

        type FeeTechAccountId: Get<Self::TechAccountId>;

        type TreasuryTechAccountId: Get<Self::TechAccountId>;

        type OutboundChannel: OutboundChannel<
            EVMChainId,
            Self::AccountId,
            AdditionalEVMOutboundData,
        >;

        /// Weight information for extrinsics in this pallet
        type WeightInfo: WeightInfo;
    }

    /// InboundChannel contract address on the ethereum side
    #[pallet::storage]
    #[pallet::getter(fn inbound_channel)]
    pub type InboundChannelAddresses<T: Config> =
        StorageMap<_, Identity, EVMChainId, H160, OptionQuery>;

    #[pallet::storage]
    pub type InboundChannelNonces<T: Config> = StorageMap<_, Identity, EVMChainId, u64, ValueQuery>;

    /// Source channel (OutboundChannel contract) on the ethereum side
    #[pallet::storage]
    #[pallet::getter(fn source_channel)]
    pub type ChannelAddresses<T: Config> = StorageMap<_, Identity, EVMChainId, H160, OptionQuery>;

    #[pallet::storage]
    pub type ChannelNonces<T: Config> = StorageMap<_, Identity, EVMChainId, u64, ValueQuery>;

    #[pallet::storage]
    #[pallet::getter(fn reward_fraction)]
    pub(super) type RewardFraction<T: Config> =
        StorageValue<_, Perbill, ValueQuery, DefaultRewardFraction>;

    #[pallet::type_value]
    pub(super) fn DefaultRewardFraction() -> Perbill {
        Perbill::from_percent(80)
    }

    /// The current storage version.
    const STORAGE_VERSION: StorageVersion = StorageVersion::new(1);

    #[pallet::pallet]
    #[pallet::generate_store(pub(super) trait Store)]
    #[pallet::storage_version(STORAGE_VERSION)]
    #[pallet::without_storage_info]
    pub struct Pallet<T>(PhantomData<T>);

    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

    #[pallet::event]
    //#[pallet::generate_deposit(pub(super) fn deposit_event)]
    /// This module has no events
    pub enum Event<T: Config> {}

    #[pallet::error]
    pub enum Error<T> {
        /// Message came from an invalid network.
        InvalidNetwork,
        /// Message came from an invalid outbound channel on the Ethereum side.
        InvalidSourceChannel,
        /// Message has an invalid envelope.
        InvalidEnvelope,
        /// Malformed MessageDispatched event
        InvalidMessageDispatchedEvent,
        /// Message has an unexpected nonce.
        InvalidNonce,
        /// Incorrect reward fraction
        InvalidRewardFraction,
        /// This contract already exists
        ContractExists,
        /// Call encoding failed.
        CallEncodeFailed,
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        #[pallet::call_index(0)]
        #[pallet::weight(<T as Config>::WeightInfo::submit())]
        pub fn submit(
            origin: OriginFor<T>,
            network_id: EVMChainId,
            log: Log,
            proof: <T::Verifier as Verifier>::Proof,
        ) -> DispatchResultWithPostInfo {
            let relayer = ensure_signed(origin)?;
            debug!("Received message from {:?}", relayer);
            // submit message to verifier for verification
            let log_hash = Keccak256::hash_of(&log);
            T::Verifier::verify(network_id.into(), log_hash, &proof)?;

            // Decode log into an Envelope
            let envelope: Envelope<T> =
                Envelope::try_from(log).map_err(|_| Error::<T>::InvalidEnvelope)?;

            ensure!(
                <ChannelAddresses<T>>::get(network_id).ok_or(Error::<T>::InvalidNetwork)?
                    == envelope.channel,
                Error::<T>::InvalidSourceChannel
            );

            // Verify message nonce
            <ChannelNonces<T>>::try_mutate(network_id, |nonce| -> DispatchResult {
                if envelope.nonce != *nonce + 1 {
                    Err(Error::<T>::InvalidNonce.into())
                } else {
                    *nonce += 1;
                    Ok(())
                }
            })?;

            Self::handle_fee(envelope.fee, &relayer);

            let message_id = MessageId::inbound(envelope.nonce);
            T::MessageDispatch::dispatch(
                network_id,
                message_id.into(),
                // TODO: fix
                Default::default(),
                &envelope.payload,
                AdditionalEVMInboundData {
                    source: envelope.source,
                },
            );

            Ok(().into())
        }

        #[pallet::call_index(1)]
        #[pallet::weight(<T as Config>::WeightInfo::message_dispatched())]
        pub fn message_dispatched(
            origin: OriginFor<T>,
            network_id: EVMChainId,
            log: Log,
            proof: <T::Verifier as Verifier>::Proof,
        ) -> DispatchResultWithPostInfo {
            let relayer = ensure_signed(origin)?;
            debug!(
                "message_dispatched: Received MessageDispatched from {:?}",
                relayer
            );
            // submit message to verifier for verification
            let log_hash = Keccak256::hash_of(&log);
            T::Verifier::verify(network_id.into(), log_hash, &proof)?;
            let message_dispatched_event: MessageDispatched = MessageDispatched::try_from(log)
                .map_err(|_| Error::<T>::InvalidMessageDispatchedEvent)?;

            ensure!(
                <InboundChannelAddresses<T>>::get(network_id).ok_or(Error::<T>::InvalidNetwork)?
                    == message_dispatched_event.channel,
                Error::<T>::InvalidSourceChannel
            );

            // Verify message nonce
            <InboundChannelNonces<T>>::try_mutate(network_id, |nonce| -> DispatchResult {
                if message_dispatched_event.nonce != *nonce + 1 {
                    Err(Error::<T>::InvalidNonce.into())
                } else {
                    *nonce += 1;
                    Ok(())
                }
            })?;

            T::MessageStatusNotifier::update_status(
                GenericNetworkId::EVM(network_id),
                MessageId::outbound(message_dispatched_event.nonce)
                    .using_encoded(|v| <T as Config>::Hashing::hash(v)),
                if message_dispatched_event.result {
                    MessageStatus::Done
                } else {
                    MessageStatus::Failed
                },
                None,
            );

            Ok(().into())
        }

        #[pallet::call_index(2)]
        #[pallet::weight(<T as Config>::WeightInfo::register_channel())]
        pub fn register_channel(
            origin: OriginFor<T>,
            network_id: EVMChainId,
            inbound_channel: H160,
            outbound_channel: H160,
        ) -> DispatchResultWithPostInfo {
            ensure_root(origin)?;
            Self::register_channel_inner(network_id, inbound_channel, outbound_channel)?;
            Ok(().into())
        }

        #[pallet::call_index(3)]
        #[pallet::weight(<T as Config>::WeightInfo::set_reward_fraction())]
        pub fn set_reward_fraction(
            origin: OriginFor<T>,
            fraction: Perbill,
        ) -> DispatchResultWithPostInfo {
            ensure_root(origin)?;
            RewardFraction::<T>::set(fraction);
            Ok(().into())
        }
    }

    impl<T: Config> AppRegistry<EVMChainId, H160> for Pallet<T> {
        fn register_app(network_id: EVMChainId, app: H160) -> DispatchResult {
            let target =
                ChannelAddresses::<T>::get(network_id).ok_or(Error::<T>::InvalidNetwork)?;

            let message = bridge_types::channel_abi::RegisterOperatorPayload { operator: app };

            T::OutboundChannel::submit(
                network_id,
                &RawOrigin::Root,
                message
                    .encode()
                    .map_err(|_| Error::<T>::CallEncodeFailed)?
                    .as_ref(),
                AdditionalEVMOutboundData {
                    target,
                    max_gas: 100000u64.into(),
                },
            )?;
            Ok(())
        }

        fn deregister_app(network_id: EVMChainId, app: H160) -> DispatchResult {
            let target =
                ChannelAddresses::<T>::get(network_id).ok_or(Error::<T>::InvalidNetwork)?;

            let message = bridge_types::channel_abi::DeregisterOperatorPayload { operator: app };

            T::OutboundChannel::submit(
                network_id,
                &RawOrigin::Root,
                message
                    .encode()
                    .map_err(|_| Error::<T>::CallEncodeFailed)?
                    .as_ref(),
                AdditionalEVMOutboundData {
                    target,
                    max_gas: 100000u64.into(),
                },
            )?;
            Ok(())
        }
    }

    impl<T: Config> Pallet<T> {
        pub fn register_channel_inner(
            network_id: EVMChainId,
            inbound_channel: H160,
            outbound_channel: H160,
        ) -> DispatchResult {
            ensure!(
                <InboundChannelAddresses<T>>::contains_key(network_id) == false,
                Error::<T>::ContractExists
            );
            <InboundChannelAddresses<T>>::insert(network_id, inbound_channel);

            ensure!(
                <ChannelAddresses<T>>::contains_key(network_id) == false,
                Error::<T>::ContractExists
            );
            <ChannelAddresses<T>>::insert(network_id, outbound_channel);
            Ok(())
        }

        /*
         * Pay the message submission fee into the relayer and treasury account.
         *
         * - If the fee is zero, do nothing
         * - Otherwise, withdraw the fee amount from the DotApp module account, returning a negative imbalance
         * - Figure out the fraction of the fee amount that should be paid to the relayer
         * - Pay the relayer if their account exists, returning a positive imbalance.
         * - Adjust the negative imbalance by offsetting the amount paid to the relayer
         * - Resolve the negative imbalance by depositing it into the treasury account
         */
        pub fn handle_fee(amount: BalanceOf<T>, relayer: &T::AccountId) {
            if amount.is_zero() {
                return;
            }
            let reward_fraction: Perbill = RewardFraction::<T>::get();
            let reward_amount = reward_fraction.mul_ceil(amount);

            if let Err(err) = technical::Pallet::<T>::transfer_out(
                &T::FeeAssetId::get(),
                &T::FeeTechAccountId::get(),
                relayer,
                reward_amount,
            ) {
                warn!("Unable to transfer reward to relayer: {:?}", err);
                return;
            }

            if let Some(treasure_amount) = amount.checked_sub(reward_amount) {
                if let Err(err) = technical::Pallet::<T>::transfer(
                    &T::FeeAssetId::get(),
                    &T::FeeTechAccountId::get(),
                    &T::TreasuryTechAccountId::get(),
                    treasure_amount,
                ) {
                    warn!("Unable to transfer to treasury: {:?}", err);
                }
            }
        }
    }

    #[pallet::genesis_config]
    pub struct GenesisConfig {
        // Ethereum network id, inbound channel address, outbound channel address
        pub networks: Vec<(EVMChainId, H160, H160)>,
        pub reward_fraction: Perbill,
    }

    #[cfg(feature = "std")]
    impl Default for GenesisConfig {
        fn default() -> Self {
            Self {
                networks: Default::default(),
                reward_fraction: Default::default(),
            }
        }
    }

    #[pallet::genesis_build]
    impl<T: Config> GenesisBuild<T> for GenesisConfig {
        fn build(&self) {
            for (network_id, inbound_channel, outbound_channel) in &self.networks {
                Pallet::<T>::register_channel_inner(
                    *network_id,
                    *inbound_channel,
                    *outbound_channel,
                )
                .unwrap();
            }
            RewardFraction::<T>::set(self.reward_fraction);
        }
    }
}
