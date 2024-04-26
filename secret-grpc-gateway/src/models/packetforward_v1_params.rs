/*
 * Secret Network
 *
 * A REST interface for queries and transactions
 *
 * The version of the OpenAPI document: v1.12
 *
 * Generated by: https://openapi-generator.tech
 */

/// PacketforwardV1Params : Params defines the set of IBC packetforward parameters.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PacketforwardV1Params {
    #[serde(rename = "fee_percentage", skip_serializing_if = "Option::is_none")]
    pub fee_percentage: Option<String>,
}

impl PacketforwardV1Params {
    /// Params defines the set of IBC packetforward parameters.
    pub fn new() -> PacketforwardV1Params {
        PacketforwardV1Params {
            fee_percentage: None,
        }
    }
}
