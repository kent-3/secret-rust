// #![allow(unused)]

use super::{Error, Result, TxOptions};
use crate::{
    clients::TxServiceClient,
    proto::cosmos::{
        base::abci::v1beta1::TxResponse,
        feegrant::v1beta1::{MsgGrantAllowance, MsgRevokeAllowance},
        tx::v1beta1::{BroadcastTxRequest, BroadcastTxResponse},
    },
    tx::{BodyBuilder, Msg, Raw, SignDoc, Tx},
};
use std::sync::Arc;
use tonic::codegen::{Body, Bytes, StdError};

#[derive(Debug, Clone)]
pub struct FeegrantServiceClient<T>
where
    T: tonic::client::GrpcService<tonic::body::BoxBody>,
    T::Error: Into<StdError>,
    T::ResponseBody: Body<Data = Bytes> + Send + 'static,
    <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    T: Clone,
{
    inner: TxServiceClient<T>,
}

#[cfg(not(target_arch = "wasm32"))]
impl FeegrantServiceClient<::tonic::transport::Channel> {
    pub fn new(channel: ::tonic::transport::Channel) -> Self {
        let inner = TxServiceClient::new(channel);
        Self { inner }
    }
}

#[cfg(target_arch = "wasm32")]
impl FeegrantServiceClient<::tonic_web_wasm_client::Client> {
    pub fn new(client: ::tonic_web_wasm_client::Client) -> Self {
        let inner = TxServiceClient::new(client);
        Self { inner }
    }
}

impl<T> FeegrantServiceClient<T>
where
    T: tonic::client::GrpcService<tonic::body::BoxBody>,
    T::Error: Into<StdError>,
    T::ResponseBody: Body<Data = Bytes> + Send + 'static,
    <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    T: Clone,
{
    pub async fn grant_allowance(
        &self,
        msg: MsgGrantAllowance,
        tx_options: TxOptions,
    ) -> Result<TxResponse> {
        todo!()
    }

    pub async fn revoke_allowance(
        &self,
        msg: MsgRevokeAllowance,
        tx_options: TxOptions,
    ) -> Result<TxResponse> {
        todo!()
    }

    fn prepare_tx<M: cosmrs::proto::traits::Message>(
        &self,
        msg: M,
        tx_options: TxOptions,
    ) -> BroadcastTxRequest {
        todo!()
    }

    async fn sign(&self, sign_doc: SignDoc) -> Result<Raw> {
        todo!()
    }

    async fn perform(
        &self,
        request: BroadcastTxRequest,
    ) -> ::tonic::Result<::tonic::Response<BroadcastTxResponse>, ::tonic::Status> {
        self.inner.clone().broadcast_tx(request).await
    }
}
