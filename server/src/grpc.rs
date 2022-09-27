/// / Fields which identify an item
#[derive(Copy)]
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
#[derive(Copy)]
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
#[derive(Copy)]
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
#[derive(Copy)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetResponse {
    /// Success of response
    #[prost(bool, tag="1")]
    pub success: bool,
}
/// Generated server implementations.
pub mod storage_grpc_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    ///Generated trait containing gRPC methods that should be implemented for use with StorageGrpcServer.
    #[async_trait]
    pub trait StorageGrpc: Send + Sync + 'static {
        async fn update_items(
            &self,
            request: tonic::Request<super::UpdateRequest>,
        ) -> Result<tonic::Response<super::UpdateResponse>, tonic::Status>;
        async fn get_items(
            &self,
            request: tonic::Request<super::GetRequest>,
        ) -> Result<tonic::Response<super::GetResponse>, tonic::Status>;
    }
    #[derive(Debug)]
    pub struct StorageGrpcServer<T: StorageGrpc> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: StorageGrpc> StorageGrpcServer<T> {
        pub fn new(inner: T) -> Self {
            Self::from_arc(Arc::new(inner))
        }
        pub fn from_arc(inner: Arc<T>) -> Self {
            let inner = _Inner(inner);
            Self {
                inner,
                accept_compression_encodings: Default::default(),
                send_compression_encodings: Default::default(),
            }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> InterceptedService<Self, F>
        where
            F: tonic::service::Interceptor,
        {
            InterceptedService::new(Self::new(inner), interceptor)
        }
        /// Enable decompressing requests with the given encoding.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.accept_compression_encodings.enable(encoding);
            self
        }
        /// Compress responses with the given encoding, if the client supports it.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.send_compression_encodings.enable(encoding);
            self
        }
    }
    impl<T, B> tonic::codegen::Service<http::Request<B>> for StorageGrpcServer<T>
    where
        T: StorageGrpc,
        B: Body + Send + 'static,
        B::Error: Into<StdError> + Send + 'static,
    {
        type Response = http::Response<tonic::body::BoxBody>;
        type Error = std::convert::Infallible;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(
            &mut self,
            _cx: &mut Context<'_>,
        ) -> Poll<Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            let inner = self.inner.clone();
            match req.uri().path() {
                "/grpc.StorageGrpc/updateItems" => {
                    #[allow(non_camel_case_types)]
                    struct updateItemsSvc<T: StorageGrpc>(pub Arc<T>);
                    impl<
                        T: StorageGrpc,
                    > tonic::server::UnaryService<super::UpdateRequest>
                    for updateItemsSvc<T> {
                        type Response = super::UpdateResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::UpdateRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).update_items(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = updateItemsSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/grpc.StorageGrpc/getItems" => {
                    #[allow(non_camel_case_types)]
                    struct getItemsSvc<T: StorageGrpc>(pub Arc<T>);
                    impl<T: StorageGrpc> tonic::server::UnaryService<super::GetRequest>
                    for getItemsSvc<T> {
                        type Response = super::GetResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).get_items(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = getItemsSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => {
                    Box::pin(async move {
                        Ok(
                            http::Response::builder()
                                .status(200)
                                .header("grpc-status", "12")
                                .header("content-type", "application/grpc")
                                .body(empty_body())
                                .unwrap(),
                        )
                    })
                }
            }
        }
    }
    impl<T: StorageGrpc> Clone for StorageGrpcServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
            }
        }
    }
    impl<T: StorageGrpc> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: StorageGrpc> tonic::server::NamedService for StorageGrpcServer<T> {
        const NAME: &'static str = "grpc.StorageGrpc";
    }
}
