// @generated
/// Generated client implementations.
pub mod app_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    #[derive(Debug, Clone)]
    pub struct AppServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl AppServiceClient<tonic::transport::Channel> {
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
    impl<T> AppServiceClient<T>
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
        ) -> AppServiceClient<InterceptedService<T, F>>
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
            AppServiceClient::new(InterceptedService::new(inner, interceptor))
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
        pub async fn get_user_id_by_email(
            &mut self,
            request: impl tonic::IntoRequest<super::GetUserIdByEmailRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetUserIdByEmailResponse>,
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
                "/viam.app.v1.AppService/GetUserIDByEmail",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("viam.app.v1.AppService", "GetUserIDByEmail"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn create_organization(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateOrganizationRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CreateOrganizationResponse>,
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
                "/viam.app.v1.AppService/CreateOrganization",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("viam.app.v1.AppService", "CreateOrganization"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn list_organizations(
            &mut self,
            request: impl tonic::IntoRequest<super::ListOrganizationsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListOrganizationsResponse>,
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
                "/viam.app.v1.AppService/ListOrganizations",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("viam.app.v1.AppService", "ListOrganizations"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_organizations_with_access_to_location(
            &mut self,
            request: impl tonic::IntoRequest<
                super::GetOrganizationsWithAccessToLocationRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::GetOrganizationsWithAccessToLocationResponse>,
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
                "/viam.app.v1.AppService/GetOrganizationsWithAccessToLocation",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "viam.app.v1.AppService",
                        "GetOrganizationsWithAccessToLocation",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn list_organizations_by_user(
            &mut self,
            request: impl tonic::IntoRequest<super::ListOrganizationsByUserRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListOrganizationsByUserResponse>,
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
                "/viam.app.v1.AppService/ListOrganizationsByUser",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("viam.app.v1.AppService", "ListOrganizationsByUser"),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_organization(
            &mut self,
            request: impl tonic::IntoRequest<super::GetOrganizationRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetOrganizationResponse>,
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
                "/viam.app.v1.AppService/GetOrganization",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("viam.app.v1.AppService", "GetOrganization"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_organization_namespace_availability(
            &mut self,
            request: impl tonic::IntoRequest<
                super::GetOrganizationNamespaceAvailabilityRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::GetOrganizationNamespaceAvailabilityResponse>,
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
                "/viam.app.v1.AppService/GetOrganizationNamespaceAvailability",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "viam.app.v1.AppService",
                        "GetOrganizationNamespaceAvailability",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn update_organization(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateOrganizationRequest>,
        ) -> std::result::Result<
            tonic::Response<super::UpdateOrganizationResponse>,
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
                "/viam.app.v1.AppService/UpdateOrganization",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("viam.app.v1.AppService", "UpdateOrganization"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn delete_organization(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteOrganizationRequest>,
        ) -> std::result::Result<
            tonic::Response<super::DeleteOrganizationResponse>,
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
                "/viam.app.v1.AppService/DeleteOrganization",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("viam.app.v1.AppService", "DeleteOrganization"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn list_organization_members(
            &mut self,
            request: impl tonic::IntoRequest<super::ListOrganizationMembersRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListOrganizationMembersResponse>,
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
                "/viam.app.v1.AppService/ListOrganizationMembers",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("viam.app.v1.AppService", "ListOrganizationMembers"),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn create_organization_invite(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateOrganizationInviteRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CreateOrganizationInviteResponse>,
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
                "/viam.app.v1.AppService/CreateOrganizationInvite",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("viam.app.v1.AppService", "CreateOrganizationInvite"),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn update_organization_invite_authorizations(
            &mut self,
            request: impl tonic::IntoRequest<
                super::UpdateOrganizationInviteAuthorizationsRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::UpdateOrganizationInviteAuthorizationsResponse>,
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
                "/viam.app.v1.AppService/UpdateOrganizationInviteAuthorizations",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "viam.app.v1.AppService",
                        "UpdateOrganizationInviteAuthorizations",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn delete_organization_member(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteOrganizationMemberRequest>,
        ) -> std::result::Result<
            tonic::Response<super::DeleteOrganizationMemberResponse>,
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
                "/viam.app.v1.AppService/DeleteOrganizationMember",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("viam.app.v1.AppService", "DeleteOrganizationMember"),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn delete_organization_invite(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteOrganizationInviteRequest>,
        ) -> std::result::Result<
            tonic::Response<super::DeleteOrganizationInviteResponse>,
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
                "/viam.app.v1.AppService/DeleteOrganizationInvite",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("viam.app.v1.AppService", "DeleteOrganizationInvite"),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn resend_organization_invite(
            &mut self,
            request: impl tonic::IntoRequest<super::ResendOrganizationInviteRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ResendOrganizationInviteResponse>,
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
                "/viam.app.v1.AppService/ResendOrganizationInvite",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("viam.app.v1.AppService", "ResendOrganizationInvite"),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn create_location(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateLocationRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CreateLocationResponse>,
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
                "/viam.app.v1.AppService/CreateLocation",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("viam.app.v1.AppService", "CreateLocation"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_location(
            &mut self,
            request: impl tonic::IntoRequest<super::GetLocationRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetLocationResponse>,
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
                "/viam.app.v1.AppService/GetLocation",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("viam.app.v1.AppService", "GetLocation"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn update_location(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateLocationRequest>,
        ) -> std::result::Result<
            tonic::Response<super::UpdateLocationResponse>,
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
                "/viam.app.v1.AppService/UpdateLocation",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("viam.app.v1.AppService", "UpdateLocation"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn delete_location(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteLocationRequest>,
        ) -> std::result::Result<
            tonic::Response<super::DeleteLocationResponse>,
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
                "/viam.app.v1.AppService/DeleteLocation",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("viam.app.v1.AppService", "DeleteLocation"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn list_locations(
            &mut self,
            request: impl tonic::IntoRequest<super::ListLocationsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListLocationsResponse>,
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
                "/viam.app.v1.AppService/ListLocations",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("viam.app.v1.AppService", "ListLocations"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn share_location(
            &mut self,
            request: impl tonic::IntoRequest<super::ShareLocationRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ShareLocationResponse>,
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
                "/viam.app.v1.AppService/ShareLocation",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("viam.app.v1.AppService", "ShareLocation"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn unshare_location(
            &mut self,
            request: impl tonic::IntoRequest<super::UnshareLocationRequest>,
        ) -> std::result::Result<
            tonic::Response<super::UnshareLocationResponse>,
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
                "/viam.app.v1.AppService/UnshareLocation",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("viam.app.v1.AppService", "UnshareLocation"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn location_auth(
            &mut self,
            request: impl tonic::IntoRequest<super::LocationAuthRequest>,
        ) -> std::result::Result<
            tonic::Response<super::LocationAuthResponse>,
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
                "/viam.app.v1.AppService/LocationAuth",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("viam.app.v1.AppService", "LocationAuth"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn create_location_secret(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateLocationSecretRequest>,
        ) -> std::result::Result<
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
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("viam.app.v1.AppService", "CreateLocationSecret"),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn delete_location_secret(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteLocationSecretRequest>,
        ) -> std::result::Result<
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
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("viam.app.v1.AppService", "DeleteLocationSecret"),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_robot(
            &mut self,
            request: impl tonic::IntoRequest<super::GetRobotRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetRobotResponse>,
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
                "/viam.app.v1.AppService/GetRobot",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("viam.app.v1.AppService", "GetRobot"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_rover_rental_robots(
            &mut self,
            request: impl tonic::IntoRequest<super::GetRoverRentalRobotsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetRoverRentalRobotsResponse>,
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
                "/viam.app.v1.AppService/GetRoverRentalRobots",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("viam.app.v1.AppService", "GetRoverRentalRobots"),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_robot_parts(
            &mut self,
            request: impl tonic::IntoRequest<super::GetRobotPartsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetRobotPartsResponse>,
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
                "/viam.app.v1.AppService/GetRobotParts",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("viam.app.v1.AppService", "GetRobotParts"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_robot_part(
            &mut self,
            request: impl tonic::IntoRequest<super::GetRobotPartRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetRobotPartResponse>,
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
                "/viam.app.v1.AppService/GetRobotPart",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("viam.app.v1.AppService", "GetRobotPart"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_robot_part_logs(
            &mut self,
            request: impl tonic::IntoRequest<super::GetRobotPartLogsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetRobotPartLogsResponse>,
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
                "/viam.app.v1.AppService/GetRobotPartLogs",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("viam.app.v1.AppService", "GetRobotPartLogs"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn tail_robot_part_logs(
            &mut self,
            request: impl tonic::IntoRequest<super::TailRobotPartLogsRequest>,
        ) -> std::result::Result<
            tonic::Response<tonic::codec::Streaming<super::TailRobotPartLogsResponse>>,
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
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("viam.app.v1.AppService", "TailRobotPartLogs"));
            self.inner.server_streaming(req, path, codec).await
        }
        pub async fn get_robot_part_history(
            &mut self,
            request: impl tonic::IntoRequest<super::GetRobotPartHistoryRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetRobotPartHistoryResponse>,
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
                "/viam.app.v1.AppService/GetRobotPartHistory",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("viam.app.v1.AppService", "GetRobotPartHistory"),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn update_robot_part(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateRobotPartRequest>,
        ) -> std::result::Result<
            tonic::Response<super::UpdateRobotPartResponse>,
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
                "/viam.app.v1.AppService/UpdateRobotPart",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("viam.app.v1.AppService", "UpdateRobotPart"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn new_robot_part(
            &mut self,
            request: impl tonic::IntoRequest<super::NewRobotPartRequest>,
        ) -> std::result::Result<
            tonic::Response<super::NewRobotPartResponse>,
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
                "/viam.app.v1.AppService/NewRobotPart",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("viam.app.v1.AppService", "NewRobotPart"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn delete_robot_part(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteRobotPartRequest>,
        ) -> std::result::Result<
            tonic::Response<super::DeleteRobotPartResponse>,
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
                "/viam.app.v1.AppService/DeleteRobotPart",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("viam.app.v1.AppService", "DeleteRobotPart"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_robot_api_keys(
            &mut self,
            request: impl tonic::IntoRequest<super::GetRobotApiKeysRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetRobotApiKeysResponse>,
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
                "/viam.app.v1.AppService/GetRobotAPIKeys",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("viam.app.v1.AppService", "GetRobotAPIKeys"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn mark_part_as_main(
            &mut self,
            request: impl tonic::IntoRequest<super::MarkPartAsMainRequest>,
        ) -> std::result::Result<
            tonic::Response<super::MarkPartAsMainResponse>,
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
                "/viam.app.v1.AppService/MarkPartAsMain",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("viam.app.v1.AppService", "MarkPartAsMain"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn mark_part_for_restart(
            &mut self,
            request: impl tonic::IntoRequest<super::MarkPartForRestartRequest>,
        ) -> std::result::Result<
            tonic::Response<super::MarkPartForRestartResponse>,
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
                "/viam.app.v1.AppService/MarkPartForRestart",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("viam.app.v1.AppService", "MarkPartForRestart"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn create_robot_part_secret(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateRobotPartSecretRequest>,
        ) -> std::result::Result<
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
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("viam.app.v1.AppService", "CreateRobotPartSecret"),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn delete_robot_part_secret(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteRobotPartSecretRequest>,
        ) -> std::result::Result<
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
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("viam.app.v1.AppService", "DeleteRobotPartSecret"),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn list_robots(
            &mut self,
            request: impl tonic::IntoRequest<super::ListRobotsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListRobotsResponse>,
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
                "/viam.app.v1.AppService/ListRobots",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("viam.app.v1.AppService", "ListRobots"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn new_robot(
            &mut self,
            request: impl tonic::IntoRequest<super::NewRobotRequest>,
        ) -> std::result::Result<
            tonic::Response<super::NewRobotResponse>,
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
                "/viam.app.v1.AppService/NewRobot",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("viam.app.v1.AppService", "NewRobot"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn update_robot(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateRobotRequest>,
        ) -> std::result::Result<
            tonic::Response<super::UpdateRobotResponse>,
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
                "/viam.app.v1.AppService/UpdateRobot",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("viam.app.v1.AppService", "UpdateRobot"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn delete_robot(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteRobotRequest>,
        ) -> std::result::Result<
            tonic::Response<super::DeleteRobotResponse>,
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
                "/viam.app.v1.AppService/DeleteRobot",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("viam.app.v1.AppService", "DeleteRobot"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn list_fragments(
            &mut self,
            request: impl tonic::IntoRequest<super::ListFragmentsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListFragmentsResponse>,
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
                "/viam.app.v1.AppService/ListFragments",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("viam.app.v1.AppService", "ListFragments"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_fragment(
            &mut self,
            request: impl tonic::IntoRequest<super::GetFragmentRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetFragmentResponse>,
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
                "/viam.app.v1.AppService/GetFragment",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("viam.app.v1.AppService", "GetFragment"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn create_fragment(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateFragmentRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CreateFragmentResponse>,
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
                "/viam.app.v1.AppService/CreateFragment",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("viam.app.v1.AppService", "CreateFragment"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn update_fragment(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateFragmentRequest>,
        ) -> std::result::Result<
            tonic::Response<super::UpdateFragmentResponse>,
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
                "/viam.app.v1.AppService/UpdateFragment",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("viam.app.v1.AppService", "UpdateFragment"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn delete_fragment(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteFragmentRequest>,
        ) -> std::result::Result<
            tonic::Response<super::DeleteFragmentResponse>,
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
                "/viam.app.v1.AppService/DeleteFragment",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("viam.app.v1.AppService", "DeleteFragment"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn add_role(
            &mut self,
            request: impl tonic::IntoRequest<super::AddRoleRequest>,
        ) -> std::result::Result<
            tonic::Response<super::AddRoleResponse>,
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
                "/viam.app.v1.AppService/AddRole",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("viam.app.v1.AppService", "AddRole"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn remove_role(
            &mut self,
            request: impl tonic::IntoRequest<super::RemoveRoleRequest>,
        ) -> std::result::Result<
            tonic::Response<super::RemoveRoleResponse>,
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
                "/viam.app.v1.AppService/RemoveRole",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("viam.app.v1.AppService", "RemoveRole"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn change_role(
            &mut self,
            request: impl tonic::IntoRequest<super::ChangeRoleRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ChangeRoleResponse>,
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
                "/viam.app.v1.AppService/ChangeRole",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("viam.app.v1.AppService", "ChangeRole"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn list_authorizations(
            &mut self,
            request: impl tonic::IntoRequest<super::ListAuthorizationsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListAuthorizationsResponse>,
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
                "/viam.app.v1.AppService/ListAuthorizations",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("viam.app.v1.AppService", "ListAuthorizations"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn check_permissions(
            &mut self,
            request: impl tonic::IntoRequest<super::CheckPermissionsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CheckPermissionsResponse>,
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
                "/viam.app.v1.AppService/CheckPermissions",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("viam.app.v1.AppService", "CheckPermissions"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_registry_item(
            &mut self,
            request: impl tonic::IntoRequest<super::GetRegistryItemRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetRegistryItemResponse>,
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
                "/viam.app.v1.AppService/GetRegistryItem",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("viam.app.v1.AppService", "GetRegistryItem"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn create_registry_item(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateRegistryItemRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CreateRegistryItemResponse>,
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
                "/viam.app.v1.AppService/CreateRegistryItem",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("viam.app.v1.AppService", "CreateRegistryItem"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn update_registry_item(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateRegistryItemRequest>,
        ) -> std::result::Result<
            tonic::Response<super::UpdateRegistryItemResponse>,
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
                "/viam.app.v1.AppService/UpdateRegistryItem",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("viam.app.v1.AppService", "UpdateRegistryItem"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn list_registry_items(
            &mut self,
            request: impl tonic::IntoRequest<super::ListRegistryItemsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListRegistryItemsResponse>,
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
                "/viam.app.v1.AppService/ListRegistryItems",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("viam.app.v1.AppService", "ListRegistryItems"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn delete_registry_item(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteRegistryItemRequest>,
        ) -> std::result::Result<
            tonic::Response<super::DeleteRegistryItemResponse>,
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
                "/viam.app.v1.AppService/DeleteRegistryItem",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("viam.app.v1.AppService", "DeleteRegistryItem"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn create_module(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateModuleRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CreateModuleResponse>,
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
                "/viam.app.v1.AppService/CreateModule",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("viam.app.v1.AppService", "CreateModule"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn update_module(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateModuleRequest>,
        ) -> std::result::Result<
            tonic::Response<super::UpdateModuleResponse>,
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
                "/viam.app.v1.AppService/UpdateModule",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("viam.app.v1.AppService", "UpdateModule"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn upload_module_file(
            &mut self,
            request: impl tonic::IntoStreamingRequest<
                Message = super::UploadModuleFileRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::UploadModuleFileResponse>,
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
                "/viam.app.v1.AppService/UploadModuleFile",
            );
            let mut req = request.into_streaming_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("viam.app.v1.AppService", "UploadModuleFile"));
            self.inner.client_streaming(req, path, codec).await
        }
        pub async fn get_module(
            &mut self,
            request: impl tonic::IntoRequest<super::GetModuleRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetModuleResponse>,
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
                "/viam.app.v1.AppService/GetModule",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("viam.app.v1.AppService", "GetModule"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn list_modules(
            &mut self,
            request: impl tonic::IntoRequest<super::ListModulesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListModulesResponse>,
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
                "/viam.app.v1.AppService/ListModules",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("viam.app.v1.AppService", "ListModules"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn create_key(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateKeyRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CreateKeyResponse>,
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
                "/viam.app.v1.AppService/CreateKey",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("viam.app.v1.AppService", "CreateKey"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn delete_key(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteKeyRequest>,
        ) -> std::result::Result<
            tonic::Response<super::DeleteKeyResponse>,
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
                "/viam.app.v1.AppService/DeleteKey",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("viam.app.v1.AppService", "DeleteKey"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn list_keys(
            &mut self,
            request: impl tonic::IntoRequest<super::ListKeysRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListKeysResponse>,
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
                "/viam.app.v1.AppService/ListKeys",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("viam.app.v1.AppService", "ListKeys"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn rotate_key(
            &mut self,
            request: impl tonic::IntoRequest<super::RotateKeyRequest>,
        ) -> std::result::Result<
            tonic::Response<super::RotateKeyResponse>,
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
                "/viam.app.v1.AppService/RotateKey",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("viam.app.v1.AppService", "RotateKey"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn create_key_from_existing_key_authorizations(
            &mut self,
            request: impl tonic::IntoRequest<
                super::CreateKeyFromExistingKeyAuthorizationsRequest,
            >,
        ) -> std::result::Result<
            tonic::Response<super::CreateKeyFromExistingKeyAuthorizationsResponse>,
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
                "/viam.app.v1.AppService/CreateKeyFromExistingKeyAuthorizations",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "viam.app.v1.AppService",
                        "CreateKeyFromExistingKeyAuthorizations",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod app_service_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with AppServiceServer.
    #[async_trait]
    pub trait AppService: Send + Sync + 'static {
        async fn get_user_id_by_email(
            &self,
            request: tonic::Request<super::GetUserIdByEmailRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetUserIdByEmailResponse>,
            tonic::Status,
        >;
        async fn create_organization(
            &self,
            request: tonic::Request<super::CreateOrganizationRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CreateOrganizationResponse>,
            tonic::Status,
        >;
        async fn list_organizations(
            &self,
            request: tonic::Request<super::ListOrganizationsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListOrganizationsResponse>,
            tonic::Status,
        >;
        async fn get_organizations_with_access_to_location(
            &self,
            request: tonic::Request<super::GetOrganizationsWithAccessToLocationRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetOrganizationsWithAccessToLocationResponse>,
            tonic::Status,
        >;
        async fn list_organizations_by_user(
            &self,
            request: tonic::Request<super::ListOrganizationsByUserRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListOrganizationsByUserResponse>,
            tonic::Status,
        >;
        async fn get_organization(
            &self,
            request: tonic::Request<super::GetOrganizationRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetOrganizationResponse>,
            tonic::Status,
        >;
        async fn get_organization_namespace_availability(
            &self,
            request: tonic::Request<super::GetOrganizationNamespaceAvailabilityRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetOrganizationNamespaceAvailabilityResponse>,
            tonic::Status,
        >;
        async fn update_organization(
            &self,
            request: tonic::Request<super::UpdateOrganizationRequest>,
        ) -> std::result::Result<
            tonic::Response<super::UpdateOrganizationResponse>,
            tonic::Status,
        >;
        async fn delete_organization(
            &self,
            request: tonic::Request<super::DeleteOrganizationRequest>,
        ) -> std::result::Result<
            tonic::Response<super::DeleteOrganizationResponse>,
            tonic::Status,
        >;
        async fn list_organization_members(
            &self,
            request: tonic::Request<super::ListOrganizationMembersRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListOrganizationMembersResponse>,
            tonic::Status,
        >;
        async fn create_organization_invite(
            &self,
            request: tonic::Request<super::CreateOrganizationInviteRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CreateOrganizationInviteResponse>,
            tonic::Status,
        >;
        async fn update_organization_invite_authorizations(
            &self,
            request: tonic::Request<super::UpdateOrganizationInviteAuthorizationsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::UpdateOrganizationInviteAuthorizationsResponse>,
            tonic::Status,
        >;
        async fn delete_organization_member(
            &self,
            request: tonic::Request<super::DeleteOrganizationMemberRequest>,
        ) -> std::result::Result<
            tonic::Response<super::DeleteOrganizationMemberResponse>,
            tonic::Status,
        >;
        async fn delete_organization_invite(
            &self,
            request: tonic::Request<super::DeleteOrganizationInviteRequest>,
        ) -> std::result::Result<
            tonic::Response<super::DeleteOrganizationInviteResponse>,
            tonic::Status,
        >;
        async fn resend_organization_invite(
            &self,
            request: tonic::Request<super::ResendOrganizationInviteRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ResendOrganizationInviteResponse>,
            tonic::Status,
        >;
        async fn create_location(
            &self,
            request: tonic::Request<super::CreateLocationRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CreateLocationResponse>,
            tonic::Status,
        >;
        async fn get_location(
            &self,
            request: tonic::Request<super::GetLocationRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetLocationResponse>,
            tonic::Status,
        >;
        async fn update_location(
            &self,
            request: tonic::Request<super::UpdateLocationRequest>,
        ) -> std::result::Result<
            tonic::Response<super::UpdateLocationResponse>,
            tonic::Status,
        >;
        async fn delete_location(
            &self,
            request: tonic::Request<super::DeleteLocationRequest>,
        ) -> std::result::Result<
            tonic::Response<super::DeleteLocationResponse>,
            tonic::Status,
        >;
        async fn list_locations(
            &self,
            request: tonic::Request<super::ListLocationsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListLocationsResponse>,
            tonic::Status,
        >;
        async fn share_location(
            &self,
            request: tonic::Request<super::ShareLocationRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ShareLocationResponse>,
            tonic::Status,
        >;
        async fn unshare_location(
            &self,
            request: tonic::Request<super::UnshareLocationRequest>,
        ) -> std::result::Result<
            tonic::Response<super::UnshareLocationResponse>,
            tonic::Status,
        >;
        async fn location_auth(
            &self,
            request: tonic::Request<super::LocationAuthRequest>,
        ) -> std::result::Result<
            tonic::Response<super::LocationAuthResponse>,
            tonic::Status,
        >;
        async fn create_location_secret(
            &self,
            request: tonic::Request<super::CreateLocationSecretRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CreateLocationSecretResponse>,
            tonic::Status,
        >;
        async fn delete_location_secret(
            &self,
            request: tonic::Request<super::DeleteLocationSecretRequest>,
        ) -> std::result::Result<
            tonic::Response<super::DeleteLocationSecretResponse>,
            tonic::Status,
        >;
        async fn get_robot(
            &self,
            request: tonic::Request<super::GetRobotRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetRobotResponse>,
            tonic::Status,
        >;
        async fn get_rover_rental_robots(
            &self,
            request: tonic::Request<super::GetRoverRentalRobotsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetRoverRentalRobotsResponse>,
            tonic::Status,
        >;
        async fn get_robot_parts(
            &self,
            request: tonic::Request<super::GetRobotPartsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetRobotPartsResponse>,
            tonic::Status,
        >;
        async fn get_robot_part(
            &self,
            request: tonic::Request<super::GetRobotPartRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetRobotPartResponse>,
            tonic::Status,
        >;
        async fn get_robot_part_logs(
            &self,
            request: tonic::Request<super::GetRobotPartLogsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetRobotPartLogsResponse>,
            tonic::Status,
        >;
        /// Server streaming response type for the TailRobotPartLogs method.
        type TailRobotPartLogsStream: futures_core::Stream<
                Item = std::result::Result<
                    super::TailRobotPartLogsResponse,
                    tonic::Status,
                >,
            >
            + Send
            + 'static;
        async fn tail_robot_part_logs(
            &self,
            request: tonic::Request<super::TailRobotPartLogsRequest>,
        ) -> std::result::Result<
            tonic::Response<Self::TailRobotPartLogsStream>,
            tonic::Status,
        >;
        async fn get_robot_part_history(
            &self,
            request: tonic::Request<super::GetRobotPartHistoryRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetRobotPartHistoryResponse>,
            tonic::Status,
        >;
        async fn update_robot_part(
            &self,
            request: tonic::Request<super::UpdateRobotPartRequest>,
        ) -> std::result::Result<
            tonic::Response<super::UpdateRobotPartResponse>,
            tonic::Status,
        >;
        async fn new_robot_part(
            &self,
            request: tonic::Request<super::NewRobotPartRequest>,
        ) -> std::result::Result<
            tonic::Response<super::NewRobotPartResponse>,
            tonic::Status,
        >;
        async fn delete_robot_part(
            &self,
            request: tonic::Request<super::DeleteRobotPartRequest>,
        ) -> std::result::Result<
            tonic::Response<super::DeleteRobotPartResponse>,
            tonic::Status,
        >;
        async fn get_robot_api_keys(
            &self,
            request: tonic::Request<super::GetRobotApiKeysRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetRobotApiKeysResponse>,
            tonic::Status,
        >;
        async fn mark_part_as_main(
            &self,
            request: tonic::Request<super::MarkPartAsMainRequest>,
        ) -> std::result::Result<
            tonic::Response<super::MarkPartAsMainResponse>,
            tonic::Status,
        >;
        async fn mark_part_for_restart(
            &self,
            request: tonic::Request<super::MarkPartForRestartRequest>,
        ) -> std::result::Result<
            tonic::Response<super::MarkPartForRestartResponse>,
            tonic::Status,
        >;
        async fn create_robot_part_secret(
            &self,
            request: tonic::Request<super::CreateRobotPartSecretRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CreateRobotPartSecretResponse>,
            tonic::Status,
        >;
        async fn delete_robot_part_secret(
            &self,
            request: tonic::Request<super::DeleteRobotPartSecretRequest>,
        ) -> std::result::Result<
            tonic::Response<super::DeleteRobotPartSecretResponse>,
            tonic::Status,
        >;
        async fn list_robots(
            &self,
            request: tonic::Request<super::ListRobotsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListRobotsResponse>,
            tonic::Status,
        >;
        async fn new_robot(
            &self,
            request: tonic::Request<super::NewRobotRequest>,
        ) -> std::result::Result<
            tonic::Response<super::NewRobotResponse>,
            tonic::Status,
        >;
        async fn update_robot(
            &self,
            request: tonic::Request<super::UpdateRobotRequest>,
        ) -> std::result::Result<
            tonic::Response<super::UpdateRobotResponse>,
            tonic::Status,
        >;
        async fn delete_robot(
            &self,
            request: tonic::Request<super::DeleteRobotRequest>,
        ) -> std::result::Result<
            tonic::Response<super::DeleteRobotResponse>,
            tonic::Status,
        >;
        async fn list_fragments(
            &self,
            request: tonic::Request<super::ListFragmentsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListFragmentsResponse>,
            tonic::Status,
        >;
        async fn get_fragment(
            &self,
            request: tonic::Request<super::GetFragmentRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetFragmentResponse>,
            tonic::Status,
        >;
        async fn create_fragment(
            &self,
            request: tonic::Request<super::CreateFragmentRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CreateFragmentResponse>,
            tonic::Status,
        >;
        async fn update_fragment(
            &self,
            request: tonic::Request<super::UpdateFragmentRequest>,
        ) -> std::result::Result<
            tonic::Response<super::UpdateFragmentResponse>,
            tonic::Status,
        >;
        async fn delete_fragment(
            &self,
            request: tonic::Request<super::DeleteFragmentRequest>,
        ) -> std::result::Result<
            tonic::Response<super::DeleteFragmentResponse>,
            tonic::Status,
        >;
        async fn add_role(
            &self,
            request: tonic::Request<super::AddRoleRequest>,
        ) -> std::result::Result<tonic::Response<super::AddRoleResponse>, tonic::Status>;
        async fn remove_role(
            &self,
            request: tonic::Request<super::RemoveRoleRequest>,
        ) -> std::result::Result<
            tonic::Response<super::RemoveRoleResponse>,
            tonic::Status,
        >;
        async fn change_role(
            &self,
            request: tonic::Request<super::ChangeRoleRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ChangeRoleResponse>,
            tonic::Status,
        >;
        async fn list_authorizations(
            &self,
            request: tonic::Request<super::ListAuthorizationsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListAuthorizationsResponse>,
            tonic::Status,
        >;
        async fn check_permissions(
            &self,
            request: tonic::Request<super::CheckPermissionsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CheckPermissionsResponse>,
            tonic::Status,
        >;
        async fn get_registry_item(
            &self,
            request: tonic::Request<super::GetRegistryItemRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetRegistryItemResponse>,
            tonic::Status,
        >;
        async fn create_registry_item(
            &self,
            request: tonic::Request<super::CreateRegistryItemRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CreateRegistryItemResponse>,
            tonic::Status,
        >;
        async fn update_registry_item(
            &self,
            request: tonic::Request<super::UpdateRegistryItemRequest>,
        ) -> std::result::Result<
            tonic::Response<super::UpdateRegistryItemResponse>,
            tonic::Status,
        >;
        async fn list_registry_items(
            &self,
            request: tonic::Request<super::ListRegistryItemsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListRegistryItemsResponse>,
            tonic::Status,
        >;
        async fn delete_registry_item(
            &self,
            request: tonic::Request<super::DeleteRegistryItemRequest>,
        ) -> std::result::Result<
            tonic::Response<super::DeleteRegistryItemResponse>,
            tonic::Status,
        >;
        async fn create_module(
            &self,
            request: tonic::Request<super::CreateModuleRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CreateModuleResponse>,
            tonic::Status,
        >;
        async fn update_module(
            &self,
            request: tonic::Request<super::UpdateModuleRequest>,
        ) -> std::result::Result<
            tonic::Response<super::UpdateModuleResponse>,
            tonic::Status,
        >;
        async fn upload_module_file(
            &self,
            request: tonic::Request<tonic::Streaming<super::UploadModuleFileRequest>>,
        ) -> std::result::Result<
            tonic::Response<super::UploadModuleFileResponse>,
            tonic::Status,
        >;
        async fn get_module(
            &self,
            request: tonic::Request<super::GetModuleRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetModuleResponse>,
            tonic::Status,
        >;
        async fn list_modules(
            &self,
            request: tonic::Request<super::ListModulesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListModulesResponse>,
            tonic::Status,
        >;
        async fn create_key(
            &self,
            request: tonic::Request<super::CreateKeyRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CreateKeyResponse>,
            tonic::Status,
        >;
        async fn delete_key(
            &self,
            request: tonic::Request<super::DeleteKeyRequest>,
        ) -> std::result::Result<
            tonic::Response<super::DeleteKeyResponse>,
            tonic::Status,
        >;
        async fn list_keys(
            &self,
            request: tonic::Request<super::ListKeysRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListKeysResponse>,
            tonic::Status,
        >;
        async fn rotate_key(
            &self,
            request: tonic::Request<super::RotateKeyRequest>,
        ) -> std::result::Result<
            tonic::Response<super::RotateKeyResponse>,
            tonic::Status,
        >;
        async fn create_key_from_existing_key_authorizations(
            &self,
            request: tonic::Request<super::CreateKeyFromExistingKeyAuthorizationsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CreateKeyFromExistingKeyAuthorizationsResponse>,
            tonic::Status,
        >;
    }
    #[derive(Debug)]
    pub struct AppServiceServer<T: AppService> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
        max_decoding_message_size: Option<usize>,
        max_encoding_message_size: Option<usize>,
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
        ) -> Poll<std::result::Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            let inner = self.inner.clone();
            match req.uri().path() {
                "/viam.app.v1.AppService/GetUserIDByEmail" => {
                    #[allow(non_camel_case_types)]
                    struct GetUserIDByEmailSvc<T: AppService>(pub Arc<T>);
                    impl<
                        T: AppService,
                    > tonic::server::UnaryService<super::GetUserIdByEmailRequest>
                    for GetUserIDByEmailSvc<T> {
                        type Response = super::GetUserIdByEmailResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetUserIdByEmailRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).get_user_id_by_email(request).await
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
                        let method = GetUserIDByEmailSvc(inner);
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
                "/viam.app.v1.AppService/CreateOrganization" => {
                    #[allow(non_camel_case_types)]
                    struct CreateOrganizationSvc<T: AppService>(pub Arc<T>);
                    impl<
                        T: AppService,
                    > tonic::server::UnaryService<super::CreateOrganizationRequest>
                    for CreateOrganizationSvc<T> {
                        type Response = super::CreateOrganizationResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CreateOrganizationRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).create_organization(request).await
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
                        let method = CreateOrganizationSvc(inner);
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
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).list_organizations(request).await
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
                        let method = ListOrganizationsSvc(inner);
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
                "/viam.app.v1.AppService/GetOrganizationsWithAccessToLocation" => {
                    #[allow(non_camel_case_types)]
                    struct GetOrganizationsWithAccessToLocationSvc<T: AppService>(
                        pub Arc<T>,
                    );
                    impl<
                        T: AppService,
                    > tonic::server::UnaryService<
                        super::GetOrganizationsWithAccessToLocationRequest,
                    > for GetOrganizationsWithAccessToLocationSvc<T> {
                        type Response = super::GetOrganizationsWithAccessToLocationResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::GetOrganizationsWithAccessToLocationRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner)
                                    .get_organizations_with_access_to_location(request)
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
                        let method = GetOrganizationsWithAccessToLocationSvc(inner);
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
                "/viam.app.v1.AppService/ListOrganizationsByUser" => {
                    #[allow(non_camel_case_types)]
                    struct ListOrganizationsByUserSvc<T: AppService>(pub Arc<T>);
                    impl<
                        T: AppService,
                    > tonic::server::UnaryService<super::ListOrganizationsByUserRequest>
                    for ListOrganizationsByUserSvc<T> {
                        type Response = super::ListOrganizationsByUserResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::ListOrganizationsByUserRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).list_organizations_by_user(request).await
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
                        let method = ListOrganizationsByUserSvc(inner);
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
                "/viam.app.v1.AppService/GetOrganization" => {
                    #[allow(non_camel_case_types)]
                    struct GetOrganizationSvc<T: AppService>(pub Arc<T>);
                    impl<
                        T: AppService,
                    > tonic::server::UnaryService<super::GetOrganizationRequest>
                    for GetOrganizationSvc<T> {
                        type Response = super::GetOrganizationResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetOrganizationRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).get_organization(request).await
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
                        let method = GetOrganizationSvc(inner);
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
                "/viam.app.v1.AppService/GetOrganizationNamespaceAvailability" => {
                    #[allow(non_camel_case_types)]
                    struct GetOrganizationNamespaceAvailabilitySvc<T: AppService>(
                        pub Arc<T>,
                    );
                    impl<
                        T: AppService,
                    > tonic::server::UnaryService<
                        super::GetOrganizationNamespaceAvailabilityRequest,
                    > for GetOrganizationNamespaceAvailabilitySvc<T> {
                        type Response = super::GetOrganizationNamespaceAvailabilityResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::GetOrganizationNamespaceAvailabilityRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner)
                                    .get_organization_namespace_availability(request)
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
                        let method = GetOrganizationNamespaceAvailabilitySvc(inner);
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
                "/viam.app.v1.AppService/UpdateOrganization" => {
                    #[allow(non_camel_case_types)]
                    struct UpdateOrganizationSvc<T: AppService>(pub Arc<T>);
                    impl<
                        T: AppService,
                    > tonic::server::UnaryService<super::UpdateOrganizationRequest>
                    for UpdateOrganizationSvc<T> {
                        type Response = super::UpdateOrganizationResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::UpdateOrganizationRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).update_organization(request).await
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
                        let method = UpdateOrganizationSvc(inner);
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
                "/viam.app.v1.AppService/DeleteOrganization" => {
                    #[allow(non_camel_case_types)]
                    struct DeleteOrganizationSvc<T: AppService>(pub Arc<T>);
                    impl<
                        T: AppService,
                    > tonic::server::UnaryService<super::DeleteOrganizationRequest>
                    for DeleteOrganizationSvc<T> {
                        type Response = super::DeleteOrganizationResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::DeleteOrganizationRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).delete_organization(request).await
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
                        let method = DeleteOrganizationSvc(inner);
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
                "/viam.app.v1.AppService/ListOrganizationMembers" => {
                    #[allow(non_camel_case_types)]
                    struct ListOrganizationMembersSvc<T: AppService>(pub Arc<T>);
                    impl<
                        T: AppService,
                    > tonic::server::UnaryService<super::ListOrganizationMembersRequest>
                    for ListOrganizationMembersSvc<T> {
                        type Response = super::ListOrganizationMembersResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::ListOrganizationMembersRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).list_organization_members(request).await
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
                        let method = ListOrganizationMembersSvc(inner);
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
                "/viam.app.v1.AppService/CreateOrganizationInvite" => {
                    #[allow(non_camel_case_types)]
                    struct CreateOrganizationInviteSvc<T: AppService>(pub Arc<T>);
                    impl<
                        T: AppService,
                    > tonic::server::UnaryService<super::CreateOrganizationInviteRequest>
                    for CreateOrganizationInviteSvc<T> {
                        type Response = super::CreateOrganizationInviteResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::CreateOrganizationInviteRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).create_organization_invite(request).await
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
                        let method = CreateOrganizationInviteSvc(inner);
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
                "/viam.app.v1.AppService/UpdateOrganizationInviteAuthorizations" => {
                    #[allow(non_camel_case_types)]
                    struct UpdateOrganizationInviteAuthorizationsSvc<T: AppService>(
                        pub Arc<T>,
                    );
                    impl<
                        T: AppService,
                    > tonic::server::UnaryService<
                        super::UpdateOrganizationInviteAuthorizationsRequest,
                    > for UpdateOrganizationInviteAuthorizationsSvc<T> {
                        type Response = super::UpdateOrganizationInviteAuthorizationsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::UpdateOrganizationInviteAuthorizationsRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner)
                                    .update_organization_invite_authorizations(request)
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
                        let method = UpdateOrganizationInviteAuthorizationsSvc(inner);
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
                "/viam.app.v1.AppService/DeleteOrganizationMember" => {
                    #[allow(non_camel_case_types)]
                    struct DeleteOrganizationMemberSvc<T: AppService>(pub Arc<T>);
                    impl<
                        T: AppService,
                    > tonic::server::UnaryService<super::DeleteOrganizationMemberRequest>
                    for DeleteOrganizationMemberSvc<T> {
                        type Response = super::DeleteOrganizationMemberResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::DeleteOrganizationMemberRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).delete_organization_member(request).await
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
                        let method = DeleteOrganizationMemberSvc(inner);
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
                "/viam.app.v1.AppService/DeleteOrganizationInvite" => {
                    #[allow(non_camel_case_types)]
                    struct DeleteOrganizationInviteSvc<T: AppService>(pub Arc<T>);
                    impl<
                        T: AppService,
                    > tonic::server::UnaryService<super::DeleteOrganizationInviteRequest>
                    for DeleteOrganizationInviteSvc<T> {
                        type Response = super::DeleteOrganizationInviteResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::DeleteOrganizationInviteRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).delete_organization_invite(request).await
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
                        let method = DeleteOrganizationInviteSvc(inner);
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
                "/viam.app.v1.AppService/ResendOrganizationInvite" => {
                    #[allow(non_camel_case_types)]
                    struct ResendOrganizationInviteSvc<T: AppService>(pub Arc<T>);
                    impl<
                        T: AppService,
                    > tonic::server::UnaryService<super::ResendOrganizationInviteRequest>
                    for ResendOrganizationInviteSvc<T> {
                        type Response = super::ResendOrganizationInviteResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::ResendOrganizationInviteRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).resend_organization_invite(request).await
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
                        let method = ResendOrganizationInviteSvc(inner);
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
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).create_location(request).await
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
                        let method = CreateLocationSvc(inner);
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
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).get_location(request).await
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
                        let method = GetLocationSvc(inner);
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
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).update_location(request).await
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
                        let method = UpdateLocationSvc(inner);
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
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).delete_location(request).await
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
                        let method = DeleteLocationSvc(inner);
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
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).list_locations(request).await
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
                        let method = ListLocationsSvc(inner);
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
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).share_location(request).await
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
                        let method = ShareLocationSvc(inner);
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
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).unshare_location(request).await
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
                        let method = UnshareLocationSvc(inner);
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
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).location_auth(request).await
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
                        let method = LocationAuthSvc(inner);
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
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).create_location_secret(request).await
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
                        let method = CreateLocationSecretSvc(inner);
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
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).delete_location_secret(request).await
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
                        let method = DeleteLocationSecretSvc(inner);
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
                            let inner = Arc::clone(&self.0);
                            let fut = async move { (*inner).get_robot(request).await };
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
                        let method = GetRobotSvc(inner);
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
                "/viam.app.v1.AppService/GetRoverRentalRobots" => {
                    #[allow(non_camel_case_types)]
                    struct GetRoverRentalRobotsSvc<T: AppService>(pub Arc<T>);
                    impl<
                        T: AppService,
                    > tonic::server::UnaryService<super::GetRoverRentalRobotsRequest>
                    for GetRoverRentalRobotsSvc<T> {
                        type Response = super::GetRoverRentalRobotsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetRoverRentalRobotsRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).get_rover_rental_robots(request).await
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
                        let method = GetRoverRentalRobotsSvc(inner);
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
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).get_robot_parts(request).await
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
                        let method = GetRobotPartsSvc(inner);
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
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).get_robot_part(request).await
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
                        let method = GetRobotPartSvc(inner);
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
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).get_robot_part_logs(request).await
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
                        let method = GetRobotPartLogsSvc(inner);
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
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).tail_robot_part_logs(request).await
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
                        let method = TailRobotPartLogsSvc(inner);
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
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).get_robot_part_history(request).await
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
                        let method = GetRobotPartHistorySvc(inner);
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
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).update_robot_part(request).await
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
                        let method = UpdateRobotPartSvc(inner);
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
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).new_robot_part(request).await
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
                        let method = NewRobotPartSvc(inner);
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
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).delete_robot_part(request).await
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
                        let method = DeleteRobotPartSvc(inner);
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
                "/viam.app.v1.AppService/GetRobotAPIKeys" => {
                    #[allow(non_camel_case_types)]
                    struct GetRobotAPIKeysSvc<T: AppService>(pub Arc<T>);
                    impl<
                        T: AppService,
                    > tonic::server::UnaryService<super::GetRobotApiKeysRequest>
                    for GetRobotAPIKeysSvc<T> {
                        type Response = super::GetRobotApiKeysResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetRobotApiKeysRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).get_robot_api_keys(request).await
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
                        let method = GetRobotAPIKeysSvc(inner);
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
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).mark_part_as_main(request).await
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
                        let method = MarkPartAsMainSvc(inner);
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
                "/viam.app.v1.AppService/MarkPartForRestart" => {
                    #[allow(non_camel_case_types)]
                    struct MarkPartForRestartSvc<T: AppService>(pub Arc<T>);
                    impl<
                        T: AppService,
                    > tonic::server::UnaryService<super::MarkPartForRestartRequest>
                    for MarkPartForRestartSvc<T> {
                        type Response = super::MarkPartForRestartResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MarkPartForRestartRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).mark_part_for_restart(request).await
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
                        let method = MarkPartForRestartSvc(inner);
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
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).create_robot_part_secret(request).await
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
                        let method = CreateRobotPartSecretSvc(inner);
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
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).delete_robot_part_secret(request).await
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
                        let method = DeleteRobotPartSecretSvc(inner);
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
                            let inner = Arc::clone(&self.0);
                            let fut = async move { (*inner).list_robots(request).await };
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
                        let method = ListRobotsSvc(inner);
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
                            let inner = Arc::clone(&self.0);
                            let fut = async move { (*inner).new_robot(request).await };
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
                        let method = NewRobotSvc(inner);
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
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).update_robot(request).await
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
                        let method = UpdateRobotSvc(inner);
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
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).delete_robot(request).await
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
                        let method = DeleteRobotSvc(inner);
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
                "/viam.app.v1.AppService/ListFragments" => {
                    #[allow(non_camel_case_types)]
                    struct ListFragmentsSvc<T: AppService>(pub Arc<T>);
                    impl<
                        T: AppService,
                    > tonic::server::UnaryService<super::ListFragmentsRequest>
                    for ListFragmentsSvc<T> {
                        type Response = super::ListFragmentsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListFragmentsRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).list_fragments(request).await
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
                        let method = ListFragmentsSvc(inner);
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
                "/viam.app.v1.AppService/GetFragment" => {
                    #[allow(non_camel_case_types)]
                    struct GetFragmentSvc<T: AppService>(pub Arc<T>);
                    impl<
                        T: AppService,
                    > tonic::server::UnaryService<super::GetFragmentRequest>
                    for GetFragmentSvc<T> {
                        type Response = super::GetFragmentResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetFragmentRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).get_fragment(request).await
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
                        let method = GetFragmentSvc(inner);
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
                "/viam.app.v1.AppService/CreateFragment" => {
                    #[allow(non_camel_case_types)]
                    struct CreateFragmentSvc<T: AppService>(pub Arc<T>);
                    impl<
                        T: AppService,
                    > tonic::server::UnaryService<super::CreateFragmentRequest>
                    for CreateFragmentSvc<T> {
                        type Response = super::CreateFragmentResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CreateFragmentRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).create_fragment(request).await
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
                        let method = CreateFragmentSvc(inner);
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
                "/viam.app.v1.AppService/UpdateFragment" => {
                    #[allow(non_camel_case_types)]
                    struct UpdateFragmentSvc<T: AppService>(pub Arc<T>);
                    impl<
                        T: AppService,
                    > tonic::server::UnaryService<super::UpdateFragmentRequest>
                    for UpdateFragmentSvc<T> {
                        type Response = super::UpdateFragmentResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::UpdateFragmentRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).update_fragment(request).await
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
                        let method = UpdateFragmentSvc(inner);
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
                "/viam.app.v1.AppService/DeleteFragment" => {
                    #[allow(non_camel_case_types)]
                    struct DeleteFragmentSvc<T: AppService>(pub Arc<T>);
                    impl<
                        T: AppService,
                    > tonic::server::UnaryService<super::DeleteFragmentRequest>
                    for DeleteFragmentSvc<T> {
                        type Response = super::DeleteFragmentResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::DeleteFragmentRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).delete_fragment(request).await
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
                        let method = DeleteFragmentSvc(inner);
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
                "/viam.app.v1.AppService/AddRole" => {
                    #[allow(non_camel_case_types)]
                    struct AddRoleSvc<T: AppService>(pub Arc<T>);
                    impl<
                        T: AppService,
                    > tonic::server::UnaryService<super::AddRoleRequest>
                    for AddRoleSvc<T> {
                        type Response = super::AddRoleResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::AddRoleRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move { (*inner).add_role(request).await };
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
                        let method = AddRoleSvc(inner);
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
                "/viam.app.v1.AppService/RemoveRole" => {
                    #[allow(non_camel_case_types)]
                    struct RemoveRoleSvc<T: AppService>(pub Arc<T>);
                    impl<
                        T: AppService,
                    > tonic::server::UnaryService<super::RemoveRoleRequest>
                    for RemoveRoleSvc<T> {
                        type Response = super::RemoveRoleResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::RemoveRoleRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move { (*inner).remove_role(request).await };
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
                        let method = RemoveRoleSvc(inner);
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
                "/viam.app.v1.AppService/ChangeRole" => {
                    #[allow(non_camel_case_types)]
                    struct ChangeRoleSvc<T: AppService>(pub Arc<T>);
                    impl<
                        T: AppService,
                    > tonic::server::UnaryService<super::ChangeRoleRequest>
                    for ChangeRoleSvc<T> {
                        type Response = super::ChangeRoleResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ChangeRoleRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move { (*inner).change_role(request).await };
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
                        let method = ChangeRoleSvc(inner);
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
                "/viam.app.v1.AppService/ListAuthorizations" => {
                    #[allow(non_camel_case_types)]
                    struct ListAuthorizationsSvc<T: AppService>(pub Arc<T>);
                    impl<
                        T: AppService,
                    > tonic::server::UnaryService<super::ListAuthorizationsRequest>
                    for ListAuthorizationsSvc<T> {
                        type Response = super::ListAuthorizationsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListAuthorizationsRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).list_authorizations(request).await
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
                        let method = ListAuthorizationsSvc(inner);
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
                "/viam.app.v1.AppService/CheckPermissions" => {
                    #[allow(non_camel_case_types)]
                    struct CheckPermissionsSvc<T: AppService>(pub Arc<T>);
                    impl<
                        T: AppService,
                    > tonic::server::UnaryService<super::CheckPermissionsRequest>
                    for CheckPermissionsSvc<T> {
                        type Response = super::CheckPermissionsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CheckPermissionsRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).check_permissions(request).await
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
                        let method = CheckPermissionsSvc(inner);
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
                "/viam.app.v1.AppService/GetRegistryItem" => {
                    #[allow(non_camel_case_types)]
                    struct GetRegistryItemSvc<T: AppService>(pub Arc<T>);
                    impl<
                        T: AppService,
                    > tonic::server::UnaryService<super::GetRegistryItemRequest>
                    for GetRegistryItemSvc<T> {
                        type Response = super::GetRegistryItemResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetRegistryItemRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).get_registry_item(request).await
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
                        let method = GetRegistryItemSvc(inner);
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
                "/viam.app.v1.AppService/CreateRegistryItem" => {
                    #[allow(non_camel_case_types)]
                    struct CreateRegistryItemSvc<T: AppService>(pub Arc<T>);
                    impl<
                        T: AppService,
                    > tonic::server::UnaryService<super::CreateRegistryItemRequest>
                    for CreateRegistryItemSvc<T> {
                        type Response = super::CreateRegistryItemResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CreateRegistryItemRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).create_registry_item(request).await
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
                        let method = CreateRegistryItemSvc(inner);
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
                "/viam.app.v1.AppService/UpdateRegistryItem" => {
                    #[allow(non_camel_case_types)]
                    struct UpdateRegistryItemSvc<T: AppService>(pub Arc<T>);
                    impl<
                        T: AppService,
                    > tonic::server::UnaryService<super::UpdateRegistryItemRequest>
                    for UpdateRegistryItemSvc<T> {
                        type Response = super::UpdateRegistryItemResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::UpdateRegistryItemRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).update_registry_item(request).await
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
                        let method = UpdateRegistryItemSvc(inner);
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
                "/viam.app.v1.AppService/ListRegistryItems" => {
                    #[allow(non_camel_case_types)]
                    struct ListRegistryItemsSvc<T: AppService>(pub Arc<T>);
                    impl<
                        T: AppService,
                    > tonic::server::UnaryService<super::ListRegistryItemsRequest>
                    for ListRegistryItemsSvc<T> {
                        type Response = super::ListRegistryItemsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListRegistryItemsRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).list_registry_items(request).await
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
                        let method = ListRegistryItemsSvc(inner);
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
                "/viam.app.v1.AppService/DeleteRegistryItem" => {
                    #[allow(non_camel_case_types)]
                    struct DeleteRegistryItemSvc<T: AppService>(pub Arc<T>);
                    impl<
                        T: AppService,
                    > tonic::server::UnaryService<super::DeleteRegistryItemRequest>
                    for DeleteRegistryItemSvc<T> {
                        type Response = super::DeleteRegistryItemResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::DeleteRegistryItemRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).delete_registry_item(request).await
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
                        let method = DeleteRegistryItemSvc(inner);
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
                "/viam.app.v1.AppService/CreateModule" => {
                    #[allow(non_camel_case_types)]
                    struct CreateModuleSvc<T: AppService>(pub Arc<T>);
                    impl<
                        T: AppService,
                    > tonic::server::UnaryService<super::CreateModuleRequest>
                    for CreateModuleSvc<T> {
                        type Response = super::CreateModuleResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CreateModuleRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).create_module(request).await
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
                        let method = CreateModuleSvc(inner);
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
                "/viam.app.v1.AppService/UpdateModule" => {
                    #[allow(non_camel_case_types)]
                    struct UpdateModuleSvc<T: AppService>(pub Arc<T>);
                    impl<
                        T: AppService,
                    > tonic::server::UnaryService<super::UpdateModuleRequest>
                    for UpdateModuleSvc<T> {
                        type Response = super::UpdateModuleResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::UpdateModuleRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).update_module(request).await
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
                        let method = UpdateModuleSvc(inner);
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
                "/viam.app.v1.AppService/UploadModuleFile" => {
                    #[allow(non_camel_case_types)]
                    struct UploadModuleFileSvc<T: AppService>(pub Arc<T>);
                    impl<
                        T: AppService,
                    > tonic::server::ClientStreamingService<
                        super::UploadModuleFileRequest,
                    > for UploadModuleFileSvc<T> {
                        type Response = super::UploadModuleFileResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                tonic::Streaming<super::UploadModuleFileRequest>,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).upload_module_file(request).await
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
                        let method = UploadModuleFileSvc(inner);
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
                        let res = grpc.client_streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/viam.app.v1.AppService/GetModule" => {
                    #[allow(non_camel_case_types)]
                    struct GetModuleSvc<T: AppService>(pub Arc<T>);
                    impl<
                        T: AppService,
                    > tonic::server::UnaryService<super::GetModuleRequest>
                    for GetModuleSvc<T> {
                        type Response = super::GetModuleResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetModuleRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move { (*inner).get_module(request).await };
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
                        let method = GetModuleSvc(inner);
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
                "/viam.app.v1.AppService/ListModules" => {
                    #[allow(non_camel_case_types)]
                    struct ListModulesSvc<T: AppService>(pub Arc<T>);
                    impl<
                        T: AppService,
                    > tonic::server::UnaryService<super::ListModulesRequest>
                    for ListModulesSvc<T> {
                        type Response = super::ListModulesResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListModulesRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).list_modules(request).await
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
                        let method = ListModulesSvc(inner);
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
                "/viam.app.v1.AppService/CreateKey" => {
                    #[allow(non_camel_case_types)]
                    struct CreateKeySvc<T: AppService>(pub Arc<T>);
                    impl<
                        T: AppService,
                    > tonic::server::UnaryService<super::CreateKeyRequest>
                    for CreateKeySvc<T> {
                        type Response = super::CreateKeyResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CreateKeyRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move { (*inner).create_key(request).await };
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
                        let method = CreateKeySvc(inner);
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
                "/viam.app.v1.AppService/DeleteKey" => {
                    #[allow(non_camel_case_types)]
                    struct DeleteKeySvc<T: AppService>(pub Arc<T>);
                    impl<
                        T: AppService,
                    > tonic::server::UnaryService<super::DeleteKeyRequest>
                    for DeleteKeySvc<T> {
                        type Response = super::DeleteKeyResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::DeleteKeyRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move { (*inner).delete_key(request).await };
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
                        let method = DeleteKeySvc(inner);
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
                "/viam.app.v1.AppService/ListKeys" => {
                    #[allow(non_camel_case_types)]
                    struct ListKeysSvc<T: AppService>(pub Arc<T>);
                    impl<
                        T: AppService,
                    > tonic::server::UnaryService<super::ListKeysRequest>
                    for ListKeysSvc<T> {
                        type Response = super::ListKeysResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListKeysRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move { (*inner).list_keys(request).await };
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
                        let method = ListKeysSvc(inner);
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
                "/viam.app.v1.AppService/RotateKey" => {
                    #[allow(non_camel_case_types)]
                    struct RotateKeySvc<T: AppService>(pub Arc<T>);
                    impl<
                        T: AppService,
                    > tonic::server::UnaryService<super::RotateKeyRequest>
                    for RotateKeySvc<T> {
                        type Response = super::RotateKeyResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::RotateKeyRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move { (*inner).rotate_key(request).await };
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
                        let method = RotateKeySvc(inner);
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
                "/viam.app.v1.AppService/CreateKeyFromExistingKeyAuthorizations" => {
                    #[allow(non_camel_case_types)]
                    struct CreateKeyFromExistingKeyAuthorizationsSvc<T: AppService>(
                        pub Arc<T>,
                    );
                    impl<
                        T: AppService,
                    > tonic::server::UnaryService<
                        super::CreateKeyFromExistingKeyAuthorizationsRequest,
                    > for CreateKeyFromExistingKeyAuthorizationsSvc<T> {
                        type Response = super::CreateKeyFromExistingKeyAuthorizationsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::CreateKeyFromExistingKeyAuthorizationsRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner)
                                    .create_key_from_existing_key_authorizations(request)
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
                        let method = CreateKeyFromExistingKeyAuthorizationsSvc(inner);
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
    impl<T: AppService> Clone for AppServiceServer<T> {
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
    impl<T: AppService> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(Arc::clone(&self.0))
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: AppService> tonic::server::NamedService for AppServiceServer<T> {
        const NAME: &'static str = "viam.app.v1.AppService";
    }
}
/// Generated client implementations.
pub mod billing_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    #[derive(Debug, Clone)]
    pub struct BillingServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl BillingServiceClient<tonic::transport::Channel> {
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
    impl<T> BillingServiceClient<T>
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
        ) -> BillingServiceClient<InterceptedService<T, F>>
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
            BillingServiceClient::new(InterceptedService::new(inner, interceptor))
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
        pub async fn get_current_month_usage(
            &mut self,
            request: impl tonic::IntoRequest<super::GetCurrentMonthUsageRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetCurrentMonthUsageResponse>,
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
                "/viam.app.v1.BillingService/GetCurrentMonthUsage",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("viam.app.v1.BillingService", "GetCurrentMonthUsage"),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_org_billing_information(
            &mut self,
            request: impl tonic::IntoRequest<super::GetOrgBillingInformationRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetOrgBillingInformationResponse>,
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
                "/viam.app.v1.BillingService/GetOrgBillingInformation",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "viam.app.v1.BillingService",
                        "GetOrgBillingInformation",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_invoices_summary(
            &mut self,
            request: impl tonic::IntoRequest<super::GetInvoicesSummaryRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetInvoicesSummaryResponse>,
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
                "/viam.app.v1.BillingService/GetInvoicesSummary",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("viam.app.v1.BillingService", "GetInvoicesSummary"),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_invoice_pdf(
            &mut self,
            request: impl tonic::IntoRequest<super::GetInvoicePdfRequest>,
        ) -> std::result::Result<
            tonic::Response<tonic::codec::Streaming<super::GetInvoicePdfResponse>>,
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
                "/viam.app.v1.BillingService/GetInvoicePdf",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("viam.app.v1.BillingService", "GetInvoicePdf"));
            self.inner.server_streaming(req, path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod billing_service_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with BillingServiceServer.
    #[async_trait]
    pub trait BillingService: Send + Sync + 'static {
        async fn get_current_month_usage(
            &self,
            request: tonic::Request<super::GetCurrentMonthUsageRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetCurrentMonthUsageResponse>,
            tonic::Status,
        >;
        async fn get_org_billing_information(
            &self,
            request: tonic::Request<super::GetOrgBillingInformationRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetOrgBillingInformationResponse>,
            tonic::Status,
        >;
        async fn get_invoices_summary(
            &self,
            request: tonic::Request<super::GetInvoicesSummaryRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetInvoicesSummaryResponse>,
            tonic::Status,
        >;
        /// Server streaming response type for the GetInvoicePdf method.
        type GetInvoicePdfStream: futures_core::Stream<
                Item = std::result::Result<super::GetInvoicePdfResponse, tonic::Status>,
            >
            + Send
            + 'static;
        async fn get_invoice_pdf(
            &self,
            request: tonic::Request<super::GetInvoicePdfRequest>,
        ) -> std::result::Result<
            tonic::Response<Self::GetInvoicePdfStream>,
            tonic::Status,
        >;
    }
    #[derive(Debug)]
    pub struct BillingServiceServer<T: BillingService> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
        max_decoding_message_size: Option<usize>,
        max_encoding_message_size: Option<usize>,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: BillingService> BillingServiceServer<T> {
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
    impl<T, B> tonic::codegen::Service<http::Request<B>> for BillingServiceServer<T>
    where
        T: BillingService,
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
                "/viam.app.v1.BillingService/GetCurrentMonthUsage" => {
                    #[allow(non_camel_case_types)]
                    struct GetCurrentMonthUsageSvc<T: BillingService>(pub Arc<T>);
                    impl<
                        T: BillingService,
                    > tonic::server::UnaryService<super::GetCurrentMonthUsageRequest>
                    for GetCurrentMonthUsageSvc<T> {
                        type Response = super::GetCurrentMonthUsageResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetCurrentMonthUsageRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).get_current_month_usage(request).await
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
                        let method = GetCurrentMonthUsageSvc(inner);
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
                "/viam.app.v1.BillingService/GetOrgBillingInformation" => {
                    #[allow(non_camel_case_types)]
                    struct GetOrgBillingInformationSvc<T: BillingService>(pub Arc<T>);
                    impl<
                        T: BillingService,
                    > tonic::server::UnaryService<super::GetOrgBillingInformationRequest>
                    for GetOrgBillingInformationSvc<T> {
                        type Response = super::GetOrgBillingInformationResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::GetOrgBillingInformationRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).get_org_billing_information(request).await
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
                        let method = GetOrgBillingInformationSvc(inner);
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
                "/viam.app.v1.BillingService/GetInvoicesSummary" => {
                    #[allow(non_camel_case_types)]
                    struct GetInvoicesSummarySvc<T: BillingService>(pub Arc<T>);
                    impl<
                        T: BillingService,
                    > tonic::server::UnaryService<super::GetInvoicesSummaryRequest>
                    for GetInvoicesSummarySvc<T> {
                        type Response = super::GetInvoicesSummaryResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetInvoicesSummaryRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).get_invoices_summary(request).await
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
                        let method = GetInvoicesSummarySvc(inner);
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
                "/viam.app.v1.BillingService/GetInvoicePdf" => {
                    #[allow(non_camel_case_types)]
                    struct GetInvoicePdfSvc<T: BillingService>(pub Arc<T>);
                    impl<
                        T: BillingService,
                    > tonic::server::ServerStreamingService<super::GetInvoicePdfRequest>
                    for GetInvoicePdfSvc<T> {
                        type Response = super::GetInvoicePdfResponse;
                        type ResponseStream = T::GetInvoicePdfStream;
                        type Future = BoxFuture<
                            tonic::Response<Self::ResponseStream>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetInvoicePdfRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).get_invoice_pdf(request).await
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
                        let method = GetInvoicePdfSvc(inner);
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
                        let res = grpc.server_streaming(method, req).await;
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
    impl<T: BillingService> Clone for BillingServiceServer<T> {
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
    impl<T: BillingService> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(Arc::clone(&self.0))
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: BillingService> tonic::server::NamedService for BillingServiceServer<T> {
        const NAME: &'static str = "viam.app.v1.BillingService";
    }
}
/// Generated client implementations.
pub mod end_user_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    #[derive(Debug, Clone)]
    pub struct EndUserServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl EndUserServiceClient<tonic::transport::Channel> {
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
    impl<T> EndUserServiceClient<T>
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
        ) -> EndUserServiceClient<InterceptedService<T, F>>
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
            EndUserServiceClient::new(InterceptedService::new(inner, interceptor))
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
        pub async fn is_legal_accepted(
            &mut self,
            request: impl tonic::IntoRequest<super::IsLegalAcceptedRequest>,
        ) -> std::result::Result<
            tonic::Response<super::IsLegalAcceptedResponse>,
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
                "/viam.app.v1.EndUserService/IsLegalAccepted",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("viam.app.v1.EndUserService", "IsLegalAccepted"),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn accept_legal(
            &mut self,
            request: impl tonic::IntoRequest<super::AcceptLegalRequest>,
        ) -> std::result::Result<
            tonic::Response<super::AcceptLegalResponse>,
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
                "/viam.app.v1.EndUserService/AcceptLegal",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("viam.app.v1.EndUserService", "AcceptLegal"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn register_auth_application(
            &mut self,
            request: impl tonic::IntoRequest<super::RegisterAuthApplicationRequest>,
        ) -> std::result::Result<
            tonic::Response<super::RegisterAuthApplicationResponse>,
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
                "/viam.app.v1.EndUserService/RegisterAuthApplication",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "viam.app.v1.EndUserService",
                        "RegisterAuthApplication",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn update_auth_application(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateAuthApplicationRequest>,
        ) -> std::result::Result<
            tonic::Response<super::UpdateAuthApplicationResponse>,
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
                "/viam.app.v1.EndUserService/UpdateAuthApplication",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "viam.app.v1.EndUserService",
                        "UpdateAuthApplication",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod end_user_service_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with EndUserServiceServer.
    #[async_trait]
    pub trait EndUserService: Send + Sync + 'static {
        async fn is_legal_accepted(
            &self,
            request: tonic::Request<super::IsLegalAcceptedRequest>,
        ) -> std::result::Result<
            tonic::Response<super::IsLegalAcceptedResponse>,
            tonic::Status,
        >;
        async fn accept_legal(
            &self,
            request: tonic::Request<super::AcceptLegalRequest>,
        ) -> std::result::Result<
            tonic::Response<super::AcceptLegalResponse>,
            tonic::Status,
        >;
        async fn register_auth_application(
            &self,
            request: tonic::Request<super::RegisterAuthApplicationRequest>,
        ) -> std::result::Result<
            tonic::Response<super::RegisterAuthApplicationResponse>,
            tonic::Status,
        >;
        async fn update_auth_application(
            &self,
            request: tonic::Request<super::UpdateAuthApplicationRequest>,
        ) -> std::result::Result<
            tonic::Response<super::UpdateAuthApplicationResponse>,
            tonic::Status,
        >;
    }
    #[derive(Debug)]
    pub struct EndUserServiceServer<T: EndUserService> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
        max_decoding_message_size: Option<usize>,
        max_encoding_message_size: Option<usize>,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: EndUserService> EndUserServiceServer<T> {
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
    impl<T, B> tonic::codegen::Service<http::Request<B>> for EndUserServiceServer<T>
    where
        T: EndUserService,
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
                "/viam.app.v1.EndUserService/IsLegalAccepted" => {
                    #[allow(non_camel_case_types)]
                    struct IsLegalAcceptedSvc<T: EndUserService>(pub Arc<T>);
                    impl<
                        T: EndUserService,
                    > tonic::server::UnaryService<super::IsLegalAcceptedRequest>
                    for IsLegalAcceptedSvc<T> {
                        type Response = super::IsLegalAcceptedResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::IsLegalAcceptedRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).is_legal_accepted(request).await
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
                        let method = IsLegalAcceptedSvc(inner);
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
                "/viam.app.v1.EndUserService/AcceptLegal" => {
                    #[allow(non_camel_case_types)]
                    struct AcceptLegalSvc<T: EndUserService>(pub Arc<T>);
                    impl<
                        T: EndUserService,
                    > tonic::server::UnaryService<super::AcceptLegalRequest>
                    for AcceptLegalSvc<T> {
                        type Response = super::AcceptLegalResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::AcceptLegalRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).accept_legal(request).await
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
                        let method = AcceptLegalSvc(inner);
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
                "/viam.app.v1.EndUserService/RegisterAuthApplication" => {
                    #[allow(non_camel_case_types)]
                    struct RegisterAuthApplicationSvc<T: EndUserService>(pub Arc<T>);
                    impl<
                        T: EndUserService,
                    > tonic::server::UnaryService<super::RegisterAuthApplicationRequest>
                    for RegisterAuthApplicationSvc<T> {
                        type Response = super::RegisterAuthApplicationResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::RegisterAuthApplicationRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).register_auth_application(request).await
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
                        let method = RegisterAuthApplicationSvc(inner);
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
                "/viam.app.v1.EndUserService/UpdateAuthApplication" => {
                    #[allow(non_camel_case_types)]
                    struct UpdateAuthApplicationSvc<T: EndUserService>(pub Arc<T>);
                    impl<
                        T: EndUserService,
                    > tonic::server::UnaryService<super::UpdateAuthApplicationRequest>
                    for UpdateAuthApplicationSvc<T> {
                        type Response = super::UpdateAuthApplicationResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::UpdateAuthApplicationRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).update_auth_application(request).await
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
                        let method = UpdateAuthApplicationSvc(inner);
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
    impl<T: EndUserService> Clone for EndUserServiceServer<T> {
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
    impl<T: EndUserService> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(Arc::clone(&self.0))
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: EndUserService> tonic::server::NamedService for EndUserServiceServer<T> {
        const NAME: &'static str = "viam.app.v1.EndUserService";
    }
}
/// Generated client implementations.
pub mod robot_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    #[derive(Debug, Clone)]
    pub struct RobotServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl RobotServiceClient<tonic::transport::Channel> {
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
    impl<T> RobotServiceClient<T>
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
        ) -> RobotServiceClient<InterceptedService<T, F>>
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
            RobotServiceClient::new(InterceptedService::new(inner, interceptor))
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
        pub async fn config(
            &mut self,
            request: impl tonic::IntoRequest<super::ConfigRequest>,
        ) -> std::result::Result<tonic::Response<super::ConfigResponse>, tonic::Status> {
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
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("viam.app.v1.RobotService", "Config"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn certificate(
            &mut self,
            request: impl tonic::IntoRequest<super::CertificateRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CertificateResponse>,
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
                "/viam.app.v1.RobotService/Certificate",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("viam.app.v1.RobotService", "Certificate"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn log(
            &mut self,
            request: impl tonic::IntoRequest<super::LogRequest>,
        ) -> std::result::Result<tonic::Response<super::LogResponse>, tonic::Status> {
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
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("viam.app.v1.RobotService", "Log"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn needs_restart(
            &mut self,
            request: impl tonic::IntoRequest<super::NeedsRestartRequest>,
        ) -> std::result::Result<
            tonic::Response<super::NeedsRestartResponse>,
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
                "/viam.app.v1.RobotService/NeedsRestart",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("viam.app.v1.RobotService", "NeedsRestart"));
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod robot_service_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with RobotServiceServer.
    #[async_trait]
    pub trait RobotService: Send + Sync + 'static {
        async fn config(
            &self,
            request: tonic::Request<super::ConfigRequest>,
        ) -> std::result::Result<tonic::Response<super::ConfigResponse>, tonic::Status>;
        async fn certificate(
            &self,
            request: tonic::Request<super::CertificateRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CertificateResponse>,
            tonic::Status,
        >;
        async fn log(
            &self,
            request: tonic::Request<super::LogRequest>,
        ) -> std::result::Result<tonic::Response<super::LogResponse>, tonic::Status>;
        async fn needs_restart(
            &self,
            request: tonic::Request<super::NeedsRestartRequest>,
        ) -> std::result::Result<
            tonic::Response<super::NeedsRestartResponse>,
            tonic::Status,
        >;
    }
    #[derive(Debug)]
    pub struct RobotServiceServer<T: RobotService> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
        max_decoding_message_size: Option<usize>,
        max_encoding_message_size: Option<usize>,
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
        ) -> Poll<std::result::Result<(), Self::Error>> {
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
                            let inner = Arc::clone(&self.0);
                            let fut = async move { (*inner).config(request).await };
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
                        let method = ConfigSvc(inner);
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
                            let inner = Arc::clone(&self.0);
                            let fut = async move { (*inner).certificate(request).await };
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
                        let method = CertificateSvc(inner);
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
                            let inner = Arc::clone(&self.0);
                            let fut = async move { (*inner).log(request).await };
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
                        let method = LogSvc(inner);
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
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).needs_restart(request).await
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
                        let method = NeedsRestartSvc(inner);
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
    impl<T: RobotService> Clone for RobotServiceServer<T> {
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
    impl<T: RobotService> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(Arc::clone(&self.0))
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: RobotService> tonic::server::NamedService for RobotServiceServer<T> {
        const NAME: &'static str = "viam.app.v1.RobotService";
    }
}
