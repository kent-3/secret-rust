/*
 * Secret Network
 *
 * A REST interface for queries and transactions
 *
 * The version of the OpenAPI document: v1.12
 *
 * Generated by: https://openapi-generator.tech
 */

/// AllBalancesResponse : QueryAllBalancesResponse is the response type for the Query/AllBalances RPC method.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AllBalancesResponse {
    /// balances is the balances of all the coins.
    #[serde(rename = "balances", skip_serializing_if = "Option::is_none")]
    pub balances: Option<Vec<crate::models::AllBalancesResponseBalancesInner>>,
    #[serde(rename = "pagination", skip_serializing_if = "Option::is_none")]
    pub pagination: Option<Box<crate::models::AccountsResponsePagination>>,
}

impl AllBalancesResponse {
    /// QueryAllBalancesResponse is the response type for the Query/AllBalances RPC method.
    pub fn new() -> AllBalancesResponse {
        AllBalancesResponse {
            balances: None,
            pagination: None,
        }
    }
}
