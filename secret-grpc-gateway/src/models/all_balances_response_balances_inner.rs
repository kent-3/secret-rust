/*
 * Secret Network
 *
 * A REST interface for queries and transactions
 *
 * The version of the OpenAPI document: v1.12
 *
 * Generated by: https://openapi-generator.tech
 */

/// AllBalancesResponseBalancesInner : Coin defines a token with a denomination and an amount.  NOTE: The amount field is an Int which implements the custom method signatures required by gogoproto.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AllBalancesResponseBalancesInner {
    #[serde(rename = "denom", skip_serializing_if = "Option::is_none")]
    pub denom: Option<String>,
    #[serde(rename = "amount", skip_serializing_if = "Option::is_none")]
    pub amount: Option<String>,
}

impl AllBalancesResponseBalancesInner {
    /// Coin defines a token with a denomination and an amount.  NOTE: The amount field is an Int which implements the custom method signatures required by gogoproto.
    pub fn new() -> AllBalancesResponseBalancesInner {
        AllBalancesResponseBalancesInner {
            denom: None,
            amount: None,
        }
    }
}
