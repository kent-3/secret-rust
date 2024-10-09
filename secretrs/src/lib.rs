//! SecretRS re-exports [`cosmrs`] at the top-level of the crate.
//! This allows `secretrs` to be used as a drop-in replacement for `cosmrs`.
//!
//! # Feature flags
//!
//! The default features are `bip32`, `getrandom`, and `grpc-core`.
//!
//! - `bip32`: Enables the `bip32` feature in `cosmrs`.
//! - `getrandom`: Enables the `getrandom` feature in `cosmrs`.
//! - `grpc-core`: Enables the use of [`tonic`] generated gRPC query and service clients.
//! - `grpc`: Enables [`tonic::transport`], which provides a fully featured HTTP/2 [`Channel`] built on top of [`tokio`], [`hyper`] and [`tower`].
//! - `rpc`: Enables the `rpc` feature in `cosmrs`.
//!
//! # WASM support
//! SecretRS has support for gRPC-web in browsers, thanks to [`tonic_web_wasm_client`]. When building for the `wasm32-unknown-unknown` target, create each [`tonic::client::Grpc<T>`] using a [`tonic_web_wasm_client::Client`] instead of [`tonic::transport::Channel`]. The `grpc` feature must be disabled (this will disable the default transport layer of tonic).
//!
//! [`cosmrs`]: https://docs.rs/cosmrs
//! [`tonic`]: https://docs.rs/tonic
//! [`tonic::transport`]: https://docs.rs/tonic/latest/tonic/transport/index.html
//! [`Channel`]: https://docs.rs/tonic/latest/tonic/transport/struct.Channel.html
//! [`tokio`]: https://docs.rs/tokio
//! [`hyper`]: https://docs.rs/hyper
//! [`tower`]: https://docs.rs/tower
//! [`tonic_web_wasm_client`]: https://docs.rs/tonic_web_wasm_client
//! [`tonic_web_wasm_client::Client`]: https://docs.rs/tonic-web-wasm-client/latest/tonic_web_wasm_client/struct.Client.html

#![warn(
    missing_debug_implementations,
    missing_docs,
    rust_2018_idioms,
    unreachable_pub
)]
#![deny(rustdoc::broken_intra_doc_links)]
#![doc(test(no_crate_inject, attr(deny(rust_2018_idioms))))]
#![cfg_attr(docsrs, feature(doc_cfg))]

pub mod compute;
#[cfg(feature = "grpc-core")]
pub mod grpc_clients;
pub mod utils;

pub use cosmrs::*;
pub use secret_sdk_proto::{self as proto, SECRET_VERSION};
