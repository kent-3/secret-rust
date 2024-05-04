#![allow(unused)]

pub mod bank;
pub mod compute;

pub use bank::BankServiceClient;
pub use compute::ComputeServiceClient;

use super::{Error, Result};
use crate::proto::cosmos::tx::v1beta1::{
    BroadcastTxRequest, BroadcastTxResponse, GetBlockWithTxsRequest, GetBlockWithTxsResponse,
    GetTxRequest, GetTxResponse, GetTxsEventRequest, GetTxsEventResponse, SimulateRequest,
    SimulateResponse,
};

use crate::{
    bank::MsgSend,
    client::{CreateClientOptions, TxOptions},
    clients::TxServiceClient,
    proto::cosmos::base::abci::v1beta1::TxResponse,
    tx::{Msg, Raw},
};
use std::sync::Arc;
use tonic::codegen::{Body, Bytes, StdError};

#[derive(Debug, Clone)]
pub struct TxSender<T>
where
    T: tonic::client::GrpcService<tonic::body::BoxBody>,
    T::Error: Into<StdError>,
    T::ResponseBody: Body<Data = Bytes> + Send + 'static,
    <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    T: Clone,
{
    pub bank: BankServiceClient<T>,
    pub compute: ComputeServiceClient<T>,
    tx: TxServiceClient<T>,
}

#[cfg(not(target_arch = "wasm32"))]
impl TxSender<::tonic::transport::Channel> {
    pub async fn connect(options: &CreateClientOptions) -> Result<Self> {
        let channel = tonic::transport::Channel::from_static(options.url)
            .connect()
            .await?;
        Ok(Self::new(channel, options))
    }

    pub fn new(channel: ::tonic::transport::Channel, options: &CreateClientOptions) -> Self {
        let bank = BankServiceClient::new(channel.clone());
        let compute = ComputeServiceClient::new(channel.clone(), options);
        let tx = TxServiceClient::new(channel.clone());
        //etc

        Self { bank, compute, tx }
    }
}

#[cfg(target_arch = "wasm32")]
impl TxSender<::tonic_web_wasm_client::Client> {
    pub fn new(client: ::tonic_web_wasm_clien::Client) -> Self {
        let bank = BankServiceClient::new(client.clone());
        let compute = ComputeServiceClient::new(client.clone());
        let tx = TxServiceClient::new(client.clone());
        Self { bank, compute, tx }
    }
}

impl<T> TxSender<T>
where
    T: tonic::client::GrpcService<tonic::body::BoxBody>,
    T::Error: Into<StdError>,
    T::ResponseBody: Body<Data = Bytes> + Send + 'static,
    <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    T: Clone,
{
    // TODO - figure out how to support multiple messages
    pub async fn broadcast(&mut self, request: BroadcastTxRequest) -> Result<BroadcastTxResponse> {
        self.tx
            .broadcast_tx(request)
            .await
            .map_err(Into::into)
            .map(::tonic::Response::into_inner)
    }
}
