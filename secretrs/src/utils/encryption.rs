//! Encryption Utilities for Secret Contracts
//!
//! # Examples
//!
//! The following example illustrates how to query a Secret contract:
//!
#![cfg_attr(feature = "grpc", doc = " ```no_run")]
#![cfg_attr(not(feature = "grpc"), doc = " ```ignore")]
//! use anyhow::Result;
//! use base64::prelude::{Engine, BASE64_STANDARD};
//! use secretrs::{
//!     utils::EnigmaUtils,
//!     grpc_clients::ComputeQueryClient,
//!     proto::secret::compute::v1beta1::QuerySecretContractRequest,
//! };
//!
//! #[derive(::serde::Serialize)]
//! #[serde(rename_all = "snake_case")]
//! enum QueryMsg {
//!     TokenInfo {},
//! }
//!
//! #[tokio::main(flavor = "current_thread")]
//! async fn main() -> Result<()> {
//!     let mut compute = ComputeQueryClient::connect("http://grpc.testnet.secretsaturn.net:9090").await?;
//!     let contract_address = "secret19gtpkk25r0c36gtlyrc6repd3q52ngmkpfszw3".to_string();
//!     let code_hash = "9a00ca4ad505e9be7e6e6dddf8d939b7ec7e9ac8e109c8681f10db9cacb36d42".to_string();
//!     let query = QueryMsg::TokenInfo {};
//!
//!     // Provide `Some(seed: [u8;32])`, or `None` to generate a random keypair
//!     let encryption_utils = EnigmaUtils::new(None, "pulsar-3")?;
//!     let encrypted = encryption_utils.encrypt(&code_hash, &query)?;
//!     // The encrypted message includes a nonce, pubkey, and ciphertext
//!
//!     // Extract the nonce to use to decrypt the response later
//!     let nonce: [u8; 32] = encrypted.nonce();
//!     // Convert the encrypted message to bytes
//!     let query: Vec<u8> = encrypted.into_inner();
//!
//!     let request = QuerySecretContractRequest { contract_address, query };
//!     let response = compute.query_secret_contract(request).await?.into_inner();
//!
//!     let decrypted_bytes = encryption_utils.decrypt(&nonce, &response.data)?;
//!     let decrypted_b64_string = String::from_utf8(decrypted_bytes)?;
//!     let decoded_bytes = BASE64_STANDARD.decode(decrypted_b64_string)?;
//!     let data = String::from_utf8(decoded_bytes)?;
//!
//!     assert_eq!(
//!         data.trim(),
//!         r#"{"token_info":{"name":"Amber","symbol":"AMBER","decimals":6,"total_supply":"8888000000"}}"#
//!     );
//!
//!     Ok(())
//! }
//! ```

use aes_siv::{siv::Aes128Siv, Key, KeyInit};
use async_trait::async_trait;
use hex::ToHex;
use hex_literal::hex;
use nanorand::rand::Rng;
use serde::Serialize;
use x25519_dalek::{PublicKey, StaticSecret};

use super::Error;
type Result<T, E = Error> = std::result::Result<T, E>;

const DEVNET_CHAIN_IDS: [&str; 1] = ["secretdev-1"];
const TESTNET_CHAIN_IDS: [&str; 1] = ["pulsar-3"];
const MAINNET_CHAIN_IDS: [&str; 3] = ["secret-2", "secret-3", "secret-4"];

const TESTNET_IO_PUBKEY: [u8; 32] =
    hex!("e2b40597d50457d95290bdee480b8bc3400e9f40c2a5d69c9519f1fee2e24933");
const MAINNET_IO_PUBKEY: [u8; 32] =
    hex!("efdfbee583877e6d12c219695030a5bfb72e0a3abdc416655aa4a30c95a4446f");
const HKDF_SALT: [u8; 32] =
    hex!("000000000000000000024bead8df69990852c202db0e0097c1a12ea637d7e96d");

// TODO: rename to SecretUtils

#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
/// Encryption Utilities for Secret Contracts.
pub trait SecretUtils {
    /// Encrypts a message for a specific contract using the associated code hash.
    async fn encrypt<M: Serialize + Sync>(
        &self,
        contract_code_hash: &str,
        msg: &M,
    ) -> Result<Vec<u8>>;
    /// Decrypts a message returned by the enclave.
    async fn decrypt(&self, nonce: &[u8; 32], ciphertext: &[u8]) -> Result<Vec<u8>>;
    /// Get the public key of the derived key pair.
    async fn get_pubkey(&self) -> [u8; 32];
    /// Get the encryption key for a specific tx, based on the nonce.
    async fn get_tx_encryption_key(&self, nonce: &[u8; 32]) -> [u8; 32];
}

