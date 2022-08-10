// @generated
/// Generated client implementations.
pub mod movement_sensor_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[derive(Debug, Clone)]
    pub struct MovementSensorServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl MovementSensorServiceClient<tonic::transport::Channel> {
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
    impl<T> MovementSensorServiceClient<T>
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
        ) -> MovementSensorServiceClient<InterceptedService<T, F>>
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
            MovementSensorServiceClient::new(InterceptedService::new(inner, interceptor))
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
        pub async fn get_linear_velocity(
            &mut self,
            request: impl tonic::IntoRequest<super::GetLinearVelocityRequest>,
        ) -> Result<tonic::Response<super::GetLinearVelocityResponse>, tonic::Status> {
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
                "/proto.api.component.movementsensor.v1.MovementSensorService/GetLinearVelocity",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn get_angular_velocity(
            &mut self,
            request: impl tonic::IntoRequest<super::GetAngularVelocityRequest>,
        ) -> Result<tonic::Response<super::GetAngularVelocityResponse>, tonic::Status> {
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
                "/proto.api.component.movementsensor.v1.MovementSensorService/GetAngularVelocity",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn get_compass_heading(
            &mut self,
            request: impl tonic::IntoRequest<super::GetCompassHeadingRequest>,
        ) -> Result<tonic::Response<super::GetCompassHeadingResponse>, tonic::Status> {
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
                "/proto.api.component.movementsensor.v1.MovementSensorService/GetCompassHeading",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn get_orientation(
            &mut self,
            request: impl tonic::IntoRequest<super::GetOrientationRequest>,
        ) -> Result<tonic::Response<super::GetOrientationResponse>, tonic::Status> {
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
                "/proto.api.component.movementsensor.v1.MovementSensorService/GetOrientation",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn get_position(
            &mut self,
            request: impl tonic::IntoRequest<super::GetPositionRequest>,
        ) -> Result<tonic::Response<super::GetPositionResponse>, tonic::Status> {
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
                "/proto.api.component.movementsensor.v1.MovementSensorService/GetPosition",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn get_properties(
            &mut self,
            request: impl tonic::IntoRequest<super::GetPropertiesRequest>,
        ) -> Result<tonic::Response<super::GetPropertiesResponse>, tonic::Status> {
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
                "/proto.api.component.movementsensor.v1.MovementSensorService/GetProperties",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn get_accuracy(
            &mut self,
            request: impl tonic::IntoRequest<super::GetAccuracyRequest>,
        ) -> Result<tonic::Response<super::GetAccuracyResponse>, tonic::Status> {
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
                "/proto.api.component.movementsensor.v1.MovementSensorService/GetAccuracy",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod movement_sensor_service_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    ///Generated trait containing gRPC methods that should be implemented for use with MovementSensorServiceServer.
    #[async_trait]
    pub trait MovementSensorService: Send + Sync + 'static {
        async fn get_linear_velocity(
            &self,
            request: tonic::Request<super::GetLinearVelocityRequest>,
        ) -> Result<tonic::Response<super::GetLinearVelocityResponse>, tonic::Status>;
        async fn get_angular_velocity(
            &self,
            request: tonic::Request<super::GetAngularVelocityRequest>,
        ) -> Result<tonic::Response<super::GetAngularVelocityResponse>, tonic::Status>;
        async fn get_compass_heading(
            &self,
            request: tonic::Request<super::GetCompassHeadingRequest>,
        ) -> Result<tonic::Response<super::GetCompassHeadingResponse>, tonic::Status>;
        async fn get_orientation(
            &self,
            request: tonic::Request<super::GetOrientationRequest>,
        ) -> Result<tonic::Response<super::GetOrientationResponse>, tonic::Status>;
        async fn get_position(
            &self,
            request: tonic::Request<super::GetPositionRequest>,
        ) -> Result<tonic::Response<super::GetPositionResponse>, tonic::Status>;
        async fn get_properties(
            &self,
            request: tonic::Request<super::GetPropertiesRequest>,
        ) -> Result<tonic::Response<super::GetPropertiesResponse>, tonic::Status>;
        async fn get_accuracy(
            &self,
            request: tonic::Request<super::GetAccuracyRequest>,
        ) -> Result<tonic::Response<super::GetAccuracyResponse>, tonic::Status>;
    }
    #[derive(Debug)]
    pub struct MovementSensorServiceServer<T: MovementSensorService> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: MovementSensorService> MovementSensorServiceServer<T> {
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
    impl<T, B> tonic::codegen::Service<http::Request<B>>
    for MovementSensorServiceServer<T>
    where
        T: MovementSensorService,
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
                "/proto.api.component.movementsensor.v1.MovementSensorService/GetLinearVelocity" => {
                    #[allow(non_camel_case_types)]
                    struct GetLinearVelocitySvc<T: MovementSensorService>(pub Arc<T>);
                    impl<
                        T: MovementSensorService,
                    > tonic::server::UnaryService<super::GetLinearVelocityRequest>
                    for GetLinearVelocitySvc<T> {
                        type Response = super::GetLinearVelocityResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetLinearVelocityRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).get_linear_velocity(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetLinearVelocitySvc(inner);
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
                "/proto.api.component.movementsensor.v1.MovementSensorService/GetAngularVelocity" => {
                    #[allow(non_camel_case_types)]
                    struct GetAngularVelocitySvc<T: MovementSensorService>(pub Arc<T>);
                    impl<
                        T: MovementSensorService,
                    > tonic::server::UnaryService<super::GetAngularVelocityRequest>
                    for GetAngularVelocitySvc<T> {
                        type Response = super::GetAngularVelocityResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetAngularVelocityRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).get_angular_velocity(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetAngularVelocitySvc(inner);
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
                "/proto.api.component.movementsensor.v1.MovementSensorService/GetCompassHeading" => {
                    #[allow(non_camel_case_types)]
                    struct GetCompassHeadingSvc<T: MovementSensorService>(pub Arc<T>);
                    impl<
                        T: MovementSensorService,
                    > tonic::server::UnaryService<super::GetCompassHeadingRequest>
                    for GetCompassHeadingSvc<T> {
                        type Response = super::GetCompassHeadingResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetCompassHeadingRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).get_compass_heading(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetCompassHeadingSvc(inner);
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
                "/proto.api.component.movementsensor.v1.MovementSensorService/GetOrientation" => {
                    #[allow(non_camel_case_types)]
                    struct GetOrientationSvc<T: MovementSensorService>(pub Arc<T>);
                    impl<
                        T: MovementSensorService,
                    > tonic::server::UnaryService<super::GetOrientationRequest>
                    for GetOrientationSvc<T> {
                        type Response = super::GetOrientationResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetOrientationRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).get_orientation(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetOrientationSvc(inner);
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
                "/proto.api.component.movementsensor.v1.MovementSensorService/GetPosition" => {
                    #[allow(non_camel_case_types)]
                    struct GetPositionSvc<T: MovementSensorService>(pub Arc<T>);
                    impl<
                        T: MovementSensorService,
                    > tonic::server::UnaryService<super::GetPositionRequest>
                    for GetPositionSvc<T> {
                        type Response = super::GetPositionResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetPositionRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).get_position(request).await
                            };
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
                "/proto.api.component.movementsensor.v1.MovementSensorService/GetProperties" => {
                    #[allow(non_camel_case_types)]
                    struct GetPropertiesSvc<T: MovementSensorService>(pub Arc<T>);
                    impl<
                        T: MovementSensorService,
                    > tonic::server::UnaryService<super::GetPropertiesRequest>
                    for GetPropertiesSvc<T> {
                        type Response = super::GetPropertiesResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetPropertiesRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).get_properties(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetPropertiesSvc(inner);
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
                "/proto.api.component.movementsensor.v1.MovementSensorService/GetAccuracy" => {
                    #[allow(non_camel_case_types)]
                    struct GetAccuracySvc<T: MovementSensorService>(pub Arc<T>);
                    impl<
                        T: MovementSensorService,
                    > tonic::server::UnaryService<super::GetAccuracyRequest>
                    for GetAccuracySvc<T> {
                        type Response = super::GetAccuracyResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetAccuracyRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).get_accuracy(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetAccuracySvc(inner);
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
    impl<T: MovementSensorService> Clone for MovementSensorServiceServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
            }
        }
    }
    impl<T: MovementSensorService> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: MovementSensorService> tonic::transport::NamedService
    for MovementSensorServiceServer<T> {
        const NAME: &'static str = "proto.api.component.movementsensor.v1.MovementSensorService";
    }
}
