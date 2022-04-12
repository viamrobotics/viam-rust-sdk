/// Matrix
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Matrix {
    #[prost(uint32, tag = "1")]
    pub rows: u32,
    #[prost(uint32, tag = "2")]
    pub cols: u32,
    #[prost(uint32, repeated, tag = "3")]
    pub data: ::prost::alloc::vec::Vec<u32>,
}
/// ForceMatrix
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReadMatrixRequest {
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReadMatrixResponse {
    #[prost(message, optional, tag = "1")]
    pub matrix: ::core::option::Option<Matrix>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DetectSlipRequest {
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DetectSlipResponse {
    #[prost(bool, tag = "1")]
    pub slip_detected: bool,
}
#[doc = r" Generated client implementations."]
pub mod force_matrix_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = " ForceMatrixService services all ForceMatrices associated with a robot"]
    #[derive(Debug, Clone)]
    pub struct ForceMatrixServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl ForceMatrixServiceClient<tonic::transport::Channel> {
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
    impl<T> ForceMatrixServiceClient<T>
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
        ) -> ForceMatrixServiceClient<InterceptedService<T, F>>
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
            ForceMatrixServiceClient::new(InterceptedService::new(inner, interceptor))
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
        #[doc = " ReadMatrix returns the matrix of force readings from the force matrix sensor"]
        pub async fn read_matrix(
            &mut self,
            request: impl tonic::IntoRequest<super::ReadMatrixRequest>,
        ) -> Result<tonic::Response<super::ReadMatrixResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/proto.api.component.forcematrix.v1.ForceMatrixService/ReadMatrix",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " DetectSlip returns whether or not slip is occurring"]
        pub async fn detect_slip(
            &mut self,
            request: impl tonic::IntoRequest<super::DetectSlipRequest>,
        ) -> Result<tonic::Response<super::DetectSlipResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/proto.api.component.forcematrix.v1.ForceMatrixService/DetectSlip",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
#[doc = r" Generated server implementations."]
pub mod force_matrix_service_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = "Generated trait containing gRPC methods that should be implemented for use with ForceMatrixServiceServer."]
    #[async_trait]
    pub trait ForceMatrixService: Send + Sync + 'static {
        #[doc = " ReadMatrix returns the matrix of force readings from the force matrix sensor"]
        async fn read_matrix(
            &self,
            request: tonic::Request<super::ReadMatrixRequest>,
        ) -> Result<tonic::Response<super::ReadMatrixResponse>, tonic::Status>;
        #[doc = " DetectSlip returns whether or not slip is occurring"]
        async fn detect_slip(
            &self,
            request: tonic::Request<super::DetectSlipRequest>,
        ) -> Result<tonic::Response<super::DetectSlipResponse>, tonic::Status>;
    }
    #[doc = " ForceMatrixService services all ForceMatrices associated with a robot"]
    #[derive(Debug)]
    pub struct ForceMatrixServiceServer<T: ForceMatrixService> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: ForceMatrixService> ForceMatrixServiceServer<T> {
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
    impl<T, B> tonic::codegen::Service<http::Request<B>> for ForceMatrixServiceServer<T>
    where
        T: ForceMatrixService,
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
                "/proto.api.component.forcematrix.v1.ForceMatrixService/ReadMatrix" => {
                    #[allow(non_camel_case_types)]
                    struct ReadMatrixSvc<T: ForceMatrixService>(pub Arc<T>);
                    impl<T: ForceMatrixService>
                        tonic::server::UnaryService<super::ReadMatrixRequest> for ReadMatrixSvc<T>
                    {
                        type Response = super::ReadMatrixResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ReadMatrixRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).read_matrix(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ReadMatrixSvc(inner);
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
                "/proto.api.component.forcematrix.v1.ForceMatrixService/DetectSlip" => {
                    #[allow(non_camel_case_types)]
                    struct DetectSlipSvc<T: ForceMatrixService>(pub Arc<T>);
                    impl<T: ForceMatrixService>
                        tonic::server::UnaryService<super::DetectSlipRequest> for DetectSlipSvc<T>
                    {
                        type Response = super::DetectSlipResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::DetectSlipRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).detect_slip(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = DetectSlipSvc(inner);
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
    impl<T: ForceMatrixService> Clone for ForceMatrixServiceServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
            }
        }
    }
    impl<T: ForceMatrixService> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: ForceMatrixService> tonic::transport::NamedService for ForceMatrixServiceServer<T> {
        const NAME: &'static str = "proto.api.component.forcematrix.v1.ForceMatrixService";
    }
}
