use aes_siv::{siv::Aes128Siv, Key, KeyInit};
use base64::prelude::{Engine, BASE64_STANDARD};
use lazy_static::lazy_static;
use nanorand::rand::Rng;
use x25519_dalek::{PublicKey, StaticSecret};

const HKDF_SALT: [u8; 32] =
    hex_literal::hex!("000000000000000000024bead8df69990852c202db0e0097c1a12ea637d7e96d");

const TESTNET_IO_PUBKEY: [u8; 32] =
    hex_literal::hex!("e24a22b31e3d34e0e00bcd32189548f1ccbdc9cda8f5a266219b908582b6f03f");

const MAINNET_IO_PUBKEY: [u8; 32] =
    hex_literal::hex!("efdfbee583877e6d12c219695030a5bfb72e0a3abdc416655aa4a30c95a4446f");

lazy_static! {
    static ref MAINNET_CONSENSUS_IO_PUBKEY: [u8; 32] = {
        let decoded_bytes = BASE64_STANDARD
            .decode("79++5YOHfm0SwhlpUDClv7cuCjq9xBZlWqSjDJWkRG8=")
            .expect("IO key could not be base64 decoded");

        let array: [u8; 32] = decoded_bytes
            .try_into()
            .expect("Decoded bytes are not of length 32");
        array
    };
}

const MAINNET_CHAIN_IDS: [&str; 3] = ["secret-2", "secret-3", "secret-4"];

pub struct EncryptionUtils {
    seed: [u8; 32],
    privkey: StaticSecret,
    pubkey: PublicKey,
    consensus_io_pubkey: [u8; 32],
}

impl EncryptionUtils {
    pub fn new(seed: Option<[u8; 32]>, chain_id: Option<&str>) -> Result<Self, String> {
        let seed = seed.unwrap_or_else(EncryptionUtils::generate_new_seed);
        let (privkey, pubkey) = EncryptionUtils::generate_x25519_key_pair(&seed);

        let consensus_io_pubkey = match chain_id {
            Some(chain_id) if MAINNET_CHAIN_IDS.contains(&chain_id) => Ok(MAINNET_IO_PUBKEY),
            Some(_) => Err("Chain ID provided is not supported on mainnet".to_string()),
            None => Err(
                "A chain ID must be provided unless an IO key is directly specified".to_string(),
            ),
        }?;

        Ok(EncryptionUtils {
            seed,
            privkey,
            pubkey,
            consensus_io_pubkey,
        })
    }

    pub fn from_io_key(
        consensus_io_pubkey: [u8; 32],
        seed: Option<[u8; 32]>,
    ) -> Result<Self, String> {
        let seed = seed.unwrap_or_else(EncryptionUtils::generate_new_seed);
        let (privkey, pubkey) = EncryptionUtils::generate_x25519_key_pair(&seed);

        Ok(EncryptionUtils {
            seed,
            privkey,
            pubkey,
            consensus_io_pubkey,
        })
    }

    fn generate_new_seed() -> [u8; 32] {
        let mut seed = [0u8; 32];
        let mut rng = nanorand::rand::ChaCha8::new();
        rng.fill_bytes(&mut seed);
        seed
    }

    fn generate_nonce() -> [u8; 32] {
        let mut nonce = [0u8; 32];
        let mut rng = nanorand::rand::ChaCha8::new();
        rng.fill_bytes(&mut nonce);
        nonce
    }

    fn generate_x25519_key_pair(seed: &[u8; 32]) -> (StaticSecret, PublicKey) {
        let secret = StaticSecret::from(*seed);
        let public = PublicKey::from(&secret);

        (secret, public)
    }

    pub fn encrypt(&self, contract_code_hash: &str, msg: &serde_json::Value) -> SecretMsg {
        let nonce = Self::generate_nonce();
        let tx_encryption_key = self.get_tx_encryption_key(&nonce);

        let mut cipher = Aes128Siv::new(&Key::<Aes128Siv>::from(tx_encryption_key));

        let plaintext = format!("{}{}", contract_code_hash, msg);
        let plaintext_bytes = plaintext.as_bytes();

        let ciphertext = cipher
            .encrypt([[]], plaintext_bytes)
            .expect("Encryption failure");

        [nonce.as_slice(), self.pubkey.as_bytes(), &ciphertext]
            .concat()
            .into()
    }

