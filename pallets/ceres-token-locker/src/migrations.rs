use crate::{AssetIdOf, Config, Timestamp, TokenLockInfo, TokenLockerData, Weight};
use common::{convert_block_number_to_timestamp, Balance};
use frame_support::debug;
use frame_support::traits::Get;
use sp_std::vec::Vec;

pub fn migrate<T: Config>() -> Weight {
    debug::RuntimeLogger::init();
    migrate_token_locker_data::<T>()
}

pub fn migrate_token_locker_data<T: Config>() -> Weight {
    let mut weight: u64 = 0;

    let current_timestamp = Timestamp::<T>::get();
    let current_block = frame_system::Pallet::<T>::block_number();
    TokenLockerData::<T>::translate_values::<Vec<(Balance, T::BlockNumber, AssetIdOf<T>)>, _>(
        |v| {
            Some(
                v.into_iter()
                    .map(|(tokens, unlocking_block, asset_id)| {
                        weight += 1;
                        let unlocking_timestamp = convert_block_number_to_timestamp::<T>(
                            unlocking_block,
                            current_block,
                            current_timestamp,
                        );

                        TokenLockInfo {
                            tokens,
                            unlocking_timestamp,
                            asset_id,
                        }
                    })
                    .collect::<Vec<TokenLockInfo<Balance, T::Moment, AssetIdOf<T>>>>(),
            )
        },
    );

    debug::info!(
        target: "runtime",
        "TokenLockInfo migrated to new version with unlocking_timestamp field"
    );

    T::DbWeight::get().reads_writes(weight, weight)
}
