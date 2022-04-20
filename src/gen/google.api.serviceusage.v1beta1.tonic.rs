// @generated
/// Generated client implementations.
pub mod service_usage_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[derive(Debug, Clone)]
    pub struct ServiceUsageClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl ServiceUsageClient<tonic::transport::Channel> {
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
    impl<T> ServiceUsageClient<T>
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
        ) -> ServiceUsageClient<InterceptedService<T, F>>
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
            ServiceUsageClient::new(InterceptedService::new(inner, interceptor))
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
        pub async fn enable_service(
            &mut self,
            request: impl tonic::IntoRequest<super::EnableServiceRequest>,
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
                "/google.api.serviceusage.v1beta1.ServiceUsage/EnableService",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn disable_service(
            &mut self,
            request: impl tonic::IntoRequest<super::DisableServiceRequest>,
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
                "/google.api.serviceusage.v1beta1.ServiceUsage/DisableService",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn get_service(
            &mut self,
            request: impl tonic::IntoRequest<super::GetServiceRequest>,
        ) -> Result<tonic::Response<super::Service>, tonic::Status> {
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
                "/google.api.serviceusage.v1beta1.ServiceUsage/GetService",
            );
            self.inner.unary(request.into_request(), path, codec).await
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
                "/google.api.serviceusage.v1beta1.ServiceUsage/ListServices",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn batch_enable_services(
            &mut self,
            request: impl tonic::IntoRequest<super::BatchEnableServicesRequest>,
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
                "/google.api.serviceusage.v1beta1.ServiceUsage/BatchEnableServices",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn list_consumer_quota_metrics(
            &mut self,
            request: impl tonic::IntoRequest<super::ListConsumerQuotaMetricsRequest>,
        ) -> Result<
                tonic::Response<super::ListConsumerQuotaMetricsResponse>,
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
                "/google.api.serviceusage.v1beta1.ServiceUsage/ListConsumerQuotaMetrics",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn get_consumer_quota_metric(
            &mut self,
            request: impl tonic::IntoRequest<super::GetConsumerQuotaMetricRequest>,
        ) -> Result<tonic::Response<super::ConsumerQuotaMetric>, tonic::Status> {
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
                "/google.api.serviceusage.v1beta1.ServiceUsage/GetConsumerQuotaMetric",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn get_consumer_quota_limit(
            &mut self,
            request: impl tonic::IntoRequest<super::GetConsumerQuotaLimitRequest>,
        ) -> Result<tonic::Response<super::ConsumerQuotaLimit>, tonic::Status> {
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
                "/google.api.serviceusage.v1beta1.ServiceUsage/GetConsumerQuotaLimit",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn create_admin_override(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateAdminOverrideRequest>,
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
                "/google.api.serviceusage.v1beta1.ServiceUsage/CreateAdminOverride",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn update_admin_override(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateAdminOverrideRequest>,
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
                "/google.api.serviceusage.v1beta1.ServiceUsage/UpdateAdminOverride",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn delete_admin_override(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteAdminOverrideRequest>,
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
                "/google.api.serviceusage.v1beta1.ServiceUsage/DeleteAdminOverride",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn list_admin_overrides(
            &mut self,
            request: impl tonic::IntoRequest<super::ListAdminOverridesRequest>,
        ) -> Result<tonic::Response<super::ListAdminOverridesResponse>, tonic::Status> {
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
                "/google.api.serviceusage.v1beta1.ServiceUsage/ListAdminOverrides",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn import_admin_overrides(
            &mut self,
            request: impl tonic::IntoRequest<super::ImportAdminOverridesRequest>,
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
                "/google.api.serviceusage.v1beta1.ServiceUsage/ImportAdminOverrides",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn create_consumer_override(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateConsumerOverrideRequest>,
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
                "/google.api.serviceusage.v1beta1.ServiceUsage/CreateConsumerOverride",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn update_consumer_override(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateConsumerOverrideRequest>,
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
                "/google.api.serviceusage.v1beta1.ServiceUsage/UpdateConsumerOverride",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn delete_consumer_override(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteConsumerOverrideRequest>,
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
                "/google.api.serviceusage.v1beta1.ServiceUsage/DeleteConsumerOverride",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn list_consumer_overrides(
            &mut self,
            request: impl tonic::IntoRequest<super::ListConsumerOverridesRequest>,
        ) -> Result<
                tonic::Response<super::ListConsumerOverridesResponse>,
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
                "/google.api.serviceusage.v1beta1.ServiceUsage/ListConsumerOverrides",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn import_consumer_overrides(
            &mut self,
            request: impl tonic::IntoRequest<super::ImportConsumerOverridesRequest>,
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
                "/google.api.serviceusage.v1beta1.ServiceUsage/ImportConsumerOverrides",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn generate_service_identity(
            &mut self,
            request: impl tonic::IntoRequest<super::GenerateServiceIdentityRequest>,
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
                "/google.api.serviceusage.v1beta1.ServiceUsage/GenerateServiceIdentity",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod service_usage_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    ///Generated trait containing gRPC methods that should be implemented for use with ServiceUsageServer.
    #[async_trait]
    pub trait ServiceUsage: Send + Sync + 'static {
        async fn enable_service(
            &self,
            request: tonic::Request<super::EnableServiceRequest>,
        ) -> Result<
                tonic::Response<super::super::super::super::longrunning::Operation>,
                tonic::Status,
            >;
        async fn disable_service(
            &self,
            request: tonic::Request<super::DisableServiceRequest>,
        ) -> Result<
                tonic::Response<super::super::super::super::longrunning::Operation>,
                tonic::Status,
            >;
        async fn get_service(
            &self,
            request: tonic::Request<super::GetServiceRequest>,
        ) -> Result<tonic::Response<super::Service>, tonic::Status>;
        async fn list_services(
            &self,
            request: tonic::Request<super::ListServicesRequest>,
        ) -> Result<tonic::Response<super::ListServicesResponse>, tonic::Status>;
        async fn batch_enable_services(
            &self,
            request: tonic::Request<super::BatchEnableServicesRequest>,
        ) -> Result<
                tonic::Response<super::super::super::super::longrunning::Operation>,
                tonic::Status,
            >;
        async fn list_consumer_quota_metrics(
            &self,
            request: tonic::Request<super::ListConsumerQuotaMetricsRequest>,
        ) -> Result<
                tonic::Response<super::ListConsumerQuotaMetricsResponse>,
                tonic::Status,
            >;
        async fn get_consumer_quota_metric(
            &self,
            request: tonic::Request<super::GetConsumerQuotaMetricRequest>,
        ) -> Result<tonic::Response<super::ConsumerQuotaMetric>, tonic::Status>;
        async fn get_consumer_quota_limit(
            &self,
            request: tonic::Request<super::GetConsumerQuotaLimitRequest>,
        ) -> Result<tonic::Response<super::ConsumerQuotaLimit>, tonic::Status>;
        async fn create_admin_override(
            &self,
            request: tonic::Request<super::CreateAdminOverrideRequest>,
        ) -> Result<
                tonic::Response<super::super::super::super::longrunning::Operation>,
                tonic::Status,
            >;
        async fn update_admin_override(
            &self,
            request: tonic::Request<super::UpdateAdminOverrideRequest>,
        ) -> Result<
                tonic::Response<super::super::super::super::longrunning::Operation>,
                tonic::Status,
            >;
        async fn delete_admin_override(
            &self,
            request: tonic::Request<super::DeleteAdminOverrideRequest>,
        ) -> Result<
                tonic::Response<super::super::super::super::longrunning::Operation>,
                tonic::Status,
            >;
        async fn list_admin_overrides(
            &self,
            request: tonic::Request<super::ListAdminOverridesRequest>,
        ) -> Result<tonic::Response<super::ListAdminOverridesResponse>, tonic::Status>;
        async fn import_admin_overrides(
            &self,
            request: tonic::Request<super::ImportAdminOverridesRequest>,
        ) -> Result<
                tonic::Response<super::super::super::super::longrunning::Operation>,
                tonic::Status,
            >;
        async fn create_consumer_override(
            &self,
            request: tonic::Request<super::CreateConsumerOverrideRequest>,
        ) -> Result<
                tonic::Response<super::super::super::super::longrunning::Operation>,
                tonic::Status,
            >;
        async fn update_consumer_override(
            &self,
            request: tonic::Request<super::UpdateConsumerOverrideRequest>,
        ) -> Result<
                tonic::Response<super::super::super::super::longrunning::Operation>,
                tonic::Status,
            >;
        async fn delete_consumer_override(
            &self,
            request: tonic::Request<super::DeleteConsumerOverrideRequest>,
        ) -> Result<
                tonic::Response<super::super::super::super::longrunning::Operation>,
                tonic::Status,
            >;
        async fn list_consumer_overrides(
            &self,
            request: tonic::Request<super::ListConsumerOverridesRequest>,
        ) -> Result<
                tonic::Response<super::ListConsumerOverridesResponse>,
                tonic::Status,
            >;
        async fn import_consumer_overrides(
            &self,
            request: tonic::Request<super::ImportConsumerOverridesRequest>,
        ) -> Result<
                tonic::Response<super::super::super::super::longrunning::Operation>,
                tonic::Status,
            >;
        async fn generate_service_identity(
            &self,
            request: tonic::Request<super::GenerateServiceIdentityRequest>,
        ) -> Result<
                tonic::Response<super::super::super::super::longrunning::Operation>,
                tonic::Status,
            >;
    }
    #[derive(Debug)]
    pub struct ServiceUsageServer<T: ServiceUsage> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: ServiceUsage> ServiceUsageServer<T> {
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
    impl<T, B> tonic::codegen::Service<http::Request<B>> for ServiceUsageServer<T>
    where
        T: ServiceUsage,
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
                "/google.api.serviceusage.v1beta1.ServiceUsage/EnableService" => {
                    #[allow(non_camel_case_types)]
                    struct EnableServiceSvc<T: ServiceUsage>(pub Arc<T>);
                    impl<
                        T: ServiceUsage,
                    > tonic::server::UnaryService<super::EnableServiceRequest>
                    for EnableServiceSvc<T> {
                        type Response = super::super::super::super::longrunning::Operation;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::EnableServiceRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).enable_service(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = EnableServiceSvc(inner);
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
                "/google.api.serviceusage.v1beta1.ServiceUsage/DisableService" => {
                    #[allow(non_camel_case_types)]
                    struct DisableServiceSvc<T: ServiceUsage>(pub Arc<T>);
                    impl<
                        T: ServiceUsage,
                    > tonic::server::UnaryService<super::DisableServiceRequest>
                    for DisableServiceSvc<T> {
                        type Response = super::super::super::super::longrunning::Operation;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::DisableServiceRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).disable_service(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = DisableServiceSvc(inner);
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
                "/google.api.serviceusage.v1beta1.ServiceUsage/GetService" => {
                    #[allow(non_camel_case_types)]
                    struct GetServiceSvc<T: ServiceUsage>(pub Arc<T>);
                    impl<
                        T: ServiceUsage,
                    > tonic::server::UnaryService<super::GetServiceRequest>
                    for GetServiceSvc<T> {
                        type Response = super::Service;
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
                "/google.api.serviceusage.v1beta1.ServiceUsage/ListServices" => {
                    #[allow(non_camel_case_types)]
                    struct ListServicesSvc<T: ServiceUsage>(pub Arc<T>);
                    impl<
                        T: ServiceUsage,
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
                "/google.api.serviceusage.v1beta1.ServiceUsage/BatchEnableServices" => {
                    #[allow(non_camel_case_types)]
                    struct BatchEnableServicesSvc<T: ServiceUsage>(pub Arc<T>);
                    impl<
                        T: ServiceUsage,
                    > tonic::server::UnaryService<super::BatchEnableServicesRequest>
                    for BatchEnableServicesSvc<T> {
                        type Response = super::super::super::super::longrunning::Operation;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::BatchEnableServicesRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).batch_enable_services(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = BatchEnableServicesSvc(inner);
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
                "/google.api.serviceusage.v1beta1.ServiceUsage/ListConsumerQuotaMetrics" => {
                    #[allow(non_camel_case_types)]
                    struct ListConsumerQuotaMetricsSvc<T: ServiceUsage>(pub Arc<T>);
                    impl<
                        T: ServiceUsage,
                    > tonic::server::UnaryService<super::ListConsumerQuotaMetricsRequest>
                    for ListConsumerQuotaMetricsSvc<T> {
                        type Response = super::ListConsumerQuotaMetricsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::ListConsumerQuotaMetricsRequest,
                            >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).list_consumer_quota_metrics(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ListConsumerQuotaMetricsSvc(inner);
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
                "/google.api.serviceusage.v1beta1.ServiceUsage/GetConsumerQuotaMetric" => {
                    #[allow(non_camel_case_types)]
                    struct GetConsumerQuotaMetricSvc<T: ServiceUsage>(pub Arc<T>);
                    impl<
                        T: ServiceUsage,
                    > tonic::server::UnaryService<super::GetConsumerQuotaMetricRequest>
                    for GetConsumerQuotaMetricSvc<T> {
                        type Response = super::ConsumerQuotaMetric;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetConsumerQuotaMetricRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).get_consumer_quota_metric(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetConsumerQuotaMetricSvc(inner);
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
                "/google.api.serviceusage.v1beta1.ServiceUsage/GetConsumerQuotaLimit" => {
                    #[allow(non_camel_case_types)]
                    struct GetConsumerQuotaLimitSvc<T: ServiceUsage>(pub Arc<T>);
                    impl<
                        T: ServiceUsage,
                    > tonic::server::UnaryService<super::GetConsumerQuotaLimitRequest>
                    for GetConsumerQuotaLimitSvc<T> {
                        type Response = super::ConsumerQuotaLimit;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetConsumerQuotaLimitRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).get_consumer_quota_limit(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetConsumerQuotaLimitSvc(inner);
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
                "/google.api.serviceusage.v1beta1.ServiceUsage/CreateAdminOverride" => {
                    #[allow(non_camel_case_types)]
                    struct CreateAdminOverrideSvc<T: ServiceUsage>(pub Arc<T>);
                    impl<
                        T: ServiceUsage,
                    > tonic::server::UnaryService<super::CreateAdminOverrideRequest>
                    for CreateAdminOverrideSvc<T> {
                        type Response = super::super::super::super::longrunning::Operation;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CreateAdminOverrideRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).create_admin_override(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = CreateAdminOverrideSvc(inner);
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
                "/google.api.serviceusage.v1beta1.ServiceUsage/UpdateAdminOverride" => {
                    #[allow(non_camel_case_types)]
                    struct UpdateAdminOverrideSvc<T: ServiceUsage>(pub Arc<T>);
                    impl<
                        T: ServiceUsage,
                    > tonic::server::UnaryService<super::UpdateAdminOverrideRequest>
                    for UpdateAdminOverrideSvc<T> {
                        type Response = super::super::super::super::longrunning::Operation;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::UpdateAdminOverrideRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).update_admin_override(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = UpdateAdminOverrideSvc(inner);
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
                "/google.api.serviceusage.v1beta1.ServiceUsage/DeleteAdminOverride" => {
                    #[allow(non_camel_case_types)]
                    struct DeleteAdminOverrideSvc<T: ServiceUsage>(pub Arc<T>);
                    impl<
                        T: ServiceUsage,
                    > tonic::server::UnaryService<super::DeleteAdminOverrideRequest>
                    for DeleteAdminOverrideSvc<T> {
                        type Response = super::super::super::super::longrunning::Operation;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::DeleteAdminOverrideRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).delete_admin_override(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = DeleteAdminOverrideSvc(inner);
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
                "/google.api.serviceusage.v1beta1.ServiceUsage/ListAdminOverrides" => {
                    #[allow(non_camel_case_types)]
                    struct ListAdminOverridesSvc<T: ServiceUsage>(pub Arc<T>);
                    impl<
                        T: ServiceUsage,
                    > tonic::server::UnaryService<super::ListAdminOverridesRequest>
                    for ListAdminOverridesSvc<T> {
                        type Response = super::ListAdminOverridesResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListAdminOverridesRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).list_admin_overrides(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ListAdminOverridesSvc(inner);
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
                "/google.api.serviceusage.v1beta1.ServiceUsage/ImportAdminOverrides" => {
                    #[allow(non_camel_case_types)]
                    struct ImportAdminOverridesSvc<T: ServiceUsage>(pub Arc<T>);
                    impl<
                        T: ServiceUsage,
                    > tonic::server::UnaryService<super::ImportAdminOverridesRequest>
                    for ImportAdminOverridesSvc<T> {
                        type Response = super::super::super::super::longrunning::Operation;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ImportAdminOverridesRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).import_admin_overrides(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ImportAdminOverridesSvc(inner);
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
                "/google.api.serviceusage.v1beta1.ServiceUsage/CreateConsumerOverride" => {
                    #[allow(non_camel_case_types)]
                    struct CreateConsumerOverrideSvc<T: ServiceUsage>(pub Arc<T>);
                    impl<
                        T: ServiceUsage,
                    > tonic::server::UnaryService<super::CreateConsumerOverrideRequest>
                    for CreateConsumerOverrideSvc<T> {
                        type Response = super::super::super::super::longrunning::Operation;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CreateConsumerOverrideRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).create_consumer_override(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = CreateConsumerOverrideSvc(inner);
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
                "/google.api.serviceusage.v1beta1.ServiceUsage/UpdateConsumerOverride" => {
                    #[allow(non_camel_case_types)]
                    struct UpdateConsumerOverrideSvc<T: ServiceUsage>(pub Arc<T>);
                    impl<
                        T: ServiceUsage,
                    > tonic::server::UnaryService<super::UpdateConsumerOverrideRequest>
                    for UpdateConsumerOverrideSvc<T> {
                        type Response = super::super::super::super::longrunning::Operation;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::UpdateConsumerOverrideRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).update_consumer_override(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = UpdateConsumerOverrideSvc(inner);
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
                "/google.api.serviceusage.v1beta1.ServiceUsage/DeleteConsumerOverride" => {
                    #[allow(non_camel_case_types)]
                    struct DeleteConsumerOverrideSvc<T: ServiceUsage>(pub Arc<T>);
                    impl<
                        T: ServiceUsage,
                    > tonic::server::UnaryService<super::DeleteConsumerOverrideRequest>
                    for DeleteConsumerOverrideSvc<T> {
                        type Response = super::super::super::super::longrunning::Operation;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::DeleteConsumerOverrideRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).delete_consumer_override(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = DeleteConsumerOverrideSvc(inner);
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
                "/google.api.serviceusage.v1beta1.ServiceUsage/ListConsumerOverrides" => {
                    #[allow(non_camel_case_types)]
                    struct ListConsumerOverridesSvc<T: ServiceUsage>(pub Arc<T>);
                    impl<
                        T: ServiceUsage,
                    > tonic::server::UnaryService<super::ListConsumerOverridesRequest>
                    for ListConsumerOverridesSvc<T> {
                        type Response = super::ListConsumerOverridesResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListConsumerOverridesRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).list_consumer_overrides(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ListConsumerOverridesSvc(inner);
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
                "/google.api.serviceusage.v1beta1.ServiceUsage/ImportConsumerOverrides" => {
                    #[allow(non_camel_case_types)]
                    struct ImportConsumerOverridesSvc<T: ServiceUsage>(pub Arc<T>);
                    impl<
                        T: ServiceUsage,
                    > tonic::server::UnaryService<super::ImportConsumerOverridesRequest>
                    for ImportConsumerOverridesSvc<T> {
                        type Response = super::super::super::super::longrunning::Operation;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::ImportConsumerOverridesRequest,
                            >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).import_consumer_overrides(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ImportConsumerOverridesSvc(inner);
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
                "/google.api.serviceusage.v1beta1.ServiceUsage/GenerateServiceIdentity" => {
                    #[allow(non_camel_case_types)]
                    struct GenerateServiceIdentitySvc<T: ServiceUsage>(pub Arc<T>);
                    impl<
                        T: ServiceUsage,
                    > tonic::server::UnaryService<super::GenerateServiceIdentityRequest>
                    for GenerateServiceIdentitySvc<T> {
                        type Response = super::super::super::super::longrunning::Operation;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::GenerateServiceIdentityRequest,
                            >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).generate_service_identity(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GenerateServiceIdentitySvc(inner);
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
    impl<T: ServiceUsage> Clone for ServiceUsageServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
            }
        }
    }
    impl<T: ServiceUsage> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: ServiceUsage> tonic::transport::NamedService for ServiceUsageServer<T> {
        const NAME: &'static str = "google.api.serviceusage.v1beta1.ServiceUsage";
    }
}
