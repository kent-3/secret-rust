#![allow(unused)]

use async_trait::async_trait;

use cosmrs::tendermint::Hash;
use cosmrs::tx::Tx;

use crate::{
    clients::TxServiceClient,
    proto::{self, traits::Message},
    Error, ErrorReport, Gas, Result,
};

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
