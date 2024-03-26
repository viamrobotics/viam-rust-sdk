// @generated
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetLinearVelocityRequest {
    /// Name of a movement sensor
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// Additional arguments to the method
    #[prost(message, optional, tag="99")]
    pub extra: ::core::option::Option<::prost_types::Struct>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetLinearVelocityResponse {
    /// Linear velocity in m/s across x/y/z axes
    #[prost(message, optional, tag="1")]
    pub linear_velocity: ::core::option::Option<super::super::super::common::v1::Vector3>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetAngularVelocityRequest {
    /// Name of a movement sensor
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// Additional arguments to the method
    #[prost(message, optional, tag="99")]
    pub extra: ::core::option::Option<::prost_types::Struct>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetAngularVelocityResponse {
    /// Angular velocity in degrees/s across x/y/z axes
    #[prost(message, optional, tag="1")]
    pub angular_velocity: ::core::option::Option<super::super::super::common::v1::Vector3>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetCompassHeadingRequest {
    /// Name of a movement sensor
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// Additional arguments to the method
    #[prost(message, optional, tag="99")]
    pub extra: ::core::option::Option<::prost_types::Struct>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetCompassHeadingResponse {
    /// A number from 0-359 in degrees where
    /// 0 is North, 90 is East, 180 is South, and 270 is   West
    #[prost(double, tag="1")]
    pub value: f64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetOrientationRequest {
    /// Name of a movement sensor
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// Additional arguments to the method
    #[prost(message, optional, tag="99")]
    pub extra: ::core::option::Option<::prost_types::Struct>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetOrientationResponse {
    /// Orientation is returned as an orientation message with
    /// OX OY OZ as unit-normalized components of the axis of the vector, and Theta in degrees
    #[prost(message, optional, tag="1")]
    pub orientation: ::core::option::Option<super::super::super::common::v1::Orientation>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetPositionRequest {
    /// Name of a movement sensor
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// Additional arguments to the method
    #[prost(message, optional, tag="99")]
    pub extra: ::core::option::Option<::prost_types::Struct>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetPositionResponse {
    /// Position is returned in a coordinate of latitute and longitude
    /// and an altidue in meters
    #[prost(message, optional, tag="1")]
    pub coordinate: ::core::option::Option<super::super::super::common::v1::GeoPoint>,
    #[prost(float, tag="2")]
    pub altitude_m: f32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetPropertiesRequest {
    /// Name of a movement sensor
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// Additional arguments to the method
    #[prost(message, optional, tag="99")]
    pub extra: ::core::option::Option<::prost_types::Struct>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetPropertiesResponse {
    #[prost(bool, tag="1")]
    pub linear_velocity_supported: bool,
    #[prost(bool, tag="2")]
    pub angular_velocity_supported: bool,
    #[prost(bool, tag="3")]
    pub orientation_supported: bool,
    #[prost(bool, tag="4")]
    pub position_supported: bool,
    #[prost(bool, tag="5")]
    pub compass_heading_supported: bool,
    #[prost(bool, tag="6")]
    pub linear_acceleration_supported: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetAccuracyRequest {
    /// Name of a movement sensor
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// Additional arguments to the method
    #[prost(message, optional, tag="99")]
    pub extra: ::core::option::Option<::prost_types::Struct>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetAccuracyResponse {
    #[prost(map="string, float", tag="1")]
    pub accuracy: ::std::collections::HashMap<::prost::alloc::string::String, f32>,
    #[prost(float, optional, tag="2")]
    pub position_hdop: ::core::option::Option<f32>,
    #[prost(float, optional, tag="3")]
    pub position_vdop: ::core::option::Option<f32>,
    #[prost(int32, optional, tag="4")]
    pub position_nmea_gga_fix: ::core::option::Option<i32>,
    #[prost(float, optional, tag="5")]
    pub compass_degrees_error: ::core::option::Option<f32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetLinearAccelerationRequest {
    /// Name of a movement sensor
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// Additional arguments to the method
    #[prost(message, optional, tag="99")]
    pub extra: ::core::option::Option<::prost_types::Struct>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetLinearAccelerationResponse {
    /// Linear acceleration in m/s across x/y/z axes
    #[prost(message, optional, tag="1")]
    pub linear_acceleration: ::core::option::Option<super::super::super::common::v1::Vector3>,
}
// @@protoc_insertion_point(module)
