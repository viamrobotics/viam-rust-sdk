#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetControlsRequest {
    /// Name of an input controller
    #[prost(string, tag = "1")]
    pub controller: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetControlsResponse {
    /// Returns a list of all the controls (buttons and axes) that are
    /// available to a given Input Controller
    #[prost(string, repeated, tag = "1")]
    pub controls: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetEventsRequest {
    /// Name of an input controller
    #[prost(string, tag = "1")]
    pub controller: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetEventsResponse {
    /// Returns a list of the most recent event for each control on a given InputController. Effectively provides the current "state" of all
    /// buttons/axes on a given input controller
    #[prost(message, repeated, tag = "1")]
    pub events: ::prost::alloc::vec::Vec<Event>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TriggerEventRequest {
    /// Name of an input controller
    #[prost(string, tag = "1")]
    pub controller: ::prost::alloc::string::String,
    /// Digitally assert a given event
    #[prost(message, optional, tag = "2")]
    pub event: ::core::option::Option<Event>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TriggerEventResponse {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Event {
    /// Timestamp of event
    #[prost(message, optional, tag = "1")]
    pub time: ::core::option::Option<::prost_types::Timestamp>,
    /// An event type (eg: ButtonPress, ButtonRelease)
    #[prost(string, tag = "2")]
    pub event: ::prost::alloc::string::String,
    /// A control, can be a button (eg: ButtonSouth) or an axis (eg: AbsoluteX)
    #[prost(string, tag = "3")]
    pub control: ::prost::alloc::string::String,
    /// 0 or 1 for buttons, -1.0 to +1.0 for axes
    #[prost(double, tag = "4")]
    pub value: f64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StreamEventsRequest {
    /// Name of an input controller
    #[prost(string, tag = "1")]
    pub controller: ::prost::alloc::string::String,
    /// A list of Events
    #[prost(message, repeated, tag = "2")]
    pub events: ::prost::alloc::vec::Vec<stream_events_request::Events>,
}
/// Nested message and enum types in `StreamEventsRequest`.
pub mod stream_events_request {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Events {
        /// Name of a control (button or axis)
        #[prost(string, tag = "1")]
        pub control: ::prost::alloc::string::String,
        /// Specify which event types to recieve events for
        /// To Do (FA): Right now this can be an empty list, but we should error in this case as opening a stream with no messages is expensive
        #[prost(string, repeated, tag = "2")]
        pub events: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
        /// Specify which event types to stop recieving events for
        /// This can be an empty list
        #[prost(string, repeated, tag = "3")]
        pub cancelled_events: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StreamEventsResponse {
    /// Event for a controller
    #[prost(message, optional, tag = "1")]
    pub event: ::core::option::Option<Event>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Status {
    #[prost(message, repeated, tag = "1")]
    pub events: ::prost::alloc::vec::Vec<Event>,
}
#[doc = r" Generated client implementations."]
pub mod input_controller_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = " InputControllerService servicestains all input controller associated with a robot"]
    #[derive(Debug, Clone)]
    pub struct InputControllerServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl InputControllerServiceClient<tonic::transport::Channel> {
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
    impl<T> InputControllerServiceClient<T>
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
        ) -> InputControllerServiceClient<InterceptedService<T, F>>
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
            InputControllerServiceClient::new(InterceptedService::new(inner, interceptor))
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
        #[doc = " GetControls returns a list of GetControls provided by the Controller"]
        pub async fn get_controls(
            &mut self,
            request: impl tonic::IntoRequest<super::GetControlsRequest>,
        ) -> Result<tonic::Response<super::GetControlsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/proto.api.component.inputcontroller.v1.InputControllerService/GetControls",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " GetEvents returns a list of events representing the last event on each control of a give Input Controller"]
        pub async fn get_events(
            &mut self,
            request: impl tonic::IntoRequest<super::GetEventsRequest>,
        ) -> Result<tonic::Response<super::GetEventsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/proto.api.component.inputcontroller.v1.InputControllerService/GetEvents",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " StreamEvents starts a stream of InputControllerEvents for the given controls (buttons/axes) on a robot's input controller "]
        pub async fn stream_events(
            &mut self,
            request: impl tonic::IntoRequest<super::StreamEventsRequest>,
        ) -> Result<
            tonic::Response<tonic::codec::Streaming<super::StreamEventsResponse>>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/proto.api.component.inputcontroller.v1.InputControllerService/StreamEvents",
            );
            self.inner
                .server_streaming(request.into_request(), path, codec)
                .await
        }
        #[doc = " TriggerEvent, where supported, injects an InputControllerEvent into an input controller to (virtually) generate events "]
        #[doc = " like button presses or axis movements  "]
        pub async fn trigger_event(
            &mut self,
            request: impl tonic::IntoRequest<super::TriggerEventRequest>,
        ) -> Result<tonic::Response<super::TriggerEventResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/proto.api.component.inputcontroller.v1.InputControllerService/TriggerEvent",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
#[doc = r" Generated server implementations."]
pub mod input_controller_service_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = "Generated trait containing gRPC methods that should be implemented for use with InputControllerServiceServer."]
    #[async_trait]
    pub trait InputControllerService: Send + Sync + 'static {
        #[doc = " GetControls returns a list of GetControls provided by the Controller"]
        async fn get_controls(
            &self,
            request: tonic::Request<super::GetControlsRequest>,
        ) -> Result<tonic::Response<super::GetControlsResponse>, tonic::Status>;
        #[doc = " GetEvents returns a list of events representing the last event on each control of a give Input Controller"]
        async fn get_events(
            &self,
            request: tonic::Request<super::GetEventsRequest>,
        ) -> Result<tonic::Response<super::GetEventsResponse>, tonic::Status>;
        #[doc = "Server streaming response type for the StreamEvents method."]
        type StreamEventsStream: futures_core::Stream<Item = Result<super::StreamEventsResponse, tonic::Status>>
            + Send
            + 'static;
        #[doc = " StreamEvents starts a stream of InputControllerEvents for the given controls (buttons/axes) on a robot's input controller "]
        async fn stream_events(
            &self,
            request: tonic::Request<super::StreamEventsRequest>,
        ) -> Result<tonic::Response<Self::StreamEventsStream>, tonic::Status>;
        #[doc = " TriggerEvent, where supported, injects an InputControllerEvent into an input controller to (virtually) generate events "]
        #[doc = " like button presses or axis movements  "]
        async fn trigger_event(
            &self,
            request: tonic::Request<super::TriggerEventRequest>,
        ) -> Result<tonic::Response<super::TriggerEventResponse>, tonic::Status>;
    }
    #[doc = " InputControllerService servicestains all input controller associated with a robot"]
    #[derive(Debug)]
    pub struct InputControllerServiceServer<T: InputControllerService> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: InputControllerService> InputControllerServiceServer<T> {
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
    impl<T, B> tonic::codegen::Service<http::Request<B>> for InputControllerServiceServer<T>
    where
        T: InputControllerService,
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
                "/proto.api.component.inputcontroller.v1.InputControllerService/GetControls" => {
                    #[allow(non_camel_case_types)]
                    struct GetControlsSvc<T: InputControllerService>(pub Arc<T>);
                    impl<T: InputControllerService>
                        tonic::server::UnaryService<super::GetControlsRequest>
                        for GetControlsSvc<T>
                    {
                        type Response = super::GetControlsResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetControlsRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).get_controls(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetControlsSvc(inner);
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
                "/proto.api.component.inputcontroller.v1.InputControllerService/GetEvents" => {
                    #[allow(non_camel_case_types)]
                    struct GetEventsSvc<T: InputControllerService>(pub Arc<T>);
                    impl<T: InputControllerService>
                        tonic::server::UnaryService<super::GetEventsRequest> for GetEventsSvc<T>
                    {
                        type Response = super::GetEventsResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetEventsRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).get_events(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetEventsSvc(inner);
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
                "/proto.api.component.inputcontroller.v1.InputControllerService/StreamEvents" => {
                    #[allow(non_camel_case_types)]
                    struct StreamEventsSvc<T: InputControllerService>(pub Arc<T>);
                    impl<T: InputControllerService>
                        tonic::server::ServerStreamingService<super::StreamEventsRequest>
                        for StreamEventsSvc<T>
                    {
                        type Response = super::StreamEventsResponse;
                        type ResponseStream = T::StreamEventsStream;
                        type Future =
                            BoxFuture<tonic::Response<Self::ResponseStream>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::StreamEventsRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).stream_events(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = StreamEventsSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.server_streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/proto.api.component.inputcontroller.v1.InputControllerService/TriggerEvent" => {
                    #[allow(non_camel_case_types)]
                    struct TriggerEventSvc<T: InputControllerService>(pub Arc<T>);
                    impl<T: InputControllerService>
                        tonic::server::UnaryService<super::TriggerEventRequest>
                        for TriggerEventSvc<T>
                    {
                        type Response = super::TriggerEventResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::TriggerEventRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).trigger_event(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = TriggerEventSvc(inner);
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
    impl<T: InputControllerService> Clone for InputControllerServiceServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
            }
        }
    }
    impl<T: InputControllerService> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: InputControllerService> tonic::transport::NamedService for InputControllerServiceServer<T> {
        const NAME: &'static str = "proto.api.component.inputcontroller.v1.InputControllerService";
    }
}
