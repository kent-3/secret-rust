use cosmrs::proto::cosmos::bank::v1beta1::{Params, QueryParamsRequest, QueryParamsResponse};

use tonic::codegen::*;
use tonic::{Request, Response, Status};

use crate::clients::ComputeQueryClient;
use crate::incubator::Error;
use crate::incubator::Result;
use crate::EncryptionUtils;

#[derive(Debug, Clone)]
pub struct ComputeQuerier<T> {
    inner: ComputeQueryClient<T>,
    // encryption: EncryptionUtils,
}

#[cfg(not(target_arch = "wasm32"))]
impl ComputeQuerier<::tonic::transport::Channel> {
    pub fn new(channel: ::tonic::transport::Channel) -> Self {
        let auth = ComputeQueryClient::new(channel);
        // let encryption = EncryptionUtils::new(None, "pulsar-3")?;
        Self {
            inner: auth,
            // encryption,
        }
    }
}

#[cfg(target_arch = "wasm32")]
impl ComputeQuerier<::tonic_web_wasm_client::Client> {
    pub fn new(client: ::tonic_web_wasm_client::Client) -> Self {
        let auth = ComputeQueryClient::new(client);
        Self { inner: auth }
    }
}

impl<T> ComputeQuerier<T>
where
    T: tonic::client::GrpcService<tonic::body::BoxBody>,
    T::Error: Into<StdError>,
    T::ResponseBody: Body<Data = Bytes> + Send + 'static,
    <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    T: Clone,
{
    pub async fn params(&self) -> Result<Params> {
        let req = QueryParamsRequest {};
        let mut client = self.inner.clone();

        // let response = client.params(req).await?;
        // let (_metadata, response, _extensions) = response.into_parts();

        // response.params.ok_or("params empty".into())
        todo!()
    }
}
