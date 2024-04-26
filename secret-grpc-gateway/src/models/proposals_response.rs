/*
 * Secret Network
 *
 * A REST interface for queries and transactions
 *
 * The version of the OpenAPI document: v1.12
 *
 * Generated by: https://openapi-generator.tech
 */

/// ProposalsResponse : QueryProposalsResponse is the response type for the Query/Proposals RPC method.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ProposalsResponse {
    #[serde(rename = "proposals", skip_serializing_if = "Option::is_none")]
    pub proposals: Option<Vec<crate::models::ProposalsResponseProposalsInner>>,
    #[serde(rename = "pagination", skip_serializing_if = "Option::is_none")]
    pub pagination: Option<Box<crate::models::AccountsResponsePagination>>,
}

impl ProposalsResponse {
    /// QueryProposalsResponse is the response type for the Query/Proposals RPC method.
    pub fn new() -> ProposalsResponse {
        ProposalsResponse {
            proposals: None,
            pagination: None,
        }
    }
}
