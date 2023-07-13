// @generated
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetDetectionsRequest {
    /// name of the vision service
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// the image, encoded as bytes
    #[prost(bytes="vec", tag="2")]
    pub image: ::prost::alloc::vec::Vec<u8>,
    /// the width of the image
    #[prost(int64, tag="3")]
    pub width: i64,
    /// the height of the image
    #[prost(int64, tag="4")]
    pub height: i64,
    /// the actual MIME type of image
    #[prost(string, tag="5")]
    pub mime_type: ::prost::alloc::string::String,
    /// Additional arguments to the method
    #[prost(message, optional, tag="99")]
    pub extra: ::core::option::Option<::prost_types::Struct>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetDetectionsResponse {
    /// the bounding boxes and labels
    #[prost(message, repeated, tag="1")]
    pub detections: ::prost::alloc::vec::Vec<Detection>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetDetectionsFromCameraRequest {
    /// name of the vision service
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// name of camera source to use as input
    #[prost(string, tag="2")]
    pub camera_name: ::prost::alloc::string::String,
    #[prost(message, optional, tag="99")]
    pub extra: ::core::option::Option<::prost_types::Struct>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetDetectionsFromCameraResponse {
    /// the bounding boxes and labels
    #[prost(message, repeated, tag="1")]
    pub detections: ::prost::alloc::vec::Vec<Detection>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Detection {
    /// the four corners of the box
    #[prost(int64, optional, tag="1")]
    pub x_min: ::core::option::Option<i64>,
    #[prost(int64, optional, tag="2")]
    pub y_min: ::core::option::Option<i64>,
    #[prost(int64, optional, tag="3")]
    pub x_max: ::core::option::Option<i64>,
    #[prost(int64, optional, tag="4")]
    pub y_max: ::core::option::Option<i64>,
    /// the confidence of the detection
    #[prost(double, tag="5")]
    pub confidence: f64,
    /// label associated with the detected object
    #[prost(string, tag="6")]
    pub class_name: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetClassificationsRequest {
    /// name of the vision service
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// the image encoded as bytes
    #[prost(bytes="vec", tag="2")]
    pub image: ::prost::alloc::vec::Vec<u8>,
    /// the width of the image
    #[prost(int32, tag="3")]
    pub width: i32,
    /// the height of the image
    #[prost(int32, tag="4")]
    pub height: i32,
    /// the actual MIME type of image
    #[prost(string, tag="5")]
    pub mime_type: ::prost::alloc::string::String,
    /// the number of classifications desired
    #[prost(int32, tag="6")]
    pub n: i32,
    /// Additional arguments to the method
    #[prost(message, optional, tag="99")]
    pub extra: ::core::option::Option<::prost_types::Struct>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetClassificationsResponse {
    #[prost(message, repeated, tag="1")]
    pub classifications: ::prost::alloc::vec::Vec<Classification>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetClassificationsFromCameraRequest {
    /// name of the vision service
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// the image encoded as bytes
    #[prost(string, tag="2")]
    pub camera_name: ::prost::alloc::string::String,
    /// the number of classifications desired
    #[prost(int32, tag="3")]
    pub n: i32,
    /// Additional arguments to the method
    #[prost(message, optional, tag="99")]
    pub extra: ::core::option::Option<::prost_types::Struct>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetClassificationsFromCameraResponse {
    #[prost(message, repeated, tag="1")]
    pub classifications: ::prost::alloc::vec::Vec<Classification>,
}
/// the general form of the output from a classifier
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Classification {
    /// the class name
    #[prost(string, tag="1")]
    pub class_name: ::prost::alloc::string::String,
    /// the confidence score of the classification
    #[prost(double, tag="2")]
    pub confidence: f64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetObjectPointCloudsRequest {
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// Name of a camera
    #[prost(string, tag="2")]
    pub camera_name: ::prost::alloc::string::String,
    /// Requested MIME type of response
    #[prost(string, tag="3")]
    pub mime_type: ::prost::alloc::string::String,
    /// Additional arguments to the method
    #[prost(message, optional, tag="99")]
    pub extra: ::core::option::Option<::prost_types::Struct>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetObjectPointCloudsResponse {
    /// Actual MIME type of response
    #[prost(string, tag="1")]
    pub mime_type: ::prost::alloc::string::String,
    /// List of objects in the scene
    #[prost(message, repeated, tag="2")]
    pub objects: ::prost::alloc::vec::Vec<super::super::super::common::v1::PointCloudObject>,
}
// @@protoc_insertion_point(module)
