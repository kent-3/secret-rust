/*
 * Secret Network
 *
 * A REST interface for queries and transactions
 *
 * The version of the OpenAPI document: v1.12
 *
 * Generated by: https://openapi-generator.tech
 */

/// CosmosBaseTendermintV1beta1GetLatestValidatorSetResponse : GetLatestValidatorSetResponse is the response type for the Query/GetValidatorSetByHeight RPC method.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CosmosBaseTendermintV1beta1GetLatestValidatorSetResponse {
    #[serde(rename = "block_height", skip_serializing_if = "Option::is_none")]
    pub block_height: Option<String>,
    #[serde(rename = "validators", skip_serializing_if = "Option::is_none")]
    pub validators: Option<Vec<crate::models::GetLatestValidatorSetResponseValidatorsInner>>,
    #[serde(rename = "pagination", skip_serializing_if = "Option::is_none")]
    pub pagination: Option<Box<crate::models::GrantsResponsePagination>>,
}

impl CosmosBaseTendermintV1beta1GetLatestValidatorSetResponse {
    /// GetLatestValidatorSetResponse is the response type for the Query/GetValidatorSetByHeight RPC method.
    pub fn new() -> CosmosBaseTendermintV1beta1GetLatestValidatorSetResponse {
        CosmosBaseTendermintV1beta1GetLatestValidatorSetResponse {
            block_height: None,
            validators: None,
            pagination: None,
        }
    }
}
