/*
 * Secret Network
 *
 * A REST interface for queries and transactions
 *
 * The version of the OpenAPI document: v1.12
 *
 * Generated by: https://openapi-generator.tech
 */

/// AuthInfoIsTheAuthorizationRelatedContentOfTheTransactionSpecificallySignersSignerModesAndFee : AuthInfo describes the fee and signer modes that are used to sign a transaction.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AuthInfoIsTheAuthorizationRelatedContentOfTheTransactionSpecificallySignersSignerModesAndFee {
    /// signer_infos defines the signing modes for the required signers. The number and order of elements must match the required signers from TxBody's messages. The first element is the primary signer and the one which pays the fee.
    #[serde(rename = "signer_infos", skip_serializing_if = "Option::is_none")]
    pub signer_infos: Option<Vec<crate::models::AuthInfoIsTheAuthorizationRelatedContentOfTheTransactionSpecificallySignersSignerModesAndFeeSignerInfosInner>>,
    #[serde(rename = "fee", skip_serializing_if = "Option::is_none")]
    pub fee: Option<Box<crate::models::AuthInfoIsTheAuthorizationRelatedContentOfTheTransactionSpecificallySignersSignerModesAndFeeFee>>,
}

impl AuthInfoIsTheAuthorizationRelatedContentOfTheTransactionSpecificallySignersSignerModesAndFee {
    /// AuthInfo describes the fee and signer modes that are used to sign a transaction.
    pub fn new(
    ) -> AuthInfoIsTheAuthorizationRelatedContentOfTheTransactionSpecificallySignersSignerModesAndFee
    {
        AuthInfoIsTheAuthorizationRelatedContentOfTheTransactionSpecificallySignersSignerModesAndFee {
            signer_infos: None,
            fee: None,
        }
    }
}
