/*
 * Secret Network
 *
 * A REST interface for queries and transactions
 *
 * The version of the OpenAPI document: v1.12
 *
 * Generated by: https://openapi-generator.tech
 */

/// ClientStateAssociatedWithTheChannel : IdentifiedClientState defines a client state with an additional client identifier field.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ClientStateAssociatedWithTheChannel {
    #[serde(rename = "client_id", skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,
    #[serde(rename = "client_state", skip_serializing_if = "Option::is_none")]
    pub client_state: Option<Box<crate::models::ClientState>>,
}

impl ClientStateAssociatedWithTheChannel {
    /// IdentifiedClientState defines a client state with an additional client identifier field.
    pub fn new() -> ClientStateAssociatedWithTheChannel {
        ClientStateAssociatedWithTheChannel {
            client_id: None,
            client_state: None,
        }
    }
}
