// @generated
/// Generated client implementations.
pub mod navigation_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[derive(Debug, Clone)]
    pub struct NavigationServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl NavigationServiceClient<tonic::transport::Channel> {
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
    impl<T> NavigationServiceClient<T>
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
        ) -> NavigationServiceClient<InterceptedService<T, F>>
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
            NavigationServiceClient::new(InterceptedService::new(inner, interceptor))
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
        pub async fn get_mode(
            &mut self,
            request: impl tonic::IntoRequest<super::GetModeRequest>,
        ) -> Result<tonic::Response<super::GetModeResponse>, tonic::Status> {
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
                "/viam.service.navigation.v1.NavigationService/GetMode",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn set_mode(
            &mut self,
            request: impl tonic::IntoRequest<super::SetModeRequest>,
        ) -> Result<tonic::Response<super::SetModeResponse>, tonic::Status> {
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
                "/viam.service.navigation.v1.NavigationService/SetMode",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn get_location(
            &mut self,
            request: impl tonic::IntoRequest<super::GetLocationRequest>,
        ) -> Result<tonic::Response<super::GetLocationResponse>, tonic::Status> {
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
                "/viam.service.navigation.v1.NavigationService/GetLocation",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn get_waypoints(
            &mut self,
            request: impl tonic::IntoRequest<super::GetWaypointsRequest>,
        ) -> Result<tonic::Response<super::GetWaypointsResponse>, tonic::Status> {
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
                "/viam.service.navigation.v1.NavigationService/GetWaypoints",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn add_waypoint(
            &mut self,
            request: impl tonic::IntoRequest<super::AddWaypointRequest>,
        ) -> Result<tonic::Response<super::AddWaypointResponse>, tonic::Status> {
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
                "/viam.service.navigation.v1.NavigationService/AddWaypoint",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn remove_waypoint(
            &mut self,
            request: impl tonic::IntoRequest<super::RemoveWaypointRequest>,
        ) -> Result<tonic::Response<super::RemoveWaypointResponse>, tonic::Status> {
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
                "/viam.service.navigation.v1.NavigationService/RemoveWaypoint",
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
                "/viam.service.navigation.v1.NavigationService/DoCommand",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod navigation_service_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    ///Generated trait containing gRPC methods that should be implemented for use with NavigationServiceServer.
    #[async_trait]
    pub trait NavigationService: Send + Sync + 'static {
        async fn get_mode(
            &self,
            request: tonic::Request<super::GetModeRequest>,
        ) -> Result<tonic::Response<super::GetModeResponse>, tonic::Status>;
        async fn set_mode(
            &self,
            request: tonic::Request<super::SetModeRequest>,
        ) -> Result<tonic::Response<super::SetModeResponse>, tonic::Status>;
        async fn get_location(
            &self,
            request: tonic::Request<super::GetLocationRequest>,
        ) -> Result<tonic::Response<super::GetLocationResponse>, tonic::Status>;
        async fn get_waypoints(
            &self,
            request: tonic::Request<super::GetWaypointsRequest>,
        ) -> Result<tonic::Response<super::GetWaypointsResponse>, tonic::Status>;
        async fn add_waypoint(
            &self,
            request: tonic::Request<super::AddWaypointRequest>,
        ) -> Result<tonic::Response<super::AddWaypointResponse>, tonic::Status>;
        async fn remove_waypoint(
            &self,
            request: tonic::Request<super::RemoveWaypointRequest>,
        ) -> Result<tonic::Response<super::RemoveWaypointResponse>, tonic::Status>;
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
    pub struct NavigationServiceServer<T: NavigationService> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: NavigationService> NavigationServiceServer<T> {
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
    impl<T, B> tonic::codegen::Service<http::Request<B>> for NavigationServiceServer<T>
    where
        T: NavigationService,
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
                "/viam.service.navigation.v1.NavigationService/GetMode" => {
                    #[allow(non_camel_case_types)]
                    struct GetModeSvc<T: NavigationService>(pub Arc<T>);
                    impl<
                        T: NavigationService,
                    > tonic::server::UnaryService<super::GetModeRequest>
                    for GetModeSvc<T> {
                        type Response = super::GetModeResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetModeRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).get_mode(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetModeSvc(inner);
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
                "/viam.service.navigation.v1.NavigationService/SetMode" => {
                    #[allow(non_camel_case_types)]
                    struct SetModeSvc<T: NavigationService>(pub Arc<T>);
                    impl<
                        T: NavigationService,
                    > tonic::server::UnaryService<super::SetModeRequest>
                    for SetModeSvc<T> {
                        type Response = super::SetModeResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SetModeRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).set_mode(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = SetModeSvc(inner);
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
                "/viam.service.navigation.v1.NavigationService/GetLocation" => {
                    #[allow(non_camel_case_types)]
                    struct GetLocationSvc<T: NavigationService>(pub Arc<T>);
                    impl<
                        T: NavigationService,
                    > tonic::server::UnaryService<super::GetLocationRequest>
                    for GetLocationSvc<T> {
                        type Response = super::GetLocationResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetLocationRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).get_location(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetLocationSvc(inner);
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
                "/viam.service.navigation.v1.NavigationService/GetWaypoints" => {
                    #[allow(non_camel_case_types)]
                    struct GetWaypointsSvc<T: NavigationService>(pub Arc<T>);
                    impl<
                        T: NavigationService,
                    > tonic::server::UnaryService<super::GetWaypointsRequest>
                    for GetWaypointsSvc<T> {
                        type Response = super::GetWaypointsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetWaypointsRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).get_waypoints(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetWaypointsSvc(inner);
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
                "/viam.service.navigation.v1.NavigationService/AddWaypoint" => {
                    #[allow(non_camel_case_types)]
                    struct AddWaypointSvc<T: NavigationService>(pub Arc<T>);
                    impl<
                        T: NavigationService,
                    > tonic::server::UnaryService<super::AddWaypointRequest>
                    for AddWaypointSvc<T> {
                        type Response = super::AddWaypointResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::AddWaypointRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).add_waypoint(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = AddWaypointSvc(inner);
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
                "/viam.service.navigation.v1.NavigationService/RemoveWaypoint" => {
                    #[allow(non_camel_case_types)]
                    struct RemoveWaypointSvc<T: NavigationService>(pub Arc<T>);
                    impl<
                        T: NavigationService,
                    > tonic::server::UnaryService<super::RemoveWaypointRequest>
                    for RemoveWaypointSvc<T> {
                        type Response = super::RemoveWaypointResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::RemoveWaypointRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).remove_waypoint(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = RemoveWaypointSvc(inner);
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
                "/viam.service.navigation.v1.NavigationService/DoCommand" => {
                    #[allow(non_camel_case_types)]
                    struct DoCommandSvc<T: NavigationService>(pub Arc<T>);
                    impl<
                        T: NavigationService,
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
    impl<T: NavigationService> Clone for NavigationServiceServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
            }
        }
    }
    impl<T: NavigationService> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: NavigationService> tonic::transport::NamedService
    for NavigationServiceServer<T> {
        const NAME: &'static str = "viam.service.navigation.v1.NavigationService";
    }
}
