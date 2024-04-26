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
pub struct SecretComputeV1beta1AbsoluteTxPosition {
    #[serde(rename = "block_height", skip_serializing_if = "Option::is_none")]
    pub block_height: Option<String>,
    #[serde(rename = "tx_index", skip_serializing_if = "Option::is_none")]
    pub tx_index: Option<String>,
}

impl SecretComputeV1beta1AbsoluteTxPosition {
    pub fn new() -> SecretComputeV1beta1AbsoluteTxPosition {
        SecretComputeV1beta1AbsoluteTxPosition {
            block_height: None,
            tx_index: None,
        }
    }
}
