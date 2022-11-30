// @generated
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetEndPositionRequest {
    /// Name of an arm
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// Additional arguments to the method
    #[prost(message, optional, tag="99")]
    pub extra: ::core::option::Option<::prost_types::Struct>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetEndPositionResponse {
    /// Returns 6d pose of the end effector relative to the base, represented by X,Y,Z coordinates which express
    /// millimeters and theta, ox, oy, oz coordinates which express an orientation vector
    #[prost(message, optional, tag="1")]
    pub pose: ::core::option::Option<super::super::super::common::v1::Pose>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct JointPositions {
    /// A list of joint positions. Rotations values are in degrees, translational values in mm.
    /// The numbers are ordered spatially from the base toward the end effector
    /// This is used in GetJointPositionsResponse and MoveToJointPositionsRequest
    #[prost(double, repeated, tag="1")]
    pub values: ::prost::alloc::vec::Vec<f64>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetJointPositionsRequest {
    /// Name of an arm
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// Additional arguments to the method
    #[prost(message, optional, tag="99")]
    pub extra: ::core::option::Option<::prost_types::Struct>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetJointPositionsResponse {
    ///a list JointPositions
    #[prost(message, optional, tag="1")]
    pub positions: ::core::option::Option<JointPositions>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MoveToPositionRequest {
    /// Name of an arm
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub to: ::core::option::Option<super::super::super::common::v1::Pose>,
    #[prost(message, optional, tag="3")]
    pub world_state: ::core::option::Option<super::super::super::common::v1::WorldState>,
    /// Additional arguments to the method
    #[prost(message, optional, tag="99")]
    pub extra: ::core::option::Option<::prost_types::Struct>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MoveToPositionResponse {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MoveToJointPositionsRequest {
    /// Name of an arm
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// A list of joint positions
    /// There should be 1 entry in the list per joint DOF, ordered spatially from the base toward the end effector
    #[prost(message, optional, tag="2")]
    pub positions: ::core::option::Option<JointPositions>,
    /// Additional arguments to the method
    #[prost(message, optional, tag="99")]
    pub extra: ::core::option::Option<::prost_types::Struct>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MoveToJointPositionsResponse {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StopRequest {
    /// Name of an arm
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// Additional arguments to the method
    #[prost(message, optional, tag="99")]
    pub extra: ::core::option::Option<::prost_types::Struct>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StopResponse {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Status {
    #[prost(message, optional, tag="1")]
    pub end_position: ::core::option::Option<super::super::super::common::v1::Pose>,
    #[prost(message, optional, tag="2")]
    pub joint_positions: ::core::option::Option<JointPositions>,
    #[prost(bool, tag="3")]
    pub is_moving: bool,
}
/// Encoded file descriptor set for the `viam.component.arm.v1` package
pub const FILE_DESCRIPTOR_SET: &[u8] = &[
    0x0a, 0x98, 0x2c, 0x0a, 0x1a, 0x63, 0x6f, 0x6d, 0x70, 0x6f, 0x6e, 0x65, 0x6e, 0x74, 0x2f, 0x61,
    0x72, 0x6d, 0x2f, 0x76, 0x31, 0x2f, 0x61, 0x72, 0x6d, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x12,
    0x15, 0x76, 0x69, 0x61, 0x6d, 0x2e, 0x63, 0x6f, 0x6d, 0x70, 0x6f, 0x6e, 0x65, 0x6e, 0x74, 0x2e,
    0x61, 0x72, 0x6d, 0x2e, 0x76, 0x31, 0x1a, 0x16, 0x63, 0x6f, 0x6d, 0x6d, 0x6f, 0x6e, 0x2f, 0x76,
    0x31, 0x2f, 0x63, 0x6f, 0x6d, 0x6d, 0x6f, 0x6e, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x1a, 0x1c,
    0x67, 0x6f, 0x6f, 0x67, 0x6c, 0x65, 0x2f, 0x61, 0x70, 0x69, 0x2f, 0x61, 0x6e, 0x6e, 0x6f, 0x74,
    0x61, 0x74, 0x69, 0x6f, 0x6e, 0x73, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x1a, 0x1c, 0x67, 0x6f,
    0x6f, 0x67, 0x6c, 0x65, 0x2f, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x62, 0x75, 0x66, 0x2f, 0x73, 0x74,
    0x72, 0x75, 0x63, 0x74, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x22, 0x5a, 0x0a, 0x15, 0x47, 0x65,
    0x74, 0x45, 0x6e, 0x64, 0x50, 0x6f, 0x73, 0x69, 0x74, 0x69, 0x6f, 0x6e, 0x52, 0x65, 0x71, 0x75,
    0x65, 0x73, 0x74, 0x12, 0x12, 0x0a, 0x04, 0x6e, 0x61, 0x6d, 0x65, 0x18, 0x01, 0x20, 0x01, 0x28,
    0x09, 0x52, 0x04, 0x6e, 0x61, 0x6d, 0x65, 0x12, 0x2d, 0x0a, 0x05, 0x65, 0x78, 0x74, 0x72, 0x61,
    0x18, 0x63, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x17, 0x2e, 0x67, 0x6f, 0x6f, 0x67, 0x6c, 0x65, 0x2e,
    0x70, 0x72, 0x6f, 0x74, 0x6f, 0x62, 0x75, 0x66, 0x2e, 0x53, 0x74, 0x72, 0x75, 0x63, 0x74, 0x52,
    0x05, 0x65, 0x78, 0x74, 0x72, 0x61, 0x22, 0x42, 0x0a, 0x16, 0x47, 0x65, 0x74, 0x45, 0x6e, 0x64,
    0x50, 0x6f, 0x73, 0x69, 0x74, 0x69, 0x6f, 0x6e, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65,
    0x12, 0x28, 0x0a, 0x04, 0x70, 0x6f, 0x73, 0x65, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x14,
    0x2e, 0x76, 0x69, 0x61, 0x6d, 0x2e, 0x63, 0x6f, 0x6d, 0x6d, 0x6f, 0x6e, 0x2e, 0x76, 0x31, 0x2e,
    0x50, 0x6f, 0x73, 0x65, 0x52, 0x04, 0x70, 0x6f, 0x73, 0x65, 0x22, 0x28, 0x0a, 0x0e, 0x4a, 0x6f,
    0x69, 0x6e, 0x74, 0x50, 0x6f, 0x73, 0x69, 0x74, 0x69, 0x6f, 0x6e, 0x73, 0x12, 0x16, 0x0a, 0x06,
    0x76, 0x61, 0x6c, 0x75, 0x65, 0x73, 0x18, 0x01, 0x20, 0x03, 0x28, 0x01, 0x52, 0x06, 0x76, 0x61,
    0x6c, 0x75, 0x65, 0x73, 0x22, 0x5d, 0x0a, 0x18, 0x47, 0x65, 0x74, 0x4a, 0x6f, 0x69, 0x6e, 0x74,
    0x50, 0x6f, 0x73, 0x69, 0x74, 0x69, 0x6f, 0x6e, 0x73, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74,
    0x12, 0x12, 0x0a, 0x04, 0x6e, 0x61, 0x6d, 0x65, 0x18, 0x01, 0x20, 0x01, 0x28, 0x09, 0x52, 0x04,
    0x6e, 0x61, 0x6d, 0x65, 0x12, 0x2d, 0x0a, 0x05, 0x65, 0x78, 0x74, 0x72, 0x61, 0x18, 0x63, 0x20,
    0x01, 0x28, 0x0b, 0x32, 0x17, 0x2e, 0x67, 0x6f, 0x6f, 0x67, 0x6c, 0x65, 0x2e, 0x70, 0x72, 0x6f,
    0x74, 0x6f, 0x62, 0x75, 0x66, 0x2e, 0x53, 0x74, 0x72, 0x75, 0x63, 0x74, 0x52, 0x05, 0x65, 0x78,
    0x74, 0x72, 0x61, 0x22, 0x60, 0x0a, 0x19, 0x47, 0x65, 0x74, 0x4a, 0x6f, 0x69, 0x6e, 0x74, 0x50,
    0x6f, 0x73, 0x69, 0x74, 0x69, 0x6f, 0x6e, 0x73, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65,
    0x12, 0x43, 0x0a, 0x09, 0x70, 0x6f, 0x73, 0x69, 0x74, 0x69, 0x6f, 0x6e, 0x73, 0x18, 0x01, 0x20,
    0x01, 0x28, 0x0b, 0x32, 0x25, 0x2e, 0x76, 0x69, 0x61, 0x6d, 0x2e, 0x63, 0x6f, 0x6d, 0x70, 0x6f,
    0x6e, 0x65, 0x6e, 0x74, 0x2e, 0x61, 0x72, 0x6d, 0x2e, 0x76, 0x31, 0x2e, 0x4a, 0x6f, 0x69, 0x6e,
    0x74, 0x50, 0x6f, 0x73, 0x69, 0x74, 0x69, 0x6f, 0x6e, 0x73, 0x52, 0x09, 0x70, 0x6f, 0x73, 0x69,
    0x74, 0x69, 0x6f, 0x6e, 0x73, 0x22, 0xd2, 0x01, 0x0a, 0x15, 0x4d, 0x6f, 0x76, 0x65, 0x54, 0x6f,
    0x50, 0x6f, 0x73, 0x69, 0x74, 0x69, 0x6f, 0x6e, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x12,
    0x12, 0x0a, 0x04, 0x6e, 0x61, 0x6d, 0x65, 0x18, 0x01, 0x20, 0x01, 0x28, 0x09, 0x52, 0x04, 0x6e,
    0x61, 0x6d, 0x65, 0x12, 0x24, 0x0a, 0x02, 0x74, 0x6f, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0b, 0x32,
    0x14, 0x2e, 0x76, 0x69, 0x61, 0x6d, 0x2e, 0x63, 0x6f, 0x6d, 0x6d, 0x6f, 0x6e, 0x2e, 0x76, 0x31,
    0x2e, 0x50, 0x6f, 0x73, 0x65, 0x52, 0x02, 0x74, 0x6f, 0x12, 0x40, 0x0a, 0x0b, 0x77, 0x6f, 0x72,
    0x6c, 0x64, 0x5f, 0x73, 0x74, 0x61, 0x74, 0x65, 0x18, 0x03, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x1a,
    0x2e, 0x76, 0x69, 0x61, 0x6d, 0x2e, 0x63, 0x6f, 0x6d, 0x6d, 0x6f, 0x6e, 0x2e, 0x76, 0x31, 0x2e,
    0x57, 0x6f, 0x72, 0x6c, 0x64, 0x53, 0x74, 0x61, 0x74, 0x65, 0x48, 0x00, 0x52, 0x0a, 0x77, 0x6f,
    0x72, 0x6c, 0x64, 0x53, 0x74, 0x61, 0x74, 0x65, 0x88, 0x01, 0x01, 0x12, 0x2d, 0x0a, 0x05, 0x65,
    0x78, 0x74, 0x72, 0x61, 0x18, 0x63, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x17, 0x2e, 0x67, 0x6f, 0x6f,
    0x67, 0x6c, 0x65, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x62, 0x75, 0x66, 0x2e, 0x53, 0x74, 0x72,
    0x75, 0x63, 0x74, 0x52, 0x05, 0x65, 0x78, 0x74, 0x72, 0x61, 0x42, 0x0e, 0x0a, 0x0c, 0x5f, 0x77,
    0x6f, 0x72, 0x6c, 0x64, 0x5f, 0x73, 0x74, 0x61, 0x74, 0x65, 0x22, 0x18, 0x0a, 0x16, 0x4d, 0x6f,
    0x76, 0x65, 0x54, 0x6f, 0x50, 0x6f, 0x73, 0x69, 0x74, 0x69, 0x6f, 0x6e, 0x52, 0x65, 0x73, 0x70,
    0x6f, 0x6e, 0x73, 0x65, 0x22, 0xa5, 0x01, 0x0a, 0x1b, 0x4d, 0x6f, 0x76, 0x65, 0x54, 0x6f, 0x4a,
    0x6f, 0x69, 0x6e, 0x74, 0x50, 0x6f, 0x73, 0x69, 0x74, 0x69, 0x6f, 0x6e, 0x73, 0x52, 0x65, 0x71,
    0x75, 0x65, 0x73, 0x74, 0x12, 0x12, 0x0a, 0x04, 0x6e, 0x61, 0x6d, 0x65, 0x18, 0x01, 0x20, 0x01,
    0x28, 0x09, 0x52, 0x04, 0x6e, 0x61, 0x6d, 0x65, 0x12, 0x43, 0x0a, 0x09, 0x70, 0x6f, 0x73, 0x69,
    0x74, 0x69, 0x6f, 0x6e, 0x73, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x25, 0x2e, 0x76, 0x69,
    0x61, 0x6d, 0x2e, 0x63, 0x6f, 0x6d, 0x70, 0x6f, 0x6e, 0x65, 0x6e, 0x74, 0x2e, 0x61, 0x72, 0x6d,
    0x2e, 0x76, 0x31, 0x2e, 0x4a, 0x6f, 0x69, 0x6e, 0x74, 0x50, 0x6f, 0x73, 0x69, 0x74, 0x69, 0x6f,
    0x6e, 0x73, 0x52, 0x09, 0x70, 0x6f, 0x73, 0x69, 0x74, 0x69, 0x6f, 0x6e, 0x73, 0x12, 0x2d, 0x0a,
    0x05, 0x65, 0x78, 0x74, 0x72, 0x61, 0x18, 0x63, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x17, 0x2e, 0x67,
    0x6f, 0x6f, 0x67, 0x6c, 0x65, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x62, 0x75, 0x66, 0x2e, 0x53,
    0x74, 0x72, 0x75, 0x63, 0x74, 0x52, 0x05, 0x65, 0x78, 0x74, 0x72, 0x61, 0x22, 0x1e, 0x0a, 0x1c,
    0x4d, 0x6f, 0x76, 0x65, 0x54, 0x6f, 0x4a, 0x6f, 0x69, 0x6e, 0x74, 0x50, 0x6f, 0x73, 0x69, 0x74,
    0x69, 0x6f, 0x6e, 0x73, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x22, 0x50, 0x0a, 0x0b,
    0x53, 0x74, 0x6f, 0x70, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x12, 0x12, 0x0a, 0x04, 0x6e,
    0x61, 0x6d, 0x65, 0x18, 0x01, 0x20, 0x01, 0x28, 0x09, 0x52, 0x04, 0x6e, 0x61, 0x6d, 0x65, 0x12,
    0x2d, 0x0a, 0x05, 0x65, 0x78, 0x74, 0x72, 0x61, 0x18, 0x63, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x17,
    0x2e, 0x67, 0x6f, 0x6f, 0x67, 0x6c, 0x65, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x62, 0x75, 0x66,
    0x2e, 0x53, 0x74, 0x72, 0x75, 0x63, 0x74, 0x52, 0x05, 0x65, 0x78, 0x74, 0x72, 0x61, 0x22, 0x0e,
    0x0a, 0x0c, 0x53, 0x74, 0x6f, 0x70, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x22, 0xae,
    0x01, 0x0a, 0x06, 0x53, 0x74, 0x61, 0x74, 0x75, 0x73, 0x12, 0x37, 0x0a, 0x0c, 0x65, 0x6e, 0x64,
    0x5f, 0x70, 0x6f, 0x73, 0x69, 0x74, 0x69, 0x6f, 0x6e, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0b, 0x32,
    0x14, 0x2e, 0x76, 0x69, 0x61, 0x6d, 0x2e, 0x63, 0x6f, 0x6d, 0x6d, 0x6f, 0x6e, 0x2e, 0x76, 0x31,
    0x2e, 0x50, 0x6f, 0x73, 0x65, 0x52, 0x0b, 0x65, 0x6e, 0x64, 0x50, 0x6f, 0x73, 0x69, 0x74, 0x69,
    0x6f, 0x6e, 0x12, 0x4e, 0x0a, 0x0f, 0x6a, 0x6f, 0x69, 0x6e, 0x74, 0x5f, 0x70, 0x6f, 0x73, 0x69,
    0x74, 0x69, 0x6f, 0x6e, 0x73, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x25, 0x2e, 0x76, 0x69,
    0x61, 0x6d, 0x2e, 0x63, 0x6f, 0x6d, 0x70, 0x6f, 0x6e, 0x65, 0x6e, 0x74, 0x2e, 0x61, 0x72, 0x6d,
    0x2e, 0x76, 0x31, 0x2e, 0x4a, 0x6f, 0x69, 0x6e, 0x74, 0x50, 0x6f, 0x73, 0x69, 0x74, 0x69, 0x6f,
    0x6e, 0x73, 0x52, 0x0e, 0x6a, 0x6f, 0x69, 0x6e, 0x74, 0x50, 0x6f, 0x73, 0x69, 0x74, 0x69, 0x6f,
    0x6e, 0x73, 0x12, 0x1b, 0x0a, 0x09, 0x69, 0x73, 0x5f, 0x6d, 0x6f, 0x76, 0x69, 0x6e, 0x67, 0x18,
    0x03, 0x20, 0x01, 0x28, 0x08, 0x52, 0x08, 0x69, 0x73, 0x4d, 0x6f, 0x76, 0x69, 0x6e, 0x67, 0x32,
    0xce, 0x06, 0x0a, 0x0a, 0x41, 0x72, 0x6d, 0x53, 0x65, 0x72, 0x76, 0x69, 0x63, 0x65, 0x12, 0xa1,
    0x01, 0x0a, 0x0e, 0x47, 0x65, 0x74, 0x45, 0x6e, 0x64, 0x50, 0x6f, 0x73, 0x69, 0x74, 0x69, 0x6f,
    0x6e, 0x12, 0x2c, 0x2e, 0x76, 0x69, 0x61, 0x6d, 0x2e, 0x63, 0x6f, 0x6d, 0x70, 0x6f, 0x6e, 0x65,
    0x6e, 0x74, 0x2e, 0x61, 0x72, 0x6d, 0x2e, 0x76, 0x31, 0x2e, 0x47, 0x65, 0x74, 0x45, 0x6e, 0x64,
    0x50, 0x6f, 0x73, 0x69, 0x74, 0x69, 0x6f, 0x6e, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x1a,
    0x2d, 0x2e, 0x76, 0x69, 0x61, 0x6d, 0x2e, 0x63, 0x6f, 0x6d, 0x70, 0x6f, 0x6e, 0x65, 0x6e, 0x74,
    0x2e, 0x61, 0x72, 0x6d, 0x2e, 0x76, 0x31, 0x2e, 0x47, 0x65, 0x74, 0x45, 0x6e, 0x64, 0x50, 0x6f,
    0x73, 0x69, 0x74, 0x69, 0x6f, 0x6e, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x22, 0x32,
    0x82, 0xd3, 0xe4, 0x93, 0x02, 0x2c, 0x12, 0x2a, 0x2f, 0x76, 0x69, 0x61, 0x6d, 0x2f, 0x61, 0x70,
    0x69, 0x2f, 0x76, 0x31, 0x2f, 0x63, 0x6f, 0x6d, 0x70, 0x6f, 0x6e, 0x65, 0x6e, 0x74, 0x2f, 0x61,
    0x72, 0x6d, 0x2f, 0x7b, 0x6e, 0x61, 0x6d, 0x65, 0x7d, 0x2f, 0x70, 0x6f, 0x73, 0x69, 0x74, 0x69,
    0x6f, 0x6e, 0x12, 0xa5, 0x01, 0x0a, 0x0e, 0x4d, 0x6f, 0x76, 0x65, 0x54, 0x6f, 0x50, 0x6f, 0x73,
    0x69, 0x74, 0x69, 0x6f, 0x6e, 0x12, 0x2c, 0x2e, 0x76, 0x69, 0x61, 0x6d, 0x2e, 0x63, 0x6f, 0x6d,
    0x70, 0x6f, 0x6e, 0x65, 0x6e, 0x74, 0x2e, 0x61, 0x72, 0x6d, 0x2e, 0x76, 0x31, 0x2e, 0x4d, 0x6f,
    0x76, 0x65, 0x54, 0x6f, 0x50, 0x6f, 0x73, 0x69, 0x74, 0x69, 0x6f, 0x6e, 0x52, 0x65, 0x71, 0x75,
    0x65, 0x73, 0x74, 0x1a, 0x2d, 0x2e, 0x76, 0x69, 0x61, 0x6d, 0x2e, 0x63, 0x6f, 0x6d, 0x70, 0x6f,
    0x6e, 0x65, 0x6e, 0x74, 0x2e, 0x61, 0x72, 0x6d, 0x2e, 0x76, 0x31, 0x2e, 0x4d, 0x6f, 0x76, 0x65,
    0x54, 0x6f, 0x50, 0x6f, 0x73, 0x69, 0x74, 0x69, 0x6f, 0x6e, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e,
    0x73, 0x65, 0x22, 0x36, 0xa0, 0x92, 0x29, 0x01, 0x82, 0xd3, 0xe4, 0x93, 0x02, 0x2c, 0x1a, 0x2a,
    0x2f, 0x76, 0x69, 0x61, 0x6d, 0x2f, 0x61, 0x70, 0x69, 0x2f, 0x76, 0x31, 0x2f, 0x63, 0x6f, 0x6d,
    0x70, 0x6f, 0x6e, 0x65, 0x6e, 0x74, 0x2f, 0x61, 0x72, 0x6d, 0x2f, 0x7b, 0x6e, 0x61, 0x6d, 0x65,
    0x7d, 0x2f, 0x70, 0x6f, 0x73, 0x69, 0x74, 0x69, 0x6f, 0x6e, 0x12, 0xb1, 0x01, 0x0a, 0x11, 0x47,
    0x65, 0x74, 0x4a, 0x6f, 0x69, 0x6e, 0x74, 0x50, 0x6f, 0x73, 0x69, 0x74, 0x69, 0x6f, 0x6e, 0x73,
    0x12, 0x2f, 0x2e, 0x76, 0x69, 0x61, 0x6d, 0x2e, 0x63, 0x6f, 0x6d, 0x70, 0x6f, 0x6e, 0x65, 0x6e,
    0x74, 0x2e, 0x61, 0x72, 0x6d, 0x2e, 0x76, 0x31, 0x2e, 0x47, 0x65, 0x74, 0x4a, 0x6f, 0x69, 0x6e,
    0x74, 0x50, 0x6f, 0x73, 0x69, 0x74, 0x69, 0x6f, 0x6e, 0x73, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73,
    0x74, 0x1a, 0x30, 0x2e, 0x76, 0x69, 0x61, 0x6d, 0x2e, 0x63, 0x6f, 0x6d, 0x70, 0x6f, 0x6e, 0x65,
    0x6e, 0x74, 0x2e, 0x61, 0x72, 0x6d, 0x2e, 0x76, 0x31, 0x2e, 0x47, 0x65, 0x74, 0x4a, 0x6f, 0x69,
    0x6e, 0x74, 0x50, 0x6f, 0x73, 0x69, 0x74, 0x69, 0x6f, 0x6e, 0x73, 0x52, 0x65, 0x73, 0x70, 0x6f,
    0x6e, 0x73, 0x65, 0x22, 0x39, 0x82, 0xd3, 0xe4, 0x93, 0x02, 0x33, 0x12, 0x31, 0x2f, 0x76, 0x69,
    0x61, 0x6d, 0x2f, 0x61, 0x70, 0x69, 0x2f, 0x76, 0x31, 0x2f, 0x63, 0x6f, 0x6d, 0x70, 0x6f, 0x6e,
    0x65, 0x6e, 0x74, 0x2f, 0x61, 0x72, 0x6d, 0x2f, 0x7b, 0x6e, 0x61, 0x6d, 0x65, 0x7d, 0x2f, 0x6a,
    0x6f, 0x69, 0x6e, 0x74, 0x5f, 0x70, 0x6f, 0x73, 0x69, 0x74, 0x69, 0x6f, 0x6e, 0x73, 0x12, 0xbe,
    0x01, 0x0a, 0x14, 0x4d, 0x6f, 0x76, 0x65, 0x54, 0x6f, 0x4a, 0x6f, 0x69, 0x6e, 0x74, 0x50, 0x6f,
    0x73, 0x69, 0x74, 0x69, 0x6f, 0x6e, 0x73, 0x12, 0x32, 0x2e, 0x76, 0x69, 0x61, 0x6d, 0x2e, 0x63,
    0x6f, 0x6d, 0x70, 0x6f, 0x6e, 0x65, 0x6e, 0x74, 0x2e, 0x61, 0x72, 0x6d, 0x2e, 0x76, 0x31, 0x2e,
    0x4d, 0x6f, 0x76, 0x65, 0x54, 0x6f, 0x4a, 0x6f, 0x69, 0x6e, 0x74, 0x50, 0x6f, 0x73, 0x69, 0x74,
    0x69, 0x6f, 0x6e, 0x73, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x1a, 0x33, 0x2e, 0x76, 0x69,
    0x61, 0x6d, 0x2e, 0x63, 0x6f, 0x6d, 0x70, 0x6f, 0x6e, 0x65, 0x6e, 0x74, 0x2e, 0x61, 0x72, 0x6d,
    0x2e, 0x76, 0x31, 0x2e, 0x4d, 0x6f, 0x76, 0x65, 0x54, 0x6f, 0x4a, 0x6f, 0x69, 0x6e, 0x74, 0x50,
    0x6f, 0x73, 0x69, 0x74, 0x69, 0x6f, 0x6e, 0x73, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65,
    0x22, 0x3d, 0xa0, 0x92, 0x29, 0x01, 0x82, 0xd3, 0xe4, 0x93, 0x02, 0x33, 0x1a, 0x31, 0x2f, 0x76,
    0x69, 0x61, 0x6d, 0x2f, 0x61, 0x70, 0x69, 0x2f, 0x76, 0x31, 0x2f, 0x63, 0x6f, 0x6d, 0x70, 0x6f,
    0x6e, 0x65, 0x6e, 0x74, 0x2f, 0x61, 0x72, 0x6d, 0x2f, 0x7b, 0x6e, 0x61, 0x6d, 0x65, 0x7d, 0x2f,
    0x6a, 0x6f, 0x69, 0x6e, 0x74, 0x5f, 0x70, 0x6f, 0x73, 0x69, 0x74, 0x69, 0x6f, 0x6e, 0x73, 0x12,
    0x7f, 0x0a, 0x04, 0x53, 0x74, 0x6f, 0x70, 0x12, 0x22, 0x2e, 0x76, 0x69, 0x61, 0x6d, 0x2e, 0x63,
    0x6f, 0x6d, 0x70, 0x6f, 0x6e, 0x65, 0x6e, 0x74, 0x2e, 0x61, 0x72, 0x6d, 0x2e, 0x76, 0x31, 0x2e,
    0x53, 0x74, 0x6f, 0x70, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x1a, 0x23, 0x2e, 0x76, 0x69,
    0x61, 0x6d, 0x2e, 0x63, 0x6f, 0x6d, 0x70, 0x6f, 0x6e, 0x65, 0x6e, 0x74, 0x2e, 0x61, 0x72, 0x6d,
    0x2e, 0x76, 0x31, 0x2e, 0x53, 0x74, 0x6f, 0x70, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65,
    0x22, 0x2e, 0x82, 0xd3, 0xe4, 0x93, 0x02, 0x28, 0x22, 0x26, 0x2f, 0x76, 0x69, 0x61, 0x6d, 0x2f,
    0x61, 0x70, 0x69, 0x2f, 0x76, 0x31, 0x2f, 0x63, 0x6f, 0x6d, 0x70, 0x6f, 0x6e, 0x65, 0x6e, 0x74,
    0x2f, 0x61, 0x72, 0x6d, 0x2f, 0x7b, 0x6e, 0x61, 0x6d, 0x65, 0x7d, 0x2f, 0x73, 0x74, 0x6f, 0x70,
    0x42, 0x3d, 0x0a, 0x19, 0x63, 0x6f, 0x6d, 0x2e, 0x76, 0x69, 0x61, 0x6d, 0x2e, 0x63, 0x6f, 0x6d,
    0x70, 0x6f, 0x6e, 0x65, 0x6e, 0x74, 0x2e, 0x61, 0x72, 0x6d, 0x2e, 0x76, 0x31, 0x5a, 0x20, 0x67,
    0x6f, 0x2e, 0x76, 0x69, 0x61, 0x6d, 0x2e, 0x63, 0x6f, 0x6d, 0x2f, 0x61, 0x70, 0x69, 0x2f, 0x63,
    0x6f, 0x6d, 0x70, 0x6f, 0x6e, 0x65, 0x6e, 0x74, 0x2f, 0x61, 0x72, 0x6d, 0x2f, 0x76, 0x31, 0x4a,
    0xa1, 0x1b, 0x0a, 0x06, 0x12, 0x04, 0x00, 0x00, 0x79, 0x01, 0x0a, 0x08, 0x0a, 0x01, 0x0c, 0x12,
    0x03, 0x00, 0x00, 0x12, 0x0a, 0x08, 0x0a, 0x01, 0x02, 0x12, 0x03, 0x02, 0x00, 0x1e, 0x0a, 0x09,
    0x0a, 0x02, 0x03, 0x00, 0x12, 0x03, 0x04, 0x00, 0x20, 0x0a, 0x09, 0x0a, 0x02, 0x03, 0x01, 0x12,
    0x03, 0x05, 0x00, 0x26, 0x0a, 0x09, 0x0a, 0x02, 0x03, 0x02, 0x12, 0x03, 0x06, 0x00, 0x26, 0x0a,
    0x08, 0x0a, 0x01, 0x08, 0x12, 0x03, 0x08, 0x00, 0x37, 0x0a, 0x09, 0x0a, 0x02, 0x08, 0x0b, 0x12,
    0x03, 0x08, 0x00, 0x37, 0x0a, 0x08, 0x0a, 0x01, 0x08, 0x12, 0x03, 0x09, 0x00, 0x32, 0x0a, 0x09,
    0x0a, 0x02, 0x08, 0x01, 0x12, 0x03, 0x09, 0x00, 0x32, 0x0a, 0x45, 0x0a, 0x02, 0x06, 0x00, 0x12,
    0x04, 0x0c, 0x00, 0x33, 0x01, 0x1a, 0x39, 0x20, 0x41, 0x6e, 0x20, 0x41, 0x72, 0x6d, 0x53, 0x65,
    0x72, 0x76, 0x69, 0x63, 0x65, 0x20, 0x73, 0x65, 0x72, 0x76, 0x69, 0x63, 0x65, 0x73, 0x20, 0x61,
    0x6c, 0x6c, 0x20, 0x61, 0x72, 0x6d, 0x73, 0x20, 0x61, 0x73, 0x73, 0x6f, 0x63, 0x69, 0x61, 0x74,
    0x65, 0x64, 0x20, 0x77, 0x69, 0x74, 0x68, 0x20, 0x61, 0x20, 0x72, 0x6f, 0x62, 0x6f, 0x74, 0x0a,
    0x0a, 0x0a, 0x0a, 0x03, 0x06, 0x00, 0x01, 0x12, 0x03, 0x0c, 0x08, 0x12, 0x0a, 0x75, 0x0a, 0x04,
    0x06, 0x00, 0x02, 0x00, 0x12, 0x04, 0x0e, 0x02, 0x12, 0x03, 0x1a, 0x67, 0x20, 0x47, 0x65, 0x74,
    0x45, 0x6e, 0x64, 0x50, 0x6f, 0x73, 0x69, 0x74, 0x69, 0x6f, 0x6e, 0x20, 0x67, 0x65, 0x74, 0x73,
    0x20, 0x74, 0x68, 0x65, 0x20, 0x63, 0x75, 0x72, 0x72, 0x65, 0x6e, 0x74, 0x20, 0x70, 0x6f, 0x73,
    0x69, 0x74, 0x69, 0x6f, 0x6e, 0x20, 0x74, 0x68, 0x65, 0x20, 0x65, 0x6e, 0x64, 0x20, 0x6f, 0x66,
    0x20, 0x74, 0x68, 0x65, 0x20, 0x72, 0x6f, 0x62, 0x6f, 0x74, 0x27, 0x73, 0x20, 0x61, 0x72, 0x6d,
    0x20, 0x65, 0x78, 0x70, 0x72, 0x65, 0x73, 0x73, 0x65, 0x64, 0x20, 0x61, 0x73, 0x20, 0x58, 0x2c,
    0x59, 0x2c, 0x5a, 0x2c, 0x6f, 0x78, 0x2c, 0x6f, 0x79, 0x2c, 0x6f, 0x7a, 0x2c, 0x74, 0x68, 0x65,
    0x74, 0x61, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x06, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x0e, 0x06,
    0x14, 0x0a, 0x0c, 0x0a, 0x05, 0x06, 0x00, 0x02, 0x00, 0x02, 0x12, 0x03, 0x0e, 0x15, 0x2a, 0x0a,
    0x0c, 0x0a, 0x05, 0x06, 0x00, 0x02, 0x00, 0x03, 0x12, 0x03, 0x0e, 0x35, 0x4b, 0x0a, 0x0d, 0x0a,
    0x05, 0x06, 0x00, 0x02, 0x00, 0x04, 0x12, 0x04, 0x0f, 0x04, 0x11, 0x06, 0x0a, 0x11, 0x0a, 0x09,
    0x06, 0x00, 0x02, 0x00, 0x04, 0xb0, 0xca, 0xbc, 0x22, 0x12, 0x04, 0x0f, 0x04, 0x11, 0x06, 0x0a,
    0xac, 0x01, 0x0a, 0x04, 0x06, 0x00, 0x02, 0x01, 0x12, 0x04, 0x16, 0x02, 0x1b, 0x03, 0x1a, 0x9d,
    0x01, 0x20, 0x4d, 0x6f, 0x76, 0x65, 0x54, 0x6f, 0x50, 0x6f, 0x73, 0x69, 0x74, 0x69, 0x6f, 0x6e,
    0x20, 0x6d, 0x6f, 0x76, 0x65, 0x73, 0x20, 0x74, 0x68, 0x65, 0x20, 0x6d, 0x6f, 0x75, 0x6e, 0x74,
    0x20, 0x70, 0x6f, 0x69, 0x6e, 0x74, 0x20, 0x6f, 0x66, 0x20, 0x74, 0x68, 0x65, 0x20, 0x72, 0x6f,
    0x62, 0x6f, 0x74, 0x27, 0x73, 0x20, 0x65, 0x6e, 0x64, 0x20, 0x65, 0x66, 0x66, 0x65, 0x63, 0x74,
    0x6f, 0x72, 0x20, 0x74, 0x6f, 0x20, 0x74, 0x68, 0x65, 0x20, 0x72, 0x65, 0x71, 0x75, 0x65, 0x73,
    0x74, 0x65, 0x64, 0x20, 0x70, 0x6f, 0x73, 0x69, 0x74, 0x69, 0x6f, 0x6e, 0x2e, 0x0a, 0x20, 0x54,
    0x68, 0x69, 0x73, 0x20, 0x77, 0x69, 0x6c, 0x6c, 0x20, 0x62, 0x6c, 0x6f, 0x63, 0x6b, 0x20, 0x75,
    0x6e, 0x74, 0x69, 0x6c, 0x20, 0x64, 0x6f, 0x6e, 0x65, 0x20, 0x6f, 0x72, 0x20, 0x61, 0x20, 0x6e,
    0x65, 0x77, 0x20, 0x6f, 0x70, 0x65, 0x72, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x20, 0x63, 0x61, 0x6e,
    0x63, 0x65, 0x6c, 0x73, 0x20, 0x74, 0x68, 0x69, 0x73, 0x20, 0x6f, 0x6e, 0x65, 0x0a, 0x0a, 0x0c,
    0x0a, 0x05, 0x06, 0x00, 0x02, 0x01, 0x01, 0x12, 0x03, 0x16, 0x06, 0x14, 0x0a, 0x0c, 0x0a, 0x05,
    0x06, 0x00, 0x02, 0x01, 0x02, 0x12, 0x03, 0x16, 0x15, 0x2a, 0x0a, 0x0c, 0x0a, 0x05, 0x06, 0x00,
    0x02, 0x01, 0x03, 0x12, 0x03, 0x16, 0x35, 0x4b, 0x0a, 0x0c, 0x0a, 0x05, 0x06, 0x00, 0x02, 0x01,
    0x04, 0x12, 0x03, 0x17, 0x04, 0x39, 0x0a, 0x0f, 0x0a, 0x08, 0x06, 0x00, 0x02, 0x01, 0x04, 0xa4,
    0x92, 0x05, 0x12, 0x03, 0x17, 0x04, 0x39, 0x0a, 0x0d, 0x0a, 0x05, 0x06, 0x00, 0x02, 0x01, 0x04,
    0x12, 0x04, 0x18, 0x04, 0x1a, 0x06, 0x0a, 0x11, 0x0a, 0x09, 0x06, 0x00, 0x02, 0x01, 0x04, 0xb0,
    0xca, 0xbc, 0x22, 0x12, 0x04, 0x18, 0x04, 0x1a, 0x06, 0x0a, 0x62, 0x0a, 0x04, 0x06, 0x00, 0x02,
    0x02, 0x12, 0x04, 0x1e, 0x02, 0x22, 0x03, 0x1a, 0x54, 0x20, 0x47, 0x65, 0x74, 0x4a, 0x6f, 0x69,
    0x6e, 0x74, 0x50, 0x6f, 0x73, 0x69, 0x74, 0x69, 0x6f, 0x6e, 0x73, 0x20, 0x6c, 0x69, 0x73, 0x74,
    0x73, 0x20, 0x74, 0x68, 0x65, 0x20, 0x6a, 0x6f, 0x69, 0x6e, 0x74, 0x20, 0x70, 0x6f, 0x73, 0x69,
    0x74, 0x69, 0x6f, 0x6e, 0x73, 0x20, 0x28, 0x69, 0x6e, 0x20, 0x64, 0x65, 0x67, 0x72, 0x65, 0x65,
    0x73, 0x29, 0x20, 0x6f, 0x66, 0x20, 0x65, 0x76, 0x65, 0x72, 0x79, 0x20, 0x6a, 0x6f, 0x69, 0x6e,
    0x74, 0x20, 0x6f, 0x6e, 0x20, 0x61, 0x20, 0x72, 0x6f, 0x62, 0x6f, 0x74, 0x0a, 0x0a, 0x0c, 0x0a,
    0x05, 0x06, 0x00, 0x02, 0x02, 0x01, 0x12, 0x03, 0x1e, 0x06, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x06,
    0x00, 0x02, 0x02, 0x02, 0x12, 0x03, 0x1e, 0x18, 0x30, 0x0a, 0x0c, 0x0a, 0x05, 0x06, 0x00, 0x02,
    0x02, 0x03, 0x12, 0x03, 0x1e, 0x3b, 0x54, 0x0a, 0x0d, 0x0a, 0x05, 0x06, 0x00, 0x02, 0x02, 0x04,
    0x12, 0x04, 0x1f, 0x04, 0x21, 0x06, 0x0a, 0x11, 0x0a, 0x09, 0x06, 0x00, 0x02, 0x02, 0x04, 0xb0,
    0xca, 0xbc, 0x22, 0x12, 0x04, 0x1f, 0x04, 0x21, 0x06, 0x0a, 0xbb, 0x01, 0x0a, 0x04, 0x06, 0x00,
    0x02, 0x03, 0x12, 0x04, 0x26, 0x02, 0x2b, 0x03, 0x1a, 0xac, 0x01, 0x20, 0x4d, 0x6f, 0x76, 0x65,
    0x54, 0x6f, 0x4a, 0x6f, 0x69, 0x6e, 0x74, 0x50, 0x6f, 0x73, 0x69, 0x74, 0x69, 0x6f, 0x6e, 0x73,
    0x20, 0x6d, 0x6f, 0x76, 0x65, 0x73, 0x20, 0x65, 0x76, 0x65, 0x72, 0x79, 0x20, 0x6a, 0x6f, 0x69,
    0x6e, 0x74, 0x20, 0x6f, 0x6e, 0x20, 0x61, 0x20, 0x72, 0x6f, 0x62, 0x6f, 0x74, 0x27, 0x73, 0x20,
    0x61, 0x72, 0x6d, 0x20, 0x74, 0x6f, 0x20, 0x73, 0x70, 0x65, 0x63, 0x69, 0x66, 0x69, 0x65, 0x64,
    0x20, 0x61, 0x6e, 0x67, 0x6c, 0x65, 0x73, 0x20, 0x77, 0x68, 0x69, 0x63, 0x68, 0x20, 0x61, 0x72,
    0x65, 0x20, 0x65, 0x78, 0x70, 0x72, 0x65, 0x73, 0x73, 0x65, 0x64, 0x20, 0x69, 0x6e, 0x20, 0x64,
    0x65, 0x67, 0x72, 0x65, 0x65, 0x73, 0x0a, 0x20, 0x54, 0x68, 0x69, 0x73, 0x20, 0x77, 0x69, 0x6c,
    0x6c, 0x20, 0x62, 0x6c, 0x6f, 0x63, 0x6b, 0x20, 0x75, 0x6e, 0x74, 0x69, 0x6c, 0x20, 0x64, 0x6f,
    0x6e, 0x65, 0x20, 0x6f, 0x72, 0x20, 0x61, 0x20, 0x6e, 0x65, 0x77, 0x20, 0x6f, 0x70, 0x65, 0x72,
    0x61, 0x74, 0x69, 0x6f, 0x6e, 0x20, 0x63, 0x61, 0x6e, 0x63, 0x65, 0x6c, 0x73, 0x20, 0x74, 0x68,
    0x69, 0x73, 0x20, 0x6f, 0x6e, 0x65, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x06, 0x00, 0x02, 0x03, 0x01,
    0x12, 0x03, 0x26, 0x06, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x06, 0x00, 0x02, 0x03, 0x02, 0x12, 0x03,
    0x26, 0x1b, 0x36, 0x0a, 0x0c, 0x0a, 0x05, 0x06, 0x00, 0x02, 0x03, 0x03, 0x12, 0x03, 0x26, 0x41,
    0x5d, 0x0a, 0x0c, 0x0a, 0x05, 0x06, 0x00, 0x02, 0x03, 0x04, 0x12, 0x03, 0x27, 0x04, 0x39, 0x0a,
    0x0f, 0x0a, 0x08, 0x06, 0x00, 0x02, 0x03, 0x04, 0xa4, 0x92, 0x05, 0x12, 0x03, 0x27, 0x04, 0x39,
    0x0a, 0x0d, 0x0a, 0x05, 0x06, 0x00, 0x02, 0x03, 0x04, 0x12, 0x04, 0x28, 0x04, 0x2a, 0x06, 0x0a,
    0x11, 0x0a, 0x09, 0x06, 0x00, 0x02, 0x03, 0x04, 0xb0, 0xca, 0xbc, 0x22, 0x12, 0x04, 0x28, 0x04,
    0x2a, 0x06, 0x0a, 0x28, 0x0a, 0x04, 0x06, 0x00, 0x02, 0x04, 0x12, 0x04, 0x2e, 0x02, 0x32, 0x03,
    0x1a, 0x1a, 0x20, 0x53, 0x74, 0x6f, 0x70, 0x20, 0x73, 0x74, 0x6f, 0x70, 0x73, 0x20, 0x61, 0x20,
    0x72, 0x6f, 0x62, 0x6f, 0x74, 0x27, 0x73, 0x20, 0x61, 0x72, 0x6d, 0x0a, 0x0a, 0x0c, 0x0a, 0x05,
    0x06, 0x00, 0x02, 0x04, 0x01, 0x12, 0x03, 0x2e, 0x06, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x06, 0x00,
    0x02, 0x04, 0x02, 0x12, 0x03, 0x2e, 0x0b, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x06, 0x00, 0x02, 0x04,
    0x03, 0x12, 0x03, 0x2e, 0x21, 0x2d, 0x0a, 0x0d, 0x0a, 0x05, 0x06, 0x00, 0x02, 0x04, 0x04, 0x12,
    0x04, 0x2f, 0x04, 0x31, 0x06, 0x0a, 0x11, 0x0a, 0x09, 0x06, 0x00, 0x02, 0x04, 0x04, 0xb0, 0xca,
    0xbc, 0x22, 0x12, 0x04, 0x2f, 0x04, 0x31, 0x06, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x00, 0x12, 0x04,
    0x35, 0x00, 0x3a, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x00, 0x01, 0x12, 0x03, 0x35, 0x08, 0x1d,
    0x0a, 0x1d, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x00, 0x12, 0x03, 0x37, 0x02, 0x12, 0x1a, 0x10, 0x20,
    0x4e, 0x61, 0x6d, 0x65, 0x20, 0x6f, 0x66, 0x20, 0x61, 0x6e, 0x20, 0x61, 0x72, 0x6d, 0x0a, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x05, 0x12, 0x03, 0x37, 0x02, 0x08, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x37, 0x09, 0x0d, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x00, 0x03, 0x12, 0x03, 0x37, 0x10, 0x11, 0x0a, 0x31, 0x0a, 0x04, 0x04, 0x00, 0x02,
    0x01, 0x12, 0x03, 0x39, 0x02, 0x24, 0x1a, 0x24, 0x20, 0x41, 0x64, 0x64, 0x69, 0x74, 0x69, 0x6f,
    0x6e, 0x61, 0x6c, 0x20, 0x61, 0x72, 0x67, 0x75, 0x6d, 0x65, 0x6e, 0x74, 0x73, 0x20, 0x74, 0x6f,
    0x20, 0x74, 0x68, 0x65, 0x20, 0x6d, 0x65, 0x74, 0x68, 0x6f, 0x64, 0x0a, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x01, 0x06, 0x12, 0x03, 0x39, 0x02, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x01, 0x01, 0x12, 0x03, 0x39, 0x19, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01,
    0x03, 0x12, 0x03, 0x39, 0x21, 0x23, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x01, 0x12, 0x04, 0x3c, 0x00,
    0x40, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x01, 0x01, 0x12, 0x03, 0x3c, 0x08, 0x1e, 0x0a, 0xcb,
    0x01, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x00, 0x12, 0x03, 0x3f, 0x02, 0x1a, 0x1a, 0xbd, 0x01, 0x20,
    0x52, 0x65, 0x74, 0x75, 0x72, 0x6e, 0x73, 0x20, 0x36, 0x64, 0x20, 0x70, 0x6f, 0x73, 0x65, 0x20,
    0x6f, 0x66, 0x20, 0x74, 0x68, 0x65, 0x20, 0x65, 0x6e, 0x64, 0x20, 0x65, 0x66, 0x66, 0x65, 0x63,
    0x74, 0x6f, 0x72, 0x20, 0x72, 0x65, 0x6c, 0x61, 0x74, 0x69, 0x76, 0x65, 0x20, 0x74, 0x6f, 0x20,
    0x74, 0x68, 0x65, 0x20, 0x62, 0x61, 0x73, 0x65, 0x2c, 0x20, 0x72, 0x65, 0x70, 0x72, 0x65, 0x73,
    0x65, 0x6e, 0x74, 0x65, 0x64, 0x20, 0x62, 0x79, 0x20, 0x58, 0x2c, 0x59, 0x2c, 0x5a, 0x20, 0x63,
    0x6f, 0x6f, 0x72, 0x64, 0x69, 0x6e, 0x61, 0x74, 0x65, 0x73, 0x20, 0x77, 0x68, 0x69, 0x63, 0x68,
    0x20, 0x65, 0x78, 0x70, 0x72, 0x65, 0x73, 0x73, 0x0a, 0x20, 0x6d, 0x69, 0x6c, 0x6c, 0x69, 0x6d,
    0x65, 0x74, 0x65, 0x72, 0x73, 0x20, 0x61, 0x6e, 0x64, 0x20, 0x74, 0x68, 0x65, 0x74, 0x61, 0x2c,
    0x20, 0x6f, 0x78, 0x2c, 0x20, 0x6f, 0x79, 0x2c, 0x20, 0x6f, 0x7a, 0x20, 0x63, 0x6f, 0x6f, 0x72,
    0x64, 0x69, 0x6e, 0x61, 0x74, 0x65, 0x73, 0x20, 0x77, 0x68, 0x69, 0x63, 0x68, 0x20, 0x65, 0x78,
    0x70, 0x72, 0x65, 0x73, 0x73, 0x20, 0x61, 0x6e, 0x20, 0x6f, 0x72, 0x69, 0x65, 0x6e, 0x74, 0x61,
    0x74, 0x69, 0x6f, 0x6e, 0x20, 0x76, 0x65, 0x63, 0x74, 0x6f, 0x72, 0x0a, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x01, 0x02, 0x00, 0x06, 0x12, 0x03, 0x3f, 0x02, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01,
    0x02, 0x00, 0x01, 0x12, 0x03, 0x3f, 0x11, 0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00,
    0x03, 0x12, 0x03, 0x3f, 0x18, 0x19, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x02, 0x12, 0x04, 0x42, 0x00,
    0x47, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x02, 0x01, 0x12, 0x03, 0x42, 0x08, 0x16, 0x0a, 0xfb,
    0x01, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x00, 0x12, 0x03, 0x46, 0x02, 0x1d, 0x1a, 0xed, 0x01, 0x20,
    0x41, 0x20, 0x6c, 0x69, 0x73, 0x74, 0x20, 0x6f, 0x66, 0x20, 0x6a, 0x6f, 0x69, 0x6e, 0x74, 0x20,
    0x70, 0x6f, 0x73, 0x69, 0x74, 0x69, 0x6f, 0x6e, 0x73, 0x2e, 0x20, 0x52, 0x6f, 0x74, 0x61, 0x74,
    0x69, 0x6f, 0x6e, 0x73, 0x20, 0x76, 0x61, 0x6c, 0x75, 0x65, 0x73, 0x20, 0x61, 0x72, 0x65, 0x20,
    0x69, 0x6e, 0x20, 0x64, 0x65, 0x67, 0x72, 0x65, 0x65, 0x73, 0x2c, 0x20, 0x74, 0x72, 0x61, 0x6e,
    0x73, 0x6c, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x61, 0x6c, 0x20, 0x76, 0x61, 0x6c, 0x75, 0x65, 0x73,
    0x20, 0x69, 0x6e, 0x20, 0x6d, 0x6d, 0x2e, 0x0a, 0x20, 0x54, 0x68, 0x65, 0x20, 0x6e, 0x75, 0x6d,
    0x62, 0x65, 0x72, 0x73, 0x20, 0x61, 0x72, 0x65, 0x20, 0x6f, 0x72, 0x64, 0x65, 0x72, 0x65, 0x64,
    0x20, 0x73, 0x70, 0x61, 0x74, 0x69, 0x61, 0x6c, 0x6c, 0x79, 0x20, 0x66, 0x72, 0x6f, 0x6d, 0x20,
    0x74, 0x68, 0x65, 0x20, 0x62, 0x61, 0x73, 0x65, 0x20, 0x74, 0x6f, 0x77, 0x61, 0x72, 0x64, 0x20,
    0x74, 0x68, 0x65, 0x20, 0x65, 0x6e, 0x64, 0x20, 0x65, 0x66, 0x66, 0x65, 0x63, 0x74, 0x6f, 0x72,
    0x0a, 0x20, 0x54, 0x68, 0x69, 0x73, 0x20, 0x69, 0x73, 0x20, 0x75, 0x73, 0x65, 0x64, 0x20, 0x69,
    0x6e, 0x20, 0x47, 0x65, 0x74, 0x4a, 0x6f, 0x69, 0x6e, 0x74, 0x50, 0x6f, 0x73, 0x69, 0x74, 0x69,
    0x6f, 0x6e, 0x73, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x20, 0x61, 0x6e, 0x64, 0x20,
    0x4d, 0x6f, 0x76, 0x65, 0x54, 0x6f, 0x4a, 0x6f, 0x69, 0x6e, 0x74, 0x50, 0x6f, 0x73, 0x69, 0x74,
    0x69, 0x6f, 0x6e, 0x73, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x0a, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x02, 0x02, 0x00, 0x04, 0x12, 0x03, 0x46, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02,
    0x02, 0x00, 0x05, 0x12, 0x03, 0x46, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00,
    0x01, 0x12, 0x03, 0x46, 0x12, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x03, 0x12,
    0x03, 0x46, 0x1b, 0x1c, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x03, 0x12, 0x04, 0x49, 0x00, 0x4e, 0x01,
    0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x03, 0x01, 0x12, 0x03, 0x49, 0x08, 0x20, 0x0a, 0x1d, 0x0a, 0x04,
    0x04, 0x03, 0x02, 0x00, 0x12, 0x03, 0x4b, 0x02, 0x12, 0x1a, 0x10, 0x20, 0x4e, 0x61, 0x6d, 0x65,
    0x20, 0x6f, 0x66, 0x20, 0x61, 0x6e, 0x20, 0x61, 0x72, 0x6d, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x03, 0x02, 0x00, 0x05, 0x12, 0x03, 0x4b, 0x02, 0x08, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02,
    0x00, 0x01, 0x12, 0x03, 0x4b, 0x09, 0x0d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x03,
    0x12, 0x03, 0x4b, 0x10, 0x11, 0x0a, 0x31, 0x0a, 0x04, 0x04, 0x03, 0x02, 0x01, 0x12, 0x03, 0x4d,
    0x02, 0x24, 0x1a, 0x24, 0x20, 0x41, 0x64, 0x64, 0x69, 0x74, 0x69, 0x6f, 0x6e, 0x61, 0x6c, 0x20,
    0x61, 0x72, 0x67, 0x75, 0x6d, 0x65, 0x6e, 0x74, 0x73, 0x20, 0x74, 0x6f, 0x20, 0x74, 0x68, 0x65,
    0x20, 0x6d, 0x65, 0x74, 0x68, 0x6f, 0x64, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x01,
    0x06, 0x12, 0x03, 0x4d, 0x02, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x01, 0x01, 0x12,
    0x03, 0x4d, 0x19, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x01, 0x03, 0x12, 0x03, 0x4d,
    0x21, 0x23, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x04, 0x12, 0x04, 0x50, 0x00, 0x53, 0x01, 0x0a, 0x0a,
    0x0a, 0x03, 0x04, 0x04, 0x01, 0x12, 0x03, 0x50, 0x08, 0x21, 0x0a, 0x23, 0x0a, 0x04, 0x04, 0x04,
    0x02, 0x00, 0x12, 0x03, 0x52, 0x02, 0x1f, 0x1a, 0x16, 0x61, 0x20, 0x6c, 0x69, 0x73, 0x74, 0x20,
    0x4a, 0x6f, 0x69, 0x6e, 0x74, 0x50, 0x6f, 0x73, 0x69, 0x74, 0x69, 0x6f, 0x6e, 0x73, 0x0a, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x00, 0x06, 0x12, 0x03, 0x52, 0x02, 0x10, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x04, 0x02, 0x00, 0x01, 0x12, 0x03, 0x52, 0x11, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x04, 0x02, 0x00, 0x03, 0x12, 0x03, 0x52, 0x1d, 0x1e, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x05, 0x12,
    0x04, 0x55, 0x00, 0x5c, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x05, 0x01, 0x12, 0x03, 0x55, 0x08,
    0x1d, 0x0a, 0x1d, 0x0a, 0x04, 0x04, 0x05, 0x02, 0x00, 0x12, 0x03, 0x57, 0x02, 0x12, 0x1a, 0x10,
    0x20, 0x4e, 0x61, 0x6d, 0x65, 0x20, 0x6f, 0x66, 0x20, 0x61, 0x6e, 0x20, 0x61, 0x72, 0x6d, 0x0a,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x00, 0x05, 0x12, 0x03, 0x57, 0x02, 0x08, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x05, 0x02, 0x00, 0x01, 0x12, 0x03, 0x57, 0x09, 0x0d, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x05, 0x02, 0x00, 0x03, 0x12, 0x03, 0x57, 0x10, 0x11, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x05,
    0x02, 0x01, 0x12, 0x03, 0x58, 0x02, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x01, 0x06,
    0x12, 0x03, 0x58, 0x02, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x01, 0x01, 0x12, 0x03,
    0x58, 0x11, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x01, 0x03, 0x12, 0x03, 0x58, 0x16,
    0x17, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x05, 0x02, 0x02, 0x12, 0x03, 0x59, 0x02, 0x30, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x05, 0x02, 0x02, 0x04, 0x12, 0x03, 0x59, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x05, 0x02, 0x02, 0x06, 0x12, 0x03, 0x59, 0x0b, 0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05,
    0x02, 0x02, 0x01, 0x12, 0x03, 0x59, 0x20, 0x2b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x02,
    0x03, 0x12, 0x03, 0x59, 0x2e, 0x2f, 0x0a, 0x31, 0x0a, 0x04, 0x04, 0x05, 0x02, 0x03, 0x12, 0x03,
    0x5b, 0x02, 0x24, 0x1a, 0x24, 0x20, 0x41, 0x64, 0x64, 0x69, 0x74, 0x69, 0x6f, 0x6e, 0x61, 0x6c,
    0x20, 0x61, 0x72, 0x67, 0x75, 0x6d, 0x65, 0x6e, 0x74, 0x73, 0x20, 0x74, 0x6f, 0x20, 0x74, 0x68,
    0x65, 0x20, 0x6d, 0x65, 0x74, 0x68, 0x6f, 0x64, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02,
    0x03, 0x06, 0x12, 0x03, 0x5b, 0x02, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x03, 0x01,
    0x12, 0x03, 0x5b, 0x19, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x03, 0x03, 0x12, 0x03,
    0x5b, 0x21, 0x23, 0x0a, 0x09, 0x0a, 0x02, 0x04, 0x06, 0x12, 0x03, 0x5e, 0x00, 0x21, 0x0a, 0x0a,
    0x0a, 0x03, 0x04, 0x06, 0x01, 0x12, 0x03, 0x5e, 0x08, 0x1e, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x07,
    0x12, 0x04, 0x60, 0x00, 0x68, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x07, 0x01, 0x12, 0x03, 0x60,
    0x08, 0x23, 0x0a, 0x1d, 0x0a, 0x04, 0x04, 0x07, 0x02, 0x00, 0x12, 0x03, 0x62, 0x02, 0x12, 0x1a,
    0x10, 0x20, 0x4e, 0x61, 0x6d, 0x65, 0x20, 0x6f, 0x66, 0x20, 0x61, 0x6e, 0x20, 0x61, 0x72, 0x6d,
    0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x00, 0x05, 0x12, 0x03, 0x62, 0x02, 0x08, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x00, 0x01, 0x12, 0x03, 0x62, 0x09, 0x0d, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x07, 0x02, 0x00, 0x03, 0x12, 0x03, 0x62, 0x10, 0x11, 0x0a, 0x95, 0x01, 0x0a, 0x04,
    0x04, 0x07, 0x02, 0x01, 0x12, 0x03, 0x65, 0x02, 0x1f, 0x1a, 0x87, 0x01, 0x20, 0x41, 0x20, 0x6c,
    0x69, 0x73, 0x74, 0x20, 0x6f, 0x66, 0x20, 0x6a, 0x6f, 0x69, 0x6e, 0x74, 0x20, 0x70, 0x6f, 0x73,
    0x69, 0x74, 0x69, 0x6f, 0x6e, 0x73, 0x0a, 0x20, 0x54, 0x68, 0x65, 0x72, 0x65, 0x20, 0x73, 0x68,
    0x6f, 0x75, 0x6c, 0x64, 0x20, 0x62, 0x65, 0x20, 0x31, 0x20, 0x65, 0x6e, 0x74, 0x72, 0x79, 0x20,
    0x69, 0x6e, 0x20, 0x74, 0x68, 0x65, 0x20, 0x6c, 0x69, 0x73, 0x74, 0x20, 0x70, 0x65, 0x72, 0x20,
    0x6a, 0x6f, 0x69, 0x6e, 0x74, 0x20, 0x44, 0x4f, 0x46, 0x2c, 0x20, 0x6f, 0x72, 0x64, 0x65, 0x72,
    0x65, 0x64, 0x20, 0x73, 0x70, 0x61, 0x74, 0x69, 0x61, 0x6c, 0x6c, 0x79, 0x20, 0x66, 0x72, 0x6f,
    0x6d, 0x20, 0x74, 0x68, 0x65, 0x20, 0x62, 0x61, 0x73, 0x65, 0x20, 0x74, 0x6f, 0x77, 0x61, 0x72,
    0x64, 0x20, 0x74, 0x68, 0x65, 0x20, 0x65, 0x6e, 0x64, 0x20, 0x65, 0x66, 0x66, 0x65, 0x63, 0x74,
    0x6f, 0x72, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x01, 0x06, 0x12, 0x03, 0x65, 0x02,
    0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x01, 0x01, 0x12, 0x03, 0x65, 0x11, 0x1a, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x01, 0x03, 0x12, 0x03, 0x65, 0x1d, 0x1e, 0x0a, 0x31, 0x0a,
    0x04, 0x04, 0x07, 0x02, 0x02, 0x12, 0x03, 0x67, 0x02, 0x24, 0x1a, 0x24, 0x20, 0x41, 0x64, 0x64,
    0x69, 0x74, 0x69, 0x6f, 0x6e, 0x61, 0x6c, 0x20, 0x61, 0x72, 0x67, 0x75, 0x6d, 0x65, 0x6e, 0x74,
    0x73, 0x20, 0x74, 0x6f, 0x20, 0x74, 0x68, 0x65, 0x20, 0x6d, 0x65, 0x74, 0x68, 0x6f, 0x64, 0x0a,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x02, 0x06, 0x12, 0x03, 0x67, 0x02, 0x18, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x07, 0x02, 0x02, 0x01, 0x12, 0x03, 0x67, 0x19, 0x1e, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x07, 0x02, 0x02, 0x03, 0x12, 0x03, 0x67, 0x21, 0x23, 0x0a, 0x09, 0x0a, 0x02, 0x04, 0x08,
    0x12, 0x03, 0x6a, 0x00, 0x27, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x08, 0x01, 0x12, 0x03, 0x6a, 0x08,
    0x24, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x09, 0x12, 0x04, 0x6c, 0x00, 0x71, 0x01, 0x0a, 0x0a, 0x0a,
    0x03, 0x04, 0x09, 0x01, 0x12, 0x03, 0x6c, 0x08, 0x13, 0x0a, 0x1d, 0x0a, 0x04, 0x04, 0x09, 0x02,
    0x00, 0x12, 0x03, 0x6e, 0x02, 0x12, 0x1a, 0x10, 0x20, 0x4e, 0x61, 0x6d, 0x65, 0x20, 0x6f, 0x66,
    0x20, 0x61, 0x6e, 0x20, 0x61, 0x72, 0x6d, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x00,
    0x05, 0x12, 0x03, 0x6e, 0x02, 0x08, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x00, 0x01, 0x12,
    0x03, 0x6e, 0x09, 0x0d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x00, 0x03, 0x12, 0x03, 0x6e,
    0x10, 0x11, 0x0a, 0x31, 0x0a, 0x04, 0x04, 0x09, 0x02, 0x01, 0x12, 0x03, 0x70, 0x02, 0x24, 0x1a,
    0x24, 0x20, 0x41, 0x64, 0x64, 0x69, 0x74, 0x69, 0x6f, 0x6e, 0x61, 0x6c, 0x20, 0x61, 0x72, 0x67,
    0x75, 0x6d, 0x65, 0x6e, 0x74, 0x73, 0x20, 0x74, 0x6f, 0x20, 0x74, 0x68, 0x65, 0x20, 0x6d, 0x65,
    0x74, 0x68, 0x6f, 0x64, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x01, 0x06, 0x12, 0x03,
    0x70, 0x02, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x01, 0x01, 0x12, 0x03, 0x70, 0x19,
    0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x01, 0x03, 0x12, 0x03, 0x70, 0x21, 0x23, 0x0a,
    0x09, 0x0a, 0x02, 0x04, 0x0a, 0x12, 0x03, 0x73, 0x00, 0x17, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x0a,
    0x01, 0x12, 0x03, 0x73, 0x08, 0x14, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x0b, 0x12, 0x04, 0x75, 0x00,
    0x79, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x0b, 0x01, 0x12, 0x03, 0x75, 0x08, 0x0e, 0x0a, 0x0b,
    0x0a, 0x04, 0x04, 0x0b, 0x02, 0x00, 0x12, 0x03, 0x76, 0x02, 0x22, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x0b, 0x02, 0x00, 0x06, 0x12, 0x03, 0x76, 0x02, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02,
    0x00, 0x01, 0x12, 0x03, 0x76, 0x11, 0x1d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x00, 0x03,
    0x12, 0x03, 0x76, 0x20, 0x21, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x0b, 0x02, 0x01, 0x12, 0x03, 0x77,
    0x02, 0x25, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x01, 0x06, 0x12, 0x03, 0x77, 0x02, 0x10,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x01, 0x01, 0x12, 0x03, 0x77, 0x11, 0x20, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x0b, 0x02, 0x01, 0x03, 0x12, 0x03, 0x77, 0x23, 0x24, 0x0a, 0x0b, 0x0a, 0x04,
    0x04, 0x0b, 0x02, 0x02, 0x12, 0x03, 0x78, 0x02, 0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02,
    0x02, 0x05, 0x12, 0x03, 0x78, 0x02, 0x06, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x02, 0x01,
    0x12, 0x03, 0x78, 0x07, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x02, 0x03, 0x12, 0x03,
    0x78, 0x13, 0x14, 0x62, 0x06, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x33,
];
include!("viam.component.arm.v1.tonic.rs");
// @@protoc_insertion_point(module)