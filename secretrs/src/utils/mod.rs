//! Utilities related to Secret.

pub mod encryption;
pub mod error;

pub use encryption::{EnigmaUtils, SecretUtils};
pub use error::Error;
