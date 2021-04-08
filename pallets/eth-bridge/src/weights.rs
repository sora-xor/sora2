//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 2.0.0-rc5

use crate::{IncomingRequestKind, IncomingTransactionRequestKind};
use common::weights::constants::EXTRINSIC_FIXED_WEIGHT;
use common::weights::PresetWeightInfo;
use frame_support::weights::{Pays, Weight};

impl crate::WeightInfo for () {
    fn register_bridge() -> Weight {
        Default::default()
    }
    fn add_asset() -> Weight {
        Default::default()
    }
    fn add_sidechain_token() -> Weight {
        Default::default()
    }
    fn transfer_to_sidechain() -> Weight {
        Default::default()
    }
    fn request_from_sidechain(kind: &IncomingRequestKind) -> (Weight, Pays) {
        let pays = if kind
            == &IncomingRequestKind::Transaction(IncomingTransactionRequestKind::TransferXOR)
        {
            Pays::No
        } else {
            Pays::Yes
        };
        (Default::default(), pays)
    }
    fn add_peer() -> Weight {
        Default::default()
    }
    fn remove_peer() -> Weight {
        Default::default()
    }
    fn force_add_peer() -> Weight {
        Default::default()
    }
    fn prepare_for_migration() -> Weight {
        Default::default()
    }
    fn migrate() -> Weight {
        Default::default()
    }
}

impl<T> crate::WeightInfo for PresetWeightInfo<T> {
    fn register_bridge() -> Weight {
        EXTRINSIC_FIXED_WEIGHT
    }
    fn add_asset() -> Weight {
        EXTRINSIC_FIXED_WEIGHT
    }
    fn add_sidechain_token() -> Weight {
        EXTRINSIC_FIXED_WEIGHT
    }
    fn transfer_to_sidechain() -> Weight {
        10 * EXTRINSIC_FIXED_WEIGHT
    }
    fn request_from_sidechain(kind: &IncomingRequestKind) -> (Weight, Pays) {
        let pays = if kind
            == &IncomingRequestKind::Transaction(IncomingTransactionRequestKind::TransferXOR)
        {
            Pays::No
        } else {
            Pays::Yes
        };
        (EXTRINSIC_FIXED_WEIGHT, pays)
    }
    fn add_peer() -> Weight {
        EXTRINSIC_FIXED_WEIGHT
    }
    fn remove_peer() -> Weight {
        EXTRINSIC_FIXED_WEIGHT
    }
    fn force_add_peer() -> Weight {
        EXTRINSIC_FIXED_WEIGHT
    }
    fn prepare_for_migration() -> Weight {
        EXTRINSIC_FIXED_WEIGHT
    }
    fn migrate() -> Weight {
        EXTRINSIC_FIXED_WEIGHT
    }
}
