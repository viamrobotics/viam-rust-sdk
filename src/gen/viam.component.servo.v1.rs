// @generated
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MoveRequest {
    /// the name of the servo, as registered
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// the degrees by which to rotate the servo. Accepted values are between 0 and 180
    #[prost(uint32, tag="2")]
    pub angle_deg: u32,
    /// Additional arguments to the method
    #[prost(message, optional, tag="99")]
    pub extra: ::core::option::Option<::prost_types::Struct>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MoveResponse {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetPositionRequest {
    /// the name of the servo, as registered
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// Additional arguments to the method
    #[prost(message, optional, tag="99")]
    pub extra: ::core::option::Option<::prost_types::Struct>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetPositionResponse {
    /// the degrees from neutral by which the servo is currently rotated. Values are between 0 and 180
    #[prost(uint32, tag="1")]
    pub position_deg: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StopRequest {
    /// Name of a servo
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
    #[prost(uint32, tag="1")]
    pub position_deg: u32,
    #[prost(bool, tag="2")]
    pub is_moving: bool,
}
/// Encoded file descriptor set for the `viam.component.servo.v1` package
pub const FILE_DESCRIPTOR_SET: &[u8] = &[
    0x0a, 0x98, 0x17, 0x0a, 0x1e, 0x63, 0x6f, 0x6d, 0x70, 0x6f, 0x6e, 0x65, 0x6e, 0x74, 0x2f, 0x73,
    0x65, 0x72, 0x76, 0x6f, 0x2f, 0x76, 0x31, 0x2f, 0x73, 0x65, 0x72, 0x76, 0x6f, 0x2e, 0x70, 0x72,
    0x6f, 0x74, 0x6f, 0x12, 0x17, 0x76, 0x69, 0x61, 0x6d, 0x2e, 0x63, 0x6f, 0x6d, 0x70, 0x6f, 0x6e,
    0x65, 0x6e, 0x74, 0x2e, 0x73, 0x65, 0x72, 0x76, 0x6f, 0x2e, 0x76, 0x31, 0x1a, 0x16, 0x63, 0x6f,
    0x6d, 0x6d, 0x6f, 0x6e, 0x2f, 0x76, 0x31, 0x2f, 0x63, 0x6f, 0x6d, 0x6d, 0x6f, 0x6e, 0x2e, 0x70,
    0x72, 0x6f, 0x74, 0x6f, 0x1a, 0x1c, 0x67, 0x6f, 0x6f, 0x67, 0x6c, 0x65, 0x2f, 0x61, 0x70, 0x69,
    0x2f, 0x61, 0x6e, 0x6e, 0x6f, 0x74, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x73, 0x2e, 0x70, 0x72, 0x6f,
    0x74, 0x6f, 0x1a, 0x1c, 0x67, 0x6f, 0x6f, 0x67, 0x6c, 0x65, 0x2f, 0x70, 0x72, 0x6f, 0x74, 0x6f,
    0x62, 0x75, 0x66, 0x2f, 0x73, 0x74, 0x72, 0x75, 0x63, 0x74, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f,
    0x22, 0x6d, 0x0a, 0x0b, 0x4d, 0x6f, 0x76, 0x65, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x12,
    0x12, 0x0a, 0x04, 0x6e, 0x61, 0x6d, 0x65, 0x18, 0x01, 0x20, 0x01, 0x28, 0x09, 0x52, 0x04, 0x6e,
    0x61, 0x6d, 0x65, 0x12, 0x1b, 0x0a, 0x09, 0x61, 0x6e, 0x67, 0x6c, 0x65, 0x5f, 0x64, 0x65, 0x67,
    0x18, 0x02, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x08, 0x61, 0x6e, 0x67, 0x6c, 0x65, 0x44, 0x65, 0x67,
    0x12, 0x2d, 0x0a, 0x05, 0x65, 0x78, 0x74, 0x72, 0x61, 0x18, 0x63, 0x20, 0x01, 0x28, 0x0b, 0x32,
    0x17, 0x2e, 0x67, 0x6f, 0x6f, 0x67, 0x6c, 0x65, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x62, 0x75,
    0x66, 0x2e, 0x53, 0x74, 0x72, 0x75, 0x63, 0x74, 0x52, 0x05, 0x65, 0x78, 0x74, 0x72, 0x61, 0x22,
    0x0e, 0x0a, 0x0c, 0x4d, 0x6f, 0x76, 0x65, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x22,
    0x57, 0x0a, 0x12, 0x47, 0x65, 0x74, 0x50, 0x6f, 0x73, 0x69, 0x74, 0x69, 0x6f, 0x6e, 0x52, 0x65,
    0x71, 0x75, 0x65, 0x73, 0x74, 0x12, 0x12, 0x0a, 0x04, 0x6e, 0x61, 0x6d, 0x65, 0x18, 0x01, 0x20,
    0x01, 0x28, 0x09, 0x52, 0x04, 0x6e, 0x61, 0x6d, 0x65, 0x12, 0x2d, 0x0a, 0x05, 0x65, 0x78, 0x74,
    0x72, 0x61, 0x18, 0x63, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x17, 0x2e, 0x67, 0x6f, 0x6f, 0x67, 0x6c,
    0x65, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x62, 0x75, 0x66, 0x2e, 0x53, 0x74, 0x72, 0x75, 0x63,
    0x74, 0x52, 0x05, 0x65, 0x78, 0x74, 0x72, 0x61, 0x22, 0x38, 0x0a, 0x13, 0x47, 0x65, 0x74, 0x50,
    0x6f, 0x73, 0x69, 0x74, 0x69, 0x6f, 0x6e, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x12,
    0x21, 0x0a, 0x0c, 0x70, 0x6f, 0x73, 0x69, 0x74, 0x69, 0x6f, 0x6e, 0x5f, 0x64, 0x65, 0x67, 0x18,
    0x01, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x0b, 0x70, 0x6f, 0x73, 0x69, 0x74, 0x69, 0x6f, 0x6e, 0x44,
    0x65, 0x67, 0x22, 0x50, 0x0a, 0x0b, 0x53, 0x74, 0x6f, 0x70, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73,
    0x74, 0x12, 0x12, 0x0a, 0x04, 0x6e, 0x61, 0x6d, 0x65, 0x18, 0x01, 0x20, 0x01, 0x28, 0x09, 0x52,
    0x04, 0x6e, 0x61, 0x6d, 0x65, 0x12, 0x2d, 0x0a, 0x05, 0x65, 0x78, 0x74, 0x72, 0x61, 0x18, 0x63,
    0x20, 0x01, 0x28, 0x0b, 0x32, 0x17, 0x2e, 0x67, 0x6f, 0x6f, 0x67, 0x6c, 0x65, 0x2e, 0x70, 0x72,
    0x6f, 0x74, 0x6f, 0x62, 0x75, 0x66, 0x2e, 0x53, 0x74, 0x72, 0x75, 0x63, 0x74, 0x52, 0x05, 0x65,
    0x78, 0x74, 0x72, 0x61, 0x22, 0x0e, 0x0a, 0x0c, 0x53, 0x74, 0x6f, 0x70, 0x52, 0x65, 0x73, 0x70,
    0x6f, 0x6e, 0x73, 0x65, 0x22, 0x48, 0x0a, 0x06, 0x53, 0x74, 0x61, 0x74, 0x75, 0x73, 0x12, 0x21,
    0x0a, 0x0c, 0x70, 0x6f, 0x73, 0x69, 0x74, 0x69, 0x6f, 0x6e, 0x5f, 0x64, 0x65, 0x67, 0x18, 0x01,
    0x20, 0x01, 0x28, 0x0d, 0x52, 0x0b, 0x70, 0x6f, 0x73, 0x69, 0x74, 0x69, 0x6f, 0x6e, 0x44, 0x65,
    0x67, 0x12, 0x1b, 0x0a, 0x09, 0x69, 0x73, 0x5f, 0x6d, 0x6f, 0x76, 0x69, 0x6e, 0x67, 0x18, 0x02,
    0x20, 0x01, 0x28, 0x08, 0x52, 0x08, 0x69, 0x73, 0x4d, 0x6f, 0x76, 0x69, 0x6e, 0x67, 0x32, 0xc3,
    0x03, 0x0a, 0x0c, 0x53, 0x65, 0x72, 0x76, 0x6f, 0x53, 0x65, 0x72, 0x76, 0x69, 0x63, 0x65, 0x12,
    0x89, 0x01, 0x0a, 0x04, 0x4d, 0x6f, 0x76, 0x65, 0x12, 0x24, 0x2e, 0x76, 0x69, 0x61, 0x6d, 0x2e,
    0x63, 0x6f, 0x6d, 0x70, 0x6f, 0x6e, 0x65, 0x6e, 0x74, 0x2e, 0x73, 0x65, 0x72, 0x76, 0x6f, 0x2e,
    0x76, 0x31, 0x2e, 0x4d, 0x6f, 0x76, 0x65, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x1a, 0x25,
    0x2e, 0x76, 0x69, 0x61, 0x6d, 0x2e, 0x63, 0x6f, 0x6d, 0x70, 0x6f, 0x6e, 0x65, 0x6e, 0x74, 0x2e,
    0x73, 0x65, 0x72, 0x76, 0x6f, 0x2e, 0x76, 0x31, 0x2e, 0x4d, 0x6f, 0x76, 0x65, 0x52, 0x65, 0x73,
    0x70, 0x6f, 0x6e, 0x73, 0x65, 0x22, 0x34, 0xa0, 0x92, 0x29, 0x01, 0x82, 0xd3, 0xe4, 0x93, 0x02,
    0x2a, 0x1a, 0x28, 0x2f, 0x76, 0x69, 0x61, 0x6d, 0x2f, 0x61, 0x70, 0x69, 0x2f, 0x76, 0x31, 0x2f,
    0x63, 0x6f, 0x6d, 0x70, 0x6f, 0x6e, 0x65, 0x6e, 0x74, 0x2f, 0x73, 0x65, 0x72, 0x76, 0x6f, 0x2f,
    0x7b, 0x6e, 0x61, 0x6d, 0x65, 0x7d, 0x2f, 0x6d, 0x6f, 0x76, 0x65, 0x12, 0x9e, 0x01, 0x0a, 0x0b,
    0x47, 0x65, 0x74, 0x50, 0x6f, 0x73, 0x69, 0x74, 0x69, 0x6f, 0x6e, 0x12, 0x2b, 0x2e, 0x76, 0x69,
    0x61, 0x6d, 0x2e, 0x63, 0x6f, 0x6d, 0x70, 0x6f, 0x6e, 0x65, 0x6e, 0x74, 0x2e, 0x73, 0x65, 0x72,
    0x76, 0x6f, 0x2e, 0x76, 0x31, 0x2e, 0x47, 0x65, 0x74, 0x50, 0x6f, 0x73, 0x69, 0x74, 0x69, 0x6f,
    0x6e, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x1a, 0x2c, 0x2e, 0x76, 0x69, 0x61, 0x6d, 0x2e,
    0x63, 0x6f, 0x6d, 0x70, 0x6f, 0x6e, 0x65, 0x6e, 0x74, 0x2e, 0x73, 0x65, 0x72, 0x76, 0x6f, 0x2e,
    0x76, 0x31, 0x2e, 0x47, 0x65, 0x74, 0x50, 0x6f, 0x73, 0x69, 0x74, 0x69, 0x6f, 0x6e, 0x52, 0x65,
    0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x22, 0x34, 0x82, 0xd3, 0xe4, 0x93, 0x02, 0x2e, 0x12, 0x2c,
    0x2f, 0x76, 0x69, 0x61, 0x6d, 0x2f, 0x61, 0x70, 0x69, 0x2f, 0x76, 0x31, 0x2f, 0x63, 0x6f, 0x6d,
    0x70, 0x6f, 0x6e, 0x65, 0x6e, 0x74, 0x2f, 0x73, 0x65, 0x72, 0x76, 0x6f, 0x2f, 0x7b, 0x6e, 0x61,
    0x6d, 0x65, 0x7d, 0x2f, 0x70, 0x6f, 0x73, 0x69, 0x74, 0x69, 0x6f, 0x6e, 0x12, 0x85, 0x01, 0x0a,
    0x04, 0x53, 0x74, 0x6f, 0x70, 0x12, 0x24, 0x2e, 0x76, 0x69, 0x61, 0x6d, 0x2e, 0x63, 0x6f, 0x6d,
    0x70, 0x6f, 0x6e, 0x65, 0x6e, 0x74, 0x2e, 0x73, 0x65, 0x72, 0x76, 0x6f, 0x2e, 0x76, 0x31, 0x2e,
    0x53, 0x74, 0x6f, 0x70, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x1a, 0x25, 0x2e, 0x76, 0x69,
    0x61, 0x6d, 0x2e, 0x63, 0x6f, 0x6d, 0x70, 0x6f, 0x6e, 0x65, 0x6e, 0x74, 0x2e, 0x73, 0x65, 0x72,
    0x76, 0x6f, 0x2e, 0x76, 0x31, 0x2e, 0x53, 0x74, 0x6f, 0x70, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e,
    0x73, 0x65, 0x22, 0x30, 0x82, 0xd3, 0xe4, 0x93, 0x02, 0x2a, 0x22, 0x28, 0x2f, 0x76, 0x69, 0x61,
    0x6d, 0x2f, 0x61, 0x70, 0x69, 0x2f, 0x76, 0x31, 0x2f, 0x63, 0x6f, 0x6d, 0x70, 0x6f, 0x6e, 0x65,
    0x6e, 0x74, 0x2f, 0x73, 0x65, 0x72, 0x76, 0x6f, 0x2f, 0x7b, 0x6e, 0x61, 0x6d, 0x65, 0x7d, 0x2f,
    0x73, 0x74, 0x6f, 0x70, 0x42, 0x41, 0x0a, 0x1b, 0x63, 0x6f, 0x6d, 0x2e, 0x76, 0x69, 0x61, 0x6d,
    0x2e, 0x63, 0x6f, 0x6d, 0x70, 0x6f, 0x6e, 0x65, 0x6e, 0x74, 0x2e, 0x73, 0x65, 0x72, 0x76, 0x6f,
    0x2e, 0x76, 0x31, 0x5a, 0x22, 0x67, 0x6f, 0x2e, 0x76, 0x69, 0x61, 0x6d, 0x2e, 0x63, 0x6f, 0x6d,
    0x2f, 0x61, 0x70, 0x69, 0x2f, 0x63, 0x6f, 0x6d, 0x70, 0x6f, 0x6e, 0x65, 0x6e, 0x74, 0x2f, 0x73,
    0x65, 0x72, 0x76, 0x6f, 0x2f, 0x76, 0x31, 0x4a, 0xb9, 0x0e, 0x0a, 0x06, 0x12, 0x04, 0x00, 0x00,
    0x48, 0x01, 0x0a, 0x08, 0x0a, 0x01, 0x0c, 0x12, 0x03, 0x00, 0x00, 0x12, 0x0a, 0x08, 0x0a, 0x01,
    0x02, 0x12, 0x03, 0x02, 0x00, 0x20, 0x0a, 0x09, 0x0a, 0x02, 0x03, 0x00, 0x12, 0x03, 0x04, 0x00,
    0x20, 0x0a, 0x09, 0x0a, 0x02, 0x03, 0x01, 0x12, 0x03, 0x05, 0x00, 0x26, 0x0a, 0x09, 0x0a, 0x02,
    0x03, 0x02, 0x12, 0x03, 0x06, 0x00, 0x26, 0x0a, 0x08, 0x0a, 0x01, 0x08, 0x12, 0x03, 0x08, 0x00,
    0x39, 0x0a, 0x09, 0x0a, 0x02, 0x08, 0x0b, 0x12, 0x03, 0x08, 0x00, 0x39, 0x0a, 0x08, 0x0a, 0x01,
    0x08, 0x12, 0x03, 0x09, 0x00, 0x34, 0x0a, 0x09, 0x0a, 0x02, 0x08, 0x01, 0x12, 0x03, 0x09, 0x00,
    0x34, 0x0a, 0x49, 0x0a, 0x02, 0x06, 0x00, 0x12, 0x04, 0x0c, 0x00, 0x23, 0x01, 0x1a, 0x3d, 0x20,
    0x41, 0x20, 0x53, 0x65, 0x72, 0x76, 0x6f, 0x53, 0x65, 0x72, 0x76, 0x69, 0x63, 0x65, 0x20, 0x6d,
    0x61, 0x69, 0x6e, 0x74, 0x61, 0x69, 0x6e, 0x73, 0x20, 0x61, 0x6c, 0x6c, 0x20, 0x73, 0x65, 0x72,
    0x76, 0x6f, 0x73, 0x20, 0x61, 0x73, 0x73, 0x6f, 0x63, 0x69, 0x61, 0x74, 0x65, 0x64, 0x20, 0x77,
    0x69, 0x74, 0x68, 0x20, 0x61, 0x20, 0x72, 0x6f, 0x62, 0x6f, 0x74, 0x0a, 0x0a, 0x0a, 0x0a, 0x03,
    0x06, 0x00, 0x01, 0x12, 0x03, 0x0c, 0x08, 0x14, 0x0a, 0x88, 0x01, 0x0a, 0x04, 0x06, 0x00, 0x02,
    0x00, 0x12, 0x04, 0x0f, 0x02, 0x14, 0x03, 0x1a, 0x7a, 0x20, 0x4d, 0x6f, 0x76, 0x65, 0x20, 0x72,
    0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x73, 0x20, 0x74, 0x68, 0x65, 0x20, 0x73, 0x65, 0x72, 0x76,
    0x6f, 0x20, 0x6f, 0x66, 0x20, 0x74, 0x68, 0x65, 0x20, 0x75, 0x6e, 0x64, 0x65, 0x72, 0x6c, 0x79,
    0x69, 0x6e, 0x67, 0x20, 0x72, 0x6f, 0x62, 0x6f, 0x74, 0x20, 0x74, 0x6f, 0x20, 0x6d, 0x6f, 0x76,
    0x65, 0x2e, 0x0a, 0x20, 0x54, 0x68, 0x69, 0x73, 0x20, 0x77, 0x69, 0x6c, 0x6c, 0x20, 0x62, 0x6c,
    0x6f, 0x63, 0x6b, 0x20, 0x75, 0x6e, 0x74, 0x69, 0x6c, 0x20, 0x64, 0x6f, 0x6e, 0x65, 0x20, 0x6f,
    0x72, 0x20, 0x61, 0x20, 0x6e, 0x65, 0x77, 0x20, 0x6f, 0x70, 0x65, 0x72, 0x61, 0x74, 0x69, 0x6f,
    0x6e, 0x20, 0x63, 0x61, 0x6e, 0x63, 0x65, 0x6c, 0x73, 0x20, 0x74, 0x68, 0x69, 0x73, 0x20, 0x6f,
    0x6e, 0x65, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x06, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x0f, 0x06,
    0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x06, 0x00, 0x02, 0x00, 0x02, 0x12, 0x03, 0x0f, 0x0b, 0x16, 0x0a,
    0x0c, 0x0a, 0x05, 0x06, 0x00, 0x02, 0x00, 0x03, 0x12, 0x03, 0x0f, 0x21, 0x2d, 0x0a, 0x0c, 0x0a,
    0x05, 0x06, 0x00, 0x02, 0x00, 0x04, 0x12, 0x03, 0x10, 0x04, 0x39, 0x0a, 0x0f, 0x0a, 0x08, 0x06,
    0x00, 0x02, 0x00, 0x04, 0xa4, 0x92, 0x05, 0x12, 0x03, 0x10, 0x04, 0x39, 0x0a, 0x0d, 0x0a, 0x05,
    0x06, 0x00, 0x02, 0x00, 0x04, 0x12, 0x04, 0x11, 0x04, 0x13, 0x06, 0x0a, 0x11, 0x0a, 0x09, 0x06,
    0x00, 0x02, 0x00, 0x04, 0xb0, 0xca, 0xbc, 0x22, 0x12, 0x04, 0x11, 0x04, 0x13, 0x06, 0x0a, 0x69,
    0x0a, 0x04, 0x06, 0x00, 0x02, 0x01, 0x12, 0x04, 0x17, 0x02, 0x1b, 0x03, 0x1a, 0x5b, 0x20, 0x47,
    0x65, 0x74, 0x50, 0x6f, 0x73, 0x69, 0x74, 0x69, 0x6f, 0x6e, 0x20, 0x72, 0x65, 0x74, 0x75, 0x72,
    0x6e, 0x73, 0x20, 0x74, 0x68, 0x65, 0x20, 0x63, 0x75, 0x72, 0x72, 0x65, 0x6e, 0x74, 0x20, 0x73,
    0x65, 0x74, 0x20, 0x61, 0x6e, 0x67, 0x6c, 0x65, 0x20, 0x28, 0x64, 0x65, 0x67, 0x72, 0x65, 0x65,
    0x73, 0x29, 0x20, 0x6f, 0x66, 0x20, 0x74, 0x68, 0x65, 0x20, 0x73, 0x65, 0x72, 0x76, 0x6f, 0x20,
    0x6f, 0x66, 0x20, 0x74, 0x68, 0x65, 0x20, 0x75, 0x6e, 0x64, 0x65, 0x72, 0x6c, 0x79, 0x69, 0x6e,
    0x67, 0x20, 0x72, 0x6f, 0x62, 0x6f, 0x74, 0x2e, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x06, 0x00, 0x02,
    0x01, 0x01, 0x12, 0x03, 0x17, 0x06, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x06, 0x00, 0x02, 0x01, 0x02,
    0x12, 0x03, 0x17, 0x12, 0x24, 0x0a, 0x0c, 0x0a, 0x05, 0x06, 0x00, 0x02, 0x01, 0x03, 0x12, 0x03,
    0x17, 0x2f, 0x42, 0x0a, 0x0d, 0x0a, 0x05, 0x06, 0x00, 0x02, 0x01, 0x04, 0x12, 0x04, 0x18, 0x04,
    0x1a, 0x06, 0x0a, 0x11, 0x0a, 0x09, 0x06, 0x00, 0x02, 0x01, 0x04, 0xb0, 0xca, 0xbc, 0x22, 0x12,
    0x04, 0x18, 0x04, 0x1a, 0x06, 0x0a, 0x2a, 0x0a, 0x04, 0x06, 0x00, 0x02, 0x02, 0x12, 0x04, 0x1e,
    0x02, 0x22, 0x03, 0x1a, 0x1c, 0x20, 0x53, 0x74, 0x6f, 0x70, 0x20, 0x73, 0x74, 0x6f, 0x70, 0x73,
    0x20, 0x61, 0x20, 0x72, 0x6f, 0x62, 0x6f, 0x74, 0x27, 0x73, 0x20, 0x73, 0x65, 0x72, 0x76, 0x6f,
    0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x06, 0x00, 0x02, 0x02, 0x01, 0x12, 0x03, 0x1e, 0x06, 0x0a, 0x0a,
    0x0c, 0x0a, 0x05, 0x06, 0x00, 0x02, 0x02, 0x02, 0x12, 0x03, 0x1e, 0x0b, 0x16, 0x0a, 0x0c, 0x0a,
    0x05, 0x06, 0x00, 0x02, 0x02, 0x03, 0x12, 0x03, 0x1e, 0x21, 0x2d, 0x0a, 0x0d, 0x0a, 0x05, 0x06,
    0x00, 0x02, 0x02, 0x04, 0x12, 0x04, 0x1f, 0x04, 0x21, 0x06, 0x0a, 0x11, 0x0a, 0x09, 0x06, 0x00,
    0x02, 0x02, 0x04, 0xb0, 0xca, 0xbc, 0x22, 0x12, 0x04, 0x1f, 0x04, 0x21, 0x06, 0x0a, 0x0a, 0x0a,
    0x02, 0x04, 0x00, 0x12, 0x04, 0x25, 0x00, 0x2c, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x00, 0x01,
    0x12, 0x03, 0x25, 0x08, 0x13, 0x0a, 0x33, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x00, 0x12, 0x03, 0x27,
    0x02, 0x12, 0x1a, 0x26, 0x20, 0x74, 0x68, 0x65, 0x20, 0x6e, 0x61, 0x6d, 0x65, 0x20, 0x6f, 0x66,
    0x20, 0x74, 0x68, 0x65, 0x20, 0x73, 0x65, 0x72, 0x76, 0x6f, 0x2c, 0x20, 0x61, 0x73, 0x20, 0x72,
    0x65, 0x67, 0x69, 0x73, 0x74, 0x65, 0x72, 0x65, 0x64, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x00, 0x05, 0x12, 0x03, 0x27, 0x02, 0x08, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00,
    0x01, 0x12, 0x03, 0x27, 0x09, 0x0d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x03, 0x12,
    0x03, 0x27, 0x10, 0x11, 0x0a, 0x5e, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x01, 0x12, 0x03, 0x29, 0x02,
    0x17, 0x1a, 0x51, 0x20, 0x74, 0x68, 0x65, 0x20, 0x64, 0x65, 0x67, 0x72, 0x65, 0x65, 0x73, 0x20,
    0x62, 0x79, 0x20, 0x77, 0x68, 0x69, 0x63, 0x68, 0x20, 0x74, 0x6f, 0x20, 0x72, 0x6f, 0x74, 0x61,
    0x74, 0x65, 0x20, 0x74, 0x68, 0x65, 0x20, 0x73, 0x65, 0x72, 0x76, 0x6f, 0x2e, 0x20, 0x41, 0x63,
    0x63, 0x65, 0x70, 0x74, 0x65, 0x64, 0x20, 0x76, 0x61, 0x6c, 0x75, 0x65, 0x73, 0x20, 0x61, 0x72,
    0x65, 0x20, 0x62, 0x65, 0x74, 0x77, 0x65, 0x65, 0x6e, 0x20, 0x30, 0x20, 0x61, 0x6e, 0x64, 0x20,
    0x31, 0x38, 0x30, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x05, 0x12, 0x03, 0x29,
    0x02, 0x08, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x01, 0x12, 0x03, 0x29, 0x09, 0x12,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x03, 0x12, 0x03, 0x29, 0x15, 0x16, 0x0a, 0x31,
    0x0a, 0x04, 0x04, 0x00, 0x02, 0x02, 0x12, 0x03, 0x2b, 0x02, 0x24, 0x1a, 0x24, 0x20, 0x41, 0x64,
    0x64, 0x69, 0x74, 0x69, 0x6f, 0x6e, 0x61, 0x6c, 0x20, 0x61, 0x72, 0x67, 0x75, 0x6d, 0x65, 0x6e,
    0x74, 0x73, 0x20, 0x74, 0x6f, 0x20, 0x74, 0x68, 0x65, 0x20, 0x6d, 0x65, 0x74, 0x68, 0x6f, 0x64,
    0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x06, 0x12, 0x03, 0x2b, 0x02, 0x18, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x01, 0x12, 0x03, 0x2b, 0x19, 0x1e, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x02, 0x03, 0x12, 0x03, 0x2b, 0x21, 0x23, 0x0a, 0x09, 0x0a, 0x02, 0x04,
    0x01, 0x12, 0x03, 0x2e, 0x00, 0x17, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x01, 0x01, 0x12, 0x03, 0x2e,
    0x08, 0x14, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x02, 0x12, 0x04, 0x30, 0x00, 0x35, 0x01, 0x0a, 0x0a,
    0x0a, 0x03, 0x04, 0x02, 0x01, 0x12, 0x03, 0x30, 0x08, 0x1a, 0x0a, 0x33, 0x0a, 0x04, 0x04, 0x02,
    0x02, 0x00, 0x12, 0x03, 0x32, 0x02, 0x12, 0x1a, 0x26, 0x20, 0x74, 0x68, 0x65, 0x20, 0x6e, 0x61,
    0x6d, 0x65, 0x20, 0x6f, 0x66, 0x20, 0x74, 0x68, 0x65, 0x20, 0x73, 0x65, 0x72, 0x76, 0x6f, 0x2c,
    0x20, 0x61, 0x73, 0x20, 0x72, 0x65, 0x67, 0x69, 0x73, 0x74, 0x65, 0x72, 0x65, 0x64, 0x0a, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x05, 0x12, 0x03, 0x32, 0x02, 0x08, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x02, 0x02, 0x00, 0x01, 0x12, 0x03, 0x32, 0x09, 0x0d, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x02, 0x02, 0x00, 0x03, 0x12, 0x03, 0x32, 0x10, 0x11, 0x0a, 0x31, 0x0a, 0x04, 0x04, 0x02, 0x02,
    0x01, 0x12, 0x03, 0x34, 0x02, 0x24, 0x1a, 0x24, 0x20, 0x41, 0x64, 0x64, 0x69, 0x74, 0x69, 0x6f,
    0x6e, 0x61, 0x6c, 0x20, 0x61, 0x72, 0x67, 0x75, 0x6d, 0x65, 0x6e, 0x74, 0x73, 0x20, 0x74, 0x6f,
    0x20, 0x74, 0x68, 0x65, 0x20, 0x6d, 0x65, 0x74, 0x68, 0x6f, 0x64, 0x0a, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x02, 0x02, 0x01, 0x06, 0x12, 0x03, 0x34, 0x02, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02,
    0x02, 0x01, 0x01, 0x12, 0x03, 0x34, 0x19, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x01,
    0x03, 0x12, 0x03, 0x34, 0x21, 0x23, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x03, 0x12, 0x04, 0x37, 0x00,
    0x3a, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x03, 0x01, 0x12, 0x03, 0x37, 0x08, 0x1b, 0x0a, 0x6d,
    0x0a, 0x04, 0x04, 0x03, 0x02, 0x00, 0x12, 0x03, 0x39, 0x02, 0x1a, 0x1a, 0x60, 0x20, 0x74, 0x68,
    0x65, 0x20, 0x64, 0x65, 0x67, 0x72, 0x65, 0x65, 0x73, 0x20, 0x66, 0x72, 0x6f, 0x6d, 0x20, 0x6e,
    0x65, 0x75, 0x74, 0x72, 0x61, 0x6c, 0x20, 0x62, 0x79, 0x20, 0x77, 0x68, 0x69, 0x63, 0x68, 0x20,
    0x74, 0x68, 0x65, 0x20, 0x73, 0x65, 0x72, 0x76, 0x6f, 0x20, 0x69, 0x73, 0x20, 0x63, 0x75, 0x72,
    0x72, 0x65, 0x6e, 0x74, 0x6c, 0x79, 0x20, 0x72, 0x6f, 0x74, 0x61, 0x74, 0x65, 0x64, 0x2e, 0x20,
    0x56, 0x61, 0x6c, 0x75, 0x65, 0x73, 0x20, 0x61, 0x72, 0x65, 0x20, 0x62, 0x65, 0x74, 0x77, 0x65,
    0x65, 0x6e, 0x20, 0x30, 0x20, 0x61, 0x6e, 0x64, 0x20, 0x31, 0x38, 0x30, 0x0a, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x03, 0x02, 0x00, 0x05, 0x12, 0x03, 0x39, 0x02, 0x08, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x03, 0x02, 0x00, 0x01, 0x12, 0x03, 0x39, 0x09, 0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02,
    0x00, 0x03, 0x12, 0x03, 0x39, 0x18, 0x19, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x04, 0x12, 0x04, 0x3c,
    0x00, 0x41, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x04, 0x01, 0x12, 0x03, 0x3c, 0x08, 0x13, 0x0a,
    0x1e, 0x0a, 0x04, 0x04, 0x04, 0x02, 0x00, 0x12, 0x03, 0x3e, 0x02, 0x12, 0x1a, 0x11, 0x20, 0x4e,
    0x61, 0x6d, 0x65, 0x20, 0x6f, 0x66, 0x20, 0x61, 0x20, 0x73, 0x65, 0x72, 0x76, 0x6f, 0x0a, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x00, 0x05, 0x12, 0x03, 0x3e, 0x02, 0x08, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x04, 0x02, 0x00, 0x01, 0x12, 0x03, 0x3e, 0x09, 0x0d, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x04, 0x02, 0x00, 0x03, 0x12, 0x03, 0x3e, 0x10, 0x11, 0x0a, 0x31, 0x0a, 0x04, 0x04, 0x04, 0x02,
    0x01, 0x12, 0x03, 0x40, 0x02, 0x24, 0x1a, 0x24, 0x20, 0x41, 0x64, 0x64, 0x69, 0x74, 0x69, 0x6f,
    0x6e, 0x61, 0x6c, 0x20, 0x61, 0x72, 0x67, 0x75, 0x6d, 0x65, 0x6e, 0x74, 0x73, 0x20, 0x74, 0x6f,
    0x20, 0x74, 0x68, 0x65, 0x20, 0x6d, 0x65, 0x74, 0x68, 0x6f, 0x64, 0x0a, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x04, 0x02, 0x01, 0x06, 0x12, 0x03, 0x40, 0x02, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04,
    0x02, 0x01, 0x01, 0x12, 0x03, 0x40, 0x19, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x01,
    0x03, 0x12, 0x03, 0x40, 0x21, 0x23, 0x0a, 0x09, 0x0a, 0x02, 0x04, 0x05, 0x12, 0x03, 0x43, 0x00,
    0x17, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x05, 0x01, 0x12, 0x03, 0x43, 0x08, 0x14, 0x0a, 0x0a, 0x0a,
    0x02, 0x04, 0x06, 0x12, 0x04, 0x45, 0x00, 0x48, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x06, 0x01,
    0x12, 0x03, 0x45, 0x08, 0x0e, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x06, 0x02, 0x00, 0x12, 0x03, 0x46,
    0x02, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x00, 0x05, 0x12, 0x03, 0x46, 0x02, 0x08,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x00, 0x01, 0x12, 0x03, 0x46, 0x09, 0x15, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x06, 0x02, 0x00, 0x03, 0x12, 0x03, 0x46, 0x18, 0x19, 0x0a, 0x0b, 0x0a, 0x04,
    0x04, 0x06, 0x02, 0x01, 0x12, 0x03, 0x47, 0x02, 0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02,
    0x01, 0x05, 0x12, 0x03, 0x47, 0x02, 0x06, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x01, 0x01,
    0x12, 0x03, 0x47, 0x07, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x01, 0x03, 0x12, 0x03,
    0x47, 0x13, 0x14, 0x62, 0x06, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x33,
];
include!("viam.component.servo.v1.tonic.rs");
// @@protoc_insertion_point(module)