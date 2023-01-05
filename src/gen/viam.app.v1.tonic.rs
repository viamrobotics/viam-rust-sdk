// @generated
/// Generated client implementations.
pub mod app_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[derive(Debug, Clone)]
    pub struct AppServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl AppServiceClient<tonic::transport::Channel> {
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
    impl<T> AppServiceClient<T>
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
        ) -> AppServiceClient<InterceptedService<T, F>>
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
            AppServiceClient::new(InterceptedService::new(inner, interceptor))
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
        pub async fn list_organizations(
            &mut self,
            request: impl tonic::IntoRequest<super::ListOrganizationsRequest>,
        ) -> Result<tonic::Response<super::ListOrganizationsResponse>, tonic::Status> {
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
                "/viam.app.v1.AppService/ListOrganizations",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn create_location(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateLocationRequest>,
        ) -> Result<tonic::Response<super::CreateLocationResponse>, tonic::Status> {
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
                "/viam.app.v1.AppService/CreateLocation",
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
                "/viam.app.v1.AppService/GetLocation",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn update_location(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateLocationRequest>,
        ) -> Result<tonic::Response<super::UpdateLocationResponse>, tonic::Status> {
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
                "/viam.app.v1.AppService/UpdateLocation",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn delete_location(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteLocationRequest>,
        ) -> Result<tonic::Response<super::DeleteLocationResponse>, tonic::Status> {
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
                "/viam.app.v1.AppService/DeleteLocation",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn list_locations(
            &mut self,
            request: impl tonic::IntoRequest<super::ListLocationsRequest>,
        ) -> Result<tonic::Response<super::ListLocationsResponse>, tonic::Status> {
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
                "/viam.app.v1.AppService/ListLocations",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn share_location(
            &mut self,
            request: impl tonic::IntoRequest<super::ShareLocationRequest>,
        ) -> Result<tonic::Response<super::ShareLocationResponse>, tonic::Status> {
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
                "/viam.app.v1.AppService/ShareLocation",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn unshare_location(
            &mut self,
            request: impl tonic::IntoRequest<super::UnshareLocationRequest>,
        ) -> Result<tonic::Response<super::UnshareLocationResponse>, tonic::Status> {
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
                "/viam.app.v1.AppService/UnshareLocation",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn location_auth(
            &mut self,
            request: impl tonic::IntoRequest<super::LocationAuthRequest>,
        ) -> Result<tonic::Response<super::LocationAuthResponse>, tonic::Status> {
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
                "/viam.app.v1.AppService/LocationAuth",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn create_location_secret(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateLocationSecretRequest>,
        ) -> Result<
                tonic::Response<super::CreateLocationSecretResponse>,
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
                "/viam.app.v1.AppService/CreateLocationSecret",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn delete_location_secret(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteLocationSecretRequest>,
        ) -> Result<
                tonic::Response<super::DeleteLocationSecretResponse>,
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
                "/viam.app.v1.AppService/DeleteLocationSecret",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn get_robot(
            &mut self,
            request: impl tonic::IntoRequest<super::GetRobotRequest>,
        ) -> Result<tonic::Response<super::GetRobotResponse>, tonic::Status> {
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
                "/viam.app.v1.AppService/GetRobot",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn get_robot_parts(
            &mut self,
            request: impl tonic::IntoRequest<super::GetRobotPartsRequest>,
        ) -> Result<tonic::Response<super::GetRobotPartsResponse>, tonic::Status> {
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
                "/viam.app.v1.AppService/GetRobotParts",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn get_robot_part(
            &mut self,
            request: impl tonic::IntoRequest<super::GetRobotPartRequest>,
        ) -> Result<tonic::Response<super::GetRobotPartResponse>, tonic::Status> {
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
                "/viam.app.v1.AppService/GetRobotPart",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn get_robot_part_logs(
            &mut self,
            request: impl tonic::IntoRequest<super::GetRobotPartLogsRequest>,
        ) -> Result<tonic::Response<super::GetRobotPartLogsResponse>, tonic::Status> {
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
                "/viam.app.v1.AppService/GetRobotPartLogs",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn tail_robot_part_logs(
            &mut self,
            request: impl tonic::IntoRequest<super::TailRobotPartLogsRequest>,
        ) -> Result<
                tonic::Response<
                    tonic::codec::Streaming<super::TailRobotPartLogsResponse>,
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
                "/viam.app.v1.AppService/TailRobotPartLogs",
            );
            self.inner.server_streaming(request.into_request(), path, codec).await
        }
        pub async fn get_robot_part_history(
            &mut self,
            request: impl tonic::IntoRequest<super::GetRobotPartHistoryRequest>,
        ) -> Result<tonic::Response<super::GetRobotPartHistoryResponse>, tonic::Status> {
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
                "/viam.app.v1.AppService/GetRobotPartHistory",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn update_robot_part(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateRobotPartRequest>,
        ) -> Result<tonic::Response<super::UpdateRobotPartResponse>, tonic::Status> {
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
                "/viam.app.v1.AppService/UpdateRobotPart",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn new_robot_part(
            &mut self,
            request: impl tonic::IntoRequest<super::NewRobotPartRequest>,
        ) -> Result<tonic::Response<super::NewRobotPartResponse>, tonic::Status> {
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
                "/viam.app.v1.AppService/NewRobotPart",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn delete_robot_part(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteRobotPartRequest>,
        ) -> Result<tonic::Response<super::DeleteRobotPartResponse>, tonic::Status> {
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
                "/viam.app.v1.AppService/DeleteRobotPart",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn mark_part_as_main(
            &mut self,
            request: impl tonic::IntoRequest<super::MarkPartAsMainRequest>,
        ) -> Result<tonic::Response<super::MarkPartAsMainResponse>, tonic::Status> {
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
                "/viam.app.v1.AppService/MarkPartAsMain",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn create_robot_part_secret(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateRobotPartSecretRequest>,
        ) -> Result<
                tonic::Response<super::CreateRobotPartSecretResponse>,
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
                "/viam.app.v1.AppService/CreateRobotPartSecret",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn delete_robot_part_secret(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteRobotPartSecretRequest>,
        ) -> Result<
                tonic::Response<super::DeleteRobotPartSecretResponse>,
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
                "/viam.app.v1.AppService/DeleteRobotPartSecret",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn list_robots(
            &mut self,
            request: impl tonic::IntoRequest<super::ListRobotsRequest>,
        ) -> Result<tonic::Response<super::ListRobotsResponse>, tonic::Status> {
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
                "/viam.app.v1.AppService/ListRobots",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn find_robots(
            &mut self,
            request: impl tonic::IntoRequest<super::FindRobotsRequest>,
        ) -> Result<tonic::Response<super::FindRobotsResponse>, tonic::Status> {
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
                "/viam.app.v1.AppService/FindRobots",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn new_robot(
            &mut self,
            request: impl tonic::IntoRequest<super::NewRobotRequest>,
        ) -> Result<tonic::Response<super::NewRobotResponse>, tonic::Status> {
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
                "/viam.app.v1.AppService/NewRobot",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn update_robot(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateRobotRequest>,
        ) -> Result<tonic::Response<super::UpdateRobotResponse>, tonic::Status> {
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
                "/viam.app.v1.AppService/UpdateRobot",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn delete_robot(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteRobotRequest>,
        ) -> Result<tonic::Response<super::DeleteRobotResponse>, tonic::Status> {
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
                "/viam.app.v1.AppService/DeleteRobot",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod app_service_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    ///Generated trait containing gRPC methods that should be implemented for use with AppServiceServer.
    #[async_trait]
    pub trait AppService: Send + Sync + 'static {
        async fn list_organizations(
            &self,
            request: tonic::Request<super::ListOrganizationsRequest>,
        ) -> Result<tonic::Response<super::ListOrganizationsResponse>, tonic::Status>;
        async fn create_location(
            &self,
            request: tonic::Request<super::CreateLocationRequest>,
        ) -> Result<tonic::Response<super::CreateLocationResponse>, tonic::Status>;
        async fn get_location(
            &self,
            request: tonic::Request<super::GetLocationRequest>,
        ) -> Result<tonic::Response<super::GetLocationResponse>, tonic::Status>;
        async fn update_location(
            &self,
            request: tonic::Request<super::UpdateLocationRequest>,
        ) -> Result<tonic::Response<super::UpdateLocationResponse>, tonic::Status>;
        async fn delete_location(
            &self,
            request: tonic::Request<super::DeleteLocationRequest>,
        ) -> Result<tonic::Response<super::DeleteLocationResponse>, tonic::Status>;
        async fn list_locations(
            &self,
            request: tonic::Request<super::ListLocationsRequest>,
        ) -> Result<tonic::Response<super::ListLocationsResponse>, tonic::Status>;
        async fn share_location(
            &self,
            request: tonic::Request<super::ShareLocationRequest>,
        ) -> Result<tonic::Response<super::ShareLocationResponse>, tonic::Status>;
        async fn unshare_location(
            &self,
            request: tonic::Request<super::UnshareLocationRequest>,
        ) -> Result<tonic::Response<super::UnshareLocationResponse>, tonic::Status>;
        async fn location_auth(
            &self,
            request: tonic::Request<super::LocationAuthRequest>,
        ) -> Result<tonic::Response<super::LocationAuthResponse>, tonic::Status>;
        async fn create_location_secret(
            &self,
            request: tonic::Request<super::CreateLocationSecretRequest>,
        ) -> Result<tonic::Response<super::CreateLocationSecretResponse>, tonic::Status>;
        async fn delete_location_secret(
            &self,
            request: tonic::Request<super::DeleteLocationSecretRequest>,
        ) -> Result<tonic::Response<super::DeleteLocationSecretResponse>, tonic::Status>;
        async fn get_robot(
            &self,
            request: tonic::Request<super::GetRobotRequest>,
        ) -> Result<tonic::Response<super::GetRobotResponse>, tonic::Status>;
        async fn get_robot_parts(
            &self,
            request: tonic::Request<super::GetRobotPartsRequest>,
        ) -> Result<tonic::Response<super::GetRobotPartsResponse>, tonic::Status>;
        async fn get_robot_part(
            &self,
            request: tonic::Request<super::GetRobotPartRequest>,
        ) -> Result<tonic::Response<super::GetRobotPartResponse>, tonic::Status>;
        async fn get_robot_part_logs(
            &self,
            request: tonic::Request<super::GetRobotPartLogsRequest>,
        ) -> Result<tonic::Response<super::GetRobotPartLogsResponse>, tonic::Status>;
        ///Server streaming response type for the TailRobotPartLogs method.
        type TailRobotPartLogsStream: futures_core::Stream<
                Item = Result<super::TailRobotPartLogsResponse, tonic::Status>,
            >
            + Send
            + 'static;
        async fn tail_robot_part_logs(
            &self,
            request: tonic::Request<super::TailRobotPartLogsRequest>,
        ) -> Result<tonic::Response<Self::TailRobotPartLogsStream>, tonic::Status>;
        async fn get_robot_part_history(
            &self,
            request: tonic::Request<super::GetRobotPartHistoryRequest>,
        ) -> Result<tonic::Response<super::GetRobotPartHistoryResponse>, tonic::Status>;
        async fn update_robot_part(
            &self,
            request: tonic::Request<super::UpdateRobotPartRequest>,
        ) -> Result<tonic::Response<super::UpdateRobotPartResponse>, tonic::Status>;
        async fn new_robot_part(
            &self,
            request: tonic::Request<super::NewRobotPartRequest>,
        ) -> Result<tonic::Response<super::NewRobotPartResponse>, tonic::Status>;
        async fn delete_robot_part(
            &self,
            request: tonic::Request<super::DeleteRobotPartRequest>,
        ) -> Result<tonic::Response<super::DeleteRobotPartResponse>, tonic::Status>;
        async fn mark_part_as_main(
            &self,
            request: tonic::Request<super::MarkPartAsMainRequest>,
        ) -> Result<tonic::Response<super::MarkPartAsMainResponse>, tonic::Status>;
        async fn create_robot_part_secret(
            &self,
            request: tonic::Request<super::CreateRobotPartSecretRequest>,
        ) -> Result<
                tonic::Response<super::CreateRobotPartSecretResponse>,
                tonic::Status,
            >;
        async fn delete_robot_part_secret(
            &self,
            request: tonic::Request<super::DeleteRobotPartSecretRequest>,
        ) -> Result<
                tonic::Response<super::DeleteRobotPartSecretResponse>,
                tonic::Status,
            >;
        async fn list_robots(
            &self,
            request: tonic::Request<super::ListRobotsRequest>,
        ) -> Result<tonic::Response<super::ListRobotsResponse>, tonic::Status>;
        async fn find_robots(
            &self,
            request: tonic::Request<super::FindRobotsRequest>,
        ) -> Result<tonic::Response<super::FindRobotsResponse>, tonic::Status>;
        async fn new_robot(
            &self,
            request: tonic::Request<super::NewRobotRequest>,
        ) -> Result<tonic::Response<super::NewRobotResponse>, tonic::Status>;
        async fn update_robot(
            &self,
            request: tonic::Request<super::UpdateRobotRequest>,
        ) -> Result<tonic::Response<super::UpdateRobotResponse>, tonic::Status>;
        async fn delete_robot(
            &self,
            request: tonic::Request<super::DeleteRobotRequest>,
        ) -> Result<tonic::Response<super::DeleteRobotResponse>, tonic::Status>;
    }
    #[derive(Debug)]
    pub struct AppServiceServer<T: AppService> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: AppService> AppServiceServer<T> {
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
    impl<T, B> tonic::codegen::Service<http::Request<B>> for AppServiceServer<T>
    where
        T: AppService,
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
                "/viam.app.v1.AppService/ListOrganizations" => {
                    #[allow(non_camel_case_types)]
                    struct ListOrganizationsSvc<T: AppService>(pub Arc<T>);
                    impl<
                        T: AppService,
                    > tonic::server::UnaryService<super::ListOrganizationsRequest>
                    for ListOrganizationsSvc<T> {
                        type Response = super::ListOrganizationsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListOrganizationsRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).list_organizations(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ListOrganizationsSvc(inner);
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
                "/viam.app.v1.AppService/CreateLocation" => {
                    #[allow(non_camel_case_types)]
                    struct CreateLocationSvc<T: AppService>(pub Arc<T>);
                    impl<
                        T: AppService,
                    > tonic::server::UnaryService<super::CreateLocationRequest>
                    for CreateLocationSvc<T> {
                        type Response = super::CreateLocationResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CreateLocationRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).create_location(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = CreateLocationSvc(inner);
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
                "/viam.app.v1.AppService/GetLocation" => {
                    #[allow(non_camel_case_types)]
                    struct GetLocationSvc<T: AppService>(pub Arc<T>);
                    impl<
                        T: AppService,
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
                "/viam.app.v1.AppService/UpdateLocation" => {
                    #[allow(non_camel_case_types)]
                    struct UpdateLocationSvc<T: AppService>(pub Arc<T>);
                    impl<
                        T: AppService,
                    > tonic::server::UnaryService<super::UpdateLocationRequest>
                    for UpdateLocationSvc<T> {
                        type Response = super::UpdateLocationResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::UpdateLocationRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).update_location(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = UpdateLocationSvc(inner);
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
                "/viam.app.v1.AppService/DeleteLocation" => {
                    #[allow(non_camel_case_types)]
                    struct DeleteLocationSvc<T: AppService>(pub Arc<T>);
                    impl<
                        T: AppService,
                    > tonic::server::UnaryService<super::DeleteLocationRequest>
                    for DeleteLocationSvc<T> {
                        type Response = super::DeleteLocationResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::DeleteLocationRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).delete_location(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = DeleteLocationSvc(inner);
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
                "/viam.app.v1.AppService/ListLocations" => {
                    #[allow(non_camel_case_types)]
                    struct ListLocationsSvc<T: AppService>(pub Arc<T>);
                    impl<
                        T: AppService,
                    > tonic::server::UnaryService<super::ListLocationsRequest>
                    for ListLocationsSvc<T> {
                        type Response = super::ListLocationsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListLocationsRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).list_locations(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ListLocationsSvc(inner);
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
                "/viam.app.v1.AppService/ShareLocation" => {
                    #[allow(non_camel_case_types)]
                    struct ShareLocationSvc<T: AppService>(pub Arc<T>);
                    impl<
                        T: AppService,
                    > tonic::server::UnaryService<super::ShareLocationRequest>
                    for ShareLocationSvc<T> {
                        type Response = super::ShareLocationResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ShareLocationRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).share_location(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ShareLocationSvc(inner);
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
                "/viam.app.v1.AppService/UnshareLocation" => {
                    #[allow(non_camel_case_types)]
                    struct UnshareLocationSvc<T: AppService>(pub Arc<T>);
                    impl<
                        T: AppService,
                    > tonic::server::UnaryService<super::UnshareLocationRequest>
                    for UnshareLocationSvc<T> {
                        type Response = super::UnshareLocationResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::UnshareLocationRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).unshare_location(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = UnshareLocationSvc(inner);
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
                "/viam.app.v1.AppService/LocationAuth" => {
                    #[allow(non_camel_case_types)]
                    struct LocationAuthSvc<T: AppService>(pub Arc<T>);
                    impl<
                        T: AppService,
                    > tonic::server::UnaryService<super::LocationAuthRequest>
                    for LocationAuthSvc<T> {
                        type Response = super::LocationAuthResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::LocationAuthRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).location_auth(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = LocationAuthSvc(inner);
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
                "/viam.app.v1.AppService/CreateLocationSecret" => {
                    #[allow(non_camel_case_types)]
                    struct CreateLocationSecretSvc<T: AppService>(pub Arc<T>);
                    impl<
                        T: AppService,
                    > tonic::server::UnaryService<super::CreateLocationSecretRequest>
                    for CreateLocationSecretSvc<T> {
                        type Response = super::CreateLocationSecretResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CreateLocationSecretRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).create_location_secret(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = CreateLocationSecretSvc(inner);
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
                "/viam.app.v1.AppService/DeleteLocationSecret" => {
                    #[allow(non_camel_case_types)]
                    struct DeleteLocationSecretSvc<T: AppService>(pub Arc<T>);
                    impl<
                        T: AppService,
                    > tonic::server::UnaryService<super::DeleteLocationSecretRequest>
                    for DeleteLocationSecretSvc<T> {
                        type Response = super::DeleteLocationSecretResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::DeleteLocationSecretRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).delete_location_secret(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = DeleteLocationSecretSvc(inner);
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
                "/viam.app.v1.AppService/GetRobot" => {
                    #[allow(non_camel_case_types)]
                    struct GetRobotSvc<T: AppService>(pub Arc<T>);
                    impl<
                        T: AppService,
                    > tonic::server::UnaryService<super::GetRobotRequest>
                    for GetRobotSvc<T> {
                        type Response = super::GetRobotResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetRobotRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).get_robot(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetRobotSvc(inner);
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
                "/viam.app.v1.AppService/GetRobotParts" => {
                    #[allow(non_camel_case_types)]
                    struct GetRobotPartsSvc<T: AppService>(pub Arc<T>);
                    impl<
                        T: AppService,
                    > tonic::server::UnaryService<super::GetRobotPartsRequest>
                    for GetRobotPartsSvc<T> {
                        type Response = super::GetRobotPartsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetRobotPartsRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).get_robot_parts(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetRobotPartsSvc(inner);
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
                "/viam.app.v1.AppService/GetRobotPart" => {
                    #[allow(non_camel_case_types)]
                    struct GetRobotPartSvc<T: AppService>(pub Arc<T>);
                    impl<
                        T: AppService,
                    > tonic::server::UnaryService<super::GetRobotPartRequest>
                    for GetRobotPartSvc<T> {
                        type Response = super::GetRobotPartResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetRobotPartRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).get_robot_part(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetRobotPartSvc(inner);
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
                "/viam.app.v1.AppService/GetRobotPartLogs" => {
                    #[allow(non_camel_case_types)]
                    struct GetRobotPartLogsSvc<T: AppService>(pub Arc<T>);
                    impl<
                        T: AppService,
                    > tonic::server::UnaryService<super::GetRobotPartLogsRequest>
                    for GetRobotPartLogsSvc<T> {
                        type Response = super::GetRobotPartLogsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetRobotPartLogsRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).get_robot_part_logs(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetRobotPartLogsSvc(inner);
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
                "/viam.app.v1.AppService/TailRobotPartLogs" => {
                    #[allow(non_camel_case_types)]
                    struct TailRobotPartLogsSvc<T: AppService>(pub Arc<T>);
                    impl<
                        T: AppService,
                    > tonic::server::ServerStreamingService<
                        super::TailRobotPartLogsRequest,
                    > for TailRobotPartLogsSvc<T> {
                        type Response = super::TailRobotPartLogsResponse;
                        type ResponseStream = T::TailRobotPartLogsStream;
                        type Future = BoxFuture<
                            tonic::Response<Self::ResponseStream>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::TailRobotPartLogsRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).tail_robot_part_logs(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = TailRobotPartLogsSvc(inner);
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
                "/viam.app.v1.AppService/GetRobotPartHistory" => {
                    #[allow(non_camel_case_types)]
                    struct GetRobotPartHistorySvc<T: AppService>(pub Arc<T>);
                    impl<
                        T: AppService,
                    > tonic::server::UnaryService<super::GetRobotPartHistoryRequest>
                    for GetRobotPartHistorySvc<T> {
                        type Response = super::GetRobotPartHistoryResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetRobotPartHistoryRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).get_robot_part_history(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetRobotPartHistorySvc(inner);
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
                "/viam.app.v1.AppService/UpdateRobotPart" => {
                    #[allow(non_camel_case_types)]
                    struct UpdateRobotPartSvc<T: AppService>(pub Arc<T>);
                    impl<
                        T: AppService,
                    > tonic::server::UnaryService<super::UpdateRobotPartRequest>
                    for UpdateRobotPartSvc<T> {
                        type Response = super::UpdateRobotPartResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::UpdateRobotPartRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).update_robot_part(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = UpdateRobotPartSvc(inner);
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
                "/viam.app.v1.AppService/NewRobotPart" => {
                    #[allow(non_camel_case_types)]
                    struct NewRobotPartSvc<T: AppService>(pub Arc<T>);
                    impl<
                        T: AppService,
                    > tonic::server::UnaryService<super::NewRobotPartRequest>
                    for NewRobotPartSvc<T> {
                        type Response = super::NewRobotPartResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::NewRobotPartRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).new_robot_part(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = NewRobotPartSvc(inner);
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
                "/viam.app.v1.AppService/DeleteRobotPart" => {
                    #[allow(non_camel_case_types)]
                    struct DeleteRobotPartSvc<T: AppService>(pub Arc<T>);
                    impl<
                        T: AppService,
                    > tonic::server::UnaryService<super::DeleteRobotPartRequest>
                    for DeleteRobotPartSvc<T> {
                        type Response = super::DeleteRobotPartResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::DeleteRobotPartRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).delete_robot_part(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = DeleteRobotPartSvc(inner);
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
                "/viam.app.v1.AppService/MarkPartAsMain" => {
                    #[allow(non_camel_case_types)]
                    struct MarkPartAsMainSvc<T: AppService>(pub Arc<T>);
                    impl<
                        T: AppService,
                    > tonic::server::UnaryService<super::MarkPartAsMainRequest>
                    for MarkPartAsMainSvc<T> {
                        type Response = super::MarkPartAsMainResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MarkPartAsMainRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).mark_part_as_main(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = MarkPartAsMainSvc(inner);
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
                "/viam.app.v1.AppService/CreateRobotPartSecret" => {
                    #[allow(non_camel_case_types)]
                    struct CreateRobotPartSecretSvc<T: AppService>(pub Arc<T>);
                    impl<
                        T: AppService,
                    > tonic::server::UnaryService<super::CreateRobotPartSecretRequest>
                    for CreateRobotPartSecretSvc<T> {
                        type Response = super::CreateRobotPartSecretResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CreateRobotPartSecretRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).create_robot_part_secret(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = CreateRobotPartSecretSvc(inner);
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
                "/viam.app.v1.AppService/DeleteRobotPartSecret" => {
                    #[allow(non_camel_case_types)]
                    struct DeleteRobotPartSecretSvc<T: AppService>(pub Arc<T>);
                    impl<
                        T: AppService,
                    > tonic::server::UnaryService<super::DeleteRobotPartSecretRequest>
                    for DeleteRobotPartSecretSvc<T> {
                        type Response = super::DeleteRobotPartSecretResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::DeleteRobotPartSecretRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).delete_robot_part_secret(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = DeleteRobotPartSecretSvc(inner);
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
                "/viam.app.v1.AppService/ListRobots" => {
                    #[allow(non_camel_case_types)]
                    struct ListRobotsSvc<T: AppService>(pub Arc<T>);
                    impl<
                        T: AppService,
                    > tonic::server::UnaryService<super::ListRobotsRequest>
                    for ListRobotsSvc<T> {
                        type Response = super::ListRobotsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListRobotsRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).list_robots(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ListRobotsSvc(inner);
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
                "/viam.app.v1.AppService/FindRobots" => {
                    #[allow(non_camel_case_types)]
                    struct FindRobotsSvc<T: AppService>(pub Arc<T>);
                    impl<
                        T: AppService,
                    > tonic::server::UnaryService<super::FindRobotsRequest>
                    for FindRobotsSvc<T> {
                        type Response = super::FindRobotsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::FindRobotsRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).find_robots(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = FindRobotsSvc(inner);
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
                "/viam.app.v1.AppService/NewRobot" => {
                    #[allow(non_camel_case_types)]
                    struct NewRobotSvc<T: AppService>(pub Arc<T>);
                    impl<
                        T: AppService,
                    > tonic::server::UnaryService<super::NewRobotRequest>
                    for NewRobotSvc<T> {
                        type Response = super::NewRobotResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::NewRobotRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).new_robot(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = NewRobotSvc(inner);
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
                "/viam.app.v1.AppService/UpdateRobot" => {
                    #[allow(non_camel_case_types)]
                    struct UpdateRobotSvc<T: AppService>(pub Arc<T>);
                    impl<
                        T: AppService,
                    > tonic::server::UnaryService<super::UpdateRobotRequest>
                    for UpdateRobotSvc<T> {
                        type Response = super::UpdateRobotResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::UpdateRobotRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).update_robot(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = UpdateRobotSvc(inner);
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
                "/viam.app.v1.AppService/DeleteRobot" => {
                    #[allow(non_camel_case_types)]
                    struct DeleteRobotSvc<T: AppService>(pub Arc<T>);
                    impl<
                        T: AppService,
                    > tonic::server::UnaryService<super::DeleteRobotRequest>
                    for DeleteRobotSvc<T> {
                        type Response = super::DeleteRobotResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::DeleteRobotRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).delete_robot(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = DeleteRobotSvc(inner);
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
    impl<T: AppService> Clone for AppServiceServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
            }
        }
    }
    impl<T: AppService> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: AppService> tonic::transport::NamedService for AppServiceServer<T> {
        const NAME: &'static str = "viam.app.v1.AppService";
    }
}
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
        pub async fn config(
            &mut self,
            request: impl tonic::IntoRequest<super::ConfigRequest>,
        ) -> Result<tonic::Response<super::ConfigResponse>, tonic::Status> {
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
                "/viam.app.v1.RobotService/Config",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn certificate(
            &mut self,
            request: impl tonic::IntoRequest<super::CertificateRequest>,
        ) -> Result<tonic::Response<super::CertificateResponse>, tonic::Status> {
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
                "/viam.app.v1.RobotService/Certificate",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn log(
            &mut self,
            request: impl tonic::IntoRequest<super::LogRequest>,
        ) -> Result<tonic::Response<super::LogResponse>, tonic::Status> {
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
                "/viam.app.v1.RobotService/Log",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn needs_restart(
            &mut self,
            request: impl tonic::IntoRequest<super::NeedsRestartRequest>,
        ) -> Result<tonic::Response<super::NeedsRestartResponse>, tonic::Status> {
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
                "/viam.app.v1.RobotService/NeedsRestart",
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
        async fn config(
            &self,
            request: tonic::Request<super::ConfigRequest>,
        ) -> Result<tonic::Response<super::ConfigResponse>, tonic::Status>;
        async fn certificate(
            &self,
            request: tonic::Request<super::CertificateRequest>,
        ) -> Result<tonic::Response<super::CertificateResponse>, tonic::Status>;
        async fn log(
            &self,
            request: tonic::Request<super::LogRequest>,
        ) -> Result<tonic::Response<super::LogResponse>, tonic::Status>;
        async fn needs_restart(
            &self,
            request: tonic::Request<super::NeedsRestartRequest>,
        ) -> Result<tonic::Response<super::NeedsRestartResponse>, tonic::Status>;
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
                "/viam.app.v1.RobotService/Config" => {
                    #[allow(non_camel_case_types)]
                    struct ConfigSvc<T: RobotService>(pub Arc<T>);
                    impl<
                        T: RobotService,
                    > tonic::server::UnaryService<super::ConfigRequest>
                    for ConfigSvc<T> {
                        type Response = super::ConfigResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ConfigRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).config(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ConfigSvc(inner);
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
                "/viam.app.v1.RobotService/Certificate" => {
                    #[allow(non_camel_case_types)]
                    struct CertificateSvc<T: RobotService>(pub Arc<T>);
                    impl<
                        T: RobotService,
                    > tonic::server::UnaryService<super::CertificateRequest>
                    for CertificateSvc<T> {
                        type Response = super::CertificateResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CertificateRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).certificate(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = CertificateSvc(inner);
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
                "/viam.app.v1.RobotService/Log" => {
                    #[allow(non_camel_case_types)]
                    struct LogSvc<T: RobotService>(pub Arc<T>);
                    impl<T: RobotService> tonic::server::UnaryService<super::LogRequest>
                    for LogSvc<T> {
                        type Response = super::LogResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::LogRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).log(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = LogSvc(inner);
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
                "/viam.app.v1.RobotService/NeedsRestart" => {
                    #[allow(non_camel_case_types)]
                    struct NeedsRestartSvc<T: RobotService>(pub Arc<T>);
                    impl<
                        T: RobotService,
                    > tonic::server::UnaryService<super::NeedsRestartRequest>
                    for NeedsRestartSvc<T> {
                        type Response = super::NeedsRestartResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::NeedsRestartRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).needs_restart(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = NeedsRestartSvc(inner);
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
        const NAME: &'static str = "viam.app.v1.RobotService";
    }
}
