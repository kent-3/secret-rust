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
pub struct GovProposalsProposalIdVotesGetResponseInner {
    #[serde(rename = "voter", skip_serializing_if = "Option::is_none")]
    pub voter: Option<String>,
    #[serde(rename = "proposal_id", skip_serializing_if = "Option::is_none")]
    pub proposal_id: Option<String>,
    #[serde(rename = "option", skip_serializing_if = "Option::is_none")]
    pub option: Option<String>,
}

impl GovProposalsProposalIdVotesGetResponseInner {
    pub fn new() -> GovProposalsProposalIdVotesGetResponseInner {
        GovProposalsProposalIdVotesGetResponseInner {
            voter: None,
            proposal_id: None,
            option: None,
        }
    }
}
