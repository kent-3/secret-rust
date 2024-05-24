use base64::prelude::{Engine, BASE64_STANDARD};
use color_eyre::{eyre::Error, owo_colors::OwoColorize, Result};
use regex::Regex;
use serde::{Deserialize, Serialize};

use secretrs::{
    grpc_clients::{ComputeQueryClient, RegistrationQueryClient},
    proto::secret::compute::v1beta1::{QueryByContractAddressRequest, QuerySecretContractRequest},
    utils::encryption::EncryptionUtils,
};

const GRPC_URL: &str = "http://grpc.testnet.secretsaturn.net:9090";
const CONTRACT_ADDRESS: &str = "secret19gtpkk25r0c36gtlyrc6repd3q52ngmkpfszw3";

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
    color_eyre::install()?;

    println!("\n{}", "Registration Module".underline().blue());
    println!("Creating `registration` query client...");
    let mut secret_registration = RegistrationQueryClient::connect(GRPC_URL).await?;

    let enclave_key_bytes = secret_registration.tx_key(()).await?.into_inner().key;
    let enclave_key = hex::encode(enclave_key_bytes);
    println!("Enclave Public Key => {:>4}", enclave_key.bright_blue());

    println!("\n{}", "Compute Module".underline().blue());
    println!("Creating `compute` query client...");
    let mut secret_compute = ComputeQueryClient::connect(GRPC_URL).await?;

    let request = QueryByContractAddressRequest {
        contract_address: CONTRACT_ADDRESS.to_string(),
    };

    let code_hash = secret_compute
        .code_hash_by_contract_address(request)
        .await?
        .into_inner()
        .code_hash;

    let query = QueryMsg::TokenInfo {};
    println!("Query => {}", serde_json::to_string(&query)?.green());

    let encryption_utils = EncryptionUtils::new(None, "pulsar-3")?;
    let encrypted = encryption_utils.encrypt(&code_hash, &query)?;
    let nonce = encrypted.nonce();
    let query = encrypted.into_inner();

    let display_request = format!(
        "QuerySecretContractRequest {{ contract_address: \"{}\", query: \"{}\" }}",
        CONTRACT_ADDRESS,
        BASE64_STANDARD.encode(&query)
    );
    println!("Request => {}", display_request.green());

    let request = QuerySecretContractRequest {
        contract_address: CONTRACT_ADDRESS.to_string(),
        query,
    };

    let response = match secret_compute.query_secret_contract(request).await {
        Ok(response) => {
            let response = response.into_inner();
            let decrypted_bytes = encryption_utils.decrypt(&nonce, &response.data)?;
            let decrypted_b64_string = String::from_utf8(decrypted_bytes)?;
            let decoded_bytes = BASE64_STANDARD.decode(decrypted_b64_string)?;
            let data = String::from_utf8(decoded_bytes)?;
            println!("Response => {}", data.green());
            assert_eq!(
                data.trim(),
                r#"{"token_info":{"name":"Amber","symbol":"AMBER","decimals":6,"total_supply":"8888000000"}}"#
            );

            Ok(data)
        }
        Err(status) => {
            let error_message = status.message();

            let re = Regex::new(r"encrypted: (.*?):").unwrap();

            if let Some(caps) = re.captures(error_message) {
                let encrypted_bytes = BASE64_STANDARD.decode(&caps[1])?;
                let decrypted_bytes = encryption_utils.decrypt(&nonce, &encrypted_bytes)?;
                let decrypted_string = String::from_utf8(decrypted_bytes)?;
                Err(Error::msg(decrypted_string))
            } else {
                Err(Error::msg(error_message.to_string()))
            }
        }
    }?;

    let deserialized: QueryAnswer = serde_json::from_str(response.trim())?;
    println!("Deserialized Response => {:?}", deserialized.green());

    Ok(())
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    TokenInfo {},
    MemberCode { address: String, key: String },
    ValidCodes { codes: Vec<String> },
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum QueryAnswer {
    TokenInfo {
        name: String,
        symbol: String,
        decimals: u8,
        total_supply: String,
    },
    MemberCode {
        code: String,
    },
    ValidCodes {
        codes: Vec<String>,
    },
    ViewingKeyError {
        msg: String,
    },
}
