// @generated
/// Generated client implementations.
pub mod service_manager_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[derive(Debug, Clone)]
    pub struct ServiceManagerClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl ServiceManagerClient<tonic::transport::Channel> {
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
    impl<T> ServiceManagerClient<T>
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
        ) -> ServiceManagerClient<InterceptedService<T, F>>
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
            ServiceManagerClient::new(InterceptedService::new(inner, interceptor))
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
        pub async fn list_services(
            &mut self,
            request: impl tonic::IntoRequest<super::ListServicesRequest>,
        ) -> Result<tonic::Response<super::ListServicesResponse>, tonic::Status> {
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
                "/google.api.servicemanagement.v1.ServiceManager/ListServices",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn get_service(
            &mut self,
            request: impl tonic::IntoRequest<super::GetServiceRequest>,
        ) -> Result<tonic::Response<super::ManagedService>, tonic::Status> {
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
                "/google.api.servicemanagement.v1.ServiceManager/GetService",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn create_service(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateServiceRequest>,
        ) -> Result<
                tonic::Response<super::super::super::super::longrunning::Operation>,
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
                "/google.api.servicemanagement.v1.ServiceManager/CreateService",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn delete_service(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteServiceRequest>,
        ) -> Result<
                tonic::Response<super::super::super::super::longrunning::Operation>,
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
                "/google.api.servicemanagement.v1.ServiceManager/DeleteService",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn undelete_service(
            &mut self,
            request: impl tonic::IntoRequest<super::UndeleteServiceRequest>,
        ) -> Result<
                tonic::Response<super::super::super::super::longrunning::Operation>,
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
                "/google.api.servicemanagement.v1.ServiceManager/UndeleteService",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn list_service_configs(
            &mut self,
            request: impl tonic::IntoRequest<super::ListServiceConfigsRequest>,
        ) -> Result<tonic::Response<super::ListServiceConfigsResponse>, tonic::Status> {
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
                "/google.api.servicemanagement.v1.ServiceManager/ListServiceConfigs",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn get_service_config(
            &mut self,
            request: impl tonic::IntoRequest<super::GetServiceConfigRequest>,
        ) -> Result<tonic::Response<super::super::super::Service>, tonic::Status> {
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
                "/google.api.servicemanagement.v1.ServiceManager/GetServiceConfig",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn create_service_config(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateServiceConfigRequest>,
        ) -> Result<tonic::Response<super::super::super::Service>, tonic::Status> {
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
                "/google.api.servicemanagement.v1.ServiceManager/CreateServiceConfig",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn submit_config_source(
            &mut self,
            request: impl tonic::IntoRequest<super::SubmitConfigSourceRequest>,
        ) -> Result<
                tonic::Response<super::super::super::super::longrunning::Operation>,
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
                "/google.api.servicemanagement.v1.ServiceManager/SubmitConfigSource",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn list_service_rollouts(
            &mut self,
            request: impl tonic::IntoRequest<super::ListServiceRolloutsRequest>,
        ) -> Result<tonic::Response<super::ListServiceRolloutsResponse>, tonic::Status> {
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
                "/google.api.servicemanagement.v1.ServiceManager/ListServiceRollouts",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn get_service_rollout(
            &mut self,
            request: impl tonic::IntoRequest<super::GetServiceRolloutRequest>,
        ) -> Result<tonic::Response<super::Rollout>, tonic::Status> {
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
                "/google.api.servicemanagement.v1.ServiceManager/GetServiceRollout",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn create_service_rollout(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateServiceRolloutRequest>,
        ) -> Result<
                tonic::Response<super::super::super::super::longrunning::Operation>,
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
                "/google.api.servicemanagement.v1.ServiceManager/CreateServiceRollout",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn generate_config_report(
            &mut self,
            request: impl tonic::IntoRequest<super::GenerateConfigReportRequest>,
        ) -> Result<
                tonic::Response<super::GenerateConfigReportResponse>,
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
                "/google.api.servicemanagement.v1.ServiceManager/GenerateConfigReport",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod service_manager_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    ///Generated trait containing gRPC methods that should be implemented for use with ServiceManagerServer.
    #[async_trait]
    pub trait ServiceManager: Send + Sync + 'static {
        async fn list_services(
            &self,
            request: tonic::Request<super::ListServicesRequest>,
        ) -> Result<tonic::Response<super::ListServicesResponse>, tonic::Status>;
        async fn get_service(
            &self,
            request: tonic::Request<super::GetServiceRequest>,
        ) -> Result<tonic::Response<super::ManagedService>, tonic::Status>;
        async fn create_service(
            &self,
            request: tonic::Request<super::CreateServiceRequest>,
        ) -> Result<
                tonic::Response<super::super::super::super::longrunning::Operation>,
                tonic::Status,
            >;
        async fn delete_service(
            &self,
            request: tonic::Request<super::DeleteServiceRequest>,
        ) -> Result<
                tonic::Response<super::super::super::super::longrunning::Operation>,
                tonic::Status,
            >;
        async fn undelete_service(
            &self,
            request: tonic::Request<super::UndeleteServiceRequest>,
        ) -> Result<
                tonic::Response<super::super::super::super::longrunning::Operation>,
                tonic::Status,
            >;
        async fn list_service_configs(
            &self,
            request: tonic::Request<super::ListServiceConfigsRequest>,
        ) -> Result<tonic::Response<super::ListServiceConfigsResponse>, tonic::Status>;
        async fn get_service_config(
            &self,
            request: tonic::Request<super::GetServiceConfigRequest>,
        ) -> Result<tonic::Response<super::super::super::Service>, tonic::Status>;
        async fn create_service_config(
            &self,
            request: tonic::Request<super::CreateServiceConfigRequest>,
        ) -> Result<tonic::Response<super::super::super::Service>, tonic::Status>;
        async fn submit_config_source(
            &self,
            request: tonic::Request<super::SubmitConfigSourceRequest>,
        ) -> Result<
                tonic::Response<super::super::super::super::longrunning::Operation>,
                tonic::Status,
            >;
        async fn list_service_rollouts(
            &self,
            request: tonic::Request<super::ListServiceRolloutsRequest>,
        ) -> Result<tonic::Response<super::ListServiceRolloutsResponse>, tonic::Status>;
        async fn get_service_rollout(
            &self,
            request: tonic::Request<super::GetServiceRolloutRequest>,
        ) -> Result<tonic::Response<super::Rollout>, tonic::Status>;
        async fn create_service_rollout(
            &self,
            request: tonic::Request<super::CreateServiceRolloutRequest>,
        ) -> Result<
                tonic::Response<super::super::super::super::longrunning::Operation>,
                tonic::Status,
            >;
        async fn generate_config_report(
            &self,
            request: tonic::Request<super::GenerateConfigReportRequest>,
        ) -> Result<tonic::Response<super::GenerateConfigReportResponse>, tonic::Status>;
    }
    #[derive(Debug)]
    pub struct ServiceManagerServer<T: ServiceManager> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: ServiceManager> ServiceManagerServer<T> {
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
    impl<T, B> tonic::codegen::Service<http::Request<B>> for ServiceManagerServer<T>
    where
        T: ServiceManager,
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
                "/google.api.servicemanagement.v1.ServiceManager/ListServices" => {
                    #[allow(non_camel_case_types)]
                    struct ListServicesSvc<T: ServiceManager>(pub Arc<T>);
                    impl<
                        T: ServiceManager,
                    > tonic::server::UnaryService<super::ListServicesRequest>
                    for ListServicesSvc<T> {
                        type Response = super::ListServicesResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListServicesRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).list_services(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ListServicesSvc(inner);
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
                "/google.api.servicemanagement.v1.ServiceManager/GetService" => {
                    #[allow(non_camel_case_types)]
                    struct GetServiceSvc<T: ServiceManager>(pub Arc<T>);
                    impl<
                        T: ServiceManager,
                    > tonic::server::UnaryService<super::GetServiceRequest>
                    for GetServiceSvc<T> {
                        type Response = super::ManagedService;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetServiceRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).get_service(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetServiceSvc(inner);
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
                "/google.api.servicemanagement.v1.ServiceManager/CreateService" => {
                    #[allow(non_camel_case_types)]
                    struct CreateServiceSvc<T: ServiceManager>(pub Arc<T>);
                    impl<
                        T: ServiceManager,
                    > tonic::server::UnaryService<super::CreateServiceRequest>
                    for CreateServiceSvc<T> {
                        type Response = super::super::super::super::longrunning::Operation;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CreateServiceRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).create_service(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = CreateServiceSvc(inner);
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
                "/google.api.servicemanagement.v1.ServiceManager/DeleteService" => {
                    #[allow(non_camel_case_types)]
                    struct DeleteServiceSvc<T: ServiceManager>(pub Arc<T>);
                    impl<
                        T: ServiceManager,
                    > tonic::server::UnaryService<super::DeleteServiceRequest>
                    for DeleteServiceSvc<T> {
                        type Response = super::super::super::super::longrunning::Operation;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::DeleteServiceRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).delete_service(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = DeleteServiceSvc(inner);
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
                "/google.api.servicemanagement.v1.ServiceManager/UndeleteService" => {
                    #[allow(non_camel_case_types)]
                    struct UndeleteServiceSvc<T: ServiceManager>(pub Arc<T>);
                    impl<
                        T: ServiceManager,
                    > tonic::server::UnaryService<super::UndeleteServiceRequest>
                    for UndeleteServiceSvc<T> {
                        type Response = super::super::super::super::longrunning::Operation;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::UndeleteServiceRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).undelete_service(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = UndeleteServiceSvc(inner);
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
                "/google.api.servicemanagement.v1.ServiceManager/ListServiceConfigs" => {
                    #[allow(non_camel_case_types)]
                    struct ListServiceConfigsSvc<T: ServiceManager>(pub Arc<T>);
                    impl<
                        T: ServiceManager,
                    > tonic::server::UnaryService<super::ListServiceConfigsRequest>
                    for ListServiceConfigsSvc<T> {
                        type Response = super::ListServiceConfigsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListServiceConfigsRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).list_service_configs(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ListServiceConfigsSvc(inner);
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
                "/google.api.servicemanagement.v1.ServiceManager/GetServiceConfig" => {
                    #[allow(non_camel_case_types)]
                    struct GetServiceConfigSvc<T: ServiceManager>(pub Arc<T>);
                    impl<
                        T: ServiceManager,
                    > tonic::server::UnaryService<super::GetServiceConfigRequest>
                    for GetServiceConfigSvc<T> {
                        type Response = super::super::super::Service;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetServiceConfigRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).get_service_config(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetServiceConfigSvc(inner);
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
                "/google.api.servicemanagement.v1.ServiceManager/CreateServiceConfig" => {
                    #[allow(non_camel_case_types)]
                    struct CreateServiceConfigSvc<T: ServiceManager>(pub Arc<T>);
                    impl<
                        T: ServiceManager,
                    > tonic::server::UnaryService<super::CreateServiceConfigRequest>
                    for CreateServiceConfigSvc<T> {
                        type Response = super::super::super::Service;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CreateServiceConfigRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).create_service_config(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = CreateServiceConfigSvc(inner);
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
                "/google.api.servicemanagement.v1.ServiceManager/SubmitConfigSource" => {
                    #[allow(non_camel_case_types)]
                    struct SubmitConfigSourceSvc<T: ServiceManager>(pub Arc<T>);
                    impl<
                        T: ServiceManager,
                    > tonic::server::UnaryService<super::SubmitConfigSourceRequest>
                    for SubmitConfigSourceSvc<T> {
                        type Response = super::super::super::super::longrunning::Operation;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SubmitConfigSourceRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).submit_config_source(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = SubmitConfigSourceSvc(inner);
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
                "/google.api.servicemanagement.v1.ServiceManager/ListServiceRollouts" => {
                    #[allow(non_camel_case_types)]
                    struct ListServiceRolloutsSvc<T: ServiceManager>(pub Arc<T>);
                    impl<
                        T: ServiceManager,
                    > tonic::server::UnaryService<super::ListServiceRolloutsRequest>
                    for ListServiceRolloutsSvc<T> {
                        type Response = super::ListServiceRolloutsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListServiceRolloutsRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).list_service_rollouts(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ListServiceRolloutsSvc(inner);
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
                "/google.api.servicemanagement.v1.ServiceManager/GetServiceRollout" => {
                    #[allow(non_camel_case_types)]
                    struct GetServiceRolloutSvc<T: ServiceManager>(pub Arc<T>);
                    impl<
                        T: ServiceManager,
                    > tonic::server::UnaryService<super::GetServiceRolloutRequest>
                    for GetServiceRolloutSvc<T> {
                        type Response = super::Rollout;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetServiceRolloutRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).get_service_rollout(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetServiceRolloutSvc(inner);
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
                "/google.api.servicemanagement.v1.ServiceManager/CreateServiceRollout" => {
                    #[allow(non_camel_case_types)]
                    struct CreateServiceRolloutSvc<T: ServiceManager>(pub Arc<T>);
                    impl<
                        T: ServiceManager,
                    > tonic::server::UnaryService<super::CreateServiceRolloutRequest>
                    for CreateServiceRolloutSvc<T> {
                        type Response = super::super::super::super::longrunning::Operation;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CreateServiceRolloutRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).create_service_rollout(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = CreateServiceRolloutSvc(inner);
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
                "/google.api.servicemanagement.v1.ServiceManager/GenerateConfigReport" => {
                    #[allow(non_camel_case_types)]
                    struct GenerateConfigReportSvc<T: ServiceManager>(pub Arc<T>);
                    impl<
                        T: ServiceManager,
                    > tonic::server::UnaryService<super::GenerateConfigReportRequest>
                    for GenerateConfigReportSvc<T> {
                        type Response = super::GenerateConfigReportResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GenerateConfigReportRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).generate_config_report(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GenerateConfigReportSvc(inner);
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
    impl<T: ServiceManager> Clone for ServiceManagerServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
            }
        }
    }
    impl<T: ServiceManager> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: ServiceManager> tonic::transport::NamedService for ServiceManagerServer<T> {
        const NAME: &'static str = "google.api.servicemanagement.v1.ServiceManager";
    }
}
