#![doc = include_str!("../README.md")]

//! # SecretRS -
//!
//! SecretRS re-exports `cosmrs` at the top-level of the crate.
//! This allows `secretrs` to be used as a drop-in replacement for `cosmrs`.

pub use cosmrs::*;

pub mod compute;

#[cfg(feature = "grpc-core")]
pub mod clients;

pub mod utils;

// TODO - this really belongs in a separate crate
// #[cfg(feature = "dev")]
// pub mod dev;

// Experimental
pub mod client;

pub use crate::utils::EncryptionUtils;

pub use secret_sdk_proto::{self as proto, SECRET_VERSION};
