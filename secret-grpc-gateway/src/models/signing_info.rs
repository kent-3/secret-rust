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
pub struct SigningInfo {
    #[serde(rename = "start_height", skip_serializing_if = "Option::is_none")]
    pub start_height: Option<String>,
    #[serde(rename = "index_offset", skip_serializing_if = "Option::is_none")]
    pub index_offset: Option<String>,
    #[serde(rename = "jailed_until", skip_serializing_if = "Option::is_none")]
    pub jailed_until: Option<String>,
    #[serde(
        rename = "missed_blocks_counter",
        skip_serializing_if = "Option::is_none"
    )]
    pub missed_blocks_counter: Option<String>,
}

impl SigningInfo {
    pub fn new() -> SigningInfo {
        SigningInfo {
            start_height: None,
            index_offset: None,
            jailed_until: None,
            missed_blocks_counter: None,
        }
    }
}