/// Use to encrypt/decrypt messages related to the `compute` module.
#[derive(Clone)]
pub struct EnigmaUtils {
    seed: [u8; 32],
    privkey: StaticSecret,
    pubkey: PublicKey,
    consensus_io_pubkey: [u8; 32],
}

use std::fmt;

impl fmt::Debug for EnigmaUtils {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("EnigmaUtils")
            .field("seed", &self.seed.encode_hex::<String>())
            .field("privkey", &"[REDACTED]")
            .field("pubkey", &self.pubkey.encode_hex::<String>())
            .field(
                "consensus_io_pubkey",
                &self.consensus_io_pubkey.encode_hex::<String>(),
            )
            .finish()
    }
}

impl EnigmaUtils {
    /// Creates a new `EnigmaUtils` instance with a seed and chain ID.
    ///
    /// The `chain_id` is used to determine the appropriate IO public key.
    ///
    /// If no seed is provided, a random seed will be generated.
    ///
    /// # Examples
    ///
    /// ```
    /// use secretrs::utils::EnigmaUtils;
    ///
    /// let utils = EnigmaUtils::new(None, "secret-4").expect("Failed to create EnigmaUtils");
    /// ```
    pub fn new(seed: Option<[u8; 32]>, chain_id: &str) -> Result<Self> {
        let seed = seed.unwrap_or_else(EnigmaUtils::generate_seed);
        let (privkey, pubkey) = EnigmaUtils::generate_x25519_key_pair(&seed);

        let consensus_io_pubkey = match chain_id {
            chain_id if MAINNET_CHAIN_IDS.contains(&chain_id) => Ok(MAINNET_IO_PUBKEY),
            chain_id if TESTNET_CHAIN_IDS.contains(&chain_id) => Ok(TESTNET_IO_PUBKEY),
            chain_id if DEVNET_CHAIN_IDS.contains(&chain_id) => Err(Error::DevnetIoKey),
            _ => Err(Error::InvalidChainId {
                chain_id: chain_id.to_string(),
            }),
        }?;

        Ok(EnigmaUtils {
            seed,
            privkey,
            pubkey,
            consensus_io_pubkey,
        })
    }

    /// Creates a new [EnigmaUtils] instance with a seed and IO public key.
    ///
    /// The `consensus_io_pubkey` is manually provided instead of being derived from a chain ID.
    ///
    /// If no seed is provided, a random seed will be generated.
    ///
    /// # Examples
    ///
    #[cfg_attr(feature = "grpc", doc = " ```no_run")]
    #[cfg_attr(not(feature = "grpc"), doc = " ```ignore")]
    /// # use anyhow::Result;
    /// # use secretrs::utils::EnigmaUtils;
    /// # use secretrs::grpc_clients::RegistrationQueryClient;
    /// # #[tokio::main(flavor = "current_thread")]
    /// # async fn main() -> Result<()> {
    /// let mut registration = RegistrationQueryClient::connect("GRPC_URL").await?;
    /// let enclave_key_bytes = registration.tx_key(()).await?.into_inner().key;
    ///
    /// let mut io_key = [0u8; 32];
    /// io_key.copy_from_slice(&enclave_key_bytes[0..32]);
    ///
    /// let utils = EnigmaUtils::from_io_key(None, io_key);
    /// # Ok(())
    /// # }
    /// ```
    pub fn from_io_key(seed: Option<[u8; 32]>, consensus_io_pubkey: [u8; 32]) -> Self {
        let seed = seed.unwrap_or_else(EnigmaUtils::generate_seed);
        let (privkey, pubkey) = EnigmaUtils::generate_x25519_key_pair(&seed);

        EnigmaUtils {
            seed,
            privkey,
            pubkey,
            consensus_io_pubkey,
        }
    }

