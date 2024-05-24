//! Error types related to utilities.

#![allow(missing_docs)]

use derive_more::From;

#[derive(Debug, From)]
pub enum Error {
    EmptyCiphertext,
    InvalidCodeHash,
    InvalidChainId {
        chain_id: String,
    },
    #[from]
    FromHex(hex::FromHexError),
    #[from]
    FromUtf8(std::string::FromUtf8Error),
    #[from]
    FromBase64(base64::DecodeError),
    #[from]
    SerdeJson(serde_json::Error),
    #[from]
    AesSiv(aes_siv::Error),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

impl std::error::Error for Error {}
