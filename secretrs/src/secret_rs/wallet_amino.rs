use cosmrs::tx::SignDoc;
use cosmrs::Any;
use cosmrs::Coin;

use super::secret_network_client::SignDocCamelCase;

pub struct AminoWallet {
    pub address: String,
    pub private_key: Vec<u8>,
    pub public_key: Vec<u8>,
}

#[derive(Debug)]
pub struct AminoMsg {
    pub r#type: String,
    pub value: Any,
}

/// Response after signing with Amino.
#[derive(Debug)]
pub struct AminoSignResponse {
    pub signed: StdSignDoc,
    pub signature: StdSignature,
}

/// Document to be signed.
#[derive(Debug)]
pub struct StdSignDoc {
    pub chain_id: String,
    pub account_number: String,
    pub sequence: String,
    pub fee: StdFee,
    pub msgs: Vec<AminoMsg>,
    pub memo: String,
}

/// Standard fee structure.
#[derive(Debug, Clone)]
pub struct StdFee {
    pub amount: Vec<Coin>,
    pub gas: String,
    pub granter: Option<String>,
}

/// Signature structure.
#[derive(Debug)]
pub struct StdSignature {
    pub pub_key: Pubkey,
    pub signature: String,
}

/// Public key type.
#[derive(Debug, Clone)]
pub struct Pubkey {
    pub key_type: String, // Flexible type, using String to avoid enum complexity.
    pub value: Vec<u8>,   // Most generic byte storage.
}

/// Algorithm types used for signing.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Algo {
    Secp256k1,
    Ed25519,
    Sr25519,
}

/// Data related to an account.
#[derive(Debug, Clone)]
pub struct AccountData {
    pub address: String, // Assuming the address is a printable string.
    pub algo: Algo,
    pub pubkey: Vec<u8>, // Public key as a byte array.
}

/// SignDoc or SignDocCamelCase, assuming they are previously defined.
/// You could use an enum or separate structs if their structures differ significantly.
#[derive(Debug, Clone)]
pub enum SignDocVariant {
    SignDoc(SignDoc),
    SignDocCamelCase(SignDocCamelCase),
}