    /// Encrypts a message for a specific contract using the associated code hash.
    ///
    /// The message must implement [`serde::Serialize`]. A 64-character hexadecimal string representing the code hash is prepended to the message before encryption. The resulting [`SecretMsg`] is a concatenation of nonce, public key, and ciphertext.
    ///
    /// # Examples
    ///
    /// ```
    /// # #[derive(::serde::Serialize)]
    /// # #[serde(rename_all = "snake_case")]
    /// # enum QueryMsg {
    /// #     HelloWorld,
    /// # }
    /// use secretrs::utils::EnigmaUtils;
    ///
    /// let utils = EnigmaUtils::new(None, "secret-4").expect("Failed to create EnigmaUtils");
    /// let contract_code_hash = "1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef";
    /// let msg = QueryMsg::HelloWorld;
    /// let encrypted_msg = utils.encrypt(contract_code_hash, &msg).expect("Encryption failed");
    /// let (nonce, pubkey, ciphertext) = encrypted_msg.into_parts();
    /// ```
    pub fn encrypt<M: ::serde::Serialize>(
        &self,
        contract_code_hash: &str,
        msg: &M,
    ) -> Result<SecretMsg> {
        if contract_code_hash.len() != 64 {
            return Err(Error::InvalidCodeHash);
        }

        let nonce = Self::generate_nonce();
        let tx_encryption_key = self.get_tx_encryption_key(&nonce);

        let mut cipher = Aes128Siv::new(&Key::<Aes128Siv>::from(tx_encryption_key));

        let msg = serde_json::to_vec(msg)?;
        let plaintext = [contract_code_hash.as_bytes(), msg.as_slice()].concat();

        let ciphertext = cipher.encrypt([[]], &plaintext).map_err(Error::AesSiv)?;

        let msg: SecretMsg = [nonce.as_slice(), self.pubkey.as_bytes(), &ciphertext]
            .concat()
            .into();

        Ok(msg)
    }

    /// Decrypts a message returned by the enclave.
    ///
    /// This function decrypts responses using the same nonce that was used to encrypt the request.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use secretrs::utils::EnigmaUtils;
    /// # use secretrs::proto::secret::compute::v1beta1::QuerySecretContractResponse;
    /// # use base64::prelude::{Engine, BASE64_STANDARD};
    /// # use ::serde::Serialize;
    /// #
    /// # #[derive(Serialize)]
    /// # #[serde(rename_all = "snake_case")]
    /// # enum QueryMsg {
    /// #     HelloWorld,
    /// # }
    /// #
    /// # use ::anyhow::{Result, Error};
    /// # fn main() -> Result<()> {
    /// # let utils = EnigmaUtils::new(None, "secret-4").expect("Failed to create EnigmaUtils");
    /// # let contract_code_hash = "1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef";
    /// # let msg = QueryMsg::HelloWorld;
    /// # let encrypted_msg = utils.encrypt(contract_code_hash, &msg).expect("Encryption failed");
    /// # let (nonce, pubkey, ciphertext) = encrypted_msg.into_parts();
    /// # let response = QuerySecretContractResponse { data: ciphertext };
    /// # // alternate method
    /// # let data = utils.decrypt(&nonce, &response.data)
    /// #     .and_then(|bytes| String::from_utf8(bytes).map_err(Into::into))
    /// #     .and_then(|b64_string| BASE64_STANDARD.decode(b64_string).map_err(Into::into))
    /// #     .and_then(|decoded_bytes| String::from_utf8(decoded_bytes).map_err(Into::into))
    /// #     .unwrap();
    /// #
    /// let decrypted_bytes = utils.decrypt(&nonce, &response.data).expect("Decryption failed");
    /// let decrypted_b64_string = String::from_utf8(decrypted_bytes).expect("Decoding error");
    /// let decoded_bytes = BASE64_STANDARD.decode(decrypted_b64_string).expect("Decoding error");
    /// let data = String::from_utf8(decoded_bytes).expect("Decoding error");
    /// #
    /// # Ok(())
    /// # }
    /// ```
    pub fn decrypt(&self, nonce: &[u8; 32], ciphertext: &[u8]) -> Result<Vec<u8>> {
        if ciphertext.is_empty() {
            return Err(Error::EmptyCiphertext);
        }

        let tx_encryption_key = self.get_tx_encryption_key(nonce);
        let mut cipher = Aes128Siv::new(&Key::<Aes128Siv>::from(tx_encryption_key));

        cipher.decrypt([[]], ciphertext).map_err(Error::AesSiv)
    }

    /// Get the seed used to derive the key pair.
    pub fn get_seed(&self) -> [u8; 32] {
        self.seed
    }

    /// Get the public key of the derived key pair.
    pub fn get_pubkey(&self) -> [u8; 32] {
        *self.pubkey.as_bytes()
    }

    fn generate_seed() -> [u8; 32] {
        let mut seed = [0u8; 32];
        let mut rng = nanorand::rand::ChaCha8::new();
        rng.fill_bytes(&mut seed);
        seed
    }

    fn generate_x25519_key_pair(seed: &[u8; 32]) -> (StaticSecret, PublicKey) {
        let secret = StaticSecret::from(*seed);
        let public = PublicKey::from(&secret);

        (secret, public)
    }

    fn generate_nonce() -> [u8; 32] {
        let mut nonce = [0u8; 32];
        let mut rng = nanorand::rand::ChaCha8::new();
        rng.fill_bytes(&mut nonce);
        nonce
    }

