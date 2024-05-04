use super::{Error, Result};
use crate::{
    clients::BankQueryClient,
    proto::cosmos::bank::v1beta1::{Params, QueryParamsRequest, QueryParamsResponse},
};
use tonic::codegen::{Body, Bytes, StdError};

#[derive(Debug, Clone)]
pub struct BankQuerier<T> {
    inner: BankQueryClient<T>,
}

#[cfg(not(target_arch = "wasm32"))]
impl BankQuerier<::tonic::transport::Channel> {
    pub fn new(channel: ::tonic::transport::Channel) -> Self {
        let auth = BankQueryClient::new(channel);
        Self { inner: auth }
    }
}

#[cfg(target_arch = "wasm32")]
impl BankQuerier<::tonic_web_wasm_client::Client> {
    pub fn new(client: ::tonic_web_wasm_client::Client) -> Self {
        let auth = BankQueryClient::new(client);
        Self { inner: auth }
    }
}

// TODO - add ability to set headers of the request (especially 'x-cosmos-block-height')

impl<T> BankQuerier<T>
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

        let response = client.params(req).await?;
        let (_metadata, response, _extensions) = response.into_parts();

        response.params.ok_or("params empty".into())
    }
}
