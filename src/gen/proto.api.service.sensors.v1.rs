#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetSensorsRequest {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetSensorsResponse {
    #[prost(message, repeated, tag = "1")]
    pub sensor_names: ::prost::alloc::vec::Vec<super::super::super::common::v1::ResourceName>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetReadingsRequest {
    #[prost(message, repeated, tag = "1")]
    pub sensor_names: ::prost::alloc::vec::Vec<super::super::super::common::v1::ResourceName>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Readings {
    #[prost(message, optional, tag = "1")]
    pub name: ::core::option::Option<super::super::super::common::v1::ResourceName>,
    #[prost(message, repeated, tag = "2")]
    pub readings: ::prost::alloc::vec::Vec<::prost_types::Value>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetReadingsResponse {
    #[prost(message, repeated, tag = "1")]
    pub readings: ::prost::alloc::vec::Vec<Readings>,
}
#[doc = r" Generated client implementations."]
pub mod sensors_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = " A SensorsService services keeps track of all sensors associated with a robot"]
    #[derive(Debug, Clone)]
    pub struct SensorsServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl SensorsServiceClient<tonic::transport::Channel> {
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
    impl<T> SensorsServiceClient<T>
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
        ) -> SensorsServiceClient<InterceptedService<T, F>>
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
            SensorsServiceClient::new(InterceptedService::new(inner, interceptor))
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
        #[doc = " GetSensors returns the list of all sensors."]
        pub async fn get_sensors(
            &mut self,
            request: impl tonic::IntoRequest<super::GetSensorsRequest>,
        ) -> Result<tonic::Response<super::GetSensorsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/proto.api.service.sensors.v1.SensorsService/GetSensors",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " GetReadings returns the list of readings for all sensors specified."]
        pub async fn get_readings(
            &mut self,
            request: impl tonic::IntoRequest<super::GetReadingsRequest>,
        ) -> Result<tonic::Response<super::GetReadingsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/proto.api.service.sensors.v1.SensorsService/GetReadings",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
#[doc = r" Generated server implementations."]
pub mod sensors_service_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = "Generated trait containing gRPC methods that should be implemented for use with SensorsServiceServer."]
    #[async_trait]
    pub trait SensorsService: Send + Sync + 'static {
        #[doc = " GetSensors returns the list of all sensors."]
        async fn get_sensors(
            &self,
            request: tonic::Request<super::GetSensorsRequest>,
        ) -> Result<tonic::Response<super::GetSensorsResponse>, tonic::Status>;
        #[doc = " GetReadings returns the list of readings for all sensors specified."]
        async fn get_readings(
            &self,
            request: tonic::Request<super::GetReadingsRequest>,
        ) -> Result<tonic::Response<super::GetReadingsResponse>, tonic::Status>;
    }
    #[doc = " A SensorsService services keeps track of all sensors associated with a robot"]
    #[derive(Debug)]
    pub struct SensorsServiceServer<T: SensorsService> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: SensorsService> SensorsServiceServer<T> {
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
    impl<T, B> tonic::codegen::Service<http::Request<B>> for SensorsServiceServer<T>
    where
        T: SensorsService,
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
                "/proto.api.service.sensors.v1.SensorsService/GetSensors" => {
                    #[allow(non_camel_case_types)]
                    struct GetSensorsSvc<T: SensorsService>(pub Arc<T>);
                    impl<T: SensorsService> tonic::server::UnaryService<super::GetSensorsRequest> for GetSensorsSvc<T> {
                        type Response = super::GetSensorsResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetSensorsRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).get_sensors(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetSensorsSvc(inner);
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
                "/proto.api.service.sensors.v1.SensorsService/GetReadings" => {
                    #[allow(non_camel_case_types)]
                    struct GetReadingsSvc<T: SensorsService>(pub Arc<T>);
                    impl<T: SensorsService> tonic::server::UnaryService<super::GetReadingsRequest>
                        for GetReadingsSvc<T>
                    {
                        type Response = super::GetReadingsResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetReadingsRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).get_readings(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetReadingsSvc(inner);
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
    impl<T: SensorsService> Clone for SensorsServiceServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
            }
        }
    }
    impl<T: SensorsService> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: SensorsService> tonic::transport::NamedService for SensorsServiceServer<T> {
        const NAME: &'static str = "proto.api.service.sensors.v1.SensorsService";
    }
}
