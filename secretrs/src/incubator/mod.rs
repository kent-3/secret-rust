// #![allow(unused)]

pub mod account;
mod query;
mod secret_network_client;
mod tx;

// I think I will abandon this wallet design
// All we really need is a cosmrs::crypto::secp256k1::signing_key::SigningKey
// From there we can get the PublicKey and AccountId (address)
// We will have to add the amino support, though.
// mod wallet_amino;
// mod wallet_direct;

// pub const GRPC_URL: &str = "http://localhost:9090";
pub const GRPC_URL: &str = "http://grpc.testnet.secretsaturn.net:9090";

pub use account::Wallet;
pub use query::Querier;
pub use secret_network_client::{CreateClientOptions, SecretNetworkClient, TxOptions};
pub use tx::{bank::BankServiceClient, TxSender};

pub type Result<T> = core::result::Result<T, Error>;
pub type Error = Box<dyn std::error::Error>;
