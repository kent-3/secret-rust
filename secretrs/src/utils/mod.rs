//! Encryption Utilities for Secret Contracts
//!
//! ```
//! use secretrs::utils::EncryptionUtils;
//! use secretrs::clients::ComputeQueryClient;
//! use secretrs::proto::secret::compute::v1beta1::QuerySecretContractRequest,
//!
//! let mut compute = ComputeQueryClient::connect("http://grpc.testnet.secretsaturn.net:9090").await?;
//! let contract_address = "secret19gtpkk25r0c36gtlyrc6repd3q52ngmkpfszw3".to_string();
//! let code_hash = "9a00ca4ad505e9be7e6e6dddf8d939b7ec7e9ac8e109c8681f10db9cacb36d42".to_string();
//! let query = QueryMsg::TokenInfo {};
//!
//! // Provide `Some(seed: [u8;32])`, or `None` to generate a random keypair
//! let encryption_utils = EncryptionUtils::new(None, "pulsar-3")?;
//! let encrypted = encryption_utils.encrypt(&code_hash, &query)?;
//!
//! let nonce: [u8; 32] = encrypted.nonce();
//! let query: Vec<u8> = encrypted.into_inner();
//!
//! let request = QuerySecretContractRequest { contract_address, query };
//! let response = compute.query_secret_contract(request).await?.into_inner();
//!
//! let decrypted_bytes = encryption_utils.decrypt(&nonce, &response.data)?;
//! let decrypted_b64_string = String::from_utf8(decrypted_bytes)?;
//! let decoded_bytes = BASE64_STANDARD.decode(decrypted_b64_string)?;
//! let data = String::from_utf8(decoded_bytes)?;
//! ````

pub(crate) mod consts;
pub mod encryption;
pub mod error;

pub use encryption::EncryptionUtils;
pub use error::Error;
