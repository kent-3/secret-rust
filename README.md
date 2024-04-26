# Secret Rust

_Insert cool description here._

## Crates

| Name                 | Description                | crates.io                                                               | docs.rs                                                             | CI Build                                                   |
| -------------------- | -------------------------- | ----------------------------------------------------------------------- | ------------------------------------------------------------------- | ---------------------------------------------------------- |
| [`secretrs`]         | Secret SDK for Rust        | [![crates.io][secretrs-crate-img]][secretrs-crate-link]                 | [![docs.rs][secretrs-docs-img]][secretrs-docs-link]                 | [![CI][secretrs-ci-img]][secretrs-ci-link]                 |
| [`secret‑sdk‑proto`] | Proto and gRPC definitions | [![crates.io][secret-sdk-proto-crate-img]][secret-sdk-proto-crate-link] | [![docs.rs][secret-sdk-proto-docs-img]][secret-sdk-proto-docs-link] | [![CI][secret-sdk-proto-ci-img]][secret-sdk-proto-ci-link] |

## Goals

- use as much upstream stuff as possible from [cosmos-rust](https://github.com/cosmos/cosmos-rust)
- use [tonic](https://crates.io/crates/tonic) gRPC clients for everything
- support gRPC-web client for wasm32 target
- (bonus) gRPC-gateway client

## Plan

- [x] `proto-build` crate like cosmos-rust and tendermint-rs each have
- [x] `secret-sdk-proto` crate that re-exports `cosmos-sdk-proto` and adds secret protobuf structs
- [x] `secretrs` crate that re-exports `cosmrs` and adds secret type conversions
  - [x] `secret-utils` provide encryption and decryption tools for use with any client
- [ ] `secret-grpc` crate with full gRPC client implementation
- [ ] `secret-grpc-gateway` crate with full gRPC-gateway (REST) client implementation

## Building Proto files from source

The proto-build crate in this repo clones and rebuilds proto files for all other crates,
simply make the required edits in [main.rs](proto-build/src/main.rs) and run

    cargo run

## Minimum Supported Rust Version

Rust **1.72**

[//]: # "crates"
[`secretrs`]: https://github.com/kent-3/secret-rust/tree/main/secretrs
[`secret‑sdk‑proto`]: https://github.com/kent-3/secret-rust/tree/main/secret-sdk-proto
[//]: # "badges"
[secretrs-crate-img]: https://img.shields.io/crates/v/secretrs.svg?logo=rust
[secretrs-crate-link]: https://crates.io/crates/secretrs
[secretrs-docs-img]: https://docs.rs/secretrs/badge.svg
[secretrs-docs-link]: https://docs.rs/secretrs/
[secretrs-ci-img]: https://github.com/kent-3/secret-rust/workflows/secretrs/badge.svg
[secretrs-ci-link]: https://github.com/kent-3/secret-rust/actions/workflows/secretrs.yml
[secret-sdk-proto-crate-img]: https://img.shields.io/crates/v/secret-sdk-proto.svg?logo=rust
[secret-sdk-proto-crate-link]: https://crates.io/crates/secret-sdk-proto
[secret-sdk-proto-docs-img]: https://docs.rs/secret-sdk-proto/badge.svg
[secret-sdk-proto-docs-link]: https://docs.rs/secret-sdk-proto/
[secret-sdk-proto-ci-img]: https://github.com/kent-3/secret-rust/workflows/secret-sdk-proto/badge.svg
[secret-sdk-proto-ci-link]: https://github.com/kent-3/secret-rust/actions/workflows/secret-sdk-proto.yml
