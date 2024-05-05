use super::{Error, Result};
use crate::{
    clients::AuthQueryClient,
    proto::cosmos::auth::v1beta1::{
        AddressBytesToStringRequest, AddressBytesToStringResponse, AddressStringToBytesRequest,
        AddressStringToBytesResponse, BaseAccount, ModuleAccount, Params,
        QueryAccountAddressByIdRequest, QueryAccountAddressByIdResponse, QueryAccountRequest,
        QueryAccountResponse, QueryAccountsRequest, QueryAccountsResponse,
        QueryModuleAccountsRequest, QueryModuleAccountsResponse, QueryParamsRequest,
        QueryParamsResponse,
    },
    query::PageRequest,
    Any,
};
use secret_sdk_proto::cosmos::auth::v1beta1::{
    QueryModuleAccountByNameRequest, QueryModuleAccountByNameResponse,
};
use tonic::codegen::{Body, Bytes, StdError};

#[derive(Debug, Clone)]
pub struct AuthQuerier<T> {
    inner: AuthQueryClient<T>,
}

#[cfg(not(target_arch = "wasm32"))]
impl AuthQuerier<::tonic::transport::Channel> {
    pub fn new(channel: ::tonic::transport::Channel) -> Self {
        let inner = AuthQueryClient::new(channel);
        Self { inner }
    }
}

#[cfg(target_arch = "wasm32")]
impl AuthQuerier<::tonic_web_wasm_client::Client> {
    pub fn new(client: ::tonic_web_wasm_client::Client) -> Self {
        let inner = AuthQueryClient::new(client);
        Self { inner }
    }
}

// TODO - add ability to set headers of the request (especially 'x-cosmos-block-height')

impl<T> AuthQuerier<T>
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

    pub async fn accounts(&self, pagination: Option<PageRequest>) -> Result<QueryAccountsResponse> {
        let pagination = pagination.map(Into::into);
        let request = QueryAccountsRequest { pagination };
        let response: QueryAccountsResponse = todo!();

        Ok(response)
    }
    pub async fn account(&self, address: impl Into<String>) -> Result<Option<Any>> {
        let address = address.into();
        let request = QueryAccountRequest { address };
        let response: QueryAccountResponse = todo!();

        // TODO - Account is `Any`. Convert to the right type by matching on type_url.
        Ok(response.account)
    }

    pub async fn account_address_by_id(&self, id: impl Into<i64>) -> Result<String> {
        let id = id.into();
        let request = QueryAccountAddressByIdRequest { id };
        let response: QueryAccountAddressByIdResponse = todo!();

        Ok(response.account_address)
    }

    pub async fn module_accounts(&self) -> Result<Vec<::cosmrs::auth::ModuleAccount>> {
        let request = QueryModuleAccountsRequest {};
        let response: QueryModuleAccountsResponse = todo!();

        // convert `Any` to proto::cosmos::auth::v1beta1::ModuleAccount to cosmrs::auth::ModuleAccount
        let accounts: Result<Vec<::cosmrs::auth::ModuleAccount>> = response
            .accounts
            .iter()
            .map(|any| {
                Ok(any.to_msg::<ModuleAccount>()?.try_into()?)
                // alternate approach:
                // let proto_account: ModuleAccount = any.to_msg()?;
                // let module_account: ::cosmrs::auth::ModuleAccount = proto_account.try_into()?;
                // Ok(module_account)
            })
            .collect();
    }

    // TODO - convert the `Any` to proto::cosmos::auth::v1beta1::ModuleAccount to cosmrs::auth::ModuleAccount
    pub async fn module_accounts_by_name(&self, name: impl Into<String>) -> Result<Option<Any>> {
        let name = name.into();
        let request = QueryModuleAccountByNameRequest { name };
        let response: QueryModuleAccountByNameResponse = todo!();

        Ok(response.account)
    }
}
