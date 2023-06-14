#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(test)]
mod mock;

#[cfg(test)]
mod test;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;
pub mod weights;

use bridge_types::{
    traits::{GasTracker, MessageStatusNotifier, TimepointProvider},
    types::{MessageDirection, MessageStatus},
    Address, GenericAccount, GenericNetworkId, GenericTimepoint, H160, H256,
};
use codec::{Decode, Encode};
use common::Balance;
use frame_support::dispatch::{DispatchResult, RuntimeDebug};
use frame_support::log;
use scale_info::TypeInfo;
use sp_core::U256;
use sp_std::prelude::*;

pub use weights::WeightInfo;

#[derive(Clone, RuntimeDebug, Encode, Decode, PartialEq, Eq, TypeInfo)]
#[scale_info(skip_type_params(T))]
pub struct BridgeRequest<AccountId, AssetId> {
    source: GenericAccount<AccountId>,
    dest: GenericAccount<AccountId>,
    asset_id: AssetId,
    amount: Balance,
    status: MessageStatus,
    start_timepoint: GenericTimepoint,
    end_timepoint: GenericTimepoint,
    direction: MessageDirection,
}

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
    use super::*;
    use bridge_types::{
        substrate::ParachainAccountId,
        traits::BridgeApp,
        types::{BridgeAppInfo, BridgeAssetInfo},
    };
    use frame_support::pallet_prelude::*;
    use frame_system::pallet_prelude::*;
    use traits::MultiCurrency;

    type AccountIdOf<T> = <T as frame_system::Config>::AccountId;
    type BalanceOf<T> = <<T as assets::Config>::Currency as MultiCurrency<AccountIdOf<T>>>::Balance;

    #[pallet::config]
    pub trait Config: frame_system::Config + assets::Config + pallet_timestamp::Config {
        type RuntimeEvent: From<Event> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

        type EthApp: BridgeApp<Self::AccountId, H160, Self::AssetId, Balance>;

        type ERC20App: BridgeApp<Self::AccountId, H160, Self::AssetId, Balance>;

        type SubstrateApp: BridgeApp<Self::AccountId, ParachainAccountId, Self::AssetId, Balance>;

        type HashiBridge: BridgeApp<Self::AccountId, H160, Self::AssetId, Balance>;

        type TimepointProvider: TimepointProvider;

        type WeightInfo: WeightInfo;
    }

    #[pallet::storage]
    #[pallet::getter(fn transactions)]
    pub(super) type Transactions<T: Config> = StorageDoubleMap<
        _,
        Blake2_128Concat,
        (GenericNetworkId, T::AccountId),
        Blake2_128Concat,
        H256,
        BridgeRequest<T::AccountId, T::AssetId>,
        OptionQuery,
    >;

    #[pallet::storage]
    #[pallet::getter(fn sender)]
    pub(super) type Senders<T: Config> = StorageDoubleMap<
        _,
        Twox64Concat,
        GenericNetworkId,
        Blake2_128Concat,
        H256,
        T::AccountId,
        OptionQuery,
    >;

    /// Fee paid for relayed tx on sidechain. Map ((Network ID, Address) => Cumulative Fee Paid).
    #[pallet::storage]
    #[pallet::getter(fn sidechain_fee_paid)]
    pub(super) type SidechainFeePaid<T: Config> =
        StorageDoubleMap<_, Blake2_128Concat, GenericNetworkId, Blake2_128Concat, Address, U256>;

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
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    /// Events for the ETH module.
    pub enum Event {
        RequestStatusUpdate(H256, MessageStatus),
        RefundFailed(H256),
    }

    #[pallet::error]
    pub enum Error<T> {
        PathIsNotAvailable,
        WrongAccountKind,
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        #[pallet::call_index(0)]
        #[pallet::weight(<T as Config>::WeightInfo::burn())]
        pub fn burn(
            origin: OriginFor<T>,
            network_id: GenericNetworkId,
            asset_id: T::AssetId,
            recipient: GenericAccount<T::AccountId>,
            amount: BalanceOf<T>,
        ) -> DispatchResultWithPostInfo {
            let sender = ensure_signed(origin)?;
            match recipient {
                GenericAccount::EVM(recipient) => {
                    if T::HashiBridge::is_asset_supported(network_id, asset_id) {
                        T::HashiBridge::transfer(network_id, asset_id, sender, recipient, amount)?;
                    } else if T::EthApp::is_asset_supported(network_id, asset_id) {
                        T::EthApp::transfer(network_id, asset_id, sender, recipient, amount)?;
                    } else {
                        T::ERC20App::transfer(network_id, asset_id, sender, recipient, amount)?;
                    }
                }
                GenericAccount::Parachain(recipient) => {
                    T::SubstrateApp::transfer(network_id, asset_id, sender, recipient, amount)?;
                }
                GenericAccount::Sora(_) | GenericAccount::Unknown | GenericAccount::Root => {
                    return Err(Error::<T>::WrongAccountKind.into())
                }
            }
            Ok(().into())
        }
    }

    impl<T: Config> Pallet<T> {
        pub fn list_apps() -> Vec<BridgeAppInfo> {
            let mut res = vec![];
            res.extend(T::EthApp::list_apps());
            res.extend(T::ERC20App::list_apps());
            res.extend(T::HashiBridge::list_apps());
            res.extend(T::SubstrateApp::list_apps());
            res
        }

        pub fn list_supported_assets(network_id: GenericNetworkId) -> Vec<BridgeAssetInfo> {
            let mut res = vec![];
            res.extend(T::EthApp::list_supported_assets(network_id));
            res.extend(T::ERC20App::list_supported_assets(network_id));
            res.extend(T::HashiBridge::list_supported_assets(network_id));
            res.extend(T::SubstrateApp::list_supported_assets(network_id));
            res
        }

        pub fn refund(
            network_id: GenericNetworkId,
            message_id: H256,
            beneficiary: GenericAccount<T::AccountId>,
            asset_id: T::AssetId,
            amount: Balance,
        ) -> DispatchResult {
            let GenericAccount::Sora(beneficiary) = beneficiary else {
                return Err(Error::<T>::WrongAccountKind.into());
            };
            if T::HashiBridge::is_asset_supported(network_id, asset_id) {
                T::HashiBridge::refund(network_id, message_id, beneficiary, asset_id, amount)?;
            } else if T::SubstrateApp::is_asset_supported(network_id, asset_id) {
                T::SubstrateApp::refund(network_id, message_id, beneficiary, asset_id, amount)?;
            } else if T::EthApp::is_asset_supported(network_id, asset_id) {
                T::EthApp::refund(network_id, message_id, beneficiary, asset_id, amount)?;
            } else {
                T::ERC20App::refund(network_id, message_id, beneficiary, asset_id, amount)?;
            }
            Ok(())
        }
    }
}

