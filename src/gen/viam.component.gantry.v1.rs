// @generated
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetPositionRequest {
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// Additional arguments to the method
    #[prost(message, optional, tag="99")]
    pub extra: ::core::option::Option<::prost_types::Struct>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetPositionResponse {
    #[prost(double, repeated, tag="1")]
    pub positions_mm: ::prost::alloc::vec::Vec<f64>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MoveToPositionRequest {
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// Number of millimeters to move the gantry by respective to each axis.
    #[prost(double, repeated, tag="2")]
    pub positions_mm: ::prost::alloc::vec::Vec<f64>,
    /// Speeds to move each gantry axis must match length and order of positions_mm.
    #[prost(double, repeated, tag="3")]
    pub speeds_mm_per_sec: ::prost::alloc::vec::Vec<f64>,
    /// Additional arguments to the method
    #[prost(message, optional, tag="99")]
    pub extra: ::core::option::Option<::prost_types::Struct>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MoveToPositionResponse {
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HomeRequest {
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// Additional arguments to the method
    #[prost(message, optional, tag="99")]
    pub extra: ::core::option::Option<::prost_types::Struct>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HomeResponse {
    /// A bool describing whether the gantry has completed homing
    #[prost(bool, tag="1")]
    pub homed: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetLengthsRequest {
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// Additional arguments to the method
    #[prost(message, optional, tag="99")]
    pub extra: ::core::option::Option<::prost_types::Struct>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetLengthsResponse {
    #[prost(double, repeated, tag="1")]
    pub lengths_mm: ::prost::alloc::vec::Vec<f64>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StopRequest {
    /// Name of a gantry
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
pub struct Status {
    #[prost(double, repeated, tag="1")]
    pub positions_mm: ::prost::alloc::vec::Vec<f64>,
    #[prost(double, repeated, tag="2")]
    pub lengths_mm: ::prost::alloc::vec::Vec<f64>,
    #[prost(bool, tag="3")]
    pub is_moving: bool,
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
// @@protoc_insertion_point(module)
