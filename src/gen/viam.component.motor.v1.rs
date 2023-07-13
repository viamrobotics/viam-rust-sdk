// @generated
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetPowerRequest {
    /// Name of a motor
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// Percentage of motor's power, between -1 and 1
    #[prost(double, tag="2")]
    pub power_pct: f64,
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
pub struct GoForRequest {
    /// Name of a motor
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// Speed of motor travel in rotations per minute
    #[prost(double, tag="2")]
    pub rpm: f64,
    /// Number of revolutions relative to motor's start position
    #[prost(double, tag="3")]
    pub revolutions: f64,
    /// Additional arguments to the method
    #[prost(message, optional, tag="99")]
    pub extra: ::core::option::Option<::prost_types::Struct>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GoForResponse {
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GoToRequest {
    /// Name of a motor
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// Speed of motor travel in rotations per minute
    #[prost(double, tag="2")]
    pub rpm: f64,
    /// Number of revolutions relative to motor's home home/zero
    #[prost(double, tag="3")]
    pub position_revolutions: f64,
    /// Additional arguments to the method
    #[prost(message, optional, tag="99")]
    pub extra: ::core::option::Option<::prost_types::Struct>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GoToResponse {
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResetZeroPositionRequest {
    /// Name of a motor
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// Motor position
    #[prost(double, tag="2")]
    pub offset: f64,
    /// Additional arguments to the method
    #[prost(message, optional, tag="99")]
    pub extra: ::core::option::Option<::prost_types::Struct>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResetZeroPositionResponse {
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetPositionRequest {
    /// Name of a motor
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// Additional arguments to the method
    #[prost(message, optional, tag="99")]
    pub extra: ::core::option::Option<::prost_types::Struct>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetPositionResponse {
    /// Current position of the motor relative to its home
    #[prost(double, tag="1")]
    pub position: f64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StopRequest {
    /// Name of a motor
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
pub struct IsPoweredRequest {
    /// Name of a motor
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// Additional arguments to the method
    #[prost(message, optional, tag="99")]
    pub extra: ::core::option::Option<::prost_types::Struct>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IsPoweredResponse {
    /// Returns true if the motor is on
    #[prost(bool, tag="1")]
    pub is_on: bool,
    /// Returns power percent (from 0 to 1, or from -1 to 1 for motors that support negative power),
    /// based on the last command sent to motor. If the last command was a stop command, this value
    /// will be 0.
    #[prost(double, tag="2")]
    pub power_pct: f64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetPropertiesRequest {
    /// Name of a motor
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// Additional arguments to the method
    #[prost(message, optional, tag="99")]
    pub extra: ::core::option::Option<::prost_types::Struct>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetPropertiesResponse {
    /// Returns true if the motor supports reporting its position
    #[prost(bool, tag="1")]
    pub position_reporting: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Status {
    /// Returns true if the motor is powered
    #[prost(bool, tag="1")]
    pub is_powered: bool,
    /// Returns current position of the motor relative to its home
    #[prost(double, tag="3")]
    pub position: f64,
    /// Returns true if the motor is moving
    #[prost(bool, tag="4")]
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
