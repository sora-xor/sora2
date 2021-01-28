#![cfg_attr(not(feature = "std"), no_std)]

#[macro_use]
extern crate alloc;
use alloc::string::String;

sp_api::decl_runtime_apis! {
    pub trait IrohaMigrationAPI {
        fn needs_migration(iroha_address: String) -> bool;
    }
}
