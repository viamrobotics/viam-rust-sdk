// @generated
// StartMappingSession

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StartMappingSessionRequest {
    /// Version to use for slam, defaults stable
    #[prost(string, tag="1")]
    pub slam_version: ::prost::alloc::string::String,
    /// Version to use for viam, defaults stable
    #[prost(string, tag="2")]
    pub viam_server_version: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub map_name: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub organization_id: ::prost::alloc::string::String,
    #[prost(string, tag="5")]
    pub location_id: ::prost::alloc::string::String,
    #[prost(string, tag="6")]
    pub robot_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag="7")]
    pub capture_interval: ::core::option::Option<CaptureInterval>,
    #[prost(message, repeated, tag="8")]
    pub sensors: ::prost::alloc::vec::Vec<SensorInfo>,
    #[prost(message, optional, tag="10")]
    pub slam_config: ::core::option::Option<::prost_types::Struct>,
    #[prost(string, tag="11")]
    pub existing_map_version: ::prost::alloc::string::String,
    #[prost(message, optional, tag="12")]
    pub module: ::core::option::Option<Module>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Module {
    #[prost(string, tag="2")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub module_id: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub version: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SensorInfo {
    #[prost(string, tag="1")]
    pub source_component_name: ::prost::alloc::string::String,
    /// type is the RDK component type
    #[prost(string, tag="2")]
    pub r#type: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub data_frequency_hz: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CaptureInterval {
    #[prost(message, optional, tag="1")]
    pub start_time: ::core::option::Option<::prost_types::Timestamp>,
    /// if no end_time specified cloud slam will be run using live sensors
    #[prost(message, optional, tag="2")]
    pub end_time: ::core::option::Option<::prost_types::Timestamp>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StartMappingSessionResponse {
    #[prost(string, tag="1")]
    pub session_id: ::prost::alloc::string::String,
}
// GetActiveMappingSessionsForRobot

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetActiveMappingSessionsForRobotRequest {
    /// assumes only one active mapping session on a robot
    #[prost(string, tag="1")]
    pub robot_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetActiveMappingSessionsForRobotResponse {
    #[prost(string, tag="1")]
    pub session_id: ::prost::alloc::string::String,
}
// GetMappingSessionPointCloud

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetMappingSessionPointCloudRequest {
    #[prost(string, tag="1")]
    pub session_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetMappingSessionPointCloudResponse {
    /// url to the pointcloud map
    #[prost(string, tag="1")]
    pub map_url: ::prost::alloc::string::String,
    /// Current position within the SLAM Map
    #[prost(message, optional, tag="2")]
    pub pose: ::core::option::Option<super::super::super::common::v1::Pose>,
}
// ListMappingSessions

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListMappingSessionsRequest {
    #[prost(string, tag="1")]
    pub organization_id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub location_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListMappingSessionsResponse {
    #[prost(message, repeated, tag="1")]
    pub session: ::prost::alloc::vec::Vec<MappingMetadata>,
}
// StopMappingSession

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StopMappingSessionRequest {
    #[prost(string, tag="1")]
    pub session_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StopMappingSessionResponse {
    #[prost(string, tag="1")]
    pub package_id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub version: ::prost::alloc::string::String,
}
// GetMappingSessionMetadataByID

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetMappingSessionMetadataByIdRequest {
    #[prost(string, tag="1")]
    pub session_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetMappingSessionMetadataByIdResponse {
    #[prost(message, optional, tag="1")]
    pub session_metadata: ::core::option::Option<MappingMetadata>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MappingMetadata {
    /// org associated with the slam session
    #[prost(string, tag="1")]
    pub org_id: ::prost::alloc::string::String,
    /// location associated with the slam session
    #[prost(string, tag="2")]
    pub location_id: ::prost::alloc::string::String,
    /// robot associated with slam session
    #[prost(string, tag="3")]
    pub robot_id: ::prost::alloc::string::String,
    /// time this document was created
    #[prost(message, optional, tag="4")]
    pub time_start_submitted: ::core::option::Option<::prost_types::Timestamp>,
    /// time the cloud run job started
    #[prost(message, optional, tag="5")]
    pub time_cloud_run_job_started: ::core::option::Option<::prost_types::Timestamp>,
    /// time StopSlamSession was called
    #[prost(message, optional, tag="6")]
    pub time_end_submitted: ::core::option::Option<::prost_types::Timestamp>,
    /// time the cloud run job ended
    #[prost(message, optional, tag="7")]
    pub time_cloud_run_job_ended: ::core::option::Option<::prost_types::Timestamp>,
    /// enums that represent “success”, “failed”, etc
    #[prost(enumeration="EndStatus", tag="8")]
    pub end_status: i32,
    /// initially unset
    #[prost(string, tag="9")]
    pub cloud_run_job_id: ::prost::alloc::string::String,
    /// version tag from request, defaults to stable
    #[prost(string, tag="10")]
    pub viam_server_version: ::prost::alloc::string::String,
    /// name of the map package
    #[prost(string, tag="11")]
    pub map_name: ::prost::alloc::string::String,
    /// version tag from request, defaults to stable
    #[prost(string, tag="12")]
    pub slam_version: ::prost::alloc::string::String,
    /// a robot config for a slam session
    #[prost(string, tag="13")]
    pub config: ::prost::alloc::string::String,
    /// additional details on the end status if needed, such as errors
    #[prost(string, tag="14")]
    pub error_msg: ::prost::alloc::string::String,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum EndStatus {
    Unspecified = 0,
    Success = 1,
    Timeout = 2,
    Fail = 3,
}
impl EndStatus {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            EndStatus::Unspecified => "END_STATUS_UNSPECIFIED",
            EndStatus::Success => "END_STATUS_SUCCESS",
            EndStatus::Timeout => "END_STATUS_TIMEOUT",
            EndStatus::Fail => "END_STATUS_FAIL",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "END_STATUS_UNSPECIFIED" => Some(Self::Unspecified),
            "END_STATUS_SUCCESS" => Some(Self::Success),
            "END_STATUS_TIMEOUT" => Some(Self::Timeout),
            "END_STATUS_FAIL" => Some(Self::Fail),
            _ => None,
        }
    }
}
// @@protoc_insertion_point(module)
