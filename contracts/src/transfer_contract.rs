#![cfg_attr(not(feature = "std"), no_std)]
use common::{AssetId32, Balance, PredefinedAssetId};
use ink::primitives::AccountId;
use scale::Encode;

/// It is a part of the runtime dispatchables API.
/// `Ink!` doesn't expose the real enum, so we need a partial definition matching our targets.
/// You should get or count index of the pallet, using `construct_runtime!`, it is zero based
#[derive(Encode)]
pub enum RuntimeCall {
    #[codec(index = 21)]
    Assets(AssetsCall),
}

/// It is a part of a pallet dispatchables API.
/// The indexes can be found in your pallet code's #[pallet::call] section and check #[pallet::call_index(x)] attribute of the call.
/// If these attributes are missing, use source-code order (0-based).
#[derive(Encode)]
pub enum AssetsCall {
    #[codec(index = 1)]
    Transfer {
        asset_id: AssetId32<PredefinedAssetId>,
        to: AccountId,
        amount: Balance,
    },
}

#[ink::contract]
mod asset_contract {
    use crate::transfer_contract::{AssetsCall, RuntimeCall};
    use common::AssetId32;
    use scale::{Decode, Encode};

    #[ink(storage)]
    #[derive(Default)]
    pub struct AssetContract;

    #[derive(Debug, PartialEq, Eq, Encode, Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum RuntimeError {
        CallRuntimeFailed,
    }

    impl AssetContract {
        #[ink(constructor)]
        pub fn new() -> Self {
            Default::default()
        }

        /// Transfer amount of asset from caller to another account.
        /// You may found list of callable extrinsic in `pallet_contracts::Config::CallFilter`
        #[ink(message)]
        pub fn transfer(
            &self,
            asset_id: [u8; 32],
            to: AccountId,
            amount: Balance,
        ) -> Result<(), RuntimeError> {
            self.env()
                .call_runtime(&RuntimeCall::Assets(AssetsCall::Transfer {
                    asset_id: AssetId32::from_bytes(asset_id),
                    to,
                    amount,
                }))
                .map_err(|_| RuntimeError::CallRuntimeFailed)
        }
    }
}

#[cfg(test)]
mod tests {
    use ink::env::DefaultEnvironment;

    fn default_accounts() -> ink::env::test::DefaultAccounts<DefaultEnvironment> {
        ink::env::test::default_accounts::<DefaultEnvironment>()
    }
}
