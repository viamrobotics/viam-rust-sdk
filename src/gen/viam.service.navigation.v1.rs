// @generated
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetModeRequest {
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// Additional arguments to the method
    #[prost(message, optional, tag="99")]
    pub extra: ::core::option::Option<::prost_types::Struct>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetModeResponse {
    #[prost(enumeration="Mode", tag="1")]
    pub mode: i32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetModeRequest {
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    #[prost(enumeration="Mode", tag="2")]
    pub mode: i32,
    /// Additional arguments to the method
    #[prost(message, optional, tag="99")]
    pub extra: ::core::option::Option<::prost_types::Struct>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetModeResponse {
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Waypoint {
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub location: ::core::option::Option<super::super::super::common::v1::GeoPoint>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetLocationRequest {
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// Additional arguments to the method
    #[prost(message, optional, tag="99")]
    pub extra: ::core::option::Option<::prost_types::Struct>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetLocationResponse {
    #[prost(message, optional, tag="1")]
    pub location: ::core::option::Option<super::super::super::common::v1::GeoPoint>,
    /// A number from [0-360) where 0 is north
    /// 90 is east, 180 is south, 270 is west
    #[prost(double, tag="2")]
    pub compass_heading: f64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetWaypointsRequest {
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// Additional arguments to the method
    #[prost(message, optional, tag="99")]
    pub extra: ::core::option::Option<::prost_types::Struct>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetWaypointsResponse {
    #[prost(message, repeated, tag="1")]
    pub waypoints: ::prost::alloc::vec::Vec<Waypoint>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddWaypointRequest {
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub location: ::core::option::Option<super::super::super::common::v1::GeoPoint>,
    /// Additional arguments to the method
    #[prost(message, optional, tag="99")]
    pub extra: ::core::option::Option<::prost_types::Struct>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddWaypointResponse {
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveWaypointRequest {
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub id: ::prost::alloc::string::String,
    /// Additional arguments to the method
    #[prost(message, optional, tag="99")]
    pub extra: ::core::option::Option<::prost_types::Struct>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveWaypointResponse {
}
/// GetObstacles will return the geopoint location and geometry of all
/// known obstacles on the navigation map. Obstacles that are detected
/// through the vision service will only be returned if this endpoint is called
/// when the robot is sensing the obstacle
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetObstaclesRequest {
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// Additional arguments to the method
    #[prost(message, optional, tag="99")]
    pub extra: ::core::option::Option<::prost_types::Struct>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetObstaclesResponse {
    /// List of all known geometries
    #[prost(message, repeated, tag="1")]
    pub obstacles: ::prost::alloc::vec::Vec<super::super::super::common::v1::GeoObstacle>,
}
/// A user provided destination and the set of geopoints that
/// the robot is expected to take to get there
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Path {
    /// The id of the user specified waypoint
    #[prost(string, tag="1")]
    pub destination_waypoint_id: ::prost::alloc::string::String,
    /// List of geopoints that the motion planner output to reach the destination
    /// The first geopoint is the starting position of the robot for that path
    #[prost(message, repeated, tag="2")]
    pub geopoints: ::prost::alloc::vec::Vec<super::super::super::common::v1::GeoPoint>,
}
/// Returns all the paths known to the navigation service
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetPathsRequest {
    /// Name of the navigation service
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    #[prost(message, optional, tag="99")]
    pub extra: ::core::option::Option<::prost_types::Struct>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetPathsResponse {
    #[prost(message, repeated, tag="1")]
    pub paths: ::prost::alloc::vec::Vec<Path>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetPropertiesRequest {
    /// Name of the navigation service
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
}
/// Returns properties information for the named navigation service
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetPropertiesResponse {
    #[prost(enumeration="MapType", tag="1")]
    pub map_type: i32,
}
/// MapType represents the various types of maps the navigation service can ingest.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum MapType {
    Unspecified = 0,
    None = 1,
    Gps = 2,
}
impl MapType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            MapType::Unspecified => "MAP_TYPE_UNSPECIFIED",
            MapType::None => "MAP_TYPE_NONE",
            MapType::Gps => "MAP_TYPE_GPS",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "MAP_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
            "MAP_TYPE_NONE" => Some(Self::None),
            "MAP_TYPE_GPS" => Some(Self::Gps),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum Mode {
    Unspecified = 0,
    Manual = 1,
    Waypoint = 2,
    Explore = 3,
}
impl Mode {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Mode::Unspecified => "MODE_UNSPECIFIED",
            Mode::Manual => "MODE_MANUAL",
            Mode::Waypoint => "MODE_WAYPOINT",
            Mode::Explore => "MODE_EXPLORE",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "MODE_UNSPECIFIED" => Some(Self::Unspecified),
            "MODE_MANUAL" => Some(Self::Manual),
            "MODE_WAYPOINT" => Some(Self::Waypoint),
            "MODE_EXPLORE" => Some(Self::Explore),
            _ => None,
        }
    }
}
// @@protoc_insertion_point(module)
