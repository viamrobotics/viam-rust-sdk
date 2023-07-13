// @generated
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MoveStraightRequest {
    /// Name of a base
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// Desired travel distance in millimeters
    #[prost(int64, tag="2")]
    pub distance_mm: i64,
    /// Desired travel velocity in millimeters/second
    #[prost(double, tag="3")]
    pub mm_per_sec: f64,
    /// Additional arguments to the method
    #[prost(message, optional, tag="99")]
    pub extra: ::core::option::Option<::prost_types::Struct>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MoveStraightResponse {
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SpinRequest {
    /// Name of a base
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// Desired angle
    #[prost(double, tag="2")]
    pub angle_deg: f64,
    /// Desired angular velocity
    #[prost(double, tag="3")]
    pub degs_per_sec: f64,
    /// Additional arguments to the method
    #[prost(message, optional, tag="99")]
    pub extra: ::core::option::Option<::prost_types::Struct>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SpinResponse {
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StopRequest {
    /// Name of a base
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// Additional arguments to the method
    #[prost(message, optional, tag="99")]
    pub extra: ::core::option::Option<::prost_types::Struct>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StopResponse {
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetPowerRequest {
    /// Name of a base
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// Desired linear power percentage as -1 -> 1
    #[prost(message, optional, tag="2")]
    pub linear: ::core::option::Option<super::super::super::common::v1::Vector3>,
    /// Desired angular power percentage % as -1 -> 1
    #[prost(message, optional, tag="3")]
    pub angular: ::core::option::Option<super::super::super::common::v1::Vector3>,
    /// Additional arguments to the method
    #[prost(message, optional, tag="99")]
    pub extra: ::core::option::Option<::prost_types::Struct>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetPowerResponse {
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetVelocityRequest {
    /// Name of a base
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// Desired linear velocity in mm per second
    #[prost(message, optional, tag="2")]
    pub linear: ::core::option::Option<super::super::super::common::v1::Vector3>,
    /// Desired angular velocity in degrees per second
    #[prost(message, optional, tag="3")]
    pub angular: ::core::option::Option<super::super::super::common::v1::Vector3>,
    /// Additional arguments to the method
    #[prost(message, optional, tag="99")]
    pub extra: ::core::option::Option<::prost_types::Struct>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetVelocityResponse {
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IsMovingRequest {
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IsMovingResponse {
    #[prost(bool, tag="1")]
    pub is_moving: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetPropertiesRequest {
    /// Name of the base
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    #[prost(message, optional, tag="99")]
    pub extra: ::core::option::Option<::prost_types::Struct>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetPropertiesResponse {
    #[prost(double, tag="1")]
    pub width_meters: f64,
    #[prost(double, tag="2")]
    pub turning_radius_meters: f64,
}
// @@protoc_insertion_point(module)
