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
pub struct GetLatestBlockResponseBlockEvidenceEvidenceInnerLightClientAttackEvidenceConflictingBlockValidatorSetValidatorsInner
{
    #[serde(rename = "address", skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    #[serde(rename = "pub_key", skip_serializing_if = "Option::is_none")]
    pub pub_key:
        Option<Box<crate::models::PublicKeyDefinesTheKeysAvailableForUseWithTendermintValidators>>,
    #[serde(rename = "voting_power", skip_serializing_if = "Option::is_none")]
    pub voting_power: Option<String>,
    #[serde(rename = "proposer_priority", skip_serializing_if = "Option::is_none")]
    pub proposer_priority: Option<String>,
}

impl GetLatestBlockResponseBlockEvidenceEvidenceInnerLightClientAttackEvidenceConflictingBlockValidatorSetValidatorsInner {
    pub fn new() -> GetLatestBlockResponseBlockEvidenceEvidenceInnerLightClientAttackEvidenceConflictingBlockValidatorSetValidatorsInner {
        GetLatestBlockResponseBlockEvidenceEvidenceInnerLightClientAttackEvidenceConflictingBlockValidatorSetValidatorsInner {
            address: None,
            pub_key: None,
            voting_power: None,
            proposer_priority: None,
        }
    }
}
