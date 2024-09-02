// This file is @generated by prost-build.
/// Params defines the parameters for the ibc-rate-limit module.
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Params {
    #[prost(string, tag = "1")]
    pub switch_status: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub pauser_address: ::prost::alloc::string::String,
}
impl ::prost::Name for Params {
    const NAME: &'static str = "Params";
    const PACKAGE: &'static str = "secret.emergencybutton.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        "secret.emergencybutton.v1beta1.Params".into()
    }
    fn type_url() -> ::prost::alloc::string::String {
        "/secret.emergencybutton.v1beta1.Params".into()
    }
}
/// ParamsRequest is the request type for the Query/Params RPC method.
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct ParamsRequest {}
impl ::prost::Name for ParamsRequest {
    const NAME: &'static str = "ParamsRequest";
    const PACKAGE: &'static str = "secret.emergencybutton.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        "secret.emergencybutton.v1beta1.ParamsRequest".into()
    }
    fn type_url() -> ::prost::alloc::string::String {
        "/secret.emergencybutton.v1beta1.ParamsRequest".into()
    }
}
/// ParamsResponse is the response type for the Query/Params RPC method.
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ParamsResponse {
    /// params defines the parameters of the module.
    #[prost(message, optional, tag = "1")]
    pub params: ::core::option::Option<Params>,
}
impl ::prost::Name for ParamsResponse {
    const NAME: &'static str = "ParamsResponse";
    const PACKAGE: &'static str = "secret.emergencybutton.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        "secret.emergencybutton.v1beta1.ParamsResponse".into()
    }
    fn type_url() -> ::prost::alloc::string::String {
        "/secret.emergencybutton.v1beta1.ParamsResponse".into()
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
        /// Params defines a gRPC query method that returns the emergencybutton
        /// module's parameters.
        pub async fn params(
            &mut self,
            request: impl tonic::IntoRequest<super::ParamsRequest>,
        ) -> std::result::Result<tonic::Response<super::ParamsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/secret.emergencybutton.v1beta1.Query/Params",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "secret.emergencybutton.v1beta1.Query",
                "Params",
            ));
            self.inner.unary(req, path, codec).await
        }
    }
}
/// GenesisState - genesis state of x/wasm
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenesisState {
    #[prost(message, optional, tag = "1")]
    pub params: ::core::option::Option<Params>,
}
impl ::prost::Name for GenesisState {
    const NAME: &'static str = "GenesisState";
    const PACKAGE: &'static str = "secret.emergencybutton.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        "secret.emergencybutton.v1beta1.GenesisState".into()
    }
    fn type_url() -> ::prost::alloc::string::String {
        "/secret.emergencybutton.v1beta1.GenesisState".into()
    }
}
/// MsgToggleIbcSwitch represents a message to toggle the emergencybutton status
/// by the defined pauser.
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgToggleIbcSwitch {
    #[prost(string, tag = "1")]
    pub sender: ::prost::alloc::string::String,
}
impl ::prost::Name for MsgToggleIbcSwitch {
    const NAME: &'static str = "MsgToggleIbcSwitch";
    const PACKAGE: &'static str = "secret.emergencybutton.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        "secret.emergencybutton.v1beta1.MsgToggleIbcSwitch".into()
    }
    fn type_url() -> ::prost::alloc::string::String {
        "/secret.emergencybutton.v1beta1.MsgToggleIbcSwitch".into()
    }
}
/// MsgToggleIbcSwitchResponse defines the response type for the toggle.
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct MsgToggleIbcSwitchResponse {}
impl ::prost::Name for MsgToggleIbcSwitchResponse {
    const NAME: &'static str = "MsgToggleIbcSwitchResponse";
    const PACKAGE: &'static str = "secret.emergencybutton.v1beta1";
    fn full_name() -> ::prost::alloc::string::String {
        "secret.emergencybutton.v1beta1.MsgToggleIbcSwitchResponse".into()
    }
    fn type_url() -> ::prost::alloc::string::String {
        "/secret.emergencybutton.v1beta1.MsgToggleIbcSwitchResponse".into()
    }
}
/// Generated client implementations.
#[cfg(feature = "grpc")]
pub mod msg_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::http::Uri;
    use tonic::codegen::*;
    /// Msg defines the bank Msg service.
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
        /// ToggleIbcSwitch defines a method for toggling the status of the
        /// emergencybutton.
        pub async fn toggle_ibc_switch(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgToggleIbcSwitch>,
        ) -> std::result::Result<tonic::Response<super::MsgToggleIbcSwitchResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/secret.emergencybutton.v1beta1.Msg/ToggleIbcSwitch",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "secret.emergencybutton.v1beta1.Msg",
                "ToggleIbcSwitch",
            ));
            self.inner.unary(req, path, codec).await
        }
    }
}
