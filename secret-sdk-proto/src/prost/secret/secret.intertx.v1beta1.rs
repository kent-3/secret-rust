// This file is @generated by prost-build.
/// QueryInterchainAccountFromAddressRequest is the request type for the Query/InterchainAccountAddress RPC
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryInterchainAccountFromAddressRequest {
    #[prost(string, tag = "1")]
    pub owner: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub connection_id: ::prost::alloc::string::String,
}
impl ::prost::Name for QueryInterchainAccountFromAddressRequest {
    const NAME: &'static str = "QueryInterchainAccountFromAddressRequest";
    const PACKAGE: &'static str = "secret.intertx.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        "secret.intertx.v1beta1.QueryInterchainAccountFromAddressRequest".into()
    }
    fn type_url() -> ::prost::alloc::string::String {
        "/secret.intertx.v1beta1.QueryInterchainAccountFromAddressRequest".into()
    }
}
/// QueryInterchainAccountFromAddressResponse the response type for the Query/InterchainAccountAddress RPC
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryInterchainAccountFromAddressResponse {
    #[prost(string, tag = "1")]
    pub interchain_account_address: ::prost::alloc::string::String,
}
impl ::prost::Name for QueryInterchainAccountFromAddressResponse {
    const NAME: &'static str = "QueryInterchainAccountFromAddressResponse";
    const PACKAGE: &'static str = "secret.intertx.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        "secret.intertx.v1beta1.QueryInterchainAccountFromAddressResponse".into()
    }
    fn type_url() -> ::prost::alloc::string::String {
        "/secret.intertx.v1beta1.QueryInterchainAccountFromAddressResponse".into()
    }
}
/// Generated client implementations.
#[cfg(feature = "grpc")]
pub mod query_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::http::Uri;
    use tonic::codegen::*;
    /// Query defines the gRPC querier service.
    #[derive(Debug, Clone)]
    pub struct QueryClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    #[cfg(feature = "grpc-transport")]
    impl QueryClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> QueryClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + std::marker::Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + std::marker::Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_origin(inner: T, origin: Uri) -> Self {
            let inner = tonic::client::Grpc::with_origin(inner, origin);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> QueryClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<http::Request<tonic::body::BoxBody>>>::Error:
                Into<StdError> + std::marker::Send + std::marker::Sync,
        {
            QueryClient::new(InterceptedService::new(inner, interceptor))
        }
        /// Compress requests with the given encoding.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.send_compressed(encoding);
            self
        }
        /// Enable decompressing responses.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.accept_compressed(encoding);
            self
        }
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_decoding_message_size(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_encoding_message_size(limit);
            self
        }
        /// QueryInterchainAccountFromAddress returns the interchain account for given owner address on a given connection pair
        pub async fn interchain_account_from_address(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryInterchainAccountFromAddressRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QueryInterchainAccountFromAddressResponse>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/secret.intertx.v1beta1.Query/InterchainAccountFromAddress",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "secret.intertx.v1beta1.Query",
                "InterchainAccountFromAddress",
            ));
            self.inner.unary(req, path, codec).await
        }
    }
}
/// MsgRegisterAccount registers an interchain account for the given owner over the specified connection pair
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgRegisterAccount {
    #[prost(string, tag = "1")]
    pub owner: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub connection_id: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub version: ::prost::alloc::string::String,
}
impl ::prost::Name for MsgRegisterAccount {
    const NAME: &'static str = "MsgRegisterAccount";
    const PACKAGE: &'static str = "secret.intertx.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        "secret.intertx.v1beta1.MsgRegisterAccount".into()
    }
    fn type_url() -> ::prost::alloc::string::String {
        "/secret.intertx.v1beta1.MsgRegisterAccount".into()
    }
}
/// MsgRegisterAccountResponse is the response type for Msg/RegisterAccount
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct MsgRegisterAccountResponse {}
impl ::prost::Name for MsgRegisterAccountResponse {
    const NAME: &'static str = "MsgRegisterAccountResponse";
    const PACKAGE: &'static str = "secret.intertx.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        "secret.intertx.v1beta1.MsgRegisterAccountResponse".into()
    }
    fn type_url() -> ::prost::alloc::string::String {
        "/secret.intertx.v1beta1.MsgRegisterAccountResponse".into()
    }
}
/// MsgSubmitTx creates and submits an arbitrary transaction msg to be executed using an interchain account
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgSubmitTx {
    #[prost(bytes = "vec", tag = "1")]
    pub owner: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag = "2")]
    pub connection_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "3")]
    pub msg: ::core::option::Option<::prost_types::Any>,
}
impl ::prost::Name for MsgSubmitTx {
    const NAME: &'static str = "MsgSubmitTx";
    const PACKAGE: &'static str = "secret.intertx.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        "secret.intertx.v1beta1.MsgSubmitTx".into()
    }
    fn type_url() -> ::prost::alloc::string::String {
        "/secret.intertx.v1beta1.MsgSubmitTx".into()
    }
}
/// MsgSubmitTxResponse defines the MsgSubmitTx response type
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct MsgSubmitTxResponse {}
impl ::prost::Name for MsgSubmitTxResponse {
    const NAME: &'static str = "MsgSubmitTxResponse";
    const PACKAGE: &'static str = "secret.intertx.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        "secret.intertx.v1beta1.MsgSubmitTxResponse".into()
    }
    fn type_url() -> ::prost::alloc::string::String {
        "/secret.intertx.v1beta1.MsgSubmitTxResponse".into()
    }
}
/// Generated client implementations.
#[cfg(feature = "grpc")]
pub mod msg_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::http::Uri;
    use tonic::codegen::*;
    /// Msg defines the ica-authentication Msg service.
    #[derive(Debug, Clone)]
    pub struct MsgClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    #[cfg(feature = "grpc-transport")]
    impl MsgClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> MsgClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + std::marker::Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + std::marker::Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_origin(inner: T, origin: Uri) -> Self {
            let inner = tonic::client::Grpc::with_origin(inner, origin);
            Self { inner }
        }
        pub fn with_interceptor<F>(inner: T, interceptor: F) -> MsgClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<http::Request<tonic::body::BoxBody>>>::Error:
                Into<StdError> + std::marker::Send + std::marker::Sync,
        {
            MsgClient::new(InterceptedService::new(inner, interceptor))
        }
        /// Compress requests with the given encoding.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.send_compressed(encoding);
            self
        }
        /// Enable decompressing responses.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.accept_compressed(encoding);
            self
        }
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_decoding_message_size(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_encoding_message_size(limit);
            self
        }
        /// Register defines a rpc handler for MsgRegisterAccount
        pub async fn register_account(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgRegisterAccount>,
        ) -> std::result::Result<tonic::Response<super::MsgRegisterAccountResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/secret.intertx.v1beta1.Msg/RegisterAccount");
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "secret.intertx.v1beta1.Msg",
                "RegisterAccount",
            ));
            self.inner.unary(req, path, codec).await
        }
        pub async fn submit_tx(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgSubmitTx>,
        ) -> std::result::Result<tonic::Response<super::MsgSubmitTxResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/secret.intertx.v1beta1.Msg/SubmitTx");
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("secret.intertx.v1beta1.Msg", "SubmitTx"));
            self.inner.unary(req, path, codec).await
        }
    }
}