impl<T: Config> GasTracker<Balance> for Pallet<T> {
    /// Records fee paid by relayer for message submission.
    /// - network_id - ethereum network id,
    /// - batch_nonce - batch nonce,
    /// - ethereum_relayer_address - relayer that had paid for the batch submission,
    /// - gas_used - gas paid for batch relaying,
    /// - gas_price - ethereum base fee in the block when batch was submitted.
    fn record_tx_fee(
        network_id: GenericNetworkId,
        batch_nonce: u64,
        ethereum_relayer_address: Address,
        gas_used: U256,
        gas_price: U256,
    ) {
        log::debug!(
            "Record tx fee: batch_nonce={}, ethereum_relayer_address={}, gas_used={}, gas_price={}",
            batch_nonce,
            ethereum_relayer_address,
            gas_used,
            gas_price,
        );

        let tx_fee = gas_used * gas_price;

        SidechainFeePaid::<T>::mutate(
            network_id,
            ethereum_relayer_address,
            |maybe_cumulative_fee| {
                let cumulative_fee = maybe_cumulative_fee.get_or_insert(U256::from(0));
                *cumulative_fee += tx_fee;
            },
        );
    }
}

impl<T: Config> MessageStatusNotifier<T::AssetId, T::AccountId, Balance> for Pallet<T> {
    fn update_status(
        network_id: GenericNetworkId,
        message_id: H256,
        mut new_status: MessageStatus,
        end_timepoint: GenericTimepoint,
    ) {
        let sender = match Senders::<T>::get(network_id, message_id) {
            Some(sender) => sender,
            None => {
                log::warn!(
                    "Message status update called for unknown message: {:?} {:?}",
                    network_id,
                    message_id
                );
                return;
            }
        };
        Transactions::<T>::mutate((network_id, sender), message_id, |req| {
            if let Some(req) = req {
                if new_status == MessageStatus::Failed
                    && req.direction == MessageDirection::Outbound
                {
                    match Pallet::<T>::refund(
                        network_id,
                        message_id,
                        req.source.clone(),
                        req.asset_id,
                        req.amount,
                    ) {
                        Ok(_) => {
                            new_status = MessageStatus::Refunded;
                        }
                        Err(_) => {
                            Self::deposit_event(Event::RefundFailed(message_id));
                        }
                    }
                }
                req.status = new_status;
                req.end_timepoint = end_timepoint;

                Self::deposit_event(Event::RequestStatusUpdate(message_id, new_status));
            }
        })
    }

    fn inbound_request(
        network_id: GenericNetworkId,
        message_id: H256,
        source: GenericAccount<T::AccountId>,
        dest: T::AccountId,
        asset_id: T::AssetId,
        amount: Balance,
        start_timepoint: GenericTimepoint,
        status: MessageStatus,
    ) {
        Self::deposit_event(Event::RequestStatusUpdate(message_id, status));
        Senders::<T>::insert(&network_id, &message_id, &dest);
        Transactions::<T>::insert(
            (&network_id, &dest),
            &message_id,
            BridgeRequest {
                source,
                dest: GenericAccount::Sora(dest.clone()),
                asset_id,
                amount,
                status,
                start_timepoint,
                end_timepoint: T::TimepointProvider::get_timepoint(),
                direction: MessageDirection::Inbound,
            },
        );
    }

    fn outbound_request(
        network_id: GenericNetworkId,
        message_id: H256,
        source: T::AccountId,
        dest: GenericAccount<T::AccountId>,
        asset_id: T::AssetId,
        amount: Balance,
        status: MessageStatus,
    ) {
        Self::deposit_event(Event::RequestStatusUpdate(message_id, status));
        Senders::<T>::insert(&network_id, &message_id, &source);
        Transactions::<T>::insert(
            (&network_id, &source),
            &message_id,
            BridgeRequest {
                source: GenericAccount::Sora(source.clone()),
                dest,
                asset_id,
                amount,
                status,
                start_timepoint: T::TimepointProvider::get_timepoint(),
                end_timepoint: GenericTimepoint::Pending,
                direction: MessageDirection::Outbound,
            },
        );
    }
}
