// I don't think we should have this account module here TBH
pub mod account;
pub(crate) mod consts;
pub mod encryption;
pub mod error;

pub use encryption::EncryptionUtils;
pub use error::Error;

