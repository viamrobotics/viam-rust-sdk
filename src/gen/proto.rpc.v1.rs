#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Credentials {
    /// type is the type of credentials being used.
    #[prost(string, tag = "1")]
    pub r#type: ::prost::alloc::string::String,
    /// payload is an opaque string used that are of the given type above.
    #[prost(string, tag = "2")]
    pub payload: ::prost::alloc::string::String,
}
/// An AuthenticateRequest contains the credentials used to authenticate.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AuthenticateRequest {
    #[prost(string, tag = "1")]
    pub entity: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub credentials: ::core::option::Option<Credentials>,
}
/// An AuthenticateResponse is returned after successful authentication.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AuthenticateResponse {
    /// access_token is a JWT where only the expiration should be deemed
    /// important.
    ///
    /// Future(erd): maybe a refresh_token
    #[prost(string, tag = "1")]
    pub access_token: ::prost::alloc::string::String,
}
/// An AuthenticateToRequest contains the entity to authenticate to.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AuthenticateToRequest {
    #[prost(string, tag = "1")]
    pub entity: ::prost::alloc::string::String,
}
/// An AuthenticateResponse is returned after successful authentication.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AuthenticateToResponse {
    /// access_token is a JWT where only the expiration should be deemed
    /// important.
    ///
    /// Future(erd): maybe a refresh_token
    #[prost(string, tag = "1")]
    pub access_token: ::prost::alloc::string::String,
}
#[doc = r" Generated client implementations."]
pub mod auth_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = " An AuthService is intended to be used as a means to perform application level"]
    #[doc = " authentication. Its sole Authenticate method should be used prior to any"]
    #[doc = " other services that a gRPC server has to offer."]
    #[derive(Debug, Clone)]
    pub struct AuthServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl AuthServiceClient<tonic::transport::Channel> {
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
    impl<T> AuthServiceClient<T>
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
        ) -> AuthServiceClient<InterceptedService<T, F>>
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
            AuthServiceClient::new(InterceptedService::new(inner, interceptor))
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
        #[doc = " Authenticate attempts to authenticate the caller. The resulting"]
        #[doc = " response contains an access token that should be used for future"]
        #[doc = " requests."]
        pub async fn authenticate(
            &mut self,
            request: impl tonic::IntoRequest<super::AuthenticateRequest>,
        ) -> Result<tonic::Response<super::AuthenticateResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/proto.rpc.v1.AuthService/Authenticate");
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
#[doc = r" Generated client implementations."]
pub mod external_auth_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = " An ExternalAuthService is intended to be used as a means to perform application level"]
    #[doc = " authentication but for an external entity that it is responsible for handling auth for."]
    #[doc = " An example of its use would be for a entity that only trusts an external source to"]
    #[doc = " perform authentication for it."]
    #[doc = " Its sole AuthenticateTo method should be used prior to any other services that the external"]
    #[doc = " entity's gRPC server has to offer."]
    #[derive(Debug, Clone)]
    pub struct ExternalAuthServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl ExternalAuthServiceClient<tonic::transport::Channel> {
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
    impl<T> ExternalAuthServiceClient<T>
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
        ) -> ExternalAuthServiceClient<InterceptedService<T, F>>
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
            ExternalAuthServiceClient::new(InterceptedService::new(inner, interceptor))
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
        #[doc = " AuthenticateTo attempts to authenticate the caller on behalf of an entity."]
        #[doc = " The resulting response contains an access token that should be used for future"]
        #[doc = " requests. This assumes that the caller is already authenticated to the"]
        #[doc = " server implementing this service."]
        pub async fn authenticate_to(
            &mut self,
            request: impl tonic::IntoRequest<super::AuthenticateToRequest>,
        ) -> Result<tonic::Response<super::AuthenticateToResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/proto.rpc.v1.ExternalAuthService/AuthenticateTo",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
