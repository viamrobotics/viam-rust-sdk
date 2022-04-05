/// AngularVelocity contains angular velocity in deg/s across x/y/z axes.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AngularVelocity {
    /// Velocity in deg/s across the x-axis
    #[prost(double, tag = "1")]
    pub x_degs_per_sec: f64,
    /// Velocity in deg/s across the y-axis
    #[prost(double, tag = "2")]
    pub y_degs_per_sec: f64,
    /// Velocity in deg/s across the z-axis
    #[prost(double, tag = "3")]
    pub z_degs_per_sec: f64,
}
/// EulerAngles are three angles used to represent the rotation of an object in 3D Euclidean space
/// The Tait–Bryan angle formalism is used, with rotations around three distinct axes in the z-y′-x″ sequence.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EulerAngles {
    /// Rotation in deg around the x-axis
    #[prost(double, tag = "1")]
    pub roll_deg: f64,
    /// Rotation in deg around the y-axis
    #[prost(double, tag = "2")]
    pub pitch_deg: f64,
    /// Rotation in deg around the z-axis
    #[prost(double, tag = "3")]
    pub yaw_deg: f64,
}
/// Acceleration contains linear acceleration in mm/s^2 across x/y/z axes.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Acceleration {
    /// Acceleration in mm/s^2 across the x-axis
    #[prost(double, tag = "1")]
    pub x_mm_per_sec_per_sec: f64,
    /// Acceleration in mm/s^2 across the y-axis
    #[prost(double, tag = "2")]
    pub y_mm_per_sec_per_sec: f64,
    /// Acceleration in mm/s^2 across the z-axis
    #[prost(double, tag = "3")]
    pub z_mm_per_sec_per_sec: f64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReadAngularVelocityRequest {
    /// Name of an IMU
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReadAngularVelocityResponse {
    /// AngularVelocity contains angular velocity in deg/s across x/y/z axes.
    #[prost(message, optional, tag = "1")]
    pub angular_velocity: ::core::option::Option<AngularVelocity>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReadOrientationRequest {
    /// Name of an IMU
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReadOrientationResponse {
    /// EulerAngles are three angles used to represent the rotation of an object in 3D Euclidean space
    /// The Tait–Bryan angle formalism is used, with rotations around three distinct axes in the z-y′-x″ sequence.
    #[prost(message, optional, tag = "1")]
    pub orientation: ::core::option::Option<EulerAngles>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReadAccelerationRequest {
    /// Name of an IMU
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReadAccelerationResponse {
    /// Acceleration contains acceleration in mm/s^2 across x/y/z axes.
    #[prost(message, optional, tag = "1")]
    pub acceleration: ::core::option::Option<Acceleration>,
}
#[doc = r" Generated client implementations."]
pub mod imu_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = " IMUService services all IMUs associated with a robot"]
    #[derive(Debug, Clone)]
    pub struct ImuServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl ImuServiceClient<tonic::transport::Channel> {
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
    impl<T> ImuServiceClient<T>
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
        ) -> ImuServiceClient<InterceptedService<T, F>>
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
            ImuServiceClient::new(InterceptedService::new(inner, interceptor))
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
        #[doc = " ReadAngularVelocity returns the most recent angular velocity reading from the given IMU."]
        pub async fn read_angular_velocity(
            &mut self,
            request: impl tonic::IntoRequest<super::ReadAngularVelocityRequest>,
        ) -> Result<tonic::Response<super::ReadAngularVelocityResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/proto.api.component.imu.v1.IMUService/ReadAngularVelocity",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " ReadOrientation returns the most recent orientation reading from the given IMU."]
        pub async fn read_orientation(
            &mut self,
            request: impl tonic::IntoRequest<super::ReadOrientationRequest>,
        ) -> Result<tonic::Response<super::ReadOrientationResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/proto.api.component.imu.v1.IMUService/ReadOrientation",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " ReadAcceleration returns the most recent acceleration reading from the given IMU."]
        pub async fn read_acceleration(
            &mut self,
            request: impl tonic::IntoRequest<super::ReadAccelerationRequest>,
        ) -> Result<tonic::Response<super::ReadAccelerationResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/proto.api.component.imu.v1.IMUService/ReadAcceleration",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
#[doc = r" Generated server implementations."]
pub mod imu_service_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = "Generated trait containing gRPC methods that should be implemented for use with ImuServiceServer."]
    #[async_trait]
    pub trait ImuService: Send + Sync + 'static {
        #[doc = " ReadAngularVelocity returns the most recent angular velocity reading from the given IMU."]
        async fn read_angular_velocity(
            &self,
            request: tonic::Request<super::ReadAngularVelocityRequest>,
        ) -> Result<tonic::Response<super::ReadAngularVelocityResponse>, tonic::Status>;
        #[doc = " ReadOrientation returns the most recent orientation reading from the given IMU."]
        async fn read_orientation(
            &self,
            request: tonic::Request<super::ReadOrientationRequest>,
        ) -> Result<tonic::Response<super::ReadOrientationResponse>, tonic::Status>;
        #[doc = " ReadAcceleration returns the most recent acceleration reading from the given IMU."]
        async fn read_acceleration(
            &self,
            request: tonic::Request<super::ReadAccelerationRequest>,
        ) -> Result<tonic::Response<super::ReadAccelerationResponse>, tonic::Status>;
    }
    #[doc = " IMUService services all IMUs associated with a robot"]
    #[derive(Debug)]
    pub struct ImuServiceServer<T: ImuService> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: ImuService> ImuServiceServer<T> {
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
    impl<T, B> tonic::codegen::Service<http::Request<B>> for ImuServiceServer<T>
    where
        T: ImuService,
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
                "/proto.api.component.imu.v1.IMUService/ReadAngularVelocity" => {
                    #[allow(non_camel_case_types)]
                    struct ReadAngularVelocitySvc<T: ImuService>(pub Arc<T>);
                    impl<T: ImuService>
                        tonic::server::UnaryService<super::ReadAngularVelocityRequest>
                        for ReadAngularVelocitySvc<T>
                    {
                        type Response = super::ReadAngularVelocityResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ReadAngularVelocityRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).read_angular_velocity(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ReadAngularVelocitySvc(inner);
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
                "/proto.api.component.imu.v1.IMUService/ReadOrientation" => {
                    #[allow(non_camel_case_types)]
                    struct ReadOrientationSvc<T: ImuService>(pub Arc<T>);
                    impl<T: ImuService> tonic::server::UnaryService<super::ReadOrientationRequest>
                        for ReadOrientationSvc<T>
                    {
                        type Response = super::ReadOrientationResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ReadOrientationRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).read_orientation(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ReadOrientationSvc(inner);
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
                "/proto.api.component.imu.v1.IMUService/ReadAcceleration" => {
                    #[allow(non_camel_case_types)]
                    struct ReadAccelerationSvc<T: ImuService>(pub Arc<T>);
                    impl<T: ImuService> tonic::server::UnaryService<super::ReadAccelerationRequest>
                        for ReadAccelerationSvc<T>
                    {
                        type Response = super::ReadAccelerationResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ReadAccelerationRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).read_acceleration(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ReadAccelerationSvc(inner);
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
    impl<T: ImuService> Clone for ImuServiceServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
            }
        }
    }
    impl<T: ImuService> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: ImuService> tonic::transport::NamedService for ImuServiceServer<T> {
        const NAME: &'static str = "proto.api.component.imu.v1.IMUService";
    }
}
