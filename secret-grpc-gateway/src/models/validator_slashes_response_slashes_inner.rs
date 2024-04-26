/*
 * Secret Network
 *
 * A REST interface for queries and transactions
 *
 * The version of the OpenAPI document: v1.12
 *
 * Generated by: https://openapi-generator.tech
 */

/// ValidatorSlashesResponseSlashesInner : ValidatorSlashEvent represents a validator slash event. Height is implicit within the store key. This is needed to calculate appropriate amount of staking tokens for delegations which are withdrawn after a slash has occurred.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ValidatorSlashesResponseSlashesInner {
    #[serde(rename = "validator_period", skip_serializing_if = "Option::is_none")]
    pub validator_period: Option<String>,
    #[serde(rename = "fraction", skip_serializing_if = "Option::is_none")]
    pub fraction: Option<String>,
}

impl ValidatorSlashesResponseSlashesInner {
    /// ValidatorSlashEvent represents a validator slash event. Height is implicit within the store key. This is needed to calculate appropriate amount of staking tokens for delegations which are withdrawn after a slash has occurred.
    pub fn new() -> ValidatorSlashesResponseSlashesInner {
        ValidatorSlashesResponseSlashesInner {
            validator_period: None,
            fraction: None,
        }
    }
}
