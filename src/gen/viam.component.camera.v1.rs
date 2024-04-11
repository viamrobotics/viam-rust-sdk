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
    /// Additional arguments to the method
    #[prost(message, optional, tag="99")]
    pub extra: ::core::option::Option<::prost_types::Struct>,
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
pub struct GetImagesRequest {
    /// Name of a camera
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetImagesResponse {
    /// list of images returned from the camera system
    #[prost(message, repeated, tag="1")]
    pub images: ::prost::alloc::vec::Vec<Image>,
    /// contains timestamp data
    #[prost(message, optional, tag="84260")]
    pub response_metadata: ::core::option::Option<super::super::super::common::v1::ResponseMetadata>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Image {
    /// the name of the sensor where the image came from
    #[prost(string, tag="1")]
    pub source_name: ::prost::alloc::string::String,
    /// format of the response image bytes
    #[prost(enumeration="Format", tag="2")]
    pub format: i32,
    /// image in bytes
    #[prost(bytes="vec", tag="3")]
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
    /// Additional arguments to the method
    #[prost(message, optional, tag="99")]
    pub extra: ::core::option::Option<::prost_types::Struct>,
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
    /// Additional arguments to the method
    #[prost(message, optional, tag="99")]
    pub extra: ::core::option::Option<::prost_types::Struct>,
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
    /// Supported MIME types by the camera
    #[prost(string, repeated, tag="4")]
    pub mime_types: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
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
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum Format {
    Unspecified = 0,
    RawRgba = 1,
    RawDepth = 2,
    Jpeg = 3,
    Png = 4,
}
impl Format {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Format::Unspecified => "FORMAT_UNSPECIFIED",
            Format::RawRgba => "FORMAT_RAW_RGBA",
            Format::RawDepth => "FORMAT_RAW_DEPTH",
            Format::Jpeg => "FORMAT_JPEG",
            Format::Png => "FORMAT_PNG",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "FORMAT_UNSPECIFIED" => Some(Self::Unspecified),
            "FORMAT_RAW_RGBA" => Some(Self::RawRgba),
            "FORMAT_RAW_DEPTH" => Some(Self::RawDepth),
            "FORMAT_JPEG" => Some(Self::Jpeg),
            "FORMAT_PNG" => Some(Self::Png),
            _ => None,
        }
    }
}
// @@protoc_insertion_point(module)
