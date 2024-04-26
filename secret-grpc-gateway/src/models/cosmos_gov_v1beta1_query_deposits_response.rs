/*
 * Secret Network
 *
 * A REST interface for queries and transactions
 *
 * The version of the OpenAPI document: v1.12
 *
 * Generated by: https://openapi-generator.tech
 */

/// CosmosGovV1beta1QueryDepositsResponse : QueryDepositsResponse is the response type for the Query/Deposits RPC method.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CosmosGovV1beta1QueryDepositsResponse {
    #[serde(rename = "deposits", skip_serializing_if = "Option::is_none")]
    pub deposits: Option<Vec<crate::models::DepositsResponseDepositsInner>>,
    #[serde(rename = "pagination", skip_serializing_if = "Option::is_none")]
    pub pagination: Option<Box<crate::models::AccountsResponsePagination>>,
}

impl CosmosGovV1beta1QueryDepositsResponse {
    /// QueryDepositsResponse is the response type for the Query/Deposits RPC method.
    pub fn new() -> CosmosGovV1beta1QueryDepositsResponse {
        CosmosGovV1beta1QueryDepositsResponse {
            deposits: None,
            pagination: None,
        }
    }
}
