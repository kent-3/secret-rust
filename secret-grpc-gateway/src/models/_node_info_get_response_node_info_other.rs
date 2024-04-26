/*
 * Secret Network
 *
 * A REST interface for queries and transactions
 *
 * The version of the OpenAPI document: v1.12
 *
 * Generated by: https://openapi-generator.tech
 */

/// NodeInfoGetResponseNodeInfoOther : more information on versions

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NodeInfoGetResponseNodeInfoOther {
    #[serde(rename = "tx_index", skip_serializing_if = "Option::is_none")]
    pub tx_index: Option<String>,
    #[serde(rename = "rpc_address", skip_serializing_if = "Option::is_none")]
    pub rpc_address: Option<String>,
}

impl NodeInfoGetResponseNodeInfoOther {
    /// more information on versions
    pub fn new() -> NodeInfoGetResponseNodeInfoOther {
        NodeInfoGetResponseNodeInfoOther {
            tx_index: None,
            rpc_address: None,
        }
    }
}
