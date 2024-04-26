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
pub struct GetLatestBlockResponseBlockEvidence {
    #[serde(rename = "evidence", skip_serializing_if = "Option::is_none")]
    pub evidence: Option<Vec<crate::models::GetLatestBlockResponseBlockEvidenceEvidenceInner>>,
}

impl GetLatestBlockResponseBlockEvidence {
    pub fn new() -> GetLatestBlockResponseBlockEvidence {
        GetLatestBlockResponseBlockEvidence { evidence: None }
    }
}
