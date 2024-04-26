/*
 * Secret Network
 *
 * A REST interface for queries and transactions
 *
 * The version of the OpenAPI document: v1.12
 *
 * Generated by: https://openapi-generator.tech
 */

/// GetValidatorSetByHeightResponse : GetValidatorSetByHeightResponse is the response type for the Query/GetValidatorSetByHeight RPC method.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GetValidatorSetByHeightResponse {
    #[serde(rename = "block_height", skip_serializing_if = "Option::is_none")]
    pub block_height: Option<String>,
    #[serde(rename = "validators", skip_serializing_if = "Option::is_none")]
    pub validators: Option<Vec<crate::models::GetLatestValidatorSetResponseValidatorsInner>>,
    #[serde(rename = "pagination", skip_serializing_if = "Option::is_none")]
    pub pagination: Option<Box<crate::models::GrantsResponsePagination>>,
}

impl GetValidatorSetByHeightResponse {
    /// GetValidatorSetByHeightResponse is the response type for the Query/GetValidatorSetByHeight RPC method.
    pub fn new() -> GetValidatorSetByHeightResponse {
        GetValidatorSetByHeightResponse {
            block_height: None,
            validators: None,
            pagination: None,
        }
    }
}
