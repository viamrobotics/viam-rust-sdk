// @generated
/// Generated client implementations.
pub mod data_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    #[derive(Debug, Clone)]
    pub struct DataServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl DataServiceClient<tonic::transport::Channel> {
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
    impl<T> DataServiceClient<T>
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
        ) -> DataServiceClient<InterceptedService<T, F>>
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
            DataServiceClient::new(InterceptedService::new(inner, interceptor))
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
        pub async fn tabular_data_by_filter(
            &mut self,
            request: impl tonic::IntoRequest<super::TabularDataByFilterRequest>,
        ) -> std::result::Result<
            tonic::Response<super::TabularDataByFilterResponse>,
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
                "/viam.app.data.v1.DataService/TabularDataByFilter",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "viam.app.data.v1.DataService",
                        "TabularDataByFilter",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn tabular_data_by_sql(
            &mut self,
            request: impl tonic::IntoRequest<super::TabularDataBySqlRequest>,
        ) -> std::result::Result<
            tonic::Response<super::TabularDataBySqlResponse>,
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
                "/viam.app.data.v1.DataService/TabularDataBySQL",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("viam.app.data.v1.DataService", "TabularDataBySQL"),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn tabular_data_by_mql(
            &mut self,
            request: impl tonic::IntoRequest<super::TabularDataByMqlRequest>,
        ) -> std::result::Result<
            tonic::Response<super::TabularDataByMqlResponse>,
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
                "/viam.app.data.v1.DataService/TabularDataByMQL",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("viam.app.data.v1.DataService", "TabularDataByMQL"),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn binary_data_by_filter(
            &mut self,
            request: impl tonic::IntoRequest<super::BinaryDataByFilterRequest>,
        ) -> std::result::Result<
            tonic::Response<super::BinaryDataByFilterResponse>,
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
                "/viam.app.data.v1.DataService/BinaryDataByFilter",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("viam.app.data.v1.DataService", "BinaryDataByFilter"),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn binary_data_by_i_ds(
            &mut self,
            request: impl tonic::IntoRequest<super::BinaryDataByIDsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::BinaryDataByIDsResponse>,
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
                "/viam.app.data.v1.DataService/BinaryDataByIDs",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("viam.app.data.v1.DataService", "BinaryDataByIDs"),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn delete_tabular_data(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteTabularDataRequest>,
        ) -> std::result::Result<
            tonic::Response<super::DeleteTabularDataResponse>,
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
                "/viam.app.data.v1.DataService/DeleteTabularData",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("viam.app.data.v1.DataService", "DeleteTabularData"),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn delete_binary_data_by_filter(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteBinaryDataByFilterRequest>,
        ) -> std::result::Result<
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
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "viam.app.data.v1.DataService",
                        "DeleteBinaryDataByFilter",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn delete_binary_data_by_i_ds(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteBinaryDataByIDsRequest>,
        ) -> std::result::Result<
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
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "viam.app.data.v1.DataService",
                        "DeleteBinaryDataByIDs",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn add_tags_to_binary_data_by_i_ds(
            &mut self,
            request: impl tonic::IntoRequest<super::AddTagsToBinaryDataByIDsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::AddTagsToBinaryDataByIDsResponse>,
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
                "/viam.app.data.v1.DataService/AddTagsToBinaryDataByIDs",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "viam.app.data.v1.DataService",
                        "AddTagsToBinaryDataByIDs",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn add_tags_to_binary_data_by_filter(
            &mut self,
            request: impl tonic::IntoRequest<super::AddTagsToBinaryDataByFilterRequest>,
        ) -> std::result::Result<
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
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "viam.app.data.v1.DataService",
                        "AddTagsToBinaryDataByFilter",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn remove_tags_from_binary_data_by_i_ds(
            &mut self,
            request: impl tonic::IntoRequest<super::RemoveTagsFromBinaryDataByIDsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::RemoveTagsFromBinaryDataByIDsResponse>,
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
                "/viam.app.data.v1.DataService/RemoveTagsFromBinaryDataByIDs",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "viam.app.data.v1.DataService",
                        "RemoveTagsFromBinaryDataByIDs",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn remove_tags_from_binary_data_by_filter(
            &mut self,
            request: impl tonic::IntoRequest<
                super::RemoveTagsFromBinaryDataByFilterRequest,
            >,
        ) -> std::result::Result<
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
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "viam.app.data.v1.DataService",
                        "RemoveTagsFromBinaryDataByFilter",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn tags_by_filter(
            &mut self,
            request: impl tonic::IntoRequest<super::TagsByFilterRequest>,
        ) -> std::result::Result<
            tonic::Response<super::TagsByFilterResponse>,
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
                "/viam.app.data.v1.DataService/TagsByFilter",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("viam.app.data.v1.DataService", "TagsByFilter"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn add_bounding_box_to_image_by_id(
            &mut self,
            request: impl tonic::IntoRequest<super::AddBoundingBoxToImageByIdRequest>,
        ) -> std::result::Result<
            tonic::Response<super::AddBoundingBoxToImageByIdResponse>,
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
                "/viam.app.data.v1.DataService/AddBoundingBoxToImageByID",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "viam.app.data.v1.DataService",
                        "AddBoundingBoxToImageByID",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn remove_bounding_box_from_image_by_id(
            &mut self,
            request: impl tonic::IntoRequest<
                super::RemoveBoundingBoxFromImageByIdRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::RemoveBoundingBoxFromImageByIdResponse>,
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
                "/viam.app.data.v1.DataService/RemoveBoundingBoxFromImageByID",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "viam.app.data.v1.DataService",
                        "RemoveBoundingBoxFromImageByID",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn bounding_box_labels_by_filter(
            &mut self,
            request: impl tonic::IntoRequest<super::BoundingBoxLabelsByFilterRequest>,
        ) -> std::result::Result<
            tonic::Response<super::BoundingBoxLabelsByFilterResponse>,
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
                "/viam.app.data.v1.DataService/BoundingBoxLabelsByFilter",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "viam.app.data.v1.DataService",
                        "BoundingBoxLabelsByFilter",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_database_connection(
            &mut self,
            request: impl tonic::IntoRequest<super::GetDatabaseConnectionRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetDatabaseConnectionResponse>,
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
                "/viam.app.data.v1.DataService/GetDatabaseConnection",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "viam.app.data.v1.DataService",
                        "GetDatabaseConnection",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn configure_database_user(
            &mut self,
            request: impl tonic::IntoRequest<super::ConfigureDatabaseUserRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ConfigureDatabaseUserResponse>,
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
                "/viam.app.data.v1.DataService/ConfigureDatabaseUser",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "viam.app.data.v1.DataService",
                        "ConfigureDatabaseUser",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn add_binary_data_to_dataset_by_i_ds(
            &mut self,
            request: impl tonic::IntoRequest<super::AddBinaryDataToDatasetByIDsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::AddBinaryDataToDatasetByIDsResponse>,
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
                "/viam.app.data.v1.DataService/AddBinaryDataToDatasetByIDs",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "viam.app.data.v1.DataService",
                        "AddBinaryDataToDatasetByIDs",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn remove_binary_data_from_dataset_by_i_ds(
            &mut self,
            request: impl tonic::IntoRequest<
                super::RemoveBinaryDataFromDatasetByIDsRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::RemoveBinaryDataFromDatasetByIDsResponse>,
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
                "/viam.app.data.v1.DataService/RemoveBinaryDataFromDatasetByIDs",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "viam.app.data.v1.DataService",
                        "RemoveBinaryDataFromDatasetByIDs",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod data_service_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with DataServiceServer.
    #[async_trait]
    pub trait DataService: Send + Sync + 'static {
        async fn tabular_data_by_filter(
            &self,
            request: tonic::Request<super::TabularDataByFilterRequest>,
        ) -> std::result::Result<
            tonic::Response<super::TabularDataByFilterResponse>,
            tonic::Status,
        >;
        async fn tabular_data_by_sql(
            &self,
            request: tonic::Request<super::TabularDataBySqlRequest>,
        ) -> std::result::Result<
            tonic::Response<super::TabularDataBySqlResponse>,
            tonic::Status,
        >;
        async fn tabular_data_by_mql(
            &self,
            request: tonic::Request<super::TabularDataByMqlRequest>,
        ) -> std::result::Result<
            tonic::Response<super::TabularDataByMqlResponse>,
            tonic::Status,
        >;
        async fn binary_data_by_filter(
            &self,
            request: tonic::Request<super::BinaryDataByFilterRequest>,
        ) -> std::result::Result<
            tonic::Response<super::BinaryDataByFilterResponse>,
            tonic::Status,
        >;
        async fn binary_data_by_i_ds(
            &self,
            request: tonic::Request<super::BinaryDataByIDsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::BinaryDataByIDsResponse>,
            tonic::Status,
        >;
        async fn delete_tabular_data(
            &self,
            request: tonic::Request<super::DeleteTabularDataRequest>,
        ) -> std::result::Result<
            tonic::Response<super::DeleteTabularDataResponse>,
            tonic::Status,
        >;
        async fn delete_binary_data_by_filter(
            &self,
            request: tonic::Request<super::DeleteBinaryDataByFilterRequest>,
        ) -> std::result::Result<
            tonic::Response<super::DeleteBinaryDataByFilterResponse>,
            tonic::Status,
        >;
        async fn delete_binary_data_by_i_ds(
            &self,
            request: tonic::Request<super::DeleteBinaryDataByIDsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::DeleteBinaryDataByIDsResponse>,
            tonic::Status,
        >;
        async fn add_tags_to_binary_data_by_i_ds(
            &self,
            request: tonic::Request<super::AddTagsToBinaryDataByIDsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::AddTagsToBinaryDataByIDsResponse>,
            tonic::Status,
        >;
        async fn add_tags_to_binary_data_by_filter(
            &self,
            request: tonic::Request<super::AddTagsToBinaryDataByFilterRequest>,
        ) -> std::result::Result<
            tonic::Response<super::AddTagsToBinaryDataByFilterResponse>,
            tonic::Status,
        >;
        async fn remove_tags_from_binary_data_by_i_ds(
            &self,
            request: tonic::Request<super::RemoveTagsFromBinaryDataByIDsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::RemoveTagsFromBinaryDataByIDsResponse>,
            tonic::Status,
        >;
        async fn remove_tags_from_binary_data_by_filter(
            &self,
            request: tonic::Request<super::RemoveTagsFromBinaryDataByFilterRequest>,
        ) -> std::result::Result<
            tonic::Response<super::RemoveTagsFromBinaryDataByFilterResponse>,
            tonic::Status,
        >;
        async fn tags_by_filter(
            &self,
            request: tonic::Request<super::TagsByFilterRequest>,
        ) -> std::result::Result<
            tonic::Response<super::TagsByFilterResponse>,
            tonic::Status,
        >;
        async fn add_bounding_box_to_image_by_id(
            &self,
            request: tonic::Request<super::AddBoundingBoxToImageByIdRequest>,
        ) -> std::result::Result<
            tonic::Response<super::AddBoundingBoxToImageByIdResponse>,
            tonic::Status,
        >;
        async fn remove_bounding_box_from_image_by_id(
            &self,
            request: tonic::Request<super::RemoveBoundingBoxFromImageByIdRequest>,
        ) -> std::result::Result<
            tonic::Response<super::RemoveBoundingBoxFromImageByIdResponse>,
            tonic::Status,
        >;
        async fn bounding_box_labels_by_filter(
            &self,
            request: tonic::Request<super::BoundingBoxLabelsByFilterRequest>,
        ) -> std::result::Result<
            tonic::Response<super::BoundingBoxLabelsByFilterResponse>,
            tonic::Status,
        >;
        async fn get_database_connection(
            &self,
            request: tonic::Request<super::GetDatabaseConnectionRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetDatabaseConnectionResponse>,
            tonic::Status,
        >;
        async fn configure_database_user(
            &self,
            request: tonic::Request<super::ConfigureDatabaseUserRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ConfigureDatabaseUserResponse>,
            tonic::Status,
        >;
        async fn add_binary_data_to_dataset_by_i_ds(
            &self,
            request: tonic::Request<super::AddBinaryDataToDatasetByIDsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::AddBinaryDataToDatasetByIDsResponse>,
            tonic::Status,
        >;
        async fn remove_binary_data_from_dataset_by_i_ds(
            &self,
            request: tonic::Request<super::RemoveBinaryDataFromDatasetByIDsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::RemoveBinaryDataFromDatasetByIDsResponse>,
            tonic::Status,
        >;
    }
    #[derive(Debug)]
    pub struct DataServiceServer<T: DataService> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
        max_decoding_message_size: Option<usize>,
        max_encoding_message_size: Option<usize>,
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
        ) -> Poll<std::result::Result<(), Self::Error>> {
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
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).tabular_data_by_filter(request).await
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
                        let method = TabularDataByFilterSvc(inner);
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
                "/viam.app.data.v1.DataService/TabularDataBySQL" => {
                    #[allow(non_camel_case_types)]
                    struct TabularDataBySQLSvc<T: DataService>(pub Arc<T>);
                    impl<
                        T: DataService,
                    > tonic::server::UnaryService<super::TabularDataBySqlRequest>
                    for TabularDataBySQLSvc<T> {
                        type Response = super::TabularDataBySqlResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::TabularDataBySqlRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).tabular_data_by_sql(request).await
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
                        let method = TabularDataBySQLSvc(inner);
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
                "/viam.app.data.v1.DataService/TabularDataByMQL" => {
                    #[allow(non_camel_case_types)]
                    struct TabularDataByMQLSvc<T: DataService>(pub Arc<T>);
                    impl<
                        T: DataService,
                    > tonic::server::UnaryService<super::TabularDataByMqlRequest>
                    for TabularDataByMQLSvc<T> {
                        type Response = super::TabularDataByMqlResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::TabularDataByMqlRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).tabular_data_by_mql(request).await
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
                        let method = TabularDataByMQLSvc(inner);
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
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).binary_data_by_filter(request).await
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
                        let method = BinaryDataByFilterSvc(inner);
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
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).binary_data_by_i_ds(request).await
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
                        let method = BinaryDataByIDsSvc(inner);
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
                "/viam.app.data.v1.DataService/DeleteTabularData" => {
                    #[allow(non_camel_case_types)]
                    struct DeleteTabularDataSvc<T: DataService>(pub Arc<T>);
                    impl<
                        T: DataService,
                    > tonic::server::UnaryService<super::DeleteTabularDataRequest>
                    for DeleteTabularDataSvc<T> {
                        type Response = super::DeleteTabularDataResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::DeleteTabularDataRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).delete_tabular_data(request).await
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
                        let method = DeleteTabularDataSvc(inner);
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
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).delete_binary_data_by_filter(request).await
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
                        let method = DeleteBinaryDataByFilterSvc(inner);
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
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).delete_binary_data_by_i_ds(request).await
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
                        let method = DeleteBinaryDataByIDsSvc(inner);
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
                "/viam.app.data.v1.DataService/AddTagsToBinaryDataByIDs" => {
                    #[allow(non_camel_case_types)]
                    struct AddTagsToBinaryDataByIDsSvc<T: DataService>(pub Arc<T>);
                    impl<
                        T: DataService,
                    > tonic::server::UnaryService<super::AddTagsToBinaryDataByIDsRequest>
                    for AddTagsToBinaryDataByIDsSvc<T> {
                        type Response = super::AddTagsToBinaryDataByIDsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::AddTagsToBinaryDataByIDsRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).add_tags_to_binary_data_by_i_ds(request).await
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
                        let method = AddTagsToBinaryDataByIDsSvc(inner);
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
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).add_tags_to_binary_data_by_filter(request).await
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
                        let method = AddTagsToBinaryDataByFilterSvc(inner);
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
                "/viam.app.data.v1.DataService/RemoveTagsFromBinaryDataByIDs" => {
                    #[allow(non_camel_case_types)]
                    struct RemoveTagsFromBinaryDataByIDsSvc<T: DataService>(pub Arc<T>);
                    impl<
                        T: DataService,
                    > tonic::server::UnaryService<
                        super::RemoveTagsFromBinaryDataByIDsRequest,
                    > for RemoveTagsFromBinaryDataByIDsSvc<T> {
                        type Response = super::RemoveTagsFromBinaryDataByIDsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::RemoveTagsFromBinaryDataByIDsRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).remove_tags_from_binary_data_by_i_ds(request).await
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
                        let method = RemoveTagsFromBinaryDataByIDsSvc(inner);
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
                            let inner = Arc::clone(&self.0);
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
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = RemoveTagsFromBinaryDataByFilterSvc(inner);
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
                "/viam.app.data.v1.DataService/TagsByFilter" => {
                    #[allow(non_camel_case_types)]
                    struct TagsByFilterSvc<T: DataService>(pub Arc<T>);
                    impl<
                        T: DataService,
                    > tonic::server::UnaryService<super::TagsByFilterRequest>
                    for TagsByFilterSvc<T> {
                        type Response = super::TagsByFilterResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::TagsByFilterRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).tags_by_filter(request).await
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
                        let method = TagsByFilterSvc(inner);
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
                "/viam.app.data.v1.DataService/AddBoundingBoxToImageByID" => {
                    #[allow(non_camel_case_types)]
                    struct AddBoundingBoxToImageByIDSvc<T: DataService>(pub Arc<T>);
                    impl<
                        T: DataService,
                    > tonic::server::UnaryService<
                        super::AddBoundingBoxToImageByIdRequest,
                    > for AddBoundingBoxToImageByIDSvc<T> {
                        type Response = super::AddBoundingBoxToImageByIdResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::AddBoundingBoxToImageByIdRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).add_bounding_box_to_image_by_id(request).await
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
                        let method = AddBoundingBoxToImageByIDSvc(inner);
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
                "/viam.app.data.v1.DataService/RemoveBoundingBoxFromImageByID" => {
                    #[allow(non_camel_case_types)]
                    struct RemoveBoundingBoxFromImageByIDSvc<T: DataService>(pub Arc<T>);
                    impl<
                        T: DataService,
                    > tonic::server::UnaryService<
                        super::RemoveBoundingBoxFromImageByIdRequest,
                    > for RemoveBoundingBoxFromImageByIDSvc<T> {
                        type Response = super::RemoveBoundingBoxFromImageByIdResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::RemoveBoundingBoxFromImageByIdRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).remove_bounding_box_from_image_by_id(request).await
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
                        let method = RemoveBoundingBoxFromImageByIDSvc(inner);
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
                "/viam.app.data.v1.DataService/BoundingBoxLabelsByFilter" => {
                    #[allow(non_camel_case_types)]
                    struct BoundingBoxLabelsByFilterSvc<T: DataService>(pub Arc<T>);
                    impl<
                        T: DataService,
                    > tonic::server::UnaryService<
                        super::BoundingBoxLabelsByFilterRequest,
                    > for BoundingBoxLabelsByFilterSvc<T> {
                        type Response = super::BoundingBoxLabelsByFilterResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::BoundingBoxLabelsByFilterRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).bounding_box_labels_by_filter(request).await
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
                        let method = BoundingBoxLabelsByFilterSvc(inner);
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
                "/viam.app.data.v1.DataService/GetDatabaseConnection" => {
                    #[allow(non_camel_case_types)]
                    struct GetDatabaseConnectionSvc<T: DataService>(pub Arc<T>);
                    impl<
                        T: DataService,
                    > tonic::server::UnaryService<super::GetDatabaseConnectionRequest>
                    for GetDatabaseConnectionSvc<T> {
                        type Response = super::GetDatabaseConnectionResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetDatabaseConnectionRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).get_database_connection(request).await
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
                        let method = GetDatabaseConnectionSvc(inner);
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
                "/viam.app.data.v1.DataService/ConfigureDatabaseUser" => {
                    #[allow(non_camel_case_types)]
                    struct ConfigureDatabaseUserSvc<T: DataService>(pub Arc<T>);
                    impl<
                        T: DataService,
                    > tonic::server::UnaryService<super::ConfigureDatabaseUserRequest>
                    for ConfigureDatabaseUserSvc<T> {
                        type Response = super::ConfigureDatabaseUserResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ConfigureDatabaseUserRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).configure_database_user(request).await
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
                        let method = ConfigureDatabaseUserSvc(inner);
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
                "/viam.app.data.v1.DataService/AddBinaryDataToDatasetByIDs" => {
                    #[allow(non_camel_case_types)]
                    struct AddBinaryDataToDatasetByIDsSvc<T: DataService>(pub Arc<T>);
                    impl<
                        T: DataService,
                    > tonic::server::UnaryService<
                        super::AddBinaryDataToDatasetByIDsRequest,
                    > for AddBinaryDataToDatasetByIDsSvc<T> {
                        type Response = super::AddBinaryDataToDatasetByIDsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::AddBinaryDataToDatasetByIDsRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).add_binary_data_to_dataset_by_i_ds(request).await
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
                        let method = AddBinaryDataToDatasetByIDsSvc(inner);
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
                "/viam.app.data.v1.DataService/RemoveBinaryDataFromDatasetByIDs" => {
                    #[allow(non_camel_case_types)]
                    struct RemoveBinaryDataFromDatasetByIDsSvc<T: DataService>(
                        pub Arc<T>,
                    );
                    impl<
                        T: DataService,
                    > tonic::server::UnaryService<
                        super::RemoveBinaryDataFromDatasetByIDsRequest,
                    > for RemoveBinaryDataFromDatasetByIDsSvc<T> {
                        type Response = super::RemoveBinaryDataFromDatasetByIDsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::RemoveBinaryDataFromDatasetByIDsRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner)
                                    .remove_binary_data_from_dataset_by_i_ds(request)
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
                        let method = RemoveBinaryDataFromDatasetByIDsSvc(inner);
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
    impl<T: DataService> Clone for DataServiceServer<T> {
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
    impl<T: DataService> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(Arc::clone(&self.0))
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: DataService> tonic::server::NamedService for DataServiceServer<T> {
        const NAME: &'static str = "viam.app.data.v1.DataService";
    }
}
