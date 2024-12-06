//! gRPC clients generated by [`tonic`].
//!
//! * Query clients are provided for each module.
//! * All transactions are broadcast using a [`TxServiceClient`].
//!
//! # Examples
#![cfg_attr(feature = "grpc", doc = " ```no_run")]
#![cfg_attr(not(feature = "grpc"), doc = " ```ignore")]
//! # use anyhow::Result;
//! # #[tokio::main(flavor = "current_thread")]
//! # async fn main() -> Result<()> {
//! use secretrs::grpc_clients::AuthQueryClient;
//! use secretrs::proto::cosmos::auth::v1beta1::QueryAccountRequest;
//!
//! let base_url = "http://grpc.testnet.secretsaturn.net:9090";
//! let mut auth_client = AuthQueryClient::connect(base_url).await?;
//!
//! let address = "secret1ap26qrlp8mcq2pg6r47w43l0y8zkqm8a450s03".to_string();
//! let request = QueryAccountRequest { address };
//! let response = auth_client.account(request).await?.into_inner();
//! # Ok(())
//! # }
//! ```
//!
//! [`tonic`]: https://docs.rs/tonic

use ::cosmrs::proto::cosmos;
use ::ibc_proto::ibc;
use ::secret_sdk_proto::secret;

// Service Clients
pub use cosmos::base::tendermint::v1beta1::service_client::ServiceClient as TendermintServiceClient;
pub use cosmos::tx::v1beta1::service_client::ServiceClient as TxServiceClient;

// Query Clients
pub use cosmos::auth::v1beta1::query_client::QueryClient as AuthQueryClient;
pub use cosmos::authz::v1beta1::query_client::QueryClient as AuthzQueryClient;
pub use cosmos::bank::v1beta1::query_client::QueryClient as BankQueryClient;
pub use cosmos::distribution::v1beta1::query_client::QueryClient as DistributionQueryClient;
pub use cosmos::evidence::v1beta1::query_client::QueryClient as EvidenceQueryClient;
pub use cosmos::feegrant::v1beta1::query_client::QueryClient as FeeGrantQueryClient;
pub use cosmos::gov::v1beta1::query_client::QueryClient as GovQueryClient;
pub use cosmos::mint::v1beta1::query_client::QueryClient as MintQueryClient;
pub use cosmos::params::v1beta1::query_client::QueryClient as ParamsQueryClient;
pub use cosmos::slashing::v1beta1::query_client::QueryClient as SlashingQueryClient;
pub use cosmos::staking::v1beta1::query_client::QueryClient as StakingQueryClient;
pub use cosmos::upgrade::v1beta1::query_client::QueryClient as UpgradeQueryClient;

// Secret Query Clients

pub use secret::compute::v1beta1::query_client::QueryClient as ComputeQueryClient;
pub use secret::emergencybutton::v1beta1::query_client::QueryClient as EmergencyButtonQueryClient;
pub use secret::registration::v1beta1::query_client::QueryClient as RegistrationQueryClient;

// IBC
pub use ibc::applications::interchain_accounts::controller::v1::query_client::QueryClient as IbcInterchainAccountsControllerQueryClient;
pub use ibc::applications::interchain_accounts::host::v1::query_client::QueryClient as IbcInterchainAccountsHostQueryClient;
pub use ibc::applications::transfer::v1::query_client::QueryClient as IbcTransferQueryClient;
pub use ibc::core::channel::v1::query_client::QueryClient as IbcChannelQueryClient;
pub use ibc::core::client::v1::query_client::QueryClient as IbcClientQueryClient;
pub use ibc::core::connection::v1::query_client::QueryClient as IbcConnectionQueryClient;

// Note: It is not possible to expose any Protobuf Msg service endpoints via gRPC.
// Transactions must be generated and signed using the CLI or programmatically
// before they can be broadcasted using gRPC.
//
// Shown for reference:
//
// Message Clients
// pub use ::cosmrs::proto::cosmos::authz::v1beta1::msg_client::MsgClient as AuthzMsgClient;
// pub use ::cosmrs::proto::cosmos::bank::v1beta1::msg_client::MsgClient as BankMsgClient;
// pub use ::cosmrs::proto::cosmos::distribution::v1beta1::msg_client::MsgClient as DistributionMsgClient;
// pub use ::cosmrs::proto::cosmos::evidence::v1beta1::msg_client::MsgClient as EvidenceMsgClient;
// pub use ::cosmrs::proto::cosmos::feegrant::v1beta1::msg_client::MsgClient as FeeGrantMsgClient;
// pub use ::cosmrs::proto::cosmos::gov::v1beta1::msg_client::MsgClient as GovMsgClient;
// pub use ::cosmrs::proto::cosmos::slashing::v1beta1::msg_client::MsgClient as SlashingMsgClient;
// pub use ::cosmrs::proto::cosmos::staking::v1beta1::msg_client::MsgClient as StakingMsgClient;
// pub use ::cosmrs::proto::cosmos::upgrade::v1beta1::msg_client::MsgClient as UpgradeMsgClient;
// pub use ::cosmrs::proto::cosmos::vesting::v1beta1::msg_client::MsgClient as VestingMsgClient;

// Secret Message Clients
// pub use ::secret_sdk_proto::secret::compute::v1beta1::msg_client::MsgClient as ComputeMsgClient;
// pub use ::secret_sdk_proto::secret::emergencybutton::v1beta1::msg_client::MsgClient as EmergencyButtonMsgClient;
// pub use ::secret_sdk_proto::secret::intertx::v1beta1::msg_client::MsgClient as InterTxMsgClient;

#[cfg(target_arch = "wasm32")]
#[cfg(test)]
mod test {
    use wasm_bindgen_test::*;

    wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    #[ignore = "response takes too long"]
    async fn client_works_in_browser() {
        use crate::grpc_clients::AuthQueryClient;
        use crate::proto::cosmos::auth::v1beta1::QueryParamsRequest;
        use ::tonic_web_wasm_client::Client;

        const GRPC_WEB_URL: &str = "https://grpc.testnet.secretsaturn.net";

        let mut secret_auth = AuthQueryClient::new(Client::new(GRPC_WEB_URL.to_string()));
        let request = QueryParamsRequest {};
        let response = secret_auth.params(request).await.expect("response");

        let (_metadata, response, _extensions) = response.into_parts();
        println!("Response => {:?}", response);
    }
}
