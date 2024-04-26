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
pub struct IbcApplicationsFeeV1QueryTotalRecvFeesResponse {
    #[serde(rename = "recv_fees", skip_serializing_if = "Option::is_none")]
    pub recv_fees: Option<Vec<crate::models::AllBalancesResponseBalancesInner>>,
}

impl IbcApplicationsFeeV1QueryTotalRecvFeesResponse {
    pub fn new() -> IbcApplicationsFeeV1QueryTotalRecvFeesResponse {
        IbcApplicationsFeeV1QueryTotalRecvFeesResponse { recv_fees: None }
    }
}
