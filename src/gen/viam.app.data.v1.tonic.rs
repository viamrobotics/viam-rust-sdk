// @generated
/// Generated client implementations.
pub mod data_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[derive(Debug, Clone)]
    pub struct DataServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl DataServiceClient<tonic::transport::Channel> {
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
    impl<T> DataServiceClient<T>
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
        ) -> DataServiceClient<InterceptedService<T, F>>
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
            DataServiceClient::new(InterceptedService::new(inner, interceptor))
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
        pub async fn tabular_data_by_filter(
            &mut self,
            request: impl tonic::IntoRequest<super::TabularDataByFilterRequest>,
        ) -> Result<tonic::Response<super::TabularDataByFilterResponse>, tonic::Status> {
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
                "/viam.app.data.v1.DataService/TabularDataByFilter",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn binary_data_by_filter(
            &mut self,
            request: impl tonic::IntoRequest<super::BinaryDataByFilterRequest>,
        ) -> Result<tonic::Response<super::BinaryDataByFilterResponse>, tonic::Status> {
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
                "/viam.app.data.v1.DataService/BinaryDataByFilter",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn binary_data_by_i_ds(
            &mut self,
            request: impl tonic::IntoRequest<super::BinaryDataByIDsRequest>,
        ) -> Result<tonic::Response<super::BinaryDataByIDsResponse>, tonic::Status> {
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
                "/viam.app.data.v1.DataService/BinaryDataByIDs",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn delete_tabular_data_by_filter(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteTabularDataByFilterRequest>,
        ) -> Result<
                tonic::Response<super::DeleteTabularDataByFilterResponse>,
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
                "/viam.app.data.v1.DataService/DeleteTabularDataByFilter",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn delete_binary_data_by_filter(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteBinaryDataByFilterRequest>,
        ) -> Result<
                tonic::Response<super::DeleteBinaryDataByFilterResponse>,
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
                "/viam.app.data.v1.DataService/DeleteBinaryDataByFilter",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn delete_binary_data_by_i_ds(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteBinaryDataByIDsRequest>,
        ) -> Result<
                tonic::Response<super::DeleteBinaryDataByIDsResponse>,
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
                "/viam.app.data.v1.DataService/DeleteBinaryDataByIDs",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn add_tags_to_binary_data_by_file_i_ds(
            &mut self,
            request: impl tonic::IntoRequest<super::AddTagsToBinaryDataByFileIDsRequest>,
        ) -> Result<
                tonic::Response<super::AddTagsToBinaryDataByFileIDsResponse>,
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
                "/viam.app.data.v1.DataService/AddTagsToBinaryDataByFileIDs",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn add_tags_to_binary_data_by_filter(
            &mut self,
            request: impl tonic::IntoRequest<super::AddTagsToBinaryDataByFilterRequest>,
        ) -> Result<
                tonic::Response<super::AddTagsToBinaryDataByFilterResponse>,
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
                "/viam.app.data.v1.DataService/AddTagsToBinaryDataByFilter",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn remove_tags_from_binary_data_by_file_i_ds(
            &mut self,
            request: impl tonic::IntoRequest<
                super::RemoveTagsFromBinaryDataByFileIDsRequest,
            >,
        ) -> Result<
                tonic::Response<super::RemoveTagsFromBinaryDataByFileIDsResponse>,
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
                "/viam.app.data.v1.DataService/RemoveTagsFromBinaryDataByFileIDs",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn remove_tags_from_binary_data_by_filter(
            &mut self,
            request: impl tonic::IntoRequest<
                super::RemoveTagsFromBinaryDataByFilterRequest,
            >,
        ) -> Result<
                tonic::Response<super::RemoveTagsFromBinaryDataByFilterResponse>,
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
                "/viam.app.data.v1.DataService/RemoveTagsFromBinaryDataByFilter",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod data_service_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    ///Generated trait containing gRPC methods that should be implemented for use with DataServiceServer.
    #[async_trait]
    pub trait DataService: Send + Sync + 'static {
        async fn tabular_data_by_filter(
            &self,
            request: tonic::Request<super::TabularDataByFilterRequest>,
        ) -> Result<tonic::Response<super::TabularDataByFilterResponse>, tonic::Status>;
        async fn binary_data_by_filter(
            &self,
            request: tonic::Request<super::BinaryDataByFilterRequest>,
        ) -> Result<tonic::Response<super::BinaryDataByFilterResponse>, tonic::Status>;
        async fn binary_data_by_i_ds(
            &self,
            request: tonic::Request<super::BinaryDataByIDsRequest>,
        ) -> Result<tonic::Response<super::BinaryDataByIDsResponse>, tonic::Status>;
        async fn delete_tabular_data_by_filter(
            &self,
            request: tonic::Request<super::DeleteTabularDataByFilterRequest>,
        ) -> Result<
                tonic::Response<super::DeleteTabularDataByFilterResponse>,
                tonic::Status,
            >;
        async fn delete_binary_data_by_filter(
            &self,
            request: tonic::Request<super::DeleteBinaryDataByFilterRequest>,
        ) -> Result<
                tonic::Response<super::DeleteBinaryDataByFilterResponse>,
                tonic::Status,
            >;
        async fn delete_binary_data_by_i_ds(
            &self,
            request: tonic::Request<super::DeleteBinaryDataByIDsRequest>,
        ) -> Result<
                tonic::Response<super::DeleteBinaryDataByIDsResponse>,
                tonic::Status,
            >;
        async fn add_tags_to_binary_data_by_file_i_ds(
            &self,
            request: tonic::Request<super::AddTagsToBinaryDataByFileIDsRequest>,
        ) -> Result<
                tonic::Response<super::AddTagsToBinaryDataByFileIDsResponse>,
                tonic::Status,
            >;
        async fn add_tags_to_binary_data_by_filter(
            &self,
            request: tonic::Request<super::AddTagsToBinaryDataByFilterRequest>,
        ) -> Result<
                tonic::Response<super::AddTagsToBinaryDataByFilterResponse>,
                tonic::Status,
            >;
        async fn remove_tags_from_binary_data_by_file_i_ds(
            &self,
            request: tonic::Request<super::RemoveTagsFromBinaryDataByFileIDsRequest>,
        ) -> Result<
                tonic::Response<super::RemoveTagsFromBinaryDataByFileIDsResponse>,
                tonic::Status,
            >;
        async fn remove_tags_from_binary_data_by_filter(
            &self,
            request: tonic::Request<super::RemoveTagsFromBinaryDataByFilterRequest>,
        ) -> Result<
                tonic::Response<super::RemoveTagsFromBinaryDataByFilterResponse>,
                tonic::Status,
            >;
    }
    #[derive(Debug)]
    pub struct DataServiceServer<T: DataService> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: DataService> DataServiceServer<T> {
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
    impl<T, B> tonic::codegen::Service<http::Request<B>> for DataServiceServer<T>
    where
        T: DataService,
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
                "/viam.app.data.v1.DataService/TabularDataByFilter" => {
                    #[allow(non_camel_case_types)]
                    struct TabularDataByFilterSvc<T: DataService>(pub Arc<T>);
                    impl<
                        T: DataService,
                    > tonic::server::UnaryService<super::TabularDataByFilterRequest>
                    for TabularDataByFilterSvc<T> {
                        type Response = super::TabularDataByFilterResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::TabularDataByFilterRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).tabular_data_by_filter(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = TabularDataByFilterSvc(inner);
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
                "/viam.app.data.v1.DataService/BinaryDataByFilter" => {
                    #[allow(non_camel_case_types)]
                    struct BinaryDataByFilterSvc<T: DataService>(pub Arc<T>);
                    impl<
                        T: DataService,
                    > tonic::server::UnaryService<super::BinaryDataByFilterRequest>
                    for BinaryDataByFilterSvc<T> {
                        type Response = super::BinaryDataByFilterResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::BinaryDataByFilterRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).binary_data_by_filter(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = BinaryDataByFilterSvc(inner);
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
                "/viam.app.data.v1.DataService/BinaryDataByIDs" => {
                    #[allow(non_camel_case_types)]
                    struct BinaryDataByIDsSvc<T: DataService>(pub Arc<T>);
                    impl<
                        T: DataService,
                    > tonic::server::UnaryService<super::BinaryDataByIDsRequest>
                    for BinaryDataByIDsSvc<T> {
                        type Response = super::BinaryDataByIDsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::BinaryDataByIDsRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).binary_data_by_i_ds(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = BinaryDataByIDsSvc(inner);
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
                "/viam.app.data.v1.DataService/DeleteTabularDataByFilter" => {
                    #[allow(non_camel_case_types)]
                    struct DeleteTabularDataByFilterSvc<T: DataService>(pub Arc<T>);
                    impl<
                        T: DataService,
                    > tonic::server::UnaryService<
                        super::DeleteTabularDataByFilterRequest,
                    > for DeleteTabularDataByFilterSvc<T> {
                        type Response = super::DeleteTabularDataByFilterResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::DeleteTabularDataByFilterRequest,
                            >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).delete_tabular_data_by_filter(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = DeleteTabularDataByFilterSvc(inner);
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
                "/viam.app.data.v1.DataService/DeleteBinaryDataByFilter" => {
                    #[allow(non_camel_case_types)]
                    struct DeleteBinaryDataByFilterSvc<T: DataService>(pub Arc<T>);
                    impl<
                        T: DataService,
                    > tonic::server::UnaryService<super::DeleteBinaryDataByFilterRequest>
                    for DeleteBinaryDataByFilterSvc<T> {
                        type Response = super::DeleteBinaryDataByFilterResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::DeleteBinaryDataByFilterRequest,
                            >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).delete_binary_data_by_filter(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = DeleteBinaryDataByFilterSvc(inner);
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
                "/viam.app.data.v1.DataService/DeleteBinaryDataByIDs" => {
                    #[allow(non_camel_case_types)]
                    struct DeleteBinaryDataByIDsSvc<T: DataService>(pub Arc<T>);
                    impl<
                        T: DataService,
                    > tonic::server::UnaryService<super::DeleteBinaryDataByIDsRequest>
                    for DeleteBinaryDataByIDsSvc<T> {
                        type Response = super::DeleteBinaryDataByIDsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::DeleteBinaryDataByIDsRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).delete_binary_data_by_i_ds(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = DeleteBinaryDataByIDsSvc(inner);
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
                "/viam.app.data.v1.DataService/AddTagsToBinaryDataByFileIDs" => {
                    #[allow(non_camel_case_types)]
                    struct AddTagsToBinaryDataByFileIDsSvc<T: DataService>(pub Arc<T>);
                    impl<
                        T: DataService,
                    > tonic::server::UnaryService<
                        super::AddTagsToBinaryDataByFileIDsRequest,
                    > for AddTagsToBinaryDataByFileIDsSvc<T> {
                        type Response = super::AddTagsToBinaryDataByFileIDsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::AddTagsToBinaryDataByFileIDsRequest,
                            >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).add_tags_to_binary_data_by_file_i_ds(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = AddTagsToBinaryDataByFileIDsSvc(inner);
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
                "/viam.app.data.v1.DataService/AddTagsToBinaryDataByFilter" => {
                    #[allow(non_camel_case_types)]
                    struct AddTagsToBinaryDataByFilterSvc<T: DataService>(pub Arc<T>);
                    impl<
                        T: DataService,
                    > tonic::server::UnaryService<
                        super::AddTagsToBinaryDataByFilterRequest,
                    > for AddTagsToBinaryDataByFilterSvc<T> {
                        type Response = super::AddTagsToBinaryDataByFilterResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::AddTagsToBinaryDataByFilterRequest,
                            >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).add_tags_to_binary_data_by_filter(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = AddTagsToBinaryDataByFilterSvc(inner);
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
                "/viam.app.data.v1.DataService/RemoveTagsFromBinaryDataByFileIDs" => {
                    #[allow(non_camel_case_types)]
                    struct RemoveTagsFromBinaryDataByFileIDsSvc<T: DataService>(
                        pub Arc<T>,
                    );
                    impl<
                        T: DataService,
                    > tonic::server::UnaryService<
                        super::RemoveTagsFromBinaryDataByFileIDsRequest,
                    > for RemoveTagsFromBinaryDataByFileIDsSvc<T> {
                        type Response = super::RemoveTagsFromBinaryDataByFileIDsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::RemoveTagsFromBinaryDataByFileIDsRequest,
                            >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner)
                                    .remove_tags_from_binary_data_by_file_i_ds(request)
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = RemoveTagsFromBinaryDataByFileIDsSvc(inner);
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
                "/viam.app.data.v1.DataService/RemoveTagsFromBinaryDataByFilter" => {
                    #[allow(non_camel_case_types)]
                    struct RemoveTagsFromBinaryDataByFilterSvc<T: DataService>(
                        pub Arc<T>,
                    );
                    impl<
                        T: DataService,
                    > tonic::server::UnaryService<
                        super::RemoveTagsFromBinaryDataByFilterRequest,
                    > for RemoveTagsFromBinaryDataByFilterSvc<T> {
                        type Response = super::RemoveTagsFromBinaryDataByFilterResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::RemoveTagsFromBinaryDataByFilterRequest,
                            >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner)
                                    .remove_tags_from_binary_data_by_filter(request)
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = RemoveTagsFromBinaryDataByFilterSvc(inner);
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
    impl<T: DataService> Clone for DataServiceServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
            }
        }
    }
    impl<T: DataService> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: DataService> tonic::transport::NamedService for DataServiceServer<T> {
        const NAME: &'static str = "viam.app.data.v1.DataService";
    }
}