#[doc = r" Generated server implementations."]
pub mod auth_service_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = "Generated trait containing gRPC methods that should be implemented for use with AuthServiceServer."]
    #[async_trait]
    pub trait AuthService: Send + Sync + 'static {
        #[doc = " Authenticate attempts to authenticate the caller. The resulting"]
        #[doc = " response contains an access token that should be used for future"]
        #[doc = " requests."]
        async fn authenticate(
            &self,
            request: tonic::Request<super::AuthenticateRequest>,
        ) -> Result<tonic::Response<super::AuthenticateResponse>, tonic::Status>;
    }
    #[doc = " An AuthService is intended to be used as a means to perform application level"]
    #[doc = " authentication. Its sole Authenticate method should be used prior to any"]
    #[doc = " other services that a gRPC server has to offer."]
    #[derive(Debug)]
    pub struct AuthServiceServer<T: AuthService> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: AuthService> AuthServiceServer<T> {
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
    impl<T, B> tonic::codegen::Service<http::Request<B>> for AuthServiceServer<T>
    where
        T: AuthService,
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
                "/proto.rpc.v1.AuthService/Authenticate" => {
                    #[allow(non_camel_case_types)]
                    struct AuthenticateSvc<T: AuthService>(pub Arc<T>);
                    impl<T: AuthService> tonic::server::UnaryService<super::AuthenticateRequest>
                        for AuthenticateSvc<T>
                    {
                        type Response = super::AuthenticateResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::AuthenticateRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).authenticate(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = AuthenticateSvc(inner);
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
    impl<T: AuthService> Clone for AuthServiceServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
            }
        }
    }
    impl<T: AuthService> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: AuthService> tonic::transport::NamedService for AuthServiceServer<T> {
        const NAME: &'static str = "proto.rpc.v1.AuthService";
    }
}
#[doc = r" Generated server implementations."]
pub mod external_auth_service_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = "Generated trait containing gRPC methods that should be implemented for use with ExternalAuthServiceServer."]
    #[async_trait]
    pub trait ExternalAuthService: Send + Sync + 'static {
        #[doc = " AuthenticateTo attempts to authenticate the caller on behalf of an entity."]
        #[doc = " The resulting response contains an access token that should be used for future"]
        #[doc = " requests. This assumes that the caller is already authenticated to the"]
        #[doc = " server implementing this service."]
        async fn authenticate_to(
            &self,
            request: tonic::Request<super::AuthenticateToRequest>,
        ) -> Result<tonic::Response<super::AuthenticateToResponse>, tonic::Status>;
    }
    #[doc = " An ExternalAuthService is intended to be used as a means to perform application level"]
    #[doc = " authentication but for an external entity that it is responsible for handling auth for."]
    #[doc = " An example of its use would be for a entity that only trusts an external source to"]
    #[doc = " perform authentication for it."]
    #[doc = " Its sole AuthenticateTo method should be used prior to any other services that the external"]
    #[doc = " entity's gRPC server has to offer."]
    #[derive(Debug)]
    pub struct ExternalAuthServiceServer<T: ExternalAuthService> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: ExternalAuthService> ExternalAuthServiceServer<T> {
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
    impl<T, B> tonic::codegen::Service<http::Request<B>> for ExternalAuthServiceServer<T>
    where
        T: ExternalAuthService,
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
                "/proto.rpc.v1.ExternalAuthService/AuthenticateTo" => {
                    #[allow(non_camel_case_types)]
                    struct AuthenticateToSvc<T: ExternalAuthService>(pub Arc<T>);
                    impl<T: ExternalAuthService>
                        tonic::server::UnaryService<super::AuthenticateToRequest>
                        for AuthenticateToSvc<T>
                    {
                        type Response = super::AuthenticateToResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::AuthenticateToRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).authenticate_to(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = AuthenticateToSvc(inner);
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
    impl<T: ExternalAuthService> Clone for ExternalAuthServiceServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
            }
        }
    }
    impl<T: ExternalAuthService> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: ExternalAuthService> tonic::transport::NamedService for ExternalAuthServiceServer<T> {
        const NAME: &'static str = "proto.rpc.v1.ExternalAuthService";
    }
}
