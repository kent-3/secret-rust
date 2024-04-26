/*
 * Secret Network
 *
 * A REST interface for queries and transactions
 *
 * The version of the OpenAPI document: v1.12
 *
 * Generated by: https://openapi-generator.tech
 */

/// CosmosGovV1beta1WeightedVoteOption : WeightedVoteOption defines a unit of vote for vote split.  Since: cosmos-sdk 0.43

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CosmosGovV1beta1WeightedVoteOption {
    /// VoteOption enumerates the valid vote options for a given governance proposal.   - VOTE_OPTION_UNSPECIFIED: VOTE_OPTION_UNSPECIFIED defines a no-op vote option.  - VOTE_OPTION_YES: VOTE_OPTION_YES defines a yes vote option.  - VOTE_OPTION_ABSTAIN: VOTE_OPTION_ABSTAIN defines an abstain vote option.  - VOTE_OPTION_NO: VOTE_OPTION_NO defines a no vote option.  - VOTE_OPTION_NO_WITH_VETO: VOTE_OPTION_NO_WITH_VETO defines a no with veto vote option.
    #[serde(rename = "option", skip_serializing_if = "Option::is_none")]
    pub option: Option<VoteOption>,
    #[serde(rename = "weight", skip_serializing_if = "Option::is_none")]
    pub weight: Option<String>,
}

impl CosmosGovV1beta1WeightedVoteOption {
    /// WeightedVoteOption defines a unit of vote for vote split.  Since: cosmos-sdk 0.43
    pub fn new() -> CosmosGovV1beta1WeightedVoteOption {
        CosmosGovV1beta1WeightedVoteOption {
            option: None,
            weight: None,
        }
    }
}

/// VoteOption enumerates the valid vote options for a given governance proposal.   - VOTE_OPTION_UNSPECIFIED: VOTE_OPTION_UNSPECIFIED defines a no-op vote option.  - VOTE_OPTION_YES: VOTE_OPTION_YES defines a yes vote option.  - VOTE_OPTION_ABSTAIN: VOTE_OPTION_ABSTAIN defines an abstain vote option.  - VOTE_OPTION_NO: VOTE_OPTION_NO defines a no vote option.  - VOTE_OPTION_NO_WITH_VETO: VOTE_OPTION_NO_WITH_VETO defines a no with veto vote option.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum VoteOption {
    #[serde(rename = "VOTE_OPTION_UNSPECIFIED")]
    Unspecified,
    #[serde(rename = "VOTE_OPTION_YES")]
    Yes,
    #[serde(rename = "VOTE_OPTION_ABSTAIN")]
    Abstain,
    #[serde(rename = "VOTE_OPTION_NO")]
    No,
    #[serde(rename = "VOTE_OPTION_NO_WITH_VETO")]
    NoWithVeto,
}

impl Default for VoteOption {
    fn default() -> VoteOption {
        Self::Unspecified
    }
}
