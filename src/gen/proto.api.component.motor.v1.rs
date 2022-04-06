#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetPowerRequest {
    /// Name of a motor
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Percentage of motor's power, between -1 and 1
    #[prost(double, tag = "2")]
    pub power_pct: f64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetPowerResponse {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GoForRequest {
    /// Name of a motor
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Speed of motor travel in rotations per minute
    #[prost(double, tag = "2")]
    pub rpm: f64,
    /// Number of revolutions relative to motor's start position
    #[prost(double, tag = "3")]
    pub revolutions: f64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GoForResponse {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GoToRequest {
    /// Name of a motor
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Speed of motor travel in rotations per minute
    #[prost(double, tag = "2")]
    pub rpm: f64,
    /// Number of revolutions relative to motor's home home/zero
    #[prost(double, tag = "3")]
    pub position_revolutions: f64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GoToResponse {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResetZeroPositionRequest {
    /// Name of a motor
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Motor position
    #[prost(double, tag = "2")]
    pub offset: f64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResetZeroPositionResponse {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetPositionRequest {
    /// Name of a motor
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetPositionResponse {
    /// Current position of the motor relative to its home
    #[prost(double, tag = "1")]
    pub position: f64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StopRequest {
    /// Name of a motor
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StopResponse {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IsPoweredRequest {
    /// Name of a motor
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IsPoweredResponse {
    /// Returns true if the motor is on
    #[prost(bool, tag = "1")]
    pub is_on: bool,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetFeaturesRequest {
    /// Name of a motor
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetFeaturesResponse {
    /// Returns true if the motor supports reporting its position
    #[prost(bool, tag = "1")]
    pub position_reporting: bool,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Status {
    /// To D0 (FA): Delete this field
    #[prost(bool, tag = "1")]
    pub is_on: bool,
    /// Returns true if the motor has position support
    #[prost(bool, tag = "2")]
    pub position_reporting: bool,
    /// Returns current position of the motor relative to its home
    #[prost(double, tag = "3")]
    pub position: f64,
}
#[doc = r" Generated client implementations."]
pub mod motor_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = " A MotorService maintains all motors associated with a robot"]
    #[derive(Debug, Clone)]
    pub struct MotorServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl MotorServiceClient<tonic::transport::Channel> {
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
    impl<T> MotorServiceClient<T>
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
        ) -> MotorServiceClient<InterceptedService<T, F>>
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
            MotorServiceClient::new(InterceptedService::new(inner, interceptor))
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
        #[doc = " SetPower sets the percentage of the motor's total power that should be employed"]
        #[doc = " expressed a value between -1 and 1 where negative values indicate a backwards"]
        #[doc = " direction and positive values a forward direction"]
        pub async fn set_power(
            &mut self,
            request: impl tonic::IntoRequest<super::SetPowerRequest>,
        ) -> Result<tonic::Response<super::SetPowerResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/proto.api.component.motor.v1.MotorService/SetPower",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " GoFor instructs the motor to turn at a specified speed, which is expressed in RPM,"]
        #[doc = " for a specified number of rotations relative to its starting position"]
        #[doc = " This method will return an error if position reporting is not supported"]
        pub async fn go_for(
            &mut self,
            request: impl tonic::IntoRequest<super::GoForRequest>,
        ) -> Result<tonic::Response<super::GoForResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/proto.api.component.motor.v1.MotorService/GoFor",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " GoTo requests the robot's motor to move to a specific position that"]
        #[doc = " is relative to its home position at a specified speed which is expressed in RPM"]
        #[doc = " This method will return an error if position reporting is not supported"]
        pub async fn go_to(
            &mut self,
            request: impl tonic::IntoRequest<super::GoToRequest>,
        ) -> Result<tonic::Response<super::GoToResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/proto.api.component.motor.v1.MotorService/GoTo",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " ResetZeroPosition sets the current position of the motor as the new zero position"]
        #[doc = " This method will return an error if position reporting is not supported"]
        pub async fn reset_zero_position(
            &mut self,
            request: impl tonic::IntoRequest<super::ResetZeroPositionRequest>,
        ) -> Result<tonic::Response<super::ResetZeroPositionResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/proto.api.component.motor.v1.MotorService/ResetZeroPosition",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Position reports the position of the robot's motor relative to its zero position"]
        #[doc = " This method will return an error if position reporting is not supported"]
        pub async fn get_position(
            &mut self,
            request: impl tonic::IntoRequest<super::GetPositionRequest>,
        ) -> Result<tonic::Response<super::GetPositionResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/proto.api.component.motor.v1.MotorService/GetPosition",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " GetFeatures returns a message of booleans indicating which optional features the robot's motor supports"]
        pub async fn get_features(
            &mut self,
            request: impl tonic::IntoRequest<super::GetFeaturesRequest>,
        ) -> Result<tonic::Response<super::GetFeaturesResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/proto.api.component.motor.v1.MotorService/GetFeatures",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Stop turns the robot's motor off"]
        #[doc = " To Do (FA): This will be deprecated"]
        pub async fn stop(
            &mut self,
            request: impl tonic::IntoRequest<super::StopRequest>,
        ) -> Result<tonic::Response<super::StopResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/proto.api.component.motor.v1.MotorService/Stop",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " IsPowered returns true if the robot's motor off"]
        #[doc = " To Do (FA): This will be deprecated"]
        pub async fn is_powered(
            &mut self,
            request: impl tonic::IntoRequest<super::IsPoweredRequest>,
        ) -> Result<tonic::Response<super::IsPoweredResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/proto.api.component.motor.v1.MotorService/IsPowered",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
#[doc = r" Generated server implementations."]
pub mod motor_service_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = "Generated trait containing gRPC methods that should be implemented for use with MotorServiceServer."]
    #[async_trait]
    pub trait MotorService: Send + Sync + 'static {
        #[doc = " SetPower sets the percentage of the motor's total power that should be employed"]
        #[doc = " expressed a value between -1 and 1 where negative values indicate a backwards"]
        #[doc = " direction and positive values a forward direction"]
        async fn set_power(
            &self,
            request: tonic::Request<super::SetPowerRequest>,
        ) -> Result<tonic::Response<super::SetPowerResponse>, tonic::Status>;
        #[doc = " GoFor instructs the motor to turn at a specified speed, which is expressed in RPM,"]
        #[doc = " for a specified number of rotations relative to its starting position"]
        #[doc = " This method will return an error if position reporting is not supported"]
        async fn go_for(
            &self,
            request: tonic::Request<super::GoForRequest>,
        ) -> Result<tonic::Response<super::GoForResponse>, tonic::Status>;
        #[doc = " GoTo requests the robot's motor to move to a specific position that"]
        #[doc = " is relative to its home position at a specified speed which is expressed in RPM"]
        #[doc = " This method will return an error if position reporting is not supported"]
        async fn go_to(
            &self,
            request: tonic::Request<super::GoToRequest>,
        ) -> Result<tonic::Response<super::GoToResponse>, tonic::Status>;
        #[doc = " ResetZeroPosition sets the current position of the motor as the new zero position"]
        #[doc = " This method will return an error if position reporting is not supported"]
        async fn reset_zero_position(
            &self,
            request: tonic::Request<super::ResetZeroPositionRequest>,
        ) -> Result<tonic::Response<super::ResetZeroPositionResponse>, tonic::Status>;
        #[doc = " Position reports the position of the robot's motor relative to its zero position"]
        #[doc = " This method will return an error if position reporting is not supported"]
        async fn get_position(
            &self,
            request: tonic::Request<super::GetPositionRequest>,
        ) -> Result<tonic::Response<super::GetPositionResponse>, tonic::Status>;
        #[doc = " GetFeatures returns a message of booleans indicating which optional features the robot's motor supports"]
        async fn get_features(
            &self,
            request: tonic::Request<super::GetFeaturesRequest>,
        ) -> Result<tonic::Response<super::GetFeaturesResponse>, tonic::Status>;
        #[doc = " Stop turns the robot's motor off"]
        #[doc = " To Do (FA): This will be deprecated"]
        async fn stop(
            &self,
            request: tonic::Request<super::StopRequest>,
        ) -> Result<tonic::Response<super::StopResponse>, tonic::Status>;
        #[doc = " IsPowered returns true if the robot's motor off"]
        #[doc = " To Do (FA): This will be deprecated"]
        async fn is_powered(
            &self,
            request: tonic::Request<super::IsPoweredRequest>,
        ) -> Result<tonic::Response<super::IsPoweredResponse>, tonic::Status>;
    }
    #[doc = " A MotorService maintains all motors associated with a robot"]
    #[derive(Debug)]
    pub struct MotorServiceServer<T: MotorService> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: MotorService> MotorServiceServer<T> {
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
    impl<T, B> tonic::codegen::Service<http::Request<B>> for MotorServiceServer<T>
    where
        T: MotorService,
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
                "/proto.api.component.motor.v1.MotorService/SetPower" => {
                    #[allow(non_camel_case_types)]
                    struct SetPowerSvc<T: MotorService>(pub Arc<T>);
                    impl<T: MotorService> tonic::server::UnaryService<super::SetPowerRequest> for SetPowerSvc<T> {
                        type Response = super::SetPowerResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SetPowerRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).set_power(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = SetPowerSvc(inner);
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
                "/proto.api.component.motor.v1.MotorService/GoFor" => {
                    #[allow(non_camel_case_types)]
                    struct GoForSvc<T: MotorService>(pub Arc<T>);
                    impl<T: MotorService> tonic::server::UnaryService<super::GoForRequest> for GoForSvc<T> {
                        type Response = super::GoForResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GoForRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).go_for(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GoForSvc(inner);
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
                "/proto.api.component.motor.v1.MotorService/GoTo" => {
                    #[allow(non_camel_case_types)]
                    struct GoToSvc<T: MotorService>(pub Arc<T>);
                    impl<T: MotorService> tonic::server::UnaryService<super::GoToRequest> for GoToSvc<T> {
                        type Response = super::GoToResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GoToRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).go_to(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GoToSvc(inner);
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
                "/proto.api.component.motor.v1.MotorService/ResetZeroPosition" => {
                    #[allow(non_camel_case_types)]
                    struct ResetZeroPositionSvc<T: MotorService>(pub Arc<T>);
                    impl<T: MotorService>
                        tonic::server::UnaryService<super::ResetZeroPositionRequest>
                        for ResetZeroPositionSvc<T>
                    {
                        type Response = super::ResetZeroPositionResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ResetZeroPositionRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).reset_zero_position(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ResetZeroPositionSvc(inner);
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
                "/proto.api.component.motor.v1.MotorService/GetPosition" => {
                    #[allow(non_camel_case_types)]
                    struct GetPositionSvc<T: MotorService>(pub Arc<T>);
                    impl<T: MotorService> tonic::server::UnaryService<super::GetPositionRequest> for GetPositionSvc<T> {
                        type Response = super::GetPositionResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetPositionRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).get_position(request).await };
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
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/proto.api.component.motor.v1.MotorService/GetFeatures" => {
                    #[allow(non_camel_case_types)]
                    struct GetFeaturesSvc<T: MotorService>(pub Arc<T>);
                    impl<T: MotorService> tonic::server::UnaryService<super::GetFeaturesRequest> for GetFeaturesSvc<T> {
                        type Response = super::GetFeaturesResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetFeaturesRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).get_features(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetFeaturesSvc(inner);
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
                "/proto.api.component.motor.v1.MotorService/Stop" => {
                    #[allow(non_camel_case_types)]
                    struct StopSvc<T: MotorService>(pub Arc<T>);
                    impl<T: MotorService> tonic::server::UnaryService<super::StopRequest> for StopSvc<T> {
                        type Response = super::StopResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::StopRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).stop(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = StopSvc(inner);
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
                "/proto.api.component.motor.v1.MotorService/IsPowered" => {
                    #[allow(non_camel_case_types)]
                    struct IsPoweredSvc<T: MotorService>(pub Arc<T>);
                    impl<T: MotorService> tonic::server::UnaryService<super::IsPoweredRequest> for IsPoweredSvc<T> {
                        type Response = super::IsPoweredResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::IsPoweredRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).is_powered(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = IsPoweredSvc(inner);
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
    impl<T: MotorService> Clone for MotorServiceServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
            }
        }
    }
    impl<T: MotorService> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: MotorService> tonic::transport::NamedService for MotorServiceServer<T> {
        const NAME: &'static str = "proto.api.component.motor.v1.MotorService";
    }
}
