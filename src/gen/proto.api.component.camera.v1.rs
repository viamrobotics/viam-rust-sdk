#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetFrameRequest {
    /// Name of a camera
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Requested MIME type of response
    #[prost(string, tag = "2")]
    pub mime_type: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetFrameResponse {
    /// Actual MIME type of response
    #[prost(string, tag = "1")]
    pub mime_type: ::prost::alloc::string::String,
    /// Frame in bytes
    #[prost(bytes = "vec", tag = "2")]
    pub image: ::prost::alloc::vec::Vec<u8>,
    /// Width of frame in px
    #[prost(int64, tag = "3")]
    pub width_px: i64,
    /// Height of frame in px
    #[prost(int64, tag = "4")]
    pub height_px: i64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RenderFrameRequest {
    /// Name of a camera
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Requested MIME type of response
    #[prost(string, tag = "2")]
    pub mime_type: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetPointCloudRequest {
    /// Name of a camera
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Requested MIME type of response
    #[prost(string, tag = "2")]
    pub mime_type: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetPointCloudResponse {
    /// Actual MIME type of response
    #[prost(string, tag = "1")]
    pub mime_type: ::prost::alloc::string::String,
    /// Frame in bytes
    #[prost(bytes = "vec", tag = "2")]
    pub point_cloud: ::prost::alloc::vec::Vec<u8>,
}
#[doc = r" Generated client implementations."]
pub mod camera_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = " A CameraService services all cameras associated with a robot"]
    #[derive(Debug, Clone)]
    pub struct CameraServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl CameraServiceClient<tonic::transport::Channel> {
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
    impl<T> CameraServiceClient<T>
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
        ) -> CameraServiceClient<InterceptedService<T, F>>
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
            CameraServiceClient::new(InterceptedService::new(inner, interceptor))
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
        #[doc = " GetFrame returns a frame from a camera of the underlying robot. A specific MIME type"]
        #[doc = " can be requested but may not necessarily be the same one returned."]
        pub async fn get_frame(
            &mut self,
            request: impl tonic::IntoRequest<super::GetFrameRequest>,
        ) -> Result<tonic::Response<super::GetFrameResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/proto.api.component.camera.v1.CameraService/GetFrame",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " RenderFrame renders a frame from a camera of the underlying robot to an HTTP response. A specific MIME type"]
        #[doc = " can be requested but may not necessarily be the same one returned."]
        pub async fn render_frame(
            &mut self,
            request: impl tonic::IntoRequest<super::RenderFrameRequest>,
        ) -> Result<
            tonic::Response<super::super::super::super::super::super::google::api::HttpBody>,
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
                "/proto.api.component.camera.v1.CameraService/RenderFrame",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " GetPointCloud returns a point cloud from a camera of the underlying robot. A specific MIME type"]
        #[doc = " can be requested but may not necessarily be the same one returned."]
        pub async fn get_point_cloud(
            &mut self,
            request: impl tonic::IntoRequest<super::GetPointCloudRequest>,
        ) -> Result<tonic::Response<super::GetPointCloudResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/proto.api.component.camera.v1.CameraService/GetPointCloud",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
#[doc = r" Generated server implementations."]
pub mod camera_service_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = "Generated trait containing gRPC methods that should be implemented for use with CameraServiceServer."]
    #[async_trait]
    pub trait CameraService: Send + Sync + 'static {
        #[doc = " GetFrame returns a frame from a camera of the underlying robot. A specific MIME type"]
        #[doc = " can be requested but may not necessarily be the same one returned."]
        async fn get_frame(
            &self,
            request: tonic::Request<super::GetFrameRequest>,
        ) -> Result<tonic::Response<super::GetFrameResponse>, tonic::Status>;
        #[doc = " RenderFrame renders a frame from a camera of the underlying robot to an HTTP response. A specific MIME type"]
        #[doc = " can be requested but may not necessarily be the same one returned."]
        async fn render_frame(
            &self,
            request: tonic::Request<super::RenderFrameRequest>,
        ) -> Result<
            tonic::Response<super::super::super::super::super::super::google::api::HttpBody>,
            tonic::Status,
        >;
        #[doc = " GetPointCloud returns a point cloud from a camera of the underlying robot. A specific MIME type"]
        #[doc = " can be requested but may not necessarily be the same one returned."]
        async fn get_point_cloud(
            &self,
            request: tonic::Request<super::GetPointCloudRequest>,
        ) -> Result<tonic::Response<super::GetPointCloudResponse>, tonic::Status>;
    }
    #[doc = " A CameraService services all cameras associated with a robot"]
    #[derive(Debug)]
    pub struct CameraServiceServer<T: CameraService> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: CameraService> CameraServiceServer<T> {
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
    impl<T, B> tonic::codegen::Service<http::Request<B>> for CameraServiceServer<T>
    where
        T: CameraService,
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
                "/proto.api.component.camera.v1.CameraService/GetFrame" => {
                    #[allow(non_camel_case_types)]
                    struct GetFrameSvc<T: CameraService>(pub Arc<T>);
                    impl<T: CameraService> tonic::server::UnaryService<super::GetFrameRequest> for GetFrameSvc<T> {
                        type Response = super::GetFrameResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetFrameRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).get_frame(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetFrameSvc(inner);
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
                "/proto.api.component.camera.v1.CameraService/RenderFrame" => {
                    #[allow(non_camel_case_types)]
                    struct RenderFrameSvc<T: CameraService>(pub Arc<T>);
                    impl<T: CameraService> tonic::server::UnaryService<super::RenderFrameRequest>
                        for RenderFrameSvc<T>
                    {
                        type Response =
                            super::super::super::super::super::super::google::api::HttpBody;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::RenderFrameRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).render_frame(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = RenderFrameSvc(inner);
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
                "/proto.api.component.camera.v1.CameraService/GetPointCloud" => {
                    #[allow(non_camel_case_types)]
                    struct GetPointCloudSvc<T: CameraService>(pub Arc<T>);
                    impl<T: CameraService> tonic::server::UnaryService<super::GetPointCloudRequest>
                        for GetPointCloudSvc<T>
                    {
                        type Response = super::GetPointCloudResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetPointCloudRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).get_point_cloud(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetPointCloudSvc(inner);
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
    impl<T: CameraService> Clone for CameraServiceServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
            }
        }
    }
    impl<T: CameraService> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: CameraService> tonic::transport::NamedService for CameraServiceServer<T> {
        const NAME: &'static str = "proto.api.component.camera.v1.CameraService";
    }
}
