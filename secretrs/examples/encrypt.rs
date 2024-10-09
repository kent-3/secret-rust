use color_eyre::Result;
use serde::{Deserialize, Serialize};

use secretrs::utils::EnigmaUtils;

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
    color_eyre::install()?;

    let code_hash = "9a00ca4ad505e9be7e6e6dddf8d939b7ec7e9ac8e109c8681f10db9cacb36d42";
    let query = QueryMsg::TokenInfo {};

    let encryption_utils = EnigmaUtils::new(None, "pulsar-3")?;
    let encrypted = encryption_utils.encrypt(code_hash, &query)?;
    let nonce = encrypted.nonce();
    let query = encrypted.into_inner();
    println!("Encrypted query: {}", hex::encode(&query));

    // Use this to decrypt responses from the enclave:
    let decrypted_bytes = encryption_utils.decrypt(&nonce, &query)?;
    println!("Decrypted query: {}", String::from_utf8(decrypted_bytes)?);

    Ok(())
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    TokenInfo {},
}
