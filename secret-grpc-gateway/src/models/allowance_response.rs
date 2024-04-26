/*
 * Secret Network
 *
 * A REST interface for queries and transactions
 *
 * The version of the OpenAPI document: v1.12
 *
 * Generated by: https://openapi-generator.tech
 */

/// AllowanceResponse : QueryAllowanceResponse is the response type for the Query/Allowance RPC method.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AllowanceResponse {
    #[serde(rename = "allowance", skip_serializing_if = "Option::is_none")]
    pub allowance:
        Option<Box<crate::models::GrantIsStoredInTheKvStoreToRecordAGrantWithFullContext>>,
}

impl AllowanceResponse {
    /// QueryAllowanceResponse is the response type for the Query/Allowance RPC method.
    pub fn new() -> AllowanceResponse {
        AllowanceResponse { allowance: None }
    }
}
