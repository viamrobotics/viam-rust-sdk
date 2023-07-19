// @generated
// StartMappingSession

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StartMappingSessionRequest {
    /// The given config contains details such as sensor, map package and algorithm-specific fields required to run slam.
    #[prost(message, optional, tag="1")]
    pub slam_config: ::core::option::Option<::prost_types::Struct>,
    /// Version to use for slam, defaults stable
    #[prost(string, tag="2")]
    pub slam_version: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub map_name: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub organization_id: ::prost::alloc::string::String,
    #[prost(string, tag="5")]
    pub location_id: ::prost::alloc::string::String,
    #[prost(string, tag="6")]
    pub robot_id: ::prost::alloc::string::String,
    /// Version to use for viam, defaults stable
    #[prost(string, tag="7")]
    pub viam_server_version: ::prost::alloc::string::String,
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
    #[prost(bool, tag="2")]
    pub save_map: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StopMappingSessionResponse {
    #[prost(string, tag="1")]
    pub package_id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub version: ::prost::alloc::string::String,
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
    /// “success”, “failed to start”, etc
    #[prost(string, tag="8")]
    pub end_status: ::prost::alloc::string::String,
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
}
// @@protoc_insertion_point(module)
