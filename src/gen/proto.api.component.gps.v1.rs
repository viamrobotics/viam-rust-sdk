#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReadLocationRequest {
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReadLocationResponse {
    #[prost(message, optional, tag = "1")]
    pub coordinate: ::core::option::Option<super::super::super::common::v1::GeoPoint>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReadAltitudeRequest {
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReadAltitudeResponse {
    #[prost(double, tag = "1")]
    pub altitude_meters: f64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReadSpeedRequest {
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReadSpeedResponse {
    #[prost(double, tag = "1")]
    pub speed_mm_per_sec: f64,
}
#[doc = r" Generated client implementations."]
pub mod gps_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = " GPSService services all GPSs associated with a robot"]
    #[derive(Debug, Clone)]
    pub struct GpsServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl GpsServiceClient<tonic::transport::Channel> {
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
    impl<T> GpsServiceClient<T>
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
        ) -> GpsServiceClient<InterceptedService<T, F>>
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
            GpsServiceClient::new(InterceptedService::new(inner, interceptor))
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
        #[doc = " ReadLocation returns the most recent location from the given GPS."]
        pub async fn read_location(
            &mut self,
            request: impl tonic::IntoRequest<super::ReadLocationRequest>,
        ) -> Result<tonic::Response<super::ReadLocationResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/proto.api.component.gps.v1.GPSService/ReadLocation",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " ReadAltitude returns the most recent altitude from the given GPS."]
        pub async fn read_altitude(
            &mut self,
            request: impl tonic::IntoRequest<super::ReadAltitudeRequest>,
        ) -> Result<tonic::Response<super::ReadAltitudeResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/proto.api.component.gps.v1.GPSService/ReadAltitude",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " ReadSpeed returns the most recent speed from the given GPS."]
        pub async fn read_speed(
            &mut self,
            request: impl tonic::IntoRequest<super::ReadSpeedRequest>,
        ) -> Result<tonic::Response<super::ReadSpeedResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/proto.api.component.gps.v1.GPSService/ReadSpeed",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
#[doc = r" Generated server implementations."]
pub mod gps_service_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = "Generated trait containing gRPC methods that should be implemented for use with GpsServiceServer."]
    #[async_trait]
    pub trait GpsService: Send + Sync + 'static {
        #[doc = " ReadLocation returns the most recent location from the given GPS."]
        async fn read_location(
            &self,
            request: tonic::Request<super::ReadLocationRequest>,
        ) -> Result<tonic::Response<super::ReadLocationResponse>, tonic::Status>;
        #[doc = " ReadAltitude returns the most recent altitude from the given GPS."]
        async fn read_altitude(
            &self,
            request: tonic::Request<super::ReadAltitudeRequest>,
        ) -> Result<tonic::Response<super::ReadAltitudeResponse>, tonic::Status>;
        #[doc = " ReadSpeed returns the most recent speed from the given GPS."]
        async fn read_speed(
            &self,
            request: tonic::Request<super::ReadSpeedRequest>,
        ) -> Result<tonic::Response<super::ReadSpeedResponse>, tonic::Status>;
    }
    #[doc = " GPSService services all GPSs associated with a robot"]
    #[derive(Debug)]
    pub struct GpsServiceServer<T: GpsService> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: GpsService> GpsServiceServer<T> {
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
    impl<T, B> tonic::codegen::Service<http::Request<B>> for GpsServiceServer<T>
    where
        T: GpsService,
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
                "/proto.api.component.gps.v1.GPSService/ReadLocation" => {
                    #[allow(non_camel_case_types)]
                    struct ReadLocationSvc<T: GpsService>(pub Arc<T>);
                    impl<T: GpsService> tonic::server::UnaryService<super::ReadLocationRequest> for ReadLocationSvc<T> {
                        type Response = super::ReadLocationResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ReadLocationRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).read_location(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ReadLocationSvc(inner);
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
                "/proto.api.component.gps.v1.GPSService/ReadAltitude" => {
                    #[allow(non_camel_case_types)]
                    struct ReadAltitudeSvc<T: GpsService>(pub Arc<T>);
                    impl<T: GpsService> tonic::server::UnaryService<super::ReadAltitudeRequest> for ReadAltitudeSvc<T> {
                        type Response = super::ReadAltitudeResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ReadAltitudeRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).read_altitude(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ReadAltitudeSvc(inner);
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
                "/proto.api.component.gps.v1.GPSService/ReadSpeed" => {
                    #[allow(non_camel_case_types)]
                    struct ReadSpeedSvc<T: GpsService>(pub Arc<T>);
                    impl<T: GpsService> tonic::server::UnaryService<super::ReadSpeedRequest> for ReadSpeedSvc<T> {
                        type Response = super::ReadSpeedResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ReadSpeedRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).read_speed(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ReadSpeedSvc(inner);
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
    impl<T: GpsService> Clone for GpsServiceServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
            }
        }
    }
    impl<T: GpsService> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: GpsService> tonic::transport::NamedService for GpsServiceServer<T> {
        const NAME: &'static str = "proto.api.component.gps.v1.GPSService";
    }
}
