// @generated
/// Generated client implementations.
pub mod robot_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[derive(Debug, Clone)]
    pub struct RobotServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl RobotServiceClient<tonic::transport::Channel> {
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
    impl<T> RobotServiceClient<T>
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
        ) -> RobotServiceClient<InterceptedService<T, F>>
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
            RobotServiceClient::new(InterceptedService::new(inner, interceptor))
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
        pub async fn get_operations(
            &mut self,
            request: impl tonic::IntoRequest<super::GetOperationsRequest>,
        ) -> Result<tonic::Response<super::GetOperationsResponse>, tonic::Status> {
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
                "/proto.api.robot.v1.RobotService/GetOperations",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn resource_names(
            &mut self,
            request: impl tonic::IntoRequest<super::ResourceNamesRequest>,
        ) -> Result<tonic::Response<super::ResourceNamesResponse>, tonic::Status> {
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
                "/proto.api.robot.v1.RobotService/ResourceNames",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn resource_rpc_subtypes(
            &mut self,
            request: impl tonic::IntoRequest<super::ResourceRpcSubtypesRequest>,
        ) -> Result<tonic::Response<super::ResourceRpcSubtypesResponse>, tonic::Status> {
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
                "/proto.api.robot.v1.RobotService/ResourceRPCSubtypes",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn cancel_operation(
            &mut self,
            request: impl tonic::IntoRequest<super::CancelOperationRequest>,
        ) -> Result<tonic::Response<super::CancelOperationResponse>, tonic::Status> {
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
                "/proto.api.robot.v1.RobotService/CancelOperation",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn block_for_operation(
            &mut self,
            request: impl tonic::IntoRequest<super::BlockForOperationRequest>,
        ) -> Result<tonic::Response<super::BlockForOperationResponse>, tonic::Status> {
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
                "/proto.api.robot.v1.RobotService/BlockForOperation",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn discover_components(
            &mut self,
            request: impl tonic::IntoRequest<super::DiscoverComponentsRequest>,
        ) -> Result<tonic::Response<super::DiscoverComponentsResponse>, tonic::Status> {
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
                "/proto.api.robot.v1.RobotService/DiscoverComponents",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn frame_system_config(
            &mut self,
            request: impl tonic::IntoRequest<super::FrameSystemConfigRequest>,
        ) -> Result<tonic::Response<super::FrameSystemConfigResponse>, tonic::Status> {
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
                "/proto.api.robot.v1.RobotService/FrameSystemConfig",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn transform_pose(
            &mut self,
            request: impl tonic::IntoRequest<super::TransformPoseRequest>,
        ) -> Result<tonic::Response<super::TransformPoseResponse>, tonic::Status> {
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
                "/proto.api.robot.v1.RobotService/TransformPose",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn get_status(
            &mut self,
            request: impl tonic::IntoRequest<super::GetStatusRequest>,
        ) -> Result<tonic::Response<super::GetStatusResponse>, tonic::Status> {
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
                "/proto.api.robot.v1.RobotService/GetStatus",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn stream_status(
            &mut self,
            request: impl tonic::IntoRequest<super::StreamStatusRequest>,
        ) -> Result<
                tonic::Response<tonic::codec::Streaming<super::StreamStatusResponse>>,
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
                "/proto.api.robot.v1.RobotService/StreamStatus",
            );
            self.inner.server_streaming(request.into_request(), path, codec).await
        }
        pub async fn stop_all(
            &mut self,
            request: impl tonic::IntoRequest<super::StopAllRequest>,
        ) -> Result<tonic::Response<super::StopAllResponse>, tonic::Status> {
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
                "/proto.api.robot.v1.RobotService/StopAll",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod robot_service_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    ///Generated trait containing gRPC methods that should be implemented for use with RobotServiceServer.
    #[async_trait]
    pub trait RobotService: Send + Sync + 'static {
        async fn get_operations(
            &self,
            request: tonic::Request<super::GetOperationsRequest>,
        ) -> Result<tonic::Response<super::GetOperationsResponse>, tonic::Status>;
        async fn resource_names(
            &self,
            request: tonic::Request<super::ResourceNamesRequest>,
        ) -> Result<tonic::Response<super::ResourceNamesResponse>, tonic::Status>;
        async fn resource_rpc_subtypes(
            &self,
            request: tonic::Request<super::ResourceRpcSubtypesRequest>,
        ) -> Result<tonic::Response<super::ResourceRpcSubtypesResponse>, tonic::Status>;
        async fn cancel_operation(
            &self,
            request: tonic::Request<super::CancelOperationRequest>,
        ) -> Result<tonic::Response<super::CancelOperationResponse>, tonic::Status>;
        async fn block_for_operation(
            &self,
            request: tonic::Request<super::BlockForOperationRequest>,
        ) -> Result<tonic::Response<super::BlockForOperationResponse>, tonic::Status>;
        async fn discover_components(
            &self,
            request: tonic::Request<super::DiscoverComponentsRequest>,
        ) -> Result<tonic::Response<super::DiscoverComponentsResponse>, tonic::Status>;
        async fn frame_system_config(
            &self,
            request: tonic::Request<super::FrameSystemConfigRequest>,
        ) -> Result<tonic::Response<super::FrameSystemConfigResponse>, tonic::Status>;
        async fn transform_pose(
            &self,
            request: tonic::Request<super::TransformPoseRequest>,
        ) -> Result<tonic::Response<super::TransformPoseResponse>, tonic::Status>;
        async fn get_status(
            &self,
            request: tonic::Request<super::GetStatusRequest>,
        ) -> Result<tonic::Response<super::GetStatusResponse>, tonic::Status>;
        ///Server streaming response type for the StreamStatus method.
        type StreamStatusStream: futures_core::Stream<
                Item = Result<super::StreamStatusResponse, tonic::Status>,
            >
            + Send
            + 'static;
        async fn stream_status(
            &self,
            request: tonic::Request<super::StreamStatusRequest>,
        ) -> Result<tonic::Response<Self::StreamStatusStream>, tonic::Status>;
        async fn stop_all(
            &self,
            request: tonic::Request<super::StopAllRequest>,
        ) -> Result<tonic::Response<super::StopAllResponse>, tonic::Status>;
    }
    #[derive(Debug)]
    pub struct RobotServiceServer<T: RobotService> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: RobotService> RobotServiceServer<T> {
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
    impl<T, B> tonic::codegen::Service<http::Request<B>> for RobotServiceServer<T>
    where
        T: RobotService,
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
                "/proto.api.robot.v1.RobotService/GetOperations" => {
                    #[allow(non_camel_case_types)]
                    struct GetOperationsSvc<T: RobotService>(pub Arc<T>);
                    impl<
                        T: RobotService,
                    > tonic::server::UnaryService<super::GetOperationsRequest>
                    for GetOperationsSvc<T> {
                        type Response = super::GetOperationsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetOperationsRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).get_operations(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetOperationsSvc(inner);
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
                "/proto.api.robot.v1.RobotService/ResourceNames" => {
                    #[allow(non_camel_case_types)]
                    struct ResourceNamesSvc<T: RobotService>(pub Arc<T>);
                    impl<
                        T: RobotService,
                    > tonic::server::UnaryService<super::ResourceNamesRequest>
                    for ResourceNamesSvc<T> {
                        type Response = super::ResourceNamesResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ResourceNamesRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).resource_names(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ResourceNamesSvc(inner);
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
                "/proto.api.robot.v1.RobotService/ResourceRPCSubtypes" => {
                    #[allow(non_camel_case_types)]
                    struct ResourceRPCSubtypesSvc<T: RobotService>(pub Arc<T>);
                    impl<
                        T: RobotService,
                    > tonic::server::UnaryService<super::ResourceRpcSubtypesRequest>
                    for ResourceRPCSubtypesSvc<T> {
                        type Response = super::ResourceRpcSubtypesResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ResourceRpcSubtypesRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).resource_rpc_subtypes(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ResourceRPCSubtypesSvc(inner);
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
                "/proto.api.robot.v1.RobotService/CancelOperation" => {
                    #[allow(non_camel_case_types)]
                    struct CancelOperationSvc<T: RobotService>(pub Arc<T>);
                    impl<
                        T: RobotService,
                    > tonic::server::UnaryService<super::CancelOperationRequest>
                    for CancelOperationSvc<T> {
                        type Response = super::CancelOperationResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CancelOperationRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).cancel_operation(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = CancelOperationSvc(inner);
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
                "/proto.api.robot.v1.RobotService/BlockForOperation" => {
                    #[allow(non_camel_case_types)]
                    struct BlockForOperationSvc<T: RobotService>(pub Arc<T>);
                    impl<
                        T: RobotService,
                    > tonic::server::UnaryService<super::BlockForOperationRequest>
                    for BlockForOperationSvc<T> {
                        type Response = super::BlockForOperationResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::BlockForOperationRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).block_for_operation(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = BlockForOperationSvc(inner);
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
                "/proto.api.robot.v1.RobotService/DiscoverComponents" => {
                    #[allow(non_camel_case_types)]
                    struct DiscoverComponentsSvc<T: RobotService>(pub Arc<T>);
                    impl<
                        T: RobotService,
                    > tonic::server::UnaryService<super::DiscoverComponentsRequest>
                    for DiscoverComponentsSvc<T> {
                        type Response = super::DiscoverComponentsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::DiscoverComponentsRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).discover_components(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = DiscoverComponentsSvc(inner);
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
                "/proto.api.robot.v1.RobotService/FrameSystemConfig" => {
                    #[allow(non_camel_case_types)]
                    struct FrameSystemConfigSvc<T: RobotService>(pub Arc<T>);
                    impl<
                        T: RobotService,
                    > tonic::server::UnaryService<super::FrameSystemConfigRequest>
                    for FrameSystemConfigSvc<T> {
                        type Response = super::FrameSystemConfigResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::FrameSystemConfigRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).frame_system_config(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = FrameSystemConfigSvc(inner);
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
                "/proto.api.robot.v1.RobotService/TransformPose" => {
                    #[allow(non_camel_case_types)]
                    struct TransformPoseSvc<T: RobotService>(pub Arc<T>);
                    impl<
                        T: RobotService,
                    > tonic::server::UnaryService<super::TransformPoseRequest>
                    for TransformPoseSvc<T> {
                        type Response = super::TransformPoseResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::TransformPoseRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).transform_pose(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = TransformPoseSvc(inner);
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
                "/proto.api.robot.v1.RobotService/GetStatus" => {
                    #[allow(non_camel_case_types)]
                    struct GetStatusSvc<T: RobotService>(pub Arc<T>);
                    impl<
                        T: RobotService,
                    > tonic::server::UnaryService<super::GetStatusRequest>
                    for GetStatusSvc<T> {
                        type Response = super::GetStatusResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
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
                "/proto.api.robot.v1.RobotService/StreamStatus" => {
                    #[allow(non_camel_case_types)]
                    struct StreamStatusSvc<T: RobotService>(pub Arc<T>);
                    impl<
                        T: RobotService,
                    > tonic::server::ServerStreamingService<super::StreamStatusRequest>
                    for StreamStatusSvc<T> {
                        type Response = super::StreamStatusResponse;
                        type ResponseStream = T::StreamStatusStream;
                        type Future = BoxFuture<
                            tonic::Response<Self::ResponseStream>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::StreamStatusRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).stream_status(request).await
                            };
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
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.server_streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/proto.api.robot.v1.RobotService/StopAll" => {
                    #[allow(non_camel_case_types)]
                    struct StopAllSvc<T: RobotService>(pub Arc<T>);
                    impl<
                        T: RobotService,
                    > tonic::server::UnaryService<super::StopAllRequest>
                    for StopAllSvc<T> {
                        type Response = super::StopAllResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::StopAllRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).stop_all(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = StopAllSvc(inner);
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
    impl<T: RobotService> Clone for RobotServiceServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
            }
        }
    }
    impl<T: RobotService> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: RobotService> tonic::transport::NamedService for RobotServiceServer<T> {
        const NAME: &'static str = "proto.api.robot.v1.RobotService";
    }
}