    /// Get the encryption key for a specific tx, based on the nonce.
    pub fn get_tx_encryption_key(&self, nonce: &[u8; 32]) -> [u8; 32] {
        let secret = &self.privkey;
        let public = x25519_dalek::PublicKey::from(self.consensus_io_pubkey);
        let shared = secret.diffie_hellman(&public);

        let ikm = &[shared.as_bytes(), nonce.as_slice()].concat();

        let mut key = [0u8; 32];
        hkdf::Hkdf::<sha2::Sha256>::new(Some(&HKDF_SALT), ikm)
            .expand(&[], &mut key)
            .expect("HKDF expansion error");
        key
    }
}

#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
impl SecretUtils for EnigmaUtils {
    async fn encrypt<M: Serialize + Sync>(
        &self,
        contract_code_hash: &str,
        msg: &M,
    ) -> Result<Vec<u8>> {
        self.encrypt(contract_code_hash, msg)
            .and_then(|msg| Ok(msg.into_inner()))
    }

    async fn decrypt(&self, nonce: &[u8; 32], ciphertext: &[u8]) -> Result<Vec<u8>> {
        self.decrypt(nonce, ciphertext)
    }

    async fn get_pubkey(&self) -> [u8; 32] {
        self.get_pubkey()
    }

    async fn get_tx_encryption_key(&self, nonce: &[u8; 32]) -> [u8; 32] {
        self.get_tx_encryption_key(nonce)
    }
}

/// The message has the following format:
/// - [0..32): nonce (32 bytes)
/// - [32..64): pubkey (32 bytes)
/// - [64..): ciphertext (remaining bytes)
#[derive(Debug, Clone)]
pub struct SecretMsg(Vec<u8>);

impl SecretMsg {
    /// Get the nonce used to encrypt the message.
    pub fn nonce(&self) -> [u8; 32] {
        let mut array = [0u8; 32];
        array.copy_from_slice(&self.0[0..32]);

        array
    }

    /// Get the public key corresponding to the private key used for message encryption.
    pub fn pubkey(&self) -> [u8; 32] {
        let mut array = [0u8; 32];
        array.copy_from_slice(&self.0[32..64]);

        array
    }

    /// Get the encrypted message itself.
    pub fn ciphertext(&self) -> Vec<u8> {
        self.0[64..].to_vec()
    }

    /// Get all three parts of the message: `(nonce, pubkey, ciphertext)`
    pub fn into_parts(&self) -> ([u8; 32], [u8; 32], Vec<u8>) {
        let mut nonce = [0u8; 32];
        nonce.copy_from_slice(&self.0[0..32]);
        let mut pubkey = [0u8; 32];
        pubkey.copy_from_slice(&self.0[32..64]);
        let ciphertext = self.0[64..].to_vec();

        (nonce, pubkey, ciphertext)
    }

    /// Consumes `self`, returning the full message as a byte vector.
    pub fn into_inner(self) -> Vec<u8> {
        self.0
    }
}

impl From<Vec<u8>> for SecretMsg {
    fn from(value: Vec<u8>) -> Self {
        SecretMsg(value)
    }
}

#[cfg(test)]
mod test {
    type Error = Box<dyn std::error::Error>;
    type Result<T> = std::result::Result<T, Error>;

    use super::*;
    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize)]
    struct Msg {
        foo: String,
        bar: u32,
    }

    #[test]
    fn enigma_utils() -> Result<()> {
        let msg = Msg {
            foo: "hello world".to_string(),
            bar: 42,
        };

        let utils = EnigmaUtils::new(None, "secret-4")?;
        let code_hash = "9a00ca4ad505e9be7e6e6dddf8d939b7ec7e9ac8e109c8681f10db9cacb36d42";
        let encrypted = utils.encrypt(code_hash, &msg)?;

        let (nonce, _pubkey, ciphertext) = encrypted.into_parts();
        let decrypted_bytes = utils.decrypt(&nonce, &ciphertext)?;

        let plaintext = format!("{}{}", code_hash, serde_json::to_string(&msg)?);
        let decrypted_msg = String::from_utf8(decrypted_bytes)?;
        assert_eq!(plaintext, decrypted_msg);

        // test that the same seed will produce the same pubkey

        let seed = utils.get_seed();
        let pubkey = utils.get_pubkey();

        let utils2 = EnigmaUtils::new(Some(seed), "secret-4")?;
        let seed2 = utils2.get_seed();
        let pubkey2 = utils2.get_pubkey();

        assert_eq!(seed, seed2);
        assert_eq!(pubkey, pubkey2);

        Ok(())
    }
}
