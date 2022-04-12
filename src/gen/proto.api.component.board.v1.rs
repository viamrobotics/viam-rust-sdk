#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StatusRequest {
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StatusResponse {
    #[prost(message, optional, tag = "1")]
    pub status: ::core::option::Option<super::super::super::common::v1::BoardStatus>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetGpioRequest {
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub pin: ::prost::alloc::string::String,
    #[prost(bool, tag = "3")]
    pub high: bool,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetGpioResponse {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetGpioRequest {
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub pin: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetGpioResponse {
    #[prost(bool, tag = "1")]
    pub high: bool,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PwmRequest {
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub pin: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PwmResponse {
    /// 0-1
    #[prost(double, tag = "1")]
    pub duty_cycle_pct: f64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetPwmRequest {
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub pin: ::prost::alloc::string::String,
    /// 0-1
    #[prost(double, tag = "3")]
    pub duty_cycle_pct: f64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetPwmResponse {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PwmFrequencyRequest {
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub pin: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PwmFrequencyResponse {
    #[prost(uint64, tag = "1")]
    pub frequency_hz: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetPwmFrequencyRequest {
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub pin: ::prost::alloc::string::String,
    #[prost(uint64, tag = "3")]
    pub frequency_hz: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetPwmFrequencyResponse {}
// Analog Reader

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReadAnalogReaderRequest {
    #[prost(string, tag = "1")]
    pub board_name: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub analog_reader_name: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReadAnalogReaderResponse {
    #[prost(int32, tag = "1")]
    pub value: i32,
}
// Digital Interrupt

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetDigitalInterruptValueRequest {
    #[prost(string, tag = "1")]
    pub board_name: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub digital_interrupt_name: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetDigitalInterruptValueResponse {
    #[prost(int64, tag = "1")]
    pub value: i64,
}
#[doc = r" Generated client implementations."]
pub mod board_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = " BoardService services all Boards associated with a robot"]
    #[derive(Debug, Clone)]
    pub struct BoardServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl BoardServiceClient<tonic::transport::Channel> {
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
    impl<T> BoardServiceClient<T>
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
        ) -> BoardServiceClient<InterceptedService<T, F>>
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
            BoardServiceClient::new(InterceptedService::new(inner, interceptor))
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
        pub async fn status(
            &mut self,
            request: impl tonic::IntoRequest<super::StatusRequest>,
        ) -> Result<tonic::Response<super::StatusResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/proto.api.component.board.v1.BoardService/Status",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn set_gpio(
            &mut self,
            request: impl tonic::IntoRequest<super::SetGpioRequest>,
        ) -> Result<tonic::Response<super::SetGpioResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/proto.api.component.board.v1.BoardService/SetGPIO",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " GetGPIO gets the high/low state of the given pin of a board of the underlying robot."]
        pub async fn get_gpio(
            &mut self,
            request: impl tonic::IntoRequest<super::GetGpioRequest>,
        ) -> Result<tonic::Response<super::GetGpioResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/proto.api.component.board.v1.BoardService/GetGPIO",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " PWM gets the duty cycle of the given pin of a board of the underlying robot."]
        pub async fn pwm(
            &mut self,
            request: impl tonic::IntoRequest<super::PwmRequest>,
        ) -> Result<tonic::Response<super::PwmResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/proto.api.component.board.v1.BoardService/PWM",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " SetPWM sets the given pin of a board of the underlying robot to the given duty cycle."]
        pub async fn set_pwm(
            &mut self,
            request: impl tonic::IntoRequest<super::SetPwmRequest>,
        ) -> Result<tonic::Response<super::SetPwmResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/proto.api.component.board.v1.BoardService/SetPWM",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " PWMFrequency gets the PWM frequency of the given pin of a board of the underlying robot."]
        pub async fn pwm_frequency(
            &mut self,
            request: impl tonic::IntoRequest<super::PwmFrequencyRequest>,
        ) -> Result<tonic::Response<super::PwmFrequencyResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/proto.api.component.board.v1.BoardService/PWMFrequency",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " SetPWMFrequency sets the given pin of a board of the underlying robot to the given PWM frequency. 0 will use the board's default PWM frequency."]
        pub async fn set_pwm_frequency(
            &mut self,
            request: impl tonic::IntoRequest<super::SetPwmFrequencyRequest>,
        ) -> Result<tonic::Response<super::SetPwmFrequencyResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/proto.api.component.board.v1.BoardService/SetPWMFrequency",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " ReadAnalogReader reads off the current value of an analog reader of a board of the underlying robot."]
        pub async fn read_analog_reader(
            &mut self,
            request: impl tonic::IntoRequest<super::ReadAnalogReaderRequest>,
        ) -> Result<tonic::Response<super::ReadAnalogReaderResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/proto.api.component.board.v1.BoardService/ReadAnalogReader",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " GetDigitalInterruptValue returns the current value of the interrupt which is based on the type of interrupt."]
        pub async fn get_digital_interrupt_value(
            &mut self,
            request: impl tonic::IntoRequest<super::GetDigitalInterruptValueRequest>,
        ) -> Result<tonic::Response<super::GetDigitalInterruptValueResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/proto.api.component.board.v1.BoardService/GetDigitalInterruptValue",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
#[doc = r" Generated server implementations."]
pub mod board_service_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = "Generated trait containing gRPC methods that should be implemented for use with BoardServiceServer."]
    #[async_trait]
    pub trait BoardService: Send + Sync + 'static {
        async fn status(
            &self,
            request: tonic::Request<super::StatusRequest>,
        ) -> Result<tonic::Response<super::StatusResponse>, tonic::Status>;
        async fn set_gpio(
            &self,
            request: tonic::Request<super::SetGpioRequest>,
        ) -> Result<tonic::Response<super::SetGpioResponse>, tonic::Status>;
        #[doc = " GetGPIO gets the high/low state of the given pin of a board of the underlying robot."]
        async fn get_gpio(
            &self,
            request: tonic::Request<super::GetGpioRequest>,
        ) -> Result<tonic::Response<super::GetGpioResponse>, tonic::Status>;
        #[doc = " PWM gets the duty cycle of the given pin of a board of the underlying robot."]
        async fn pwm(
            &self,
            request: tonic::Request<super::PwmRequest>,
        ) -> Result<tonic::Response<super::PwmResponse>, tonic::Status>;
        #[doc = " SetPWM sets the given pin of a board of the underlying robot to the given duty cycle."]
        async fn set_pwm(
            &self,
            request: tonic::Request<super::SetPwmRequest>,
        ) -> Result<tonic::Response<super::SetPwmResponse>, tonic::Status>;
        #[doc = " PWMFrequency gets the PWM frequency of the given pin of a board of the underlying robot."]
        async fn pwm_frequency(
            &self,
            request: tonic::Request<super::PwmFrequencyRequest>,
        ) -> Result<tonic::Response<super::PwmFrequencyResponse>, tonic::Status>;
        #[doc = " SetPWMFrequency sets the given pin of a board of the underlying robot to the given PWM frequency. 0 will use the board's default PWM frequency."]
        async fn set_pwm_frequency(
            &self,
            request: tonic::Request<super::SetPwmFrequencyRequest>,
        ) -> Result<tonic::Response<super::SetPwmFrequencyResponse>, tonic::Status>;
        #[doc = " ReadAnalogReader reads off the current value of an analog reader of a board of the underlying robot."]
        async fn read_analog_reader(
            &self,
            request: tonic::Request<super::ReadAnalogReaderRequest>,
        ) -> Result<tonic::Response<super::ReadAnalogReaderResponse>, tonic::Status>;
        #[doc = " GetDigitalInterruptValue returns the current value of the interrupt which is based on the type of interrupt."]
        async fn get_digital_interrupt_value(
            &self,
            request: tonic::Request<super::GetDigitalInterruptValueRequest>,
        ) -> Result<tonic::Response<super::GetDigitalInterruptValueResponse>, tonic::Status>;
    }
    #[doc = " BoardService services all Boards associated with a robot"]
    #[derive(Debug)]
    pub struct BoardServiceServer<T: BoardService> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: BoardService> BoardServiceServer<T> {
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
    impl<T, B> tonic::codegen::Service<http::Request<B>> for BoardServiceServer<T>
    where
        T: BoardService,
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
                "/proto.api.component.board.v1.BoardService/Status" => {
                    #[allow(non_camel_case_types)]
                    struct StatusSvc<T: BoardService>(pub Arc<T>);
                    impl<T: BoardService> tonic::server::UnaryService<super::StatusRequest> for StatusSvc<T> {
                        type Response = super::StatusResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::StatusRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).status(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = StatusSvc(inner);
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
                "/proto.api.component.board.v1.BoardService/SetGPIO" => {
                    #[allow(non_camel_case_types)]
                    struct SetGPIOSvc<T: BoardService>(pub Arc<T>);
                    impl<T: BoardService> tonic::server::UnaryService<super::SetGpioRequest> for SetGPIOSvc<T> {
                        type Response = super::SetGpioResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SetGpioRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).set_gpio(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = SetGPIOSvc(inner);
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
                "/proto.api.component.board.v1.BoardService/GetGPIO" => {
                    #[allow(non_camel_case_types)]
                    struct GetGPIOSvc<T: BoardService>(pub Arc<T>);
                    impl<T: BoardService> tonic::server::UnaryService<super::GetGpioRequest> for GetGPIOSvc<T> {
                        type Response = super::GetGpioResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetGpioRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).get_gpio(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetGPIOSvc(inner);
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
                "/proto.api.component.board.v1.BoardService/PWM" => {
                    #[allow(non_camel_case_types)]
                    struct PWMSvc<T: BoardService>(pub Arc<T>);
                    impl<T: BoardService> tonic::server::UnaryService<super::PwmRequest> for PWMSvc<T> {
                        type Response = super::PwmResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::PwmRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).pwm(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = PWMSvc(inner);
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
                "/proto.api.component.board.v1.BoardService/SetPWM" => {
                    #[allow(non_camel_case_types)]
                    struct SetPWMSvc<T: BoardService>(pub Arc<T>);
                    impl<T: BoardService> tonic::server::UnaryService<super::SetPwmRequest> for SetPWMSvc<T> {
                        type Response = super::SetPwmResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SetPwmRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).set_pwm(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = SetPWMSvc(inner);
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
                "/proto.api.component.board.v1.BoardService/PWMFrequency" => {
                    #[allow(non_camel_case_types)]
                    struct PWMFrequencySvc<T: BoardService>(pub Arc<T>);
                    impl<T: BoardService> tonic::server::UnaryService<super::PwmFrequencyRequest>
                        for PWMFrequencySvc<T>
                    {
                        type Response = super::PwmFrequencyResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::PwmFrequencyRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).pwm_frequency(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = PWMFrequencySvc(inner);
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
                "/proto.api.component.board.v1.BoardService/SetPWMFrequency" => {
                    #[allow(non_camel_case_types)]
                    struct SetPWMFrequencySvc<T: BoardService>(pub Arc<T>);
                    impl<T: BoardService> tonic::server::UnaryService<super::SetPwmFrequencyRequest>
                        for SetPWMFrequencySvc<T>
                    {
                        type Response = super::SetPwmFrequencyResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SetPwmFrequencyRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).set_pwm_frequency(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = SetPWMFrequencySvc(inner);
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
                "/proto.api.component.board.v1.BoardService/ReadAnalogReader" => {
                    #[allow(non_camel_case_types)]
                    struct ReadAnalogReaderSvc<T: BoardService>(pub Arc<T>);
                    impl<T: BoardService>
                        tonic::server::UnaryService<super::ReadAnalogReaderRequest>
                        for ReadAnalogReaderSvc<T>
                    {
                        type Response = super::ReadAnalogReaderResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ReadAnalogReaderRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).read_analog_reader(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ReadAnalogReaderSvc(inner);
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
                "/proto.api.component.board.v1.BoardService/GetDigitalInterruptValue" => {
                    #[allow(non_camel_case_types)]
                    struct GetDigitalInterruptValueSvc<T: BoardService>(pub Arc<T>);
                    impl<T: BoardService>
                        tonic::server::UnaryService<super::GetDigitalInterruptValueRequest>
                        for GetDigitalInterruptValueSvc<T>
                    {
                        type Response = super::GetDigitalInterruptValueResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetDigitalInterruptValueRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut =
                                async move { (*inner).get_digital_interrupt_value(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetDigitalInterruptValueSvc(inner);
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
    impl<T: BoardService> Clone for BoardServiceServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
            }
        }
    }
    impl<T: BoardService> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: BoardService> tonic::transport::NamedService for BoardServiceServer<T> {
        const NAME: &'static str = "proto.api.component.board.v1.BoardService";
    }
}
