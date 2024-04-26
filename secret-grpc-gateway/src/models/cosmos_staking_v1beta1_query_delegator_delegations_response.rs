/*
 * Secret Network
 *
 * A REST interface for queries and transactions
 *
 * The version of the OpenAPI document: v1.12
 *
 * Generated by: https://openapi-generator.tech
 */

/// CosmosStakingV1beta1QueryDelegatorDelegationsResponse : QueryDelegatorDelegationsResponse is response type for the Query/DelegatorDelegations RPC method.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CosmosStakingV1beta1QueryDelegatorDelegationsResponse {
    /// delegation_responses defines all the delegations' info of a delegator.
    #[serde(
        rename = "delegation_responses",
        skip_serializing_if = "Option::is_none"
    )]
    pub delegation_responses:
        Option<Vec<crate::models::DelegatorDelegationsResponseDelegationResponsesInner>>,
    #[serde(rename = "pagination", skip_serializing_if = "Option::is_none")]
    pub pagination: Option<Box<crate::models::AccountsResponsePagination>>,
}

impl CosmosStakingV1beta1QueryDelegatorDelegationsResponse {
    /// QueryDelegatorDelegationsResponse is response type for the Query/DelegatorDelegations RPC method.
    pub fn new() -> CosmosStakingV1beta1QueryDelegatorDelegationsResponse {
        CosmosStakingV1beta1QueryDelegatorDelegationsResponse {
            delegation_responses: None,
            pagination: None,
        }
    }
}
