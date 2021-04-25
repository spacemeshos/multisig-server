#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UserMessage {
    /// timestamp, seconds sinc epoch
    #[prost(uint64, tag = "1")]
    pub created: u64,
    /// vault address, and in future smart contract address or a multi-sig accounts db address.
    #[prost(bytes = "vec", tag = "2")]
    pub address: ::prost::alloc::vec::Vec<u8>,
    /// one of the supported types so receiver knows how to deserlize the binary data
    #[prost(enumeration = "TransactionType", tag = "3")]
    pub transaction_type: i32,
    /// binary protobuf signed transaction data
    #[prost(bytes = "vec", tag = "4")]
    pub transaction_data: ::prost::alloc::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StoreMessageRequest {
    #[prost(message, optional, tag = "1")]
    pub user_message: ::core::option::Option<UserMessage>,
}
/// empty response with 0 status code means success. Non-zero grpc status code indicates an error
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StoreMessageResponse {}
/// a request to get all message for an address (e.g. vault contract app instance or multichain account)
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetMessagesRequest {
    #[prost(bytes = "vec", tag = "1")]
    pub address: ::prost::alloc::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetMessagesResponse {
    /// returns all stored user messages for the source address (including user's own messages)
    #[prost(message, repeated, tag = "2")]
    pub user_messages: ::prost::alloc::vec::Vec<UserMessage>,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum TransactionType {
    /// Withdraw request from a vault contract
    VaultWithdraw = 0,
    /// Change a vault contract daily spending account
    VaultChangeDailySpendAccount = 1,
    /// Change a vault contract daily spending amount
    VaultChangeDailySpendAmount = 2,
    /// a request to spend from a multi-sig account (no smart contract)
    CoinSpend = 3,
}
#[doc = r" Generated client implementations."]
pub mod multi_sig_service_client {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    pub struct MultiSigServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl MultiSigServiceClient<tonic::transport::Channel> {
        #[doc = r" Attempt to create a new client by connecting to a given endpoint."]
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: std::convert::TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> MultiSigServiceClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::ResponseBody: Body + HttpBody + Send + 'static,
        T::Error: Into<StdError>,
        <T::ResponseBody as HttpBody>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_interceptor(inner: T, interceptor: impl Into<tonic::Interceptor>) -> Self {
            let inner = tonic::client::Grpc::with_interceptor(inner, interceptor);
            Self { inner }
        }
        #[doc = " Store a multi-sig message in the service"]
        pub async fn store_message(
            &mut self,
            request: impl tonic::IntoRequest<super::StoreMessageRequest>,
        ) -> Result<tonic::Response<super::StoreMessageResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/api.MultiSigService/StoreMessage");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Get multi-sig message for a source address"]
        pub async fn get_messages(
            &mut self,
            request: impl tonic::IntoRequest<super::GetMessagesRequest>,
        ) -> Result<tonic::Response<super::GetMessagesResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/api.MultiSigService/GetMessages");
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
    impl<T: Clone> Clone for MultiSigServiceClient<T> {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.clone(),
            }
        }
    }
    impl<T> std::fmt::Debug for MultiSigServiceClient<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "MultiSigServiceClient {{ ... }}")
        }
    }
}
#[doc = r" Generated server implementations."]
pub mod multi_sig_service_server {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = "Generated trait containing gRPC methods that should be implemented for use with MultiSigServiceServer."]
    #[async_trait]
    pub trait MultiSigService: Send + Sync + 'static {
        #[doc = " Store a multi-sig message in the service"]
        async fn store_message(
            &self,
            request: tonic::Request<super::StoreMessageRequest>,
        ) -> Result<tonic::Response<super::StoreMessageResponse>, tonic::Status>;
        #[doc = " Get multi-sig message for a source address"]
        async fn get_messages(
            &self,
            request: tonic::Request<super::GetMessagesRequest>,
        ) -> Result<tonic::Response<super::GetMessagesResponse>, tonic::Status>;
    }
    #[derive(Debug)]
    pub struct MultiSigServiceServer<T: MultiSigService> {
        inner: _Inner<T>,
    }
    struct _Inner<T>(Arc<T>, Option<tonic::Interceptor>);
    impl<T: MultiSigService> MultiSigServiceServer<T> {
        pub fn new(inner: T) -> Self {
            let inner = Arc::new(inner);
            let inner = _Inner(inner, None);
            Self { inner }
        }
        pub fn with_interceptor(inner: T, interceptor: impl Into<tonic::Interceptor>) -> Self {
            let inner = Arc::new(inner);
            let inner = _Inner(inner, Some(interceptor.into()));
            Self { inner }
        }
    }
    impl<T, B> Service<http::Request<B>> for MultiSigServiceServer<T>
    where
        T: MultiSigService,
        B: HttpBody + Send + Sync + 'static,
        B::Error: Into<StdError> + Send + 'static,
    {
        type Response = http::Response<tonic::body::BoxBody>;
        type Error = Never;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            let inner = self.inner.clone();
            match req.uri().path() {
                "/api.MultiSigService/StoreMessage" => {
                    #[allow(non_camel_case_types)]
                    struct StoreMessageSvc<T: MultiSigService>(pub Arc<T>);
                    impl<T: MultiSigService> tonic::server::UnaryService<super::StoreMessageRequest>
                        for StoreMessageSvc<T>
                    {
                        type Response = super::StoreMessageResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::StoreMessageRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).store_message(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = StoreMessageSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/api.MultiSigService/GetMessages" => {
                    #[allow(non_camel_case_types)]
                    struct GetMessagesSvc<T: MultiSigService>(pub Arc<T>);
                    impl<T: MultiSigService> tonic::server::UnaryService<super::GetMessagesRequest>
                        for GetMessagesSvc<T>
                    {
                        type Response = super::GetMessagesResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetMessagesRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).get_messages(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = GetMessagesSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => Box::pin(async move {
                    Ok(http::Response::builder()
                        .status(200)
                        .header("grpc-status", "12")
                        .header("content-type", "application/grpc")
                        .body(tonic::body::BoxBody::empty())
                        .unwrap())
                }),
            }
        }
    }
    impl<T: MultiSigService> Clone for MultiSigServiceServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self { inner }
        }
    }
    impl<T: MultiSigService> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone(), self.1.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: MultiSigService> tonic::transport::NamedService for MultiSigServiceServer<T> {
        const NAME: &'static str = "api.MultiSigService";
    }
}
