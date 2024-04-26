use color_eyre::Result;
use serde::{Deserialize, Serialize};

const CODE_HASH: &str = "9a00ca4ad505e9be7e6e6dddf8d939b7ec7e9ac8e109c8681f10db9cacb36d42";
const TESTNET_ENCLAVE_KEY: &str =
    "e24a22b31e3d34e0e00bcd32189548f1ccbdc9cda8f5a266219b908582b6f03f";

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    TokenInfo {},
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
    let query = QueryMsg::TokenInfo {};
    let account = secretrs::utils::Account::random();

    let (_nonce, encrypted) =
        secretrs::utils::encrypt_msg(&query, CODE_HASH, &account, TESTNET_ENCLAVE_KEY).await?;

    println!("{}", hex::encode(&encrypted));

    // Use this to decrypt responses from the enclave:
    //
    // let decrypter = secret_utils::decrypter(&nonce, &account, TESTNET_ENCLAVE_KEY).await?;
    // let decrypted_bytes = decrypter.decrypt(&response.data)?;

    Ok(())
}
