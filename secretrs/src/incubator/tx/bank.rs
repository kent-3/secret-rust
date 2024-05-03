#![allow(unused)]

use crate::{
    clients::TxServiceClient,
    incubator::{secret_network_client::TxOptions, Error, Result, GRPC_URL},
};
use cosmrs::{
    bank::{MsgMultiSend, MsgSend},
    tx::{BodyBuilder, Msg, Raw, SignDoc, Tx},
};
use secret_sdk_proto::cosmos::{
    base::abci::v1beta1::TxResponse,
    tx::v1beta1::{BroadcastTxRequest, BroadcastTxResponse},
};
use tonic::codegen::{Body, Bytes, StdError};

#[derive(Debug, Clone)]
pub struct BankServiceClient<T>
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
impl BankServiceClient<::tonic::transport::Channel> {
    pub fn new(channel: ::tonic::transport::Channel) -> Self {
        let tx = TxServiceClient::new(channel);
        Self { inner: tx }
    }
}

#[cfg(target_arch = "wasm32")]
impl BankServiceClient<::tonic_web_wasm_client::Client> {
    pub fn new(client: ::tonic_web_wasm_client::Client) -> Self {
        let tx = TxServiceClient::<::tonic_web_wasm_client::Client>::new(client);
        Self { inner: tx }
    }
}

impl<T> BankServiceClient<T>
where
    T: tonic::client::GrpcService<tonic::body::BoxBody>,
    T::Error: Into<StdError>,
    T::ResponseBody: Body<Data = Bytes> + Send + 'static,
    <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    T: Clone,
{
    pub async fn send(&self, msg: MsgSend, tx_options: TxOptions) -> Result<TxResponse> {
        let tx_request = self.prepare_tx(msg, tx_options);
        let tx_response = self
            .perform(tx_request)
            .await?
            .into_inner()
            .tx_response
            .ok_or("no response")?;

        Ok(tx_response)
    }

    pub async fn multi_send(&self, msg: MsgMultiSend, tx_options: TxOptions) -> Result<TxResponse> {
        let tx_request = self.prepare_tx(msg, tx_options);
        let tx_response = self
            .perform(tx_request)
            .await?
            .into_inner()
            .tx_response
            .ok_or("no response")?;

        Ok(tx_response)
    }

    fn prepare_tx<M: cosmrs::tx::Msg>(&self, msg: M, tx_options: TxOptions) -> BroadcastTxRequest {
        let request = BroadcastTxRequest {
            tx_bytes: vec![],
            mode: tx_options.broadcast_mode.into(),
        };

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
