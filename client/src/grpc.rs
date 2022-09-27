/// / Fields which identify an item
#[derive(Eq, Copy)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Signature {
    /// Organization ID
    #[prost(uint32, tag="1")]
    pub id_org: u32,
    /// Dictionary ID
    #[prost(uint32, tag="2")]
    pub id_dict: u32,
    /// Item ID
    #[prost(uint32, tag="3")]
    pub id_item: u32,
}
/// / Request to update entries
#[derive(Eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateRequest {
    /// If true, check if update would be successful without applying changes
    #[prost(bool, tag="1")]
    pub dry_run: bool,
    /// Which item to update
    #[prost(message, optional, tag="2")]
    pub sig: ::core::option::Option<Signature>,
    /// User ID
    #[prost(uint32, tag="3")]
    pub id_user: u32,
    /// Fields to update
    #[prost(map="string, string", tag="4")]
    pub updates: ::std::collections::HashMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
}
/// / Result of update request
#[derive(Eq, Copy)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateResponse {
    /// Success of operation
    #[prost(bool, tag="1")]
    pub success: bool,
    /// If field was changed
    #[prost(bool, tag="2")]
    pub changed: bool,
}
/// / Get items
#[derive(Eq, Copy)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetRequest {
    /// Item to get
    #[prost(message, optional, tag="1")]
    pub sig: ::core::option::Option<Signature>,
    /// User ID
    #[prost(uint32, tag="2")]
    pub id_user: u32,
}
/// / Result of Get Attempt
#[derive(Eq, Copy)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetResponse {
    /// Success of response
    #[prost(bool, tag="1")]
    pub success: bool,
}
/// Generated client implementations.
pub mod storage_grpc_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    #[derive(Debug, Clone)]
    pub struct StorageGrpcClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl StorageGrpcClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: std::convert::TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> StorageGrpcClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
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
        ) -> StorageGrpcClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
            >>::Error: Into<StdError> + Send + Sync,
        {
            StorageGrpcClient::new(InterceptedService::new(inner, interceptor))
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
        pub async fn update_items(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateRequest>,
        ) -> Result<tonic::Response<super::UpdateResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/grpc.StorageGrpc/updateItems",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn get_items(
            &mut self,
            request: impl tonic::IntoRequest<super::GetRequest>,
        ) -> Result<tonic::Response<super::GetResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/grpc.StorageGrpc/getItems",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
