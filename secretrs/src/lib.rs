//! ## Re-exports
//!
//! SecretRS re-exports the following crates for easy access:
//!
//! - `cosmrs`: re-exported as `secretrs`
//! - `secret-sdk-proto`: re-exported as `secretrs::proto`

// This allows `secretrs` to act like a drop-in replacement for `cosmrs`.
pub use cosmrs::*;

pub mod compute;

#[cfg(feature = "grpc-core")]
pub mod clients;

pub mod utils;

#[cfg(feature = "dev")]
pub mod dev;

// Experimental mods
pub mod secret_rs;

pub use crate::{
    secret_rs::{account, Wallet},
    utils::EncryptionUtils,
};

pub use secret_sdk_proto::{self as proto, SECRET_VERSION};
