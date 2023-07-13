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
pub struct GetLatestMapInfoRequest {
    /// Name of the SLAM algo
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetLatestMapInfoResponse {
    #[prost(message, optional, tag="1")]
    pub last_map_update: ::core::option::Option<::prost_types::Timestamp>,
}
// @@protoc_insertion_point(module)
