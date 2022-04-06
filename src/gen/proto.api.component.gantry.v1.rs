#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetPositionRequest {
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetPositionResponse {
    #[prost(double, repeated, tag = "1")]
    pub positions_mm: ::prost::alloc::vec::Vec<f64>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MoveToPositionRequest {
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    #[prost(double, repeated, tag = "2")]
    pub positions_mm: ::prost::alloc::vec::Vec<f64>,
    #[prost(message, optional, tag = "3")]
    pub world_state: ::core::option::Option<super::super::super::common::v1::WorldState>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MoveToPositionResponse {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetLengthsRequest {
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetLengthsResponse {
    #[prost(double, repeated, tag = "1")]
    pub lengths_mm: ::prost::alloc::vec::Vec<f64>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Status {
    #[prost(double, repeated, tag = "1")]
    pub positions_mm: ::prost::alloc::vec::Vec<f64>,
    #[prost(double, repeated, tag = "2")]
    pub lengths_mm: ::prost::alloc::vec::Vec<f64>,
}
#[doc = r" Generated client implementations."]
pub mod gantry_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = " An GantryService services all gantries associated with a robot"]
    #[derive(Debug, Clone)]
    pub struct GantryServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl GantryServiceClient<tonic::transport::Channel> {
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
    impl<T> GantryServiceClient<T>
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
        ) -> GantryServiceClient<InterceptedService<T, F>>
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
            GantryServiceClient::new(InterceptedService::new(inner, interceptor))
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
        #[doc = " GetPosition gets the current position of a gantry of the underlying robot."]
        pub async fn get_position(
            &mut self,
            request: impl tonic::IntoRequest<super::GetPositionRequest>,
        ) -> Result<tonic::Response<super::GetPositionResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/proto.api.component.gantry.v1.GantryService/GetPosition",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " MoveToPosition moves a gantry of the underlying robot to the requested position."]
        pub async fn move_to_position(
            &mut self,
            request: impl tonic::IntoRequest<super::MoveToPositionRequest>,
        ) -> Result<tonic::Response<super::MoveToPositionResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/proto.api.component.gantry.v1.GantryService/MoveToPosition",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " GetLengths gets the lengths of a gantry of the underlying robot."]
        pub async fn get_lengths(
            &mut self,
            request: impl tonic::IntoRequest<super::GetLengthsRequest>,
        ) -> Result<tonic::Response<super::GetLengthsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/proto.api.component.gantry.v1.GantryService/GetLengths",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
#[doc = r" Generated server implementations."]
pub mod gantry_service_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = "Generated trait containing gRPC methods that should be implemented for use with GantryServiceServer."]
    #[async_trait]
    pub trait GantryService: Send + Sync + 'static {
        #[doc = " GetPosition gets the current position of a gantry of the underlying robot."]
        async fn get_position(
            &self,
            request: tonic::Request<super::GetPositionRequest>,
        ) -> Result<tonic::Response<super::GetPositionResponse>, tonic::Status>;
        #[doc = " MoveToPosition moves a gantry of the underlying robot to the requested position."]
        async fn move_to_position(
            &self,
            request: tonic::Request<super::MoveToPositionRequest>,
        ) -> Result<tonic::Response<super::MoveToPositionResponse>, tonic::Status>;
        #[doc = " GetLengths gets the lengths of a gantry of the underlying robot."]
        async fn get_lengths(
            &self,
            request: tonic::Request<super::GetLengthsRequest>,
        ) -> Result<tonic::Response<super::GetLengthsResponse>, tonic::Status>;
    }
    #[doc = " An GantryService services all gantries associated with a robot"]
    #[derive(Debug)]
    pub struct GantryServiceServer<T: GantryService> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: GantryService> GantryServiceServer<T> {
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
    impl<T, B> tonic::codegen::Service<http::Request<B>> for GantryServiceServer<T>
    where
        T: GantryService,
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
                "/proto.api.component.gantry.v1.GantryService/GetPosition" => {
                    #[allow(non_camel_case_types)]
                    struct GetPositionSvc<T: GantryService>(pub Arc<T>);
                    impl<T: GantryService> tonic::server::UnaryService<super::GetPositionRequest>
                        for GetPositionSvc<T>
                    {
                        type Response = super::GetPositionResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetPositionRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).get_position(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetPositionSvc(inner);
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
                "/proto.api.component.gantry.v1.GantryService/MoveToPosition" => {
                    #[allow(non_camel_case_types)]
                    struct MoveToPositionSvc<T: GantryService>(pub Arc<T>);
                    impl<T: GantryService> tonic::server::UnaryService<super::MoveToPositionRequest>
                        for MoveToPositionSvc<T>
                    {
                        type Response = super::MoveToPositionResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MoveToPositionRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).move_to_position(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = MoveToPositionSvc(inner);
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
                "/proto.api.component.gantry.v1.GantryService/GetLengths" => {
                    #[allow(non_camel_case_types)]
                    struct GetLengthsSvc<T: GantryService>(pub Arc<T>);
                    impl<T: GantryService> tonic::server::UnaryService<super::GetLengthsRequest> for GetLengthsSvc<T> {
                        type Response = super::GetLengthsResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetLengthsRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).get_lengths(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetLengthsSvc(inner);
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
    impl<T: GantryService> Clone for GantryServiceServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
            }
        }
    }
    impl<T: GantryService> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: GantryService> tonic::transport::NamedService for GantryServiceServer<T> {
        const NAME: &'static str = "proto.api.component.gantry.v1.GantryService";
    }
}
