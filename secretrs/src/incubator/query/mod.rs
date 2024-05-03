#![allow(unused)]

pub mod auth;
pub mod bank;
pub mod compute;

use cosmrs::bank::MsgSend;
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

use auth::AuthQuerier;
use bank::BankQuerier;
use compute::ComputeQuerier;

// IDEA - trait Querier. can get tx by hash, perform txs_query, and all the specific querier types

#[derive(Debug, Clone)]
pub struct Querier<T>
where
    T: tonic::client::GrpcService<tonic::body::BoxBody>,
    T::Error: Into<StdError>,
    T::ResponseBody: Body<Data = Bytes> + Send + 'static,
    <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    T: Clone,
{
    pub auth: AuthQuerier<T>,
    pub bank: BankQuerier<T>,
    pub compute: ComputeQuerier<T>,
}

#[cfg(not(target_arch = "wasm32"))]
impl Querier<::tonic::transport::Channel> {
    pub async fn connect(url: &'static str) -> Result<Self> {
        let channel = ::tonic::transport::Channel::from_static(url)
            .connect()
            .await?;
        let auth = AuthQuerier::new(channel.clone());
        let bank = BankQuerier::new(channel.clone());
        let compute = ComputeQuerier::new(channel.clone());

        Ok(Self {
            auth,
            bank,
            compute,
        })
    }
    pub async fn new(channel: ::tonic::transport::Channel) -> Result<Self> {
        let auth = AuthQuerier::new(channel.clone());
        let bank = BankQuerier::new(channel.clone());
        let compute = ComputeQuerier::new(channel.clone());

        Ok(Self {
            auth,
            bank,
            compute,
        })
    }
}
