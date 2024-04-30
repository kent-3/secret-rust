#![allow(unused)]

pub mod account;
mod secret_network_client;

// I think I will abandon this wallet design
// All we really need is a cosmrs::crypto::secp256k1::signing_key::SigningKey
// From there we can get the PublicKey and AccountId (address)
// We will have to add the amino support, though.
// mod wallet_amino;
// mod wallet_direct;

pub use account::Wallet;
