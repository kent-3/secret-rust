/*
 * Secret Network
 *
 * A REST interface for queries and transactions
 *
 * The version of the OpenAPI document: v1.12
 *
 * Generated by: https://openapi-generator.tech
 */

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AuthAccountsAddressGetResponseValue {
    #[serde(rename = "account_number", skip_serializing_if = "Option::is_none")]
    pub account_number: Option<String>,
    #[serde(rename = "address", skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    #[serde(rename = "coins", skip_serializing_if = "Option::is_none")]
    pub coins: Option<Vec<crate::models::TxsHashGetResponseTxFeeAmountInner>>,
    #[serde(rename = "public_key", skip_serializing_if = "Option::is_none")]
    pub public_key: Option<Box<crate::models::AuthAccountsAddressGetResponseValuePublicKey>>,
    #[serde(rename = "sequence", skip_serializing_if = "Option::is_none")]
    pub sequence: Option<String>,
}

impl AuthAccountsAddressGetResponseValue {
    pub fn new() -> AuthAccountsAddressGetResponseValue {
        AuthAccountsAddressGetResponseValue {
            account_number: None,
            address: None,
            coins: None,
            public_key: None,
            sequence: None,
        }
    }
}
