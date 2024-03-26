// @generated
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetPositionRequest {
    /// Name of slam service
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetPositionResponse {
    /// Current position of the specified component in the SLAM Map
    #[prost(message, optional, tag="1")]
    pub pose: ::core::option::Option<super::super::super::common::v1::Pose>,
    /// This is usually the name of the camera that is in the SLAM config
    #[prost(string, tag="2")]
    pub component_reference: ::prost::alloc::string::String,
    /// Additional information in the response
    #[prost(message, optional, tag="99")]
    pub extra: ::core::option::Option<::prost_types::Struct>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetPointCloudMapRequest {
    /// Name of slam service
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// For SLAM services that implement handling an edited map, this boolean
    /// should indicate whether to return that edited map. If the SLAM service
    /// does not handle edited maps, the unedited map will be returned instead.
    #[prost(bool, optional, tag="2")]
    pub return_edited_map: ::core::option::Option<bool>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetPointCloudMapResponse {
    /// One chunk of the PointCloud.
    /// For a given GetPointCloudMap request, concatenating all
    /// GetPointCloudMapResponse.point_cloud_pcd_chunk values in the
    /// order received result in the complete pointcloud in standard PCD
    /// format where XY is the ground plane and positive Z is up, following
    /// the Right Hand Rule.
    ///
    /// Read more about the pointcloud format here:
    /// <https://pointclouds.org/documentation/tutorials/pcd_file_format.html>
    ///
    /// Viam expects pointcloud data with fields "x y z" or "x y z rgb", and for
    /// this to be specified in the pointcloud header in the FIELDS entry. If color
    /// data is included in the pointcloud, Viam's services assume that the color
    /// value encodes a confidence score for that data point. Viam expects the
    /// confidence score to be encoded in the blue parameter of the RGB value, on a
    /// scale from 1-100.
    ///
    /// Pointclouds are little endian encoded.
    #[prost(bytes="vec", tag="1")]
    pub point_cloud_pcd_chunk: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetInternalStateRequest {
    /// Name of slam service
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetInternalStateResponse {
    /// Chunk of the internal state of the SLAM algorithm required to continue
    /// mapping/localization
    #[prost(bytes="vec", tag="1")]
    pub internal_state_chunk: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetPropertiesRequest {
    /// Name of the slam service
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
}
/// Returns properties information for the named slam service
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetPropertiesResponse {
    #[prost(bool, tag="1")]
    pub cloud_slam: bool,
    #[prost(enumeration="MappingMode", tag="2")]
    pub mapping_mode: i32,
    #[prost(string, optional, tag="3")]
    pub internal_state_file_type: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(message, repeated, tag="4")]
    pub sensor_info: ::prost::alloc::vec::Vec<SensorInfo>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SensorInfo {
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    #[prost(enumeration="SensorType", tag="2")]
    pub r#type: i32,
}
/// MappingMode represnts the various form of mapping and localizing SLAM can perform.
/// These include, creating a new map, localizing on an existiing map and updating an
/// exisiting map.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum MappingMode {
    Unspecified = 0,
    CreateNewMap = 1,
    LocalizeOnly = 2,
    UpdateExistingMap = 3,
}
impl MappingMode {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            MappingMode::Unspecified => "MAPPING_MODE_UNSPECIFIED",
            MappingMode::CreateNewMap => "MAPPING_MODE_CREATE_NEW_MAP",
            MappingMode::LocalizeOnly => "MAPPING_MODE_LOCALIZE_ONLY",
            MappingMode::UpdateExistingMap => "MAPPING_MODE_UPDATE_EXISTING_MAP",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "MAPPING_MODE_UNSPECIFIED" => Some(Self::Unspecified),
            "MAPPING_MODE_CREATE_NEW_MAP" => Some(Self::CreateNewMap),
            "MAPPING_MODE_LOCALIZE_ONLY" => Some(Self::LocalizeOnly),
            "MAPPING_MODE_UPDATE_EXISTING_MAP" => Some(Self::UpdateExistingMap),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum SensorType {
    Unspecified = 0,
    Camera = 1,
    MovementSensor = 2,
}
impl SensorType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            SensorType::Unspecified => "SENSOR_TYPE_UNSPECIFIED",
            SensorType::Camera => "SENSOR_TYPE_CAMERA",
            SensorType::MovementSensor => "SENSOR_TYPE_MOVEMENT_SENSOR",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "SENSOR_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
            "SENSOR_TYPE_CAMERA" => Some(Self::Camera),
            "SENSOR_TYPE_MOVEMENT_SENSOR" => Some(Self::MovementSensor),
            _ => None,
        }
    }
}
// @@protoc_insertion_point(module)
