#![allow(unused)]

use async_trait::async_trait;

use cosmrs::tendermint::Hash;
use cosmrs::tx::Tx;

use crate::{
    proto::{self, traits::Message},
    Error, ErrorReport, Gas, Result,
};

pub use ::cosmrs::proto::cosmos::auth::v1beta1::query_client::QueryClient as AuthQueryClient;
pub use ::cosmrs::proto::cosmos::authz::v1beta1::query_client::QueryClient as AuthzQueryClient;
pub use ::cosmrs::proto::cosmos::bank::v1beta1::query_client::QueryClient as BankQueryClient;
pub use ::cosmrs::proto::cosmos::distribution::v1beta1::query_client::QueryClient as DistributionQueryClient;
pub use ::cosmrs::proto::cosmos::evidence::v1beta1::query_client::QueryClient as EvidenceQueryClient;
pub use ::cosmrs::proto::cosmos::feegrant::v1beta1::query_client::QueryClient as FeeGrantQueryClient;
pub use ::cosmrs::proto::cosmos::gov::v1beta1::query_client::QueryClient as GovQueryClient;
pub use ::cosmrs::proto::cosmos::mint::v1beta1::query_client::QueryClient as MintQueryClient;
pub use ::cosmrs::proto::cosmos::params::v1beta1::query_client::QueryClient as ParamsQueryClient;
pub use ::cosmrs::proto::cosmos::slashing::v1beta1::query_client::QueryClient as SlashingQueryClient;
pub use ::cosmrs::proto::cosmos::staking::v1beta1::query_client::QueryClient as StakingQueryClient;
pub use ::cosmrs::proto::cosmos::upgrade::v1beta1::query_client::QueryClient as UpgradeQueryClient;

pub use ::cosmrs::proto::cosmos::base::tendermint::v1beta1::service_client::ServiceClient as TendermintQueryClient;

pub use ::secret_sdk_proto::secret::compute::v1beta1::query_client::QueryClient as ComputeQueryClient;
pub use ::secret_sdk_proto::secret::emergencybutton::v1beta1::query_client::QueryClient as EmergencyButtonQueryClient;
pub use ::secret_sdk_proto::secret::intertx::v1beta1::query_client::QueryClient as InterTxQueryClient;
pub use ::secret_sdk_proto::secret::registration::v1beta1::query_client::QueryClient as RegistrationQueryClient;

pub use ::cosmrs::proto::cosmos::tx::v1beta1::service_client::ServiceClient as TxServiceClient;

// Experimental!

#[async_trait]
pub trait GrpcClient {
    async fn grpc_find_by_hash(
        grpc_client: &mut TxServiceClient<::tonic::transport::Channel>,
        tx_hash: Hash,
    ) -> Result<Tx>;
}

#[async_trait]
impl GrpcClient for Tx {
    async fn grpc_find_by_hash(
        grpc_client: &mut TxServiceClient<::tonic::transport::Channel>,
        tx_hash: Hash,
    ) -> Result<Tx> {
        use cosmrs::proto::cosmos::tx::v1beta1::GetTxRequest;

        grpc_client
            .get_tx(GetTxRequest {
                hash: tx_hash.to_string(),
            })
            .await
            .map_err(ErrorReport::from)
            .and_then(|resp| {
                resp.into_inner()
                    .tx
                    .ok_or(Error::TxNotFound { hash: tx_hash })
                    .map_err(ErrorReport::from)
            })
            .and_then(TryInto::try_into)
    }
}
