pub mod account;
pub(crate) mod consts;
pub(crate) mod crypto;
pub mod error;
pub mod types;

pub mod encryption;

pub use crate::utils::error::Error;
pub use account::Account;
pub use types::{CodeHash, CodeId, Contract, TxResponse};

use crate::utils::error::Result;
use crypto::{Decrypter, Nonce};
use std::str::FromStr;

pub async fn encrypt_msg<M: serde::Serialize>(
    msg: &M,
    code_hash: &str,
    account: &Account,
    enclave_public_key: &str,
) -> Result<(Nonce, Vec<u8>)> {
    let code_hash = CodeHash::from_str(code_hash)?;

    let enclave_key = crypto::clone_into_key(&hex::decode(enclave_public_key)?);
    let msg = serde_json::to_vec(msg).expect("msg cannot be serialized as JSON");
    let plaintext = [code_hash.to_hex_string().as_bytes(), msg.as_slice()].concat();
    encrypt_msg_raw(&plaintext, account, enclave_key).await
}

async fn encrypt_msg_raw(
    msg: &[u8],
    account: &Account,
    enclave_public_key: crypto::Key,
) -> Result<(Nonce, Vec<u8>)> {
    let (prvk, pubk) = account.prv_pub_bytes();
    let io_key = enclave_public_key;
    let nonce_ciphertext = crypto::encrypt(&prvk, &pubk, &io_key, msg)?;
    Ok(nonce_ciphertext)
}

pub async fn decrypter(
    nonce: &Nonce,
    account: &Account,
    enclave_public_key: &str,
) -> Result<Decrypter> {
    let (secret, _) = account.prv_pub_bytes();
    let enclave_key = crypto::clone_into_key(&hex::decode(enclave_public_key)?);
    let io_key = enclave_key;
    Ok(Decrypter::new(secret, io_key, *nonce))
}
