// @generated
/// Generated client implementations.
pub mod vision_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[derive(Debug, Clone)]
    pub struct VisionServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl VisionServiceClient<tonic::transport::Channel> {
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
    impl<T> VisionServiceClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::Error: Into<StdError>,
        T::ResponseBody: Default + Body<Data = Bytes> + Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> VisionServiceClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
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
            VisionServiceClient::new(InterceptedService::new(inner, interceptor))
        }
        /// Compress requests with `gzip`.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_gzip(mut self) -> Self {
            self.inner = self.inner.send_gzip();
            self
        }
        /// Enable decompressing responses with `gzip`.
        #[must_use]
        pub fn accept_gzip(mut self) -> Self {
            self.inner = self.inner.accept_gzip();
            self
        }
        pub async fn get_detections_from_camera(
            &mut self,
            request: impl tonic::IntoRequest<super::GetDetectionsFromCameraRequest>,
        ) -> Result<
                tonic::Response<super::GetDetectionsFromCameraResponse>,
                tonic::Status,
            > {
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
                "/viam.service.vision.v1.VisionService/GetDetectionsFromCamera",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn get_detections(
            &mut self,
            request: impl tonic::IntoRequest<super::GetDetectionsRequest>,
        ) -> Result<tonic::Response<super::GetDetectionsResponse>, tonic::Status> {
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
                "/viam.service.vision.v1.VisionService/GetDetections",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn get_classifications_from_camera(
            &mut self,
            request: impl tonic::IntoRequest<super::GetClassificationsFromCameraRequest>,
        ) -> Result<
                tonic::Response<super::GetClassificationsFromCameraResponse>,
                tonic::Status,
            > {
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
                "/viam.service.vision.v1.VisionService/GetClassificationsFromCamera",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn get_classifications(
            &mut self,
            request: impl tonic::IntoRequest<super::GetClassificationsRequest>,
        ) -> Result<tonic::Response<super::GetClassificationsResponse>, tonic::Status> {
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
                "/viam.service.vision.v1.VisionService/GetClassifications",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn get_object_point_clouds(
            &mut self,
            request: impl tonic::IntoRequest<super::GetObjectPointCloudsRequest>,
        ) -> Result<
                tonic::Response<super::GetObjectPointCloudsResponse>,
                tonic::Status,
            > {
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
                "/viam.service.vision.v1.VisionService/GetObjectPointClouds",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn do_command(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::super::super::common::v1::DoCommandRequest,
            >,
        ) -> Result<
                tonic::Response<
                    super::super::super::super::common::v1::DoCommandResponse,
                >,
                tonic::Status,
            > {
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
                "/viam.service.vision.v1.VisionService/DoCommand",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod vision_service_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    ///Generated trait containing gRPC methods that should be implemented for use with VisionServiceServer.
    #[async_trait]
    pub trait VisionService: Send + Sync + 'static {
        async fn get_detections_from_camera(
            &self,
            request: tonic::Request<super::GetDetectionsFromCameraRequest>,
        ) -> Result<
                tonic::Response<super::GetDetectionsFromCameraResponse>,
                tonic::Status,
            >;
        async fn get_detections(
            &self,
            request: tonic::Request<super::GetDetectionsRequest>,
        ) -> Result<tonic::Response<super::GetDetectionsResponse>, tonic::Status>;
        async fn get_classifications_from_camera(
            &self,
            request: tonic::Request<super::GetClassificationsFromCameraRequest>,
        ) -> Result<
                tonic::Response<super::GetClassificationsFromCameraResponse>,
                tonic::Status,
            >;
        async fn get_classifications(
            &self,
            request: tonic::Request<super::GetClassificationsRequest>,
        ) -> Result<tonic::Response<super::GetClassificationsResponse>, tonic::Status>;
        async fn get_object_point_clouds(
            &self,
            request: tonic::Request<super::GetObjectPointCloudsRequest>,
        ) -> Result<tonic::Response<super::GetObjectPointCloudsResponse>, tonic::Status>;
        async fn do_command(
            &self,
            request: tonic::Request<
                super::super::super::super::common::v1::DoCommandRequest,
            >,
        ) -> Result<
                tonic::Response<
                    super::super::super::super::common::v1::DoCommandResponse,
                >,
                tonic::Status,
            >;
    }
    #[derive(Debug)]
    pub struct VisionServiceServer<T: VisionService> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: VisionService> VisionServiceServer<T> {
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
        /// Enable decompressing requests with `gzip`.
        #[must_use]
        pub fn accept_gzip(mut self) -> Self {
            self.accept_compression_encodings.enable_gzip();
            self
        }
        /// Compress responses with `gzip`, if the client supports it.
        #[must_use]
        pub fn send_gzip(mut self) -> Self {
            self.send_compression_encodings.enable_gzip();
            self
        }
    }
    impl<T, B> tonic::codegen::Service<http::Request<B>> for VisionServiceServer<T>
    where
        T: VisionService,
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
                "/viam.service.vision.v1.VisionService/GetDetectionsFromCamera" => {
                    #[allow(non_camel_case_types)]
                    struct GetDetectionsFromCameraSvc<T: VisionService>(pub Arc<T>);
                    impl<
                        T: VisionService,
                    > tonic::server::UnaryService<super::GetDetectionsFromCameraRequest>
                    for GetDetectionsFromCameraSvc<T> {
                        type Response = super::GetDetectionsFromCameraResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::GetDetectionsFromCameraRequest,
                            >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).get_detections_from_camera(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetDetectionsFromCameraSvc(inner);
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
                "/viam.service.vision.v1.VisionService/GetDetections" => {
                    #[allow(non_camel_case_types)]
                    struct GetDetectionsSvc<T: VisionService>(pub Arc<T>);
                    impl<
                        T: VisionService,
                    > tonic::server::UnaryService<super::GetDetectionsRequest>
                    for GetDetectionsSvc<T> {
                        type Response = super::GetDetectionsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetDetectionsRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).get_detections(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetDetectionsSvc(inner);
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
                "/viam.service.vision.v1.VisionService/GetClassificationsFromCamera" => {
                    #[allow(non_camel_case_types)]
                    struct GetClassificationsFromCameraSvc<T: VisionService>(pub Arc<T>);
                    impl<
                        T: VisionService,
                    > tonic::server::UnaryService<
                        super::GetClassificationsFromCameraRequest,
                    > for GetClassificationsFromCameraSvc<T> {
                        type Response = super::GetClassificationsFromCameraResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::GetClassificationsFromCameraRequest,
                            >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).get_classifications_from_camera(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetClassificationsFromCameraSvc(inner);
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
                "/viam.service.vision.v1.VisionService/GetClassifications" => {
                    #[allow(non_camel_case_types)]
                    struct GetClassificationsSvc<T: VisionService>(pub Arc<T>);
                    impl<
                        T: VisionService,
                    > tonic::server::UnaryService<super::GetClassificationsRequest>
                    for GetClassificationsSvc<T> {
                        type Response = super::GetClassificationsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetClassificationsRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).get_classifications(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetClassificationsSvc(inner);
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
                "/viam.service.vision.v1.VisionService/GetObjectPointClouds" => {
                    #[allow(non_camel_case_types)]
                    struct GetObjectPointCloudsSvc<T: VisionService>(pub Arc<T>);
                    impl<
                        T: VisionService,
                    > tonic::server::UnaryService<super::GetObjectPointCloudsRequest>
                    for GetObjectPointCloudsSvc<T> {
                        type Response = super::GetObjectPointCloudsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetObjectPointCloudsRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).get_object_point_clouds(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetObjectPointCloudsSvc(inner);
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
                "/viam.service.vision.v1.VisionService/DoCommand" => {
                    #[allow(non_camel_case_types)]
                    struct DoCommandSvc<T: VisionService>(pub Arc<T>);
                    impl<
                        T: VisionService,
                    > tonic::server::UnaryService<
                        super::super::super::super::common::v1::DoCommandRequest,
                    > for DoCommandSvc<T> {
                        type Response = super::super::super::super::common::v1::DoCommandResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::super::super::super::common::v1::DoCommandRequest,
                            >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).do_command(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = DoCommandSvc(inner);
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
    impl<T: VisionService> Clone for VisionServiceServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
            }
        }
    }
    impl<T: VisionService> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: VisionService> tonic::transport::NamedService for VisionServiceServer<T> {
        const NAME: &'static str = "viam.service.vision.v1.VisionService";
    }
}
