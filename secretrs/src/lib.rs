//! # SecretRS -
//!
//! SecretRS re-exports `cosmrs` at the top-level of the crate.
//! This allows `secretrs` to be used as a drop-in replacement for `cosmrs`.

pub mod compute;

#[cfg(feature = "grpc-core")]
pub mod grpc_clients;
pub mod utils;

pub use utils::EncryptionUtils;

pub use cosmrs::*;
pub use secret_sdk_proto::{self as proto, SECRET_VERSION};
