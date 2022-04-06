#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Status {
    #[prost(message, optional, tag = "1")]
    pub name: ::core::option::Option<super::super::super::common::v1::ResourceName>,
    #[prost(message, optional, tag = "2")]
    pub status: ::core::option::Option<::prost_types::Struct>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetStatusRequest {
    #[prost(message, repeated, tag = "1")]
    pub resource_names: ::prost::alloc::vec::Vec<super::super::super::common::v1::ResourceName>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetStatusResponse {
    #[prost(message, repeated, tag = "1")]
    pub status: ::prost::alloc::vec::Vec<Status>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StreamStatusRequest {
    #[prost(message, repeated, tag = "1")]
    pub resource_names: ::prost::alloc::vec::Vec<super::super::super::common::v1::ResourceName>,
    /// how often to send a new status.
    #[prost(message, optional, tag = "2")]
    pub every: ::core::option::Option<::prost_types::Duration>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StreamStatusResponse {
    #[prost(message, repeated, tag = "1")]
    pub status: ::prost::alloc::vec::Vec<Status>,
}
#[doc = r" Generated client implementations."]
pub mod status_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = " A StatusService services keeps track of all status associated with a robot"]
    #[derive(Debug, Clone)]
    pub struct StatusServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl StatusServiceClient<tonic::transport::Channel> {
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
    impl<T> StatusServiceClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::ResponseBody: Body + Send + 'static,
        T::Error: Into<StdError>,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> StatusServiceClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<http::Request<tonic::body::BoxBody>>>::Error:
                Into<StdError> + Send + Sync,
        {
            StatusServiceClient::new(InterceptedService::new(inner, interceptor))
        }
        #[doc = r" Compress requests with `gzip`."]
        #[doc = r""]
        #[doc = r" This requires the server to support it otherwise it might respond with an"]
        #[doc = r" error."]
        pub fn send_gzip(mut self) -> Self {
            self.inner = self.inner.send_gzip();
            self
        }
        #[doc = r" Enable decompressing responses with `gzip`."]
        pub fn accept_gzip(mut self) -> Self {
            self.inner = self.inner.accept_gzip();
            self
        }
        #[doc = " GetStatus returns the list of all statuses requested. An empty request signifies all resources."]
        pub async fn get_status(
            &mut self,
            request: impl tonic::IntoRequest<super::GetStatusRequest>,
        ) -> Result<tonic::Response<super::GetStatusResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/proto.api.service.status.v1.StatusService/GetStatus",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " StreamStatus periodically sends the status of all statuses requested. An empty request signifies all resources."]
        pub async fn stream_status(
            &mut self,
            request: impl tonic::IntoRequest<super::StreamStatusRequest>,
        ) -> Result<
            tonic::Response<tonic::codec::Streaming<super::StreamStatusResponse>>,
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
                "/proto.api.service.status.v1.StatusService/StreamStatus",
            );
            self.inner
                .server_streaming(request.into_request(), path, codec)
                .await
        }
    }
}
#[doc = r" Generated server implementations."]
pub mod status_service_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = "Generated trait containing gRPC methods that should be implemented for use with StatusServiceServer."]
    #[async_trait]
    pub trait StatusService: Send + Sync + 'static {
        #[doc = " GetStatus returns the list of all statuses requested. An empty request signifies all resources."]
        async fn get_status(
            &self,
            request: tonic::Request<super::GetStatusRequest>,
        ) -> Result<tonic::Response<super::GetStatusResponse>, tonic::Status>;
        #[doc = "Server streaming response type for the StreamStatus method."]
        type StreamStatusStream: futures_core::Stream<Item = Result<super::StreamStatusResponse, tonic::Status>>
            + Send
            + 'static;
        #[doc = " StreamStatus periodically sends the status of all statuses requested. An empty request signifies all resources."]
        async fn stream_status(
            &self,
            request: tonic::Request<super::StreamStatusRequest>,
        ) -> Result<tonic::Response<Self::StreamStatusStream>, tonic::Status>;
    }
    #[doc = " A StatusService services keeps track of all status associated with a robot"]
    #[derive(Debug)]
    pub struct StatusServiceServer<T: StatusService> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: StatusService> StatusServiceServer<T> {
        pub fn new(inner: T) -> Self {
            let inner = Arc::new(inner);
            let inner = _Inner(inner);
            Self {
                inner,
                accept_compression_encodings: Default::default(),
                send_compression_encodings: Default::default(),
            }
        }
        pub fn with_interceptor<F>(inner: T, interceptor: F) -> InterceptedService<Self, F>
        where
            F: tonic::service::Interceptor,
        {
            InterceptedService::new(Self::new(inner), interceptor)
        }
        #[doc = r" Enable decompressing requests with `gzip`."]
        pub fn accept_gzip(mut self) -> Self {
            self.accept_compression_encodings.enable_gzip();
            self
        }
        #[doc = r" Compress responses with `gzip`, if the client supports it."]
        pub fn send_gzip(mut self) -> Self {
            self.send_compression_encodings.enable_gzip();
            self
        }
    }
    impl<T, B> tonic::codegen::Service<http::Request<B>> for StatusServiceServer<T>
    where
        T: StatusService,
        B: Body + Send + 'static,
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
                "/proto.api.service.status.v1.StatusService/GetStatus" => {
                    #[allow(non_camel_case_types)]
                    struct GetStatusSvc<T: StatusService>(pub Arc<T>);
                    impl<T: StatusService> tonic::server::UnaryService<super::GetStatusRequest> for GetStatusSvc<T> {
                        type Response = super::GetStatusResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetStatusRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).get_status(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetStatusSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/proto.api.service.status.v1.StatusService/StreamStatus" => {
                    #[allow(non_camel_case_types)]
                    struct StreamStatusSvc<T: StatusService>(pub Arc<T>);
                    impl<T: StatusService>
                        tonic::server::ServerStreamingService<super::StreamStatusRequest>
                        for StreamStatusSvc<T>
                    {
                        type Response = super::StreamStatusResponse;
                        type ResponseStream = T::StreamStatusStream;
                        type Future =
                            BoxFuture<tonic::Response<Self::ResponseStream>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::StreamStatusRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).stream_status(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = StreamStatusSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.server_streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => Box::pin(async move {
                    Ok(http::Response::builder()
                        .status(200)
                        .header("grpc-status", "12")
                        .header("content-type", "application/grpc")
                        .body(empty_body())
                        .unwrap())
                }),
            }
        }
    }
    impl<T: StatusService> Clone for StatusServiceServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
            }
        }
    }
    impl<T: StatusService> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: StatusService> tonic::transport::NamedService for StatusServiceServer<T> {
        const NAME: &'static str = "proto.api.service.status.v1.StatusService";
    }
}
