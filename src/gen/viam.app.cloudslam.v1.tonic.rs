// @generated
/// Generated client implementations.
pub mod cloud_slam_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    #[derive(Debug, Clone)]
    pub struct CloudSlamServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl CloudSlamServiceClient<tonic::transport::Channel> {
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
    impl<T> CloudSlamServiceClient<T>
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
        ) -> CloudSlamServiceClient<InterceptedService<T, F>>
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
            CloudSlamServiceClient::new(InterceptedService::new(inner, interceptor))
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
        pub async fn start_mapping_session(
            &mut self,
            request: impl tonic::IntoRequest<super::StartMappingSessionRequest>,
        ) -> std::result::Result<
            tonic::Response<super::StartMappingSessionResponse>,
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
                "/viam.app.cloudslam.v1.CloudSLAMService/StartMappingSession",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "viam.app.cloudslam.v1.CloudSLAMService",
                        "StartMappingSession",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_active_mapping_sessions_for_robot(
            &mut self,
            request: impl tonic::IntoRequest<
                super::GetActiveMappingSessionsForRobotRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::GetActiveMappingSessionsForRobotResponse>,
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
                "/viam.app.cloudslam.v1.CloudSLAMService/GetActiveMappingSessionsForRobot",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "viam.app.cloudslam.v1.CloudSLAMService",
                        "GetActiveMappingSessionsForRobot",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_mapping_session_point_cloud(
            &mut self,
            request: impl tonic::IntoRequest<super::GetMappingSessionPointCloudRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetMappingSessionPointCloudResponse>,
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
                "/viam.app.cloudslam.v1.CloudSLAMService/GetMappingSessionPointCloud",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "viam.app.cloudslam.v1.CloudSLAMService",
                        "GetMappingSessionPointCloud",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn list_mapping_sessions(
            &mut self,
            request: impl tonic::IntoRequest<super::ListMappingSessionsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListMappingSessionsResponse>,
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
                "/viam.app.cloudslam.v1.CloudSLAMService/ListMappingSessions",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "viam.app.cloudslam.v1.CloudSLAMService",
                        "ListMappingSessions",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn stop_mapping_session(
            &mut self,
            request: impl tonic::IntoRequest<super::StopMappingSessionRequest>,
        ) -> std::result::Result<
            tonic::Response<super::StopMappingSessionResponse>,
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
                "/viam.app.cloudslam.v1.CloudSLAMService/StopMappingSession",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "viam.app.cloudslam.v1.CloudSLAMService",
                        "StopMappingSession",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_mapping_session_metadata_by_id(
            &mut self,
            request: impl tonic::IntoRequest<super::GetMappingSessionMetadataByIdRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetMappingSessionMetadataByIdResponse>,
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
                "/viam.app.cloudslam.v1.CloudSLAMService/GetMappingSessionMetadataByID",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "viam.app.cloudslam.v1.CloudSLAMService",
                        "GetMappingSessionMetadataByID",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod cloud_slam_service_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with CloudSlamServiceServer.
    #[async_trait]
    pub trait CloudSlamService: Send + Sync + 'static {
        async fn start_mapping_session(
            &self,
            request: tonic::Request<super::StartMappingSessionRequest>,
        ) -> std::result::Result<
            tonic::Response<super::StartMappingSessionResponse>,
            tonic::Status,
        >;
        async fn get_active_mapping_sessions_for_robot(
            &self,
            request: tonic::Request<super::GetActiveMappingSessionsForRobotRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetActiveMappingSessionsForRobotResponse>,
            tonic::Status,
        >;
        async fn get_mapping_session_point_cloud(
            &self,
            request: tonic::Request<super::GetMappingSessionPointCloudRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetMappingSessionPointCloudResponse>,
            tonic::Status,
        >;
        async fn list_mapping_sessions(
            &self,
            request: tonic::Request<super::ListMappingSessionsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListMappingSessionsResponse>,
            tonic::Status,
        >;
        async fn stop_mapping_session(
            &self,
            request: tonic::Request<super::StopMappingSessionRequest>,
        ) -> std::result::Result<
            tonic::Response<super::StopMappingSessionResponse>,
            tonic::Status,
        >;
        async fn get_mapping_session_metadata_by_id(
            &self,
            request: tonic::Request<super::GetMappingSessionMetadataByIdRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetMappingSessionMetadataByIdResponse>,
            tonic::Status,
        >;
    }
    #[derive(Debug)]
    pub struct CloudSlamServiceServer<T: CloudSlamService> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
        max_decoding_message_size: Option<usize>,
        max_encoding_message_size: Option<usize>,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: CloudSlamService> CloudSlamServiceServer<T> {
        pub fn new(inner: T) -> Self {
            Self::from_arc(Arc::new(inner))
        }
        pub fn from_arc(inner: Arc<T>) -> Self {
            let inner = _Inner(inner);
            Self {
                inner,
                accept_compression_encodings: Default::default(),
                send_compression_encodings: Default::default(),
                max_decoding_message_size: None,
                max_encoding_message_size: None,
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
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.max_decoding_message_size = Some(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.max_encoding_message_size = Some(limit);
            self
        }
    }
    impl<T, B> tonic::codegen::Service<http::Request<B>> for CloudSlamServiceServer<T>
    where
        T: CloudSlamService,
        B: Body + Send + 'static,
        B::Error: Into<StdError> + Send + 'static,
    {
        type Response = http::Response<tonic::body::BoxBody>;
        type Error = std::convert::Infallible;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(
            &mut self,
            _cx: &mut Context<'_>,
        ) -> Poll<std::result::Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            let inner = self.inner.clone();
            match req.uri().path() {
                "/viam.app.cloudslam.v1.CloudSLAMService/StartMappingSession" => {
                    #[allow(non_camel_case_types)]
                    struct StartMappingSessionSvc<T: CloudSlamService>(pub Arc<T>);
                    impl<
                        T: CloudSlamService,
                    > tonic::server::UnaryService<super::StartMappingSessionRequest>
                    for StartMappingSessionSvc<T> {
                        type Response = super::StartMappingSessionResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::StartMappingSessionRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).start_mapping_session(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = StartMappingSessionSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/viam.app.cloudslam.v1.CloudSLAMService/GetActiveMappingSessionsForRobot" => {
                    #[allow(non_camel_case_types)]
                    struct GetActiveMappingSessionsForRobotSvc<T: CloudSlamService>(
                        pub Arc<T>,
                    );
                    impl<
                        T: CloudSlamService,
                    > tonic::server::UnaryService<
                        super::GetActiveMappingSessionsForRobotRequest,
                    > for GetActiveMappingSessionsForRobotSvc<T> {
                        type Response = super::GetActiveMappingSessionsForRobotResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::GetActiveMappingSessionsForRobotRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner)
                                    .get_active_mapping_sessions_for_robot(request)
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetActiveMappingSessionsForRobotSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/viam.app.cloudslam.v1.CloudSLAMService/GetMappingSessionPointCloud" => {
                    #[allow(non_camel_case_types)]
                    struct GetMappingSessionPointCloudSvc<T: CloudSlamService>(
                        pub Arc<T>,
                    );
                    impl<
                        T: CloudSlamService,
                    > tonic::server::UnaryService<
                        super::GetMappingSessionPointCloudRequest,
                    > for GetMappingSessionPointCloudSvc<T> {
                        type Response = super::GetMappingSessionPointCloudResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::GetMappingSessionPointCloudRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).get_mapping_session_point_cloud(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetMappingSessionPointCloudSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/viam.app.cloudslam.v1.CloudSLAMService/ListMappingSessions" => {
                    #[allow(non_camel_case_types)]
                    struct ListMappingSessionsSvc<T: CloudSlamService>(pub Arc<T>);
                    impl<
                        T: CloudSlamService,
                    > tonic::server::UnaryService<super::ListMappingSessionsRequest>
                    for ListMappingSessionsSvc<T> {
                        type Response = super::ListMappingSessionsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListMappingSessionsRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).list_mapping_sessions(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ListMappingSessionsSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/viam.app.cloudslam.v1.CloudSLAMService/StopMappingSession" => {
                    #[allow(non_camel_case_types)]
                    struct StopMappingSessionSvc<T: CloudSlamService>(pub Arc<T>);
                    impl<
                        T: CloudSlamService,
                    > tonic::server::UnaryService<super::StopMappingSessionRequest>
                    for StopMappingSessionSvc<T> {
                        type Response = super::StopMappingSessionResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::StopMappingSessionRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).stop_mapping_session(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = StopMappingSessionSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/viam.app.cloudslam.v1.CloudSLAMService/GetMappingSessionMetadataByID" => {
                    #[allow(non_camel_case_types)]
                    struct GetMappingSessionMetadataByIDSvc<T: CloudSlamService>(
                        pub Arc<T>,
                    );
                    impl<
                        T: CloudSlamService,
                    > tonic::server::UnaryService<
                        super::GetMappingSessionMetadataByIdRequest,
                    > for GetMappingSessionMetadataByIDSvc<T> {
                        type Response = super::GetMappingSessionMetadataByIdResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::GetMappingSessionMetadataByIdRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).get_mapping_session_metadata_by_id(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetMappingSessionMetadataByIDSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
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
    impl<T: CloudSlamService> Clone for CloudSlamServiceServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
                max_decoding_message_size: self.max_decoding_message_size,
                max_encoding_message_size: self.max_encoding_message_size,
            }
        }
    }
    impl<T: CloudSlamService> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(Arc::clone(&self.0))
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: CloudSlamService> tonic::server::NamedService for CloudSlamServiceServer<T> {
        const NAME: &'static str = "viam.app.cloudslam.v1.CloudSLAMService";
    }
}
