#![allow(unused)]

use cosmrs::bank::MsgSend;
use cosmrs::tx::Msg;
use cosmrs::tx::Raw;

use secret_sdk_proto::cosmos::base::abci::v1beta1::TxResponse;
use secret_sdk_proto::cosmos::tx::v1beta1::{BroadcastTxRequest, BroadcastTxResponse};
use tonic::codegen::*;
use tonic::{Request, Response, Status};

use crate::clients::*;
use crate::incubator::secret_network_client::TxOptions;
use crate::incubator::Error;
use crate::incubator::Result;
use crate::incubator::GRPC_URL;

pub mod bank;
use crate::clients::TxServiceClient;
pub use bank::BankServiceClient;

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
    // pub compute: ComputeServiceClient<T>,
    tx: TxServiceClient<T>,
}

#[cfg(not(target_arch = "wasm32"))]
impl TxSender<::tonic::transport::Channel> {
    pub async fn connect(url: &'static str) -> Result<Self> {
        let channel = tonic::transport::Channel::from_static(url)
            .connect()
            .await?;
        let bank = BankServiceClient::new(channel.clone());
        let tx = TxServiceClient::new(channel.clone());

        Ok(Self { bank, tx })
    }
    pub async fn new(channel: ::tonic::transport::Channel) -> Result<Self> {
        let bank = BankServiceClient::new(channel.clone());
        let tx = TxServiceClient::new(channel.clone());

        Ok(Self { bank, tx })
    }
}

#[cfg(target_arch = "wasm32")]
impl TxSender<::tonic_web_wasm_client::Client> {
    pub fn new(tx_client: TxServiceClient<::tonic_web_wasm_clien::Client>) -> Self {
        let bank = BankServiceClient::new(tx_client.clone());
        let tx = tx_client.clone();
        Self { bank, tx }
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
    pub fn broadcast(&self, msg: Vec<impl Msg>) {
        todo!()
    }
}
