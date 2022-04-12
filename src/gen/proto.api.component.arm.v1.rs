#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetEndPositionRequest {
    /// Name of an arm
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetEndPositionResponse {
    /// Returns 6d pose of the end effector relative to the base, represented by X,Y,Z coordinates which express
    /// millimeters and theta, ox, oy, oz coordinates which express an orientation vector
    #[prost(message, optional, tag = "1")]
    pub pose: ::core::option::Option<super::super::super::common::v1::Pose>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct JointPositions {
    /// A list of joint positions represented in degrees
    /// The numbers are ordered spatially from the base toward the end effector
    /// This is used in GetJointPositionsResponse and MoveToJointPositionsRequest
    #[prost(double, repeated, tag = "1")]
    pub degrees: ::prost::alloc::vec::Vec<f64>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetJointPositionsRequest {
    /// Name of an arm
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetJointPositionsResponse {
    ///a list JointPositions
    #[prost(message, optional, tag = "1")]
    pub position_degs: ::core::option::Option<JointPositions>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MoveToPositionRequest {
    /// Name of an arm
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub to: ::core::option::Option<super::super::super::common::v1::Pose>,
    #[prost(message, optional, tag = "3")]
    pub world_state: ::core::option::Option<super::super::super::common::v1::WorldState>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MoveToPositionResponse {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MoveToJointPositionsRequest {
    /// Name of an arm
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// A list of joint positions represented in degrees
    /// There should be 1 entry in the list per joint, ordered spatially from the base toward the end effector
    #[prost(message, optional, tag = "2")]
    pub position_degs: ::core::option::Option<JointPositions>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MoveToJointPositionsResponse {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Status {
    #[prost(message, optional, tag = "1")]
    pub end_position: ::core::option::Option<super::super::super::common::v1::Pose>,
    #[prost(message, optional, tag = "2")]
    pub joint_positions: ::core::option::Option<JointPositions>,
}
#[doc = r" Generated client implementations."]
pub mod arm_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = " An ArmService services all arms associated with a robot"]
    #[derive(Debug, Clone)]
    pub struct ArmServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl ArmServiceClient<tonic::transport::Channel> {
        #[doc = r" Attempt to create a new client by connecting to a given endpoint."]
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: std::convert::TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> ArmServiceClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::ResponseBody: Body + Send + 'static,
        T::Error: Into<StdError>,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> ArmServiceClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<http::Request<tonic::body::BoxBody>>>::Error:
                Into<StdError> + Send + Sync,
        {
            ArmServiceClient::new(InterceptedService::new(inner, interceptor))
        }
        #[doc = r" Compress requests with `gzip`."]
        #[doc = r""]
        #[doc = r" This requires the server to support it otherwise it might respond with an"]
        #[doc = r" error."]
        pub fn send_gzip(mut self) -> Self {
            self.inner = self.inner.send_gzip();
            self
        }
        #[doc = r" Enable decompressing responses with `gzip`."]
        pub fn accept_gzip(mut self) -> Self {
            self.inner = self.inner.accept_gzip();
            self
        }
        #[doc = " GetEndPosition gets the current position the end of the robot's arm expressed as X,Y,Z,ox,oy,oz,theta"]
        pub async fn get_end_position(
            &mut self,
            request: impl tonic::IntoRequest<super::GetEndPositionRequest>,
        ) -> Result<tonic::Response<super::GetEndPositionResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/proto.api.component.arm.v1.ArmService/GetEndPosition",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " MoveToPosition moves the mount point of the robot's end effector to the requested position."]
        pub async fn move_to_position(
            &mut self,
            request: impl tonic::IntoRequest<super::MoveToPositionRequest>,
        ) -> Result<tonic::Response<super::MoveToPositionResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/proto.api.component.arm.v1.ArmService/MoveToPosition",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " GetJointPositions lists the joint positions (in degrees) of every joint on a robot"]
        pub async fn get_joint_positions(
            &mut self,
            request: impl tonic::IntoRequest<super::GetJointPositionsRequest>,
        ) -> Result<tonic::Response<super::GetJointPositionsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/proto.api.component.arm.v1.ArmService/GetJointPositions",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " MoveToJointPositions moves every joint on a robot's arm to specified angles which are expressed in degrees"]
        pub async fn move_to_joint_positions(
            &mut self,
            request: impl tonic::IntoRequest<super::MoveToJointPositionsRequest>,
        ) -> Result<tonic::Response<super::MoveToJointPositionsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/proto.api.component.arm.v1.ArmService/MoveToJointPositions",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
#[doc = r" Generated server implementations."]
pub mod arm_service_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = "Generated trait containing gRPC methods that should be implemented for use with ArmServiceServer."]
    #[async_trait]
    pub trait ArmService: Send + Sync + 'static {
        #[doc = " GetEndPosition gets the current position the end of the robot's arm expressed as X,Y,Z,ox,oy,oz,theta"]
        async fn get_end_position(
            &self,
            request: tonic::Request<super::GetEndPositionRequest>,
        ) -> Result<tonic::Response<super::GetEndPositionResponse>, tonic::Status>;
        #[doc = " MoveToPosition moves the mount point of the robot's end effector to the requested position."]
        async fn move_to_position(
            &self,
            request: tonic::Request<super::MoveToPositionRequest>,
        ) -> Result<tonic::Response<super::MoveToPositionResponse>, tonic::Status>;
        #[doc = " GetJointPositions lists the joint positions (in degrees) of every joint on a robot"]
        async fn get_joint_positions(
            &self,
            request: tonic::Request<super::GetJointPositionsRequest>,
        ) -> Result<tonic::Response<super::GetJointPositionsResponse>, tonic::Status>;
        #[doc = " MoveToJointPositions moves every joint on a robot's arm to specified angles which are expressed in degrees"]
        async fn move_to_joint_positions(
            &self,
            request: tonic::Request<super::MoveToJointPositionsRequest>,
        ) -> Result<tonic::Response<super::MoveToJointPositionsResponse>, tonic::Status>;
    }
    #[doc = " An ArmService services all arms associated with a robot"]
    #[derive(Debug)]
    pub struct ArmServiceServer<T: ArmService> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: ArmService> ArmServiceServer<T> {
        pub fn new(inner: T) -> Self {
            let inner = Arc::new(inner);
            let inner = _Inner(inner);
            Self {
                inner,
                accept_compression_encodings: Default::default(),
                send_compression_encodings: Default::default(),
            }
        }
        pub fn with_interceptor<F>(inner: T, interceptor: F) -> InterceptedService<Self, F>
        where
            F: tonic::service::Interceptor,
        {
            InterceptedService::new(Self::new(inner), interceptor)
        }
        #[doc = r" Enable decompressing requests with `gzip`."]
        pub fn accept_gzip(mut self) -> Self {
            self.accept_compression_encodings.enable_gzip();
            self
        }
        #[doc = r" Compress responses with `gzip`, if the client supports it."]
        pub fn send_gzip(mut self) -> Self {
            self.send_compression_encodings.enable_gzip();
            self
        }
    }
    impl<T, B> tonic::codegen::Service<http::Request<B>> for ArmServiceServer<T>
    where
        T: ArmService,
        B: Body + Send + 'static,
        B::Error: Into<StdError> + Send + 'static,
    {
        type Response = http::Response<tonic::body::BoxBody>;
        type Error = Never;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            let inner = self.inner.clone();
            match req.uri().path() {
                "/proto.api.component.arm.v1.ArmService/GetEndPosition" => {
                    #[allow(non_camel_case_types)]
                    struct GetEndPositionSvc<T: ArmService>(pub Arc<T>);
                    impl<T: ArmService> tonic::server::UnaryService<super::GetEndPositionRequest>
                        for GetEndPositionSvc<T>
                    {
                        type Response = super::GetEndPositionResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetEndPositionRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).get_end_position(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetEndPositionSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/proto.api.component.arm.v1.ArmService/MoveToPosition" => {
                    #[allow(non_camel_case_types)]
                    struct MoveToPositionSvc<T: ArmService>(pub Arc<T>);
                    impl<T: ArmService> tonic::server::UnaryService<super::MoveToPositionRequest>
                        for MoveToPositionSvc<T>
                    {
                        type Response = super::MoveToPositionResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MoveToPositionRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).move_to_position(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = MoveToPositionSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/proto.api.component.arm.v1.ArmService/GetJointPositions" => {
                    #[allow(non_camel_case_types)]
                    struct GetJointPositionsSvc<T: ArmService>(pub Arc<T>);
                    impl<T: ArmService> tonic::server::UnaryService<super::GetJointPositionsRequest>
                        for GetJointPositionsSvc<T>
                    {
                        type Response = super::GetJointPositionsResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetJointPositionsRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).get_joint_positions(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetJointPositionsSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/proto.api.component.arm.v1.ArmService/MoveToJointPositions" => {
                    #[allow(non_camel_case_types)]
                    struct MoveToJointPositionsSvc<T: ArmService>(pub Arc<T>);
                    impl<T: ArmService>
                        tonic::server::UnaryService<super::MoveToJointPositionsRequest>
                        for MoveToJointPositionsSvc<T>
                    {
                        type Response = super::MoveToJointPositionsResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MoveToJointPositionsRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut =
                                async move { (*inner).move_to_joint_positions(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = MoveToJointPositionsSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => Box::pin(async move {
                    Ok(http::Response::builder()
                        .status(200)
                        .header("grpc-status", "12")
                        .header("content-type", "application/grpc")
                        .body(empty_body())
                        .unwrap())
                }),
            }
        }
    }
    impl<T: ArmService> Clone for ArmServiceServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
            }
        }
    }
    impl<T: ArmService> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: ArmService> tonic::transport::NamedService for ArmServiceServer<T> {
        const NAME: &'static str = "proto.api.component.arm.v1.ArmService";
    }
}
