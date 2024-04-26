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
pub struct SecretComputeV1beta1ContractInfo {
    #[serde(rename = "code_id", skip_serializing_if = "Option::is_none")]
    pub code_id: Option<String>,
    #[serde(rename = "creator", skip_serializing_if = "Option::is_none")]
    pub creator: Option<String>,
    /// Label is mandatory metadata to be stored with a contract instance.
    #[serde(rename = "label", skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[serde(rename = "created", skip_serializing_if = "Option::is_none")]
    pub created: Option<Box<crate::models::AbsoluteTxPositionCanBeUsedToSortContracts1>>,
    #[serde(rename = "ibc_port_id", skip_serializing_if = "Option::is_none")]
    pub ibc_port_id: Option<String>,
    #[serde(rename = "admin", skip_serializing_if = "Option::is_none")]
    pub admin: Option<String>,
    #[serde(rename = "admin_proof", skip_serializing_if = "Option::is_none")]
    pub admin_proof: Option<String>,
}

impl SecretComputeV1beta1ContractInfo {
    pub fn new() -> SecretComputeV1beta1ContractInfo {
        SecretComputeV1beta1ContractInfo {
            code_id: None,
            creator: None,
            label: None,
            created: None,
            ibc_port_id: None,
            admin: None,
            admin_proof: None,
        }
    }
}
