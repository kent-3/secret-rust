/*
 * Secret Network
 *
 * A REST interface for queries and transactions
 *
 * The version of the OpenAPI document: v1.12
 *
 * Generated by: https://openapi-generator.tech
 */

/// CosmosTxV1beta1GetBlockWithTxsResponse : GetBlockWithTxsResponse is the response type for the Service.GetBlockWithTxs method.  Since: cosmos-sdk 0.45.2

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CosmosTxV1beta1GetBlockWithTxsResponse {
    /// txs are the transactions in the block.
    #[serde(rename = "txs", skip_serializing_if = "Option::is_none")]
    pub txs: Option<Vec<crate::models::GetTxsEventResponseTxsInner>>,
    #[serde(rename = "block_id", skip_serializing_if = "Option::is_none")]
    pub block_id: Option<Box<crate::models::BlockId1>>,
    #[serde(rename = "block", skip_serializing_if = "Option::is_none")]
    pub block: Option<Box<crate::models::GetLatestBlockResponseBlock>>,
    #[serde(rename = "pagination", skip_serializing_if = "Option::is_none")]
    pub pagination: Option<Box<crate::models::GetTxsEventResponsePagination>>,
}

impl CosmosTxV1beta1GetBlockWithTxsResponse {
    /// GetBlockWithTxsResponse is the response type for the Service.GetBlockWithTxs method.  Since: cosmos-sdk 0.45.2
    pub fn new() -> CosmosTxV1beta1GetBlockWithTxsResponse {
        CosmosTxV1beta1GetBlockWithTxsResponse {
            txs: None,
            block_id: None,
            block: None,
            pagination: None,
        }
    }
}
