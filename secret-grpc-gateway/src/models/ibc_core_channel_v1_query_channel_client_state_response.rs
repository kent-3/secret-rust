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
pub struct IbcCoreChannelV1QueryChannelClientStateResponse {
    #[serde(
        rename = "identified_client_state",
        skip_serializing_if = "Option::is_none"
    )]
    pub identified_client_state: Option<Box<crate::models::ClientStateAssociatedWithTheChannel>>,
    #[serde(rename = "proof", skip_serializing_if = "Option::is_none")]
    pub proof: Option<String>,
    #[serde(rename = "proof_height", skip_serializing_if = "Option::is_none")]
    pub proof_height: Option<Box<crate::models::HeightAtWhichTheProofWasRetrieved>>,
}

impl IbcCoreChannelV1QueryChannelClientStateResponse {
    pub fn new() -> IbcCoreChannelV1QueryChannelClientStateResponse {
        IbcCoreChannelV1QueryChannelClientStateResponse {
            identified_client_state: None,
            proof: None,
            proof_height: None,
        }
    }
}
