/*
 * Secret Network
 *
 * A REST interface for queries and transactions
 *
 * The version of the OpenAPI document: v1.12
 *
 * Generated by: https://openapi-generator.tech
 */

/// CosmosUpgradeV1beta1QueryAppliedPlanResponse : QueryAppliedPlanResponse is the response type for the Query/AppliedPlan RPC method.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CosmosUpgradeV1beta1QueryAppliedPlanResponse {
    /// height is the block height at which the plan was applied.
    #[serde(rename = "height", skip_serializing_if = "Option::is_none")]
    pub height: Option<String>,
}

impl CosmosUpgradeV1beta1QueryAppliedPlanResponse {
    /// QueryAppliedPlanResponse is the response type for the Query/AppliedPlan RPC method.
    pub fn new() -> CosmosUpgradeV1beta1QueryAppliedPlanResponse {
        CosmosUpgradeV1beta1QueryAppliedPlanResponse { height: None }
    }
}
