// @generated
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetImageRequest {
    /// Name of a camera
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// Requested MIME type of response
    #[prost(string, tag="2")]
    pub mime_type: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetImageResponse {
    /// Actual MIME type of response
    #[prost(string, tag="1")]
    pub mime_type: ::prost::alloc::string::String,
    /// Frame in bytes
    #[prost(bytes="vec", tag="2")]
    pub image: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RenderFrameRequest {
    /// Name of a camera
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// Requested MIME type of response
    #[prost(string, tag="2")]
    pub mime_type: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetPointCloudRequest {
    /// Name of a camera
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// Requested MIME type of response
    #[prost(string, tag="2")]
    pub mime_type: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetPointCloudResponse {
    /// Actual MIME type of response
    #[prost(string, tag="1")]
    pub mime_type: ::prost::alloc::string::String,
    /// Frame in bytes
    #[prost(bytes="vec", tag="2")]
    pub point_cloud: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetPropertiesRequest {
    /// Name of a camera
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetPropertiesResponse {
    /// A boolean property determining whether the camera supports the return of pointcloud data
    #[prost(bool, tag="1")]
    pub supports_pcd: bool,
    /// Parameters for doing a perspective of a 3D scene to a 2D plane
    /// If camera does not provide intrinsic parameters, leave the field empty
    /// Initializing the parameters with 0-values is considered an error
    #[prost(message, optional, tag="2")]
    pub intrinsic_parameters: ::core::option::Option<IntrinsicParameters>,
    /// Parameters for modeling lens distortion in cameras
    /// If camera does not provide distortion parameters, leave the field empty
    /// Initializing the parameters with 0-values is considered an error
    #[prost(message, optional, tag="3")]
    pub distortion_parameters: ::core::option::Option<DistortionParameters>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Webcams {
    #[prost(message, repeated, tag="1")]
    pub webcams: ::prost::alloc::vec::Vec<Webcam>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Webcam {
    /// Camera driver label (for internal use only)
    #[prost(string, tag="1")]
    pub label: ::prost::alloc::string::String,
    /// Camera driver status
    #[prost(string, tag="2")]
    pub status: ::prost::alloc::string::String,
    /// Camera properties
    #[prost(message, repeated, tag="3")]
    pub properties: ::prost::alloc::vec::Vec<Property>,
    /// Camera human-readable driver name
    #[prost(string, tag="4")]
    pub name: ::prost::alloc::string::String,
    /// Camera unique identifier
    #[prost(string, tag="5")]
    pub id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Property {
    /// Video resolution width in px
    #[prost(int32, tag="1")]
    pub width_px: i32,
    /// Video resolution height in px
    #[prost(int32, tag="2")]
    pub height_px: i32,
    /// Video frame format
    #[prost(string, tag="3")]
    pub frame_format: ::prost::alloc::string::String,
    /// Video frame rate in fps
    #[prost(float, tag="4")]
    pub frame_rate: f32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IntrinsicParameters {
    #[prost(uint32, tag="1")]
    pub width_px: u32,
    #[prost(uint32, tag="2")]
    pub height_px: u32,
    #[prost(double, tag="3")]
    pub focal_x_px: f64,
    #[prost(double, tag="4")]
    pub focal_y_px: f64,
    #[prost(double, tag="5")]
    pub center_x_px: f64,
    #[prost(double, tag="6")]
    pub center_y_px: f64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DistortionParameters {
    #[prost(string, tag="1")]
    pub model: ::prost::alloc::string::String,
    #[prost(double, repeated, tag="2")]
    pub parameters: ::prost::alloc::vec::Vec<f64>,
}
// @@protoc_insertion_point(module)
