use derive_more::From;
use hex_literal::hex;
use nanorand::rand::Rng;

use aes_siv::{siv::Aes128Siv, Key, KeyInit};
use x25519_dalek::{PublicKey, StaticSecret};

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, From)]
pub enum Error {
    #[from]
    Custom(String),

    #[from]
    FromHex(hex::FromHexError),

    #[from]
    FromUtf8(std::string::FromUtf8Error),

    #[from]
    SerdeJson(serde_json::Error),

    #[from]
    AesSiv(aes_siv::Error),
}

impl Error {
    pub fn custom(value: impl std::fmt::Display) -> Self {
        Self::Custom(value.to_string())
    }
}

impl From<&str> for Error {
    fn from(value: &str) -> Self {
        Self::Custom(value.to_string())
    }
}

impl core::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

impl std::error::Error for Error {}

const DEVNET_CHAIN_IDS: [&str; 1] = ["secretdev-1"];
const TESTNET_CHAIN_IDS: [&str; 1] = ["pulsar-3"];
const MAINNET_CHAIN_IDS: [&str; 3] = ["secret-2", "secret-3", "secret-4"];

const DEVNET_IO_PUBKEY: [u8; 32] =
    hex!("ea7f818284fec45ee630ec2ee7f1b160fbfb169042c13525e0024b88f204d96b");
const TESTNET_IO_PUBKEY: [u8; 32] =
    hex!("e24a22b31e3d34e0e00bcd32189548f1ccbdc9cda8f5a266219b908582b6f03f");
const MAINNET_IO_PUBKEY: [u8; 32] =
    hex!("efdfbee583877e6d12c219695030a5bfb72e0a3abdc416655aa4a30c95a4446f");
const HKDF_SALT: [u8; 32] =
    hex!("000000000000000000024bead8df69990852c202db0e0097c1a12ea637d7e96d");

pub struct EncryptionUtils {
    seed: [u8; 32],
    privkey: StaticSecret,
    pubkey: PublicKey,
    consensus_io_pubkey: [u8; 32],
}

impl EncryptionUtils {
    pub fn new(seed: Option<[u8; 32]>, chain_id: &str) -> Result<Self> {
        let seed = seed.unwrap_or_else(EncryptionUtils::generate_seed);
        let (privkey, pubkey) = EncryptionUtils::generate_x25519_key_pair(&seed);

        let consensus_io_pubkey = match chain_id {
            chain_id if MAINNET_CHAIN_IDS.contains(&chain_id) => Ok(MAINNET_IO_PUBKEY),
            chain_id if TESTNET_CHAIN_IDS.contains(&chain_id) => Ok(TESTNET_IO_PUBKEY),
            chain_id if DEVNET_CHAIN_IDS.contains(&chain_id) => Ok(DEVNET_IO_PUBKEY),
            _ => Err(Error::custom(format!("{chain_id} is not supported"))),
        }?;

        Ok(EncryptionUtils {
            seed,
            privkey,
            pubkey,
            consensus_io_pubkey,
        })
    }

    pub fn new_with_io_key(seed: Option<[u8; 32]>, consensus_io_pubkey: [u8; 32]) -> Result<Self> {
        let seed = seed.unwrap_or_else(EncryptionUtils::generate_seed);
        let (privkey, pubkey) = EncryptionUtils::generate_x25519_key_pair(&seed);

        Ok(EncryptionUtils {
            seed,
            privkey,
            pubkey,
            consensus_io_pubkey,
        })
    }

    pub fn encrypt<M: serde::Serialize>(
        &self,
        contract_code_hash: &str,
        msg: &M,
    ) -> Result<SecretMsg> {
        if contract_code_hash.len() != 64 {
            return Err(Error::custom("invalid code hash"));
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

    pub fn decrypt(&self, nonce: &[u8; 32], ciphertext: &[u8]) -> Result<Vec<u8>> {
        if ciphertext.is_empty() {
            return Err(Error::custom("ciphertext is empty"));
        }

        let tx_encryption_key = self.get_tx_encryption_key(nonce);
        let mut cipher = Aes128Siv::new(&Key::<Aes128Siv>::from(tx_encryption_key));

        cipher.decrypt([[]], ciphertext).map_err(Error::AesSiv)
    }

    pub fn get_seed(&self) -> [u8; 32] {
        self.seed
    }

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

    fn get_tx_encryption_key(&self, nonce: &[u8; 32]) -> [u8; 32] {
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

/// The message has the following format:
/// - [0..32): nonce (32 bytes)
/// - [32..64): pubkey (32 bytes)
/// - [64..): ciphertext (remaining bytes)
pub struct SecretMsg(Vec<u8>);

#[allow(unused)]
impl SecretMsg {
    fn nonce(&self) -> [u8; 32] {
        let mut array = [0u8; 32];
        array.copy_from_slice(&self.0[0..32]);

        array
    }

    fn pubkey(&self) -> [u8; 32] {
        let mut array = [0u8; 32];
        array.copy_from_slice(&self.0[32..64]);

        array
    }

    fn ciphertext(&self) -> Vec<u8> {
        self.0[64..].to_vec()
    }

    /// Consumes `self` returning the parts of the message.
    ///
    /// Returns `(nonce, pubkey, ciphertext)`
    fn into_parts(self) -> ([u8; 32], [u8; 32], Vec<u8>) {
        let mut nonce = [0u8; 32];
        nonce.copy_from_slice(&self.0[0..32]);
        let mut pubkey = [0u8; 32];
        pubkey.copy_from_slice(&self.0[32..64]);
        let ciphertext = self.0[64..].to_vec();

        (nonce, pubkey, ciphertext)
    }

    /// Consumes `self` returning the message as `Vec<u8>`
    fn into_inner(self) -> Vec<u8> {
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
    type Result<T> = core::result::Result<T, Error>;

    use super::*;
    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize)]
    struct Msg {
        foo: String,
        bar: u32,
    }

    #[test]
    fn encryption_utils() -> Result<()> {
        let msg = Msg {
            foo: "hello world".to_string(),
            bar: 42,
        };

        let utils = EncryptionUtils::new(None, "secret-4")?;
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

        let utils2 = EncryptionUtils::new(Some(seed), "secret-4")?;
        let seed2 = utils2.get_seed();
        let pubkey2 = utils2.get_pubkey();

        assert_eq!(seed, seed2);
        assert_eq!(pubkey, pubkey2);

        Ok(())
    }
}
