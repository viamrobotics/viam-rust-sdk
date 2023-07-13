// @generated
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetPositionRequest {
    /// Name of encoder
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// If supplied, the response will return the specified
    /// position type. If the driver does not implement
    /// the requested type, this call will return an error.
    /// If position type is not specified, the response
    /// will return a default according to the driver.
    #[prost(enumeration="PositionType", optional, tag="2")]
    pub position_type: ::core::option::Option<i32>,
    /// Additional arguments to the method
    #[prost(message, optional, tag="99")]
    pub extra: ::core::option::Option<::prost_types::Struct>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetPositionResponse {
    #[prost(float, tag="1")]
    pub value: f32,
    #[prost(enumeration="PositionType", tag="2")]
    pub position_type: i32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResetPositionRequest {
    /// Name of an encoder
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// Additional arguments to the method
    #[prost(message, optional, tag="99")]
    pub extra: ::core::option::Option<::prost_types::Struct>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResetPositionResponse {
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetPropertiesRequest {
    /// Name of the encoder
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
    pub ticks_count_supported: bool,
    #[prost(bool, tag="2")]
    pub angle_degrees_supported: bool,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum PositionType {
    Unspecified = 0,
    /// Return type for relative encoders that report
    /// how far they've gone from a start position
    TicksCount = 1,
    /// Return type for absolute encoders that report
    /// their position in degrees along the radial axis
    AngleDegrees = 2,
}
impl PositionType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            PositionType::Unspecified => "POSITION_TYPE_UNSPECIFIED",
            PositionType::TicksCount => "POSITION_TYPE_TICKS_COUNT",
            PositionType::AngleDegrees => "POSITION_TYPE_ANGLE_DEGREES",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "POSITION_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
            "POSITION_TYPE_TICKS_COUNT" => Some(Self::TicksCount),
            "POSITION_TYPE_ANGLE_DEGREES" => Some(Self::AngleDegrees),
            _ => None,
        }
    }
}
// @@protoc_insertion_point(module)
