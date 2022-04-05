#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetObjectPointCloudsRequest {
    /// Name of a camera
    #[prost(string, tag = "1")]
    pub camera_name: ::prost::alloc::string::String,
    /// Name of the segmentation algorithm
    #[prost(string, tag = "2")]
    pub segmenter_name: ::prost::alloc::string::String,
    /// Requested MIME type of response
    #[prost(string, tag = "3")]
    pub mime_type: ::prost::alloc::string::String,
    /// parameters for the chosen segmenter
    #[prost(message, optional, tag = "4")]
    pub parameters: ::core::option::Option<::prost_types::Struct>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetObjectPointCloudsResponse {
    /// Actual MIME type of response
    #[prost(string, tag = "1")]
    pub mime_type: ::prost::alloc::string::String,
    /// List of objects in the scene
    #[prost(message, repeated, tag = "2")]
    pub objects: ::prost::alloc::vec::Vec<super::super::super::common::v1::PointCloudObject>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetSegmenterParametersRequest {
    /// Name of the segmentation algo
    #[prost(string, tag = "1")]
    pub segmenter_name: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TypedParameter {
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub r#type: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetSegmenterParametersResponse {
    /// parameter names of the segmenter in the request
    #[prost(message, repeated, tag = "1")]
    pub parameters: ::prost::alloc::vec::Vec<TypedParameter>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetSegmentersRequest {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetSegmentersResponse {
    /// segmenters in the registry
    #[prost(string, repeated, tag = "1")]
    pub segmenters: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[doc = r" Generated client implementations."]
pub mod object_segmentation_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = " A ObjectSegmentationService declares the gRPC contract for an object segmentation service"]
    #[derive(Debug, Clone)]
    pub struct ObjectSegmentationServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl ObjectSegmentationServiceClient<tonic::transport::Channel> {
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
    impl<T> ObjectSegmentationServiceClient<T>
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
        ) -> ObjectSegmentationServiceClient<InterceptedService<T, F>>
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
            ObjectSegmentationServiceClient::new(InterceptedService::new(inner, interceptor))
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
        #[doc = " GetObjectPointClouds returns all the found objects in a pointcloud from a camera of the underlying robot,"]
        #[doc = " as well as the 3-vector center of each of the found objects."]
        #[doc = " A specific MIME type can be requested but may not necessarily be the same one returned."]
        pub async fn get_object_point_clouds(
            &mut self,
            request: impl tonic::IntoRequest<super::GetObjectPointCloudsRequest>,
        ) -> Result<tonic::Response<super::GetObjectPointCloudsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http :: uri :: PathAndQuery :: from_static ("/proto.api.service.objectsegmentation.v1.ObjectSegmentationService/GetObjectPointClouds") ;
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " GetSegmenterParameters returns the parameter fields needed for the given segmenter."]
        pub async fn get_segmenter_parameters(
            &mut self,
            request: impl tonic::IntoRequest<super::GetSegmenterParametersRequest>,
        ) -> Result<tonic::Response<super::GetSegmenterParametersResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http :: uri :: PathAndQuery :: from_static ("/proto.api.service.objectsegmentation.v1.ObjectSegmentationService/GetSegmenterParameters") ;
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " GetSegmenters returns the list of segmenters in the registry."]
        pub async fn get_segmenters(
            &mut self,
            request: impl tonic::IntoRequest<super::GetSegmentersRequest>,
        ) -> Result<tonic::Response<super::GetSegmentersResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/proto.api.service.objectsegmentation.v1.ObjectSegmentationService/GetSegmenters",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
#[doc = r" Generated server implementations."]
pub mod object_segmentation_service_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = "Generated trait containing gRPC methods that should be implemented for use with ObjectSegmentationServiceServer."]
    #[async_trait]
    pub trait ObjectSegmentationService: Send + Sync + 'static {
        #[doc = " GetObjectPointClouds returns all the found objects in a pointcloud from a camera of the underlying robot,"]
        #[doc = " as well as the 3-vector center of each of the found objects."]
        #[doc = " A specific MIME type can be requested but may not necessarily be the same one returned."]
        async fn get_object_point_clouds(
            &self,
            request: tonic::Request<super::GetObjectPointCloudsRequest>,
        ) -> Result<tonic::Response<super::GetObjectPointCloudsResponse>, tonic::Status>;
        #[doc = " GetSegmenterParameters returns the parameter fields needed for the given segmenter."]
        async fn get_segmenter_parameters(
            &self,
            request: tonic::Request<super::GetSegmenterParametersRequest>,
        ) -> Result<tonic::Response<super::GetSegmenterParametersResponse>, tonic::Status>;
        #[doc = " GetSegmenters returns the list of segmenters in the registry."]
        async fn get_segmenters(
            &self,
            request: tonic::Request<super::GetSegmentersRequest>,
        ) -> Result<tonic::Response<super::GetSegmentersResponse>, tonic::Status>;
    }
    #[doc = " A ObjectSegmentationService declares the gRPC contract for an object segmentation service"]
    #[derive(Debug)]
    pub struct ObjectSegmentationServiceServer<T: ObjectSegmentationService> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: ObjectSegmentationService> ObjectSegmentationServiceServer<T> {
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
    impl<T, B> tonic::codegen::Service<http::Request<B>> for ObjectSegmentationServiceServer<T>
    where
        T: ObjectSegmentationService,
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
            match req . uri () . path () { "/proto.api.service.objectsegmentation.v1.ObjectSegmentationService/GetObjectPointClouds" => { # [allow (non_camel_case_types)] struct GetObjectPointCloudsSvc < T : ObjectSegmentationService > (pub Arc < T >) ; impl < T : ObjectSegmentationService > tonic :: server :: UnaryService < super :: GetObjectPointCloudsRequest > for GetObjectPointCloudsSvc < T > { type Response = super :: GetObjectPointCloudsResponse ; type Future = BoxFuture < tonic :: Response < Self :: Response > , tonic :: Status > ; fn call (& mut self , request : tonic :: Request < super :: GetObjectPointCloudsRequest >) -> Self :: Future { let inner = self . 0 . clone () ; let fut = async move { (* inner) . get_object_point_clouds (request) . await } ; Box :: pin (fut) } } let accept_compression_encodings = self . accept_compression_encodings ; let send_compression_encodings = self . send_compression_encodings ; let inner = self . inner . clone () ; let fut = async move { let inner = inner . 0 ; let method = GetObjectPointCloudsSvc (inner) ; let codec = tonic :: codec :: ProstCodec :: default () ; let mut grpc = tonic :: server :: Grpc :: new (codec) . apply_compression_config (accept_compression_encodings , send_compression_encodings) ; let res = grpc . unary (method , req) . await ; Ok (res) } ; Box :: pin (fut) } "/proto.api.service.objectsegmentation.v1.ObjectSegmentationService/GetSegmenterParameters" => { # [allow (non_camel_case_types)] struct GetSegmenterParametersSvc < T : ObjectSegmentationService > (pub Arc < T >) ; impl < T : ObjectSegmentationService > tonic :: server :: UnaryService < super :: GetSegmenterParametersRequest > for GetSegmenterParametersSvc < T > { type Response = super :: GetSegmenterParametersResponse ; type Future = BoxFuture < tonic :: Response < Self :: Response > , tonic :: Status > ; fn call (& mut self , request : tonic :: Request < super :: GetSegmenterParametersRequest >) -> Self :: Future { let inner = self . 0 . clone () ; let fut = async move { (* inner) . get_segmenter_parameters (request) . await } ; Box :: pin (fut) } } let accept_compression_encodings = self . accept_compression_encodings ; let send_compression_encodings = self . send_compression_encodings ; let inner = self . inner . clone () ; let fut = async move { let inner = inner . 0 ; let method = GetSegmenterParametersSvc (inner) ; let codec = tonic :: codec :: ProstCodec :: default () ; let mut grpc = tonic :: server :: Grpc :: new (codec) . apply_compression_config (accept_compression_encodings , send_compression_encodings) ; let res = grpc . unary (method , req) . await ; Ok (res) } ; Box :: pin (fut) } "/proto.api.service.objectsegmentation.v1.ObjectSegmentationService/GetSegmenters" => { # [allow (non_camel_case_types)] struct GetSegmentersSvc < T : ObjectSegmentationService > (pub Arc < T >) ; impl < T : ObjectSegmentationService > tonic :: server :: UnaryService < super :: GetSegmentersRequest > for GetSegmentersSvc < T > { type Response = super :: GetSegmentersResponse ; type Future = BoxFuture < tonic :: Response < Self :: Response > , tonic :: Status > ; fn call (& mut self , request : tonic :: Request < super :: GetSegmentersRequest >) -> Self :: Future { let inner = self . 0 . clone () ; let fut = async move { (* inner) . get_segmenters (request) . await } ; Box :: pin (fut) } } let accept_compression_encodings = self . accept_compression_encodings ; let send_compression_encodings = self . send_compression_encodings ; let inner = self . inner . clone () ; let fut = async move { let inner = inner . 0 ; let method = GetSegmentersSvc (inner) ; let codec = tonic :: codec :: ProstCodec :: default () ; let mut grpc = tonic :: server :: Grpc :: new (codec) . apply_compression_config (accept_compression_encodings , send_compression_encodings) ; let res = grpc . unary (method , req) . await ; Ok (res) } ; Box :: pin (fut) } _ => Box :: pin (async move { Ok (http :: Response :: builder () . status (200) . header ("grpc-status" , "12") . header ("content-type" , "application/grpc") . body (empty_body ()) . unwrap ()) }) , }
        }
    }
    impl<T: ObjectSegmentationService> Clone for ObjectSegmentationServiceServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
            }
        }
    }
    impl<T: ObjectSegmentationService> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: ObjectSegmentationService> tonic::transport::NamedService
        for ObjectSegmentationServiceServer<T>
    {
        const NAME: &'static str =
            "proto.api.service.objectsegmentation.v1.ObjectSegmentationService";
    }
}
