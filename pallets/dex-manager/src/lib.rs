#![cfg_attr(not(feature = "std"), no_std)]

use assets::AssetIdOf;
use common::{hash, prelude::EnsureDEXManager, ManagementMode};
use frame_support::{
    decl_error, decl_event, decl_module, decl_storage, dispatch::DispatchResult, ensure,
    sp_runtime::DispatchError, weights::Weight, IterableStorageMap,
};
use frame_system::{self as system, ensure_signed, RawOrigin};
use permissions::{Scope, INIT_DEX, MANAGE_DEX};
use sp_std::vec::Vec;

mod weights;

mod benchmarking;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

type DEXInfo<T> = common::prelude::DEXInfo<AssetIdOf<T>>;

pub trait WeightInfo {
    fn initialize_dex() -> Weight;
    fn set_fee() -> Weight;
    fn set_protocol_fee() -> Weight;
}

pub trait Trait: common::Trait + assets::Trait {
    type Event: From<Event<Self>> + Into<<Self as system::Trait>::Event>;

    /// Weight information for extrinsics in this pallet.
    type WeightInfo: WeightInfo;
}

decl_storage! {
    trait Store for Module<T: Trait> as DEXManager {
        // TODO: compare performance with separate tables
        pub DEXInfos get(fn dex_id): map hasher(twox_64_concat) T::DEXId => Option<DEXInfo<T>>;
    }
    add_extra_genesis {
        config(dex_list): Vec<(T::DEXId, DEXInfo<T>)>;

        build(|config: &GenesisConfig<T>| {
            config.dex_list.iter().for_each(|(dex_id, dex_info)| {
                DEXInfos::<T>::insert(dex_id.clone(), dex_info);
            })
        })
    }
}

decl_event!(
    pub enum Event<T>
    where
        DEXId = <T as common::Trait>::DEXId,
    {
        /// New DEX has been registered. [DEX Id]
        DEXInitialized(DEXId),
        /// Default fee setting has been changed. [DEX Id, Swap fee in basis points]
        FeeChanged(DEXId, u16),
        /// Default protocol fee setting has been changed. [DEX Id, Protocol fee in basis points]
        ProtocolFeeChanged(DEXId, u16),
    }
);

decl_error! {
    pub enum Error for Module<T: Trait> {
        /// DEX with given id is already registered.
        DEXIdAlreadyExists,
        /// DEX with given Id is not registered.
        DEXDoesNotExist,
        /// Numeric value provided as fee is not valid, e.g. out of basis-point range.
        InvalidFeeValue,
        /// Account with given Id is not registered.
        InvalidAccountId,
    }
}

decl_module! {
    pub struct Module<T: Trait> for enum Call
    where
        origin: T::Origin,
    {
        type Error = Error<T>;

        fn deposit_event() = default;

        /// Initialize DEX in network with given Id, Base Asset, if fees are not given then defaults are applied.
        ///
        /// - `dex_id`: ID of the exchange.
        /// - `fee`: value of fee on swaps in basis points.
        /// - `protocol_fee`: value of fee fraction for protocol beneficiary in basis points.
        #[weight = <T as Trait>::WeightInfo::initialize_dex()]
        pub fn initialize_dex(origin, dex_id: T::DEXId, base_asset_id: T::AssetId, owner_account_id: T::AccountId, is_public: bool) -> DispatchResult {
            let who = ensure_signed(origin)?;
            permissions::Module::<T>::check_permission(who.clone(), INIT_DEX)?;
            ensure!(!DEXInfos::<T>::contains_key(&dex_id), Error::<T>::DEXIdAlreadyExists);
            // Construct DEX information.
            let new_dex_info = DEXInfo::<T> {
                base_asset_id,
                is_public,
            };
            DEXInfos::<T>::insert(dex_id.clone(), new_dex_info);
            // Create permission for designated owner account.
            match permissions::Module::<T>::assign_permission(
                owner_account_id.clone(),
                &owner_account_id,
                MANAGE_DEX,
                Scope::Limited(hash(&dex_id))
            ) {
                Err(permissions::Error::<T>::PermissionAlreadyExists) => {}
                result => result?,
            }
            Self::deposit_event(RawEvent::DEXInitialized(dex_id));
            Ok(())
        }
    }
}

impl<T: Trait> EnsureDEXManager<T::DEXId, T::AccountId, DispatchError> for Module<T> {
    fn ensure_can_manage<OuterOrigin>(
        dex_id: &T::DEXId,
        origin: OuterOrigin,
        mode: ManagementMode,
    ) -> Result<Option<T::AccountId>, DispatchError>
    where
        OuterOrigin: Into<Result<RawOrigin<T::AccountId>, OuterOrigin>>,
    {
        match origin.into() {
            Ok(RawOrigin::Signed(who)) => {
                let dex_info = Self::get_dex_info(&dex_id)?;
                // If DEX is public, anyone can manage it, otherwise confirm ownership.
                if !dex_info.is_public || mode != ManagementMode::Public {
                    Self::ensure_direct_manager(&dex_id, &who)?;
                }
                Ok(Some(who))
            }
            _ => Err(Error::<T>::InvalidAccountId.into()),
        }
    }
}

impl<T: Trait> Module<T> {
    pub fn get_dex_info(dex_id: &T::DEXId) -> Result<DEXInfo<T>, DispatchError> {
        Ok(DEXInfos::<T>::get(&dex_id).ok_or(Error::<T>::DEXDoesNotExist)?)
    }

    pub fn ensure_dex_exists(dex_id: &T::DEXId) -> DispatchResult {
        ensure!(
            DEXInfos::<T>::contains_key(&dex_id),
            Error::<T>::DEXDoesNotExist
        );
        Ok(())
    }

    pub fn list_dex_ids() -> Vec<T::DEXId> {
        DEXInfos::<T>::iter().map(|(k, _)| k).collect()
    }

    fn ensure_direct_manager(dex_id: &T::DEXId, who: &T::AccountId) -> DispatchResult {
        permissions::Module::<T>::check_permission_with_scope(
            who.clone(),
            MANAGE_DEX,
            &Scope::Limited(hash(&dex_id)),
        )
        .map_err(|e| e.into())
    }
}