    pub fn decrypt(&self, nonce: &[u8; 32], ciphertext: &[u8]) -> Vec<u8> {
        // do we need a guard in the event the ciphertext is empty?

        let tx_encryption_key = self.get_tx_encryption_key(nonce);
        let mut cipher = Aes128Siv::new(&Key::<Aes128Siv>::from(tx_encryption_key));

        cipher
            .decrypt([[]], ciphertext)
            .expect("Encryption failure")
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

    // fn extract_nonce(data: &[u8]) -> Result<[u8; 32], &'static str> {
    //     if data.len() < 32 {
    //         return Err("Data is less than 32 bytes long");
    //     }
    //
    //     let mut array = [0u8; 32];
    //     array.copy_from_slice(&data[0..32]);
    //     Ok(array)
    // }
    //
    // fn extract_pubkey(data: &[u8]) -> Result<[u8; 32], &'static str> {
    //     if data.len() < 32 {
    //         return Err("Data is less than 32 bytes long");
    //     }
    //
    //     let mut array = [0u8; 32];
    //     array.copy_from_slice(&data[32..64]);
    //     Ok(array)
    // }
    //
    // fn extract_parts(data: Vec<u8>) -> Result<([u8; 32], [u8; 32], Vec<u8>), &'static str> {
    //     if data.len() < 64 {
    //         return Err("Input data is less than 64 bytes long");
    //     }
    //
    //     let mut nonce = [0u8; 32];
    //     nonce.copy_from_slice(&data[0..32]);
    //     let mut pubkey = [0u8; 32];
    //     pubkey.copy_from_slice(&data[32..64]);
    //     let ciphertext = data[64..].to_vec();
    //
    //     Ok((nonce, pubkey, ciphertext))
    // }
}

/// The message has the following format:
/// - [0..32): nonce (32 bytes)
/// - [32..64): pubkey (32 bytes)
/// - [64..): ciphertext (remaining bytes)
pub struct SecretMsg(Vec<u8>);

impl SecretMsg {
    fn into_nonce(self) -> [u8; 32] {
        let mut array = [0u8; 32];
        array.copy_from_slice(&self.0[0..32]);

        array
    }

    fn into_pubkey(self) -> [u8; 32] {
        let mut array = [0u8; 32];
        array.copy_from_slice(&self.0[32..64]);

        array
    }

    fn into_ciphertext(self) -> Vec<u8> {
        self.0[64..].to_vec()
    }

    fn into_parts(self) -> ([u8; 32], [u8; 32], Vec<u8>) {
        let mut nonce = [0u8; 32];
        nonce.copy_from_slice(&self.0[0..32]);
        let mut pubkey = [0u8; 32];
        pubkey.copy_from_slice(&self.0[32..64]);
        let ciphertext = self.0[64..].to_vec();

        (nonce, pubkey, ciphertext)
    }

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
    use super::*;

    // #[tokio::test]
    #[test]
    fn encryption_utils() {
        let utils = EncryptionUtils::new(None, Some("secret-4")).unwrap();
        let code_hash = "9a00ca4ad505e9be7e6e6dddf8d939b7ec7e9ac8e109c8681f10db9cacb36d42";
        let msg = serde_json::Value::from("barfoo".to_string());
        let plaintext = format!("{}{}", code_hash, msg.to_string());
        let encrypted = utils.encrypt(code_hash, &msg);
        let (nonce, _pubkey, ciphertext) = encrypted.into_parts();
        let decrypted_bytes = utils.decrypt(&nonce, &ciphertext);
        let decrypted_msg = String::from_utf8(decrypted_bytes).unwrap();

        assert_eq!(plaintext, decrypted_msg)
    }
}