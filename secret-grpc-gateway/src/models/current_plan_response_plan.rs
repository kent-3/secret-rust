/*
 * Secret Network
 *
 * A REST interface for queries and transactions
 *
 * The version of the OpenAPI document: v1.12
 *
 * Generated by: https://openapi-generator.tech
 */

/// CurrentPlanResponsePlan : plan is the current upgrade plan.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CurrentPlanResponsePlan {
    /// Sets the name for the upgrade. This name will be used by the upgraded version of the software to apply any special \"on-upgrade\" commands during the first BeginBlock method after the upgrade is applied. It is also used to detect whether a software version can handle a given upgrade. If no upgrade handler with this name has been set in the software, it will be assumed that the software is out-of-date when the upgrade Time or Height is reached and the software will exit.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Deprecated: Time based upgrades have been deprecated. Time based upgrade logic has been removed from the SDK. If this field is not empty, an error will be thrown.
    #[serde(rename = "time", skip_serializing_if = "Option::is_none")]
    pub time: Option<String>,
    /// The height at which the upgrade must be performed. Only used if Time is not set.
    #[serde(rename = "height", skip_serializing_if = "Option::is_none")]
    pub height: Option<String>,
    #[serde(rename = "info", skip_serializing_if = "Option::is_none")]
    pub info: Option<String>,
    #[serde(
        rename = "upgraded_client_state",
        skip_serializing_if = "Option::is_none"
    )]
    pub upgraded_client_state: Option<Box<crate::models::AccountsAreTheExistingAccountsInner>>,
}

impl CurrentPlanResponsePlan {
    /// plan is the current upgrade plan.
    pub fn new() -> CurrentPlanResponsePlan {
        CurrentPlanResponsePlan {
            name: None,
            time: None,
            height: None,
            info: None,
            upgraded_client_state: None,
        }
    }
}
