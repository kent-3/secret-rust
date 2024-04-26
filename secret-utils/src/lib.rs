pub mod account;
pub(crate) mod consts;
pub(crate) mod crypto;
pub mod error;
pub mod types;

pub use account::Account;
pub use error::Error;
pub use types::{CodeHash, CodeId, Contract, TxResponse};

use crypto::{Decrypter, Nonce};
use error::Result;
use std::str::FromStr;

// IDEA - for a nice API, take as input only a code_hash, enclave_public_key, and some entropy.
// do a sha256 on the entropy to get a [u8;32] to use as the prv_key. generate a pub_key.
// return a struct that has methods for encrypt and decrypt
//
// let enigma = secretrs::Enigma::new(code_hash, enclave_public_key, entropy);
// let encrypted = enigma.encrypt(msg);
//
// OR
//
// let (encrypter, decrypter) = secretrs::Enigma::new(code_hash, enclave_public_key, entropy);
// let encrypted = encrypter.encrypt(msg);

// TODO - instead of `Account`, accept a key / seed [u8; 32]
// For queries, the specific keys don't matter, so it doesn't make sense to require `Account`
// as input. You only need to provide a private key / seed, and it returns a prv_pub pair.
pub async fn encrypt_msg<M: serde::Serialize>(
    msg: &M,
    code_hash: &str,
    account: &Account,
    enclave_public_key: &str,
) -> Result<(Nonce, Vec<u8>)> {
    let code_hash = CodeHash::from_str(&code_hash)?;

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
