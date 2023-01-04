// @generated
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SubmitTrainingJobRequest {
    #[prost(message, optional, tag="1")]
    pub filter: ::core::option::Option<super::super::data::v1::Filter>,
    #[prost(string, tag="2")]
    pub organization_id: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub model_name: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub model_version: ::prost::alloc::string::String,
    #[prost(enumeration="ModelType", tag="5")]
    pub model_type: i32,
    #[prost(string, repeated, tag="6")]
    pub tags: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SubmitTrainingJobResponse {
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetTrainingJobRequest {
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetTrainingJobResponse {
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub metadata: ::core::option::Option<TrainingJobMetadata>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TrainingJobMetadata {
    #[prost(message, optional, tag="1")]
    pub request: ::core::option::Option<SubmitTrainingJobRequest>,
    #[prost(enumeration="TrainingStatus", tag="2")]
    pub status: i32,
    #[prost(message, optional, tag="3")]
    pub created_on: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(message, optional, tag="4")]
    pub last_modified: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(string, tag="5")]
    pub synced_model_id: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TrainingJob {
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub metadata: ::core::option::Option<TrainingJobMetadata>,
    #[prost(string, tag="3")]
    pub output_path: ::prost::alloc::string::String,
    /// The vertex_job_id is the id of the Vertex AI custom training job
    /// backing our concept of a TrainingJob.
    #[prost(string, tag="4")]
    pub vertex_job_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag="5")]
    pub model_metadata: ::core::option::Option<::prost_types::Struct>,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ModelType {
    Unspecified = 0,
    SingleLabelClassification = 1,
    MultiLabelClassification = 2,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum TrainingStatus {
    Unspecified = 0,
    Pending = 1,
    InProgress = 2,
    Completed = 3,
    Failed = 4,
    Submitting = 5,
}
/// Encoded file descriptor set for the `viam.app.mltraining.v1` package
pub const FILE_DESCRIPTOR_SET: &[u8] = &[
    0x0a, 0x88, 0x21, 0x0a, 0x23, 0x61, 0x70, 0x70, 0x2f, 0x6d, 0x6c, 0x74, 0x72, 0x61, 0x69, 0x6e,
    0x69, 0x6e, 0x67, 0x2f, 0x76, 0x31, 0x2f, 0x6d, 0x6c, 0x5f, 0x74, 0x72, 0x61, 0x69, 0x6e, 0x69,
    0x6e, 0x67, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x12, 0x16, 0x76, 0x69, 0x61, 0x6d, 0x2e, 0x61,
    0x70, 0x70, 0x2e, 0x6d, 0x6c, 0x74, 0x72, 0x61, 0x69, 0x6e, 0x69, 0x6e, 0x67, 0x2e, 0x76, 0x31,
    0x1a, 0x16, 0x61, 0x70, 0x70, 0x2f, 0x64, 0x61, 0x74, 0x61, 0x2f, 0x76, 0x31, 0x2f, 0x64, 0x61,
    0x74, 0x61, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x1a, 0x1c, 0x67, 0x6f, 0x6f, 0x67, 0x6c, 0x65,
    0x2f, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x62, 0x75, 0x66, 0x2f, 0x73, 0x74, 0x72, 0x75, 0x63, 0x74,
    0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x1a, 0x1f, 0x67, 0x6f, 0x6f, 0x67, 0x6c, 0x65, 0x2f, 0x70,
    0x72, 0x6f, 0x74, 0x6f, 0x62, 0x75, 0x66, 0x2f, 0x74, 0x69, 0x6d, 0x65, 0x73, 0x74, 0x61, 0x6d,
    0x70, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x1a, 0x16, 0x74, 0x61, 0x67, 0x67, 0x65, 0x72, 0x2f,
    0x76, 0x31, 0x2f, 0x74, 0x61, 0x67, 0x67, 0x65, 0x72, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x22,
    0x8f, 0x02, 0x0a, 0x18, 0x53, 0x75, 0x62, 0x6d, 0x69, 0x74, 0x54, 0x72, 0x61, 0x69, 0x6e, 0x69,
    0x6e, 0x67, 0x4a, 0x6f, 0x62, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x12, 0x30, 0x0a, 0x06,
    0x66, 0x69, 0x6c, 0x74, 0x65, 0x72, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x18, 0x2e, 0x76,
    0x69, 0x61, 0x6d, 0x2e, 0x61, 0x70, 0x70, 0x2e, 0x64, 0x61, 0x74, 0x61, 0x2e, 0x76, 0x31, 0x2e,
    0x46, 0x69, 0x6c, 0x74, 0x65, 0x72, 0x52, 0x06, 0x66, 0x69, 0x6c, 0x74, 0x65, 0x72, 0x12, 0x27,
    0x0a, 0x0f, 0x6f, 0x72, 0x67, 0x61, 0x6e, 0x69, 0x7a, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x5f, 0x69,
    0x64, 0x18, 0x02, 0x20, 0x01, 0x28, 0x09, 0x52, 0x0e, 0x6f, 0x72, 0x67, 0x61, 0x6e, 0x69, 0x7a,
    0x61, 0x74, 0x69, 0x6f, 0x6e, 0x49, 0x64, 0x12, 0x1d, 0x0a, 0x0a, 0x6d, 0x6f, 0x64, 0x65, 0x6c,
    0x5f, 0x6e, 0x61, 0x6d, 0x65, 0x18, 0x03, 0x20, 0x01, 0x28, 0x09, 0x52, 0x09, 0x6d, 0x6f, 0x64,
    0x65, 0x6c, 0x4e, 0x61, 0x6d, 0x65, 0x12, 0x23, 0x0a, 0x0d, 0x6d, 0x6f, 0x64, 0x65, 0x6c, 0x5f,
    0x76, 0x65, 0x72, 0x73, 0x69, 0x6f, 0x6e, 0x18, 0x04, 0x20, 0x01, 0x28, 0x09, 0x52, 0x0c, 0x6d,
    0x6f, 0x64, 0x65, 0x6c, 0x56, 0x65, 0x72, 0x73, 0x69, 0x6f, 0x6e, 0x12, 0x40, 0x0a, 0x0a, 0x6d,
    0x6f, 0x64, 0x65, 0x6c, 0x5f, 0x74, 0x79, 0x70, 0x65, 0x18, 0x05, 0x20, 0x01, 0x28, 0x0e, 0x32,
    0x21, 0x2e, 0x76, 0x69, 0x61, 0x6d, 0x2e, 0x61, 0x70, 0x70, 0x2e, 0x6d, 0x6c, 0x74, 0x72, 0x61,
    0x69, 0x6e, 0x69, 0x6e, 0x67, 0x2e, 0x76, 0x31, 0x2e, 0x4d, 0x6f, 0x64, 0x65, 0x6c, 0x54, 0x79,
    0x70, 0x65, 0x52, 0x09, 0x6d, 0x6f, 0x64, 0x65, 0x6c, 0x54, 0x79, 0x70, 0x65, 0x12, 0x12, 0x0a,
    0x04, 0x74, 0x61, 0x67, 0x73, 0x18, 0x06, 0x20, 0x03, 0x28, 0x09, 0x52, 0x04, 0x74, 0x61, 0x67,
    0x73, 0x22, 0x2b, 0x0a, 0x19, 0x53, 0x75, 0x62, 0x6d, 0x69, 0x74, 0x54, 0x72, 0x61, 0x69, 0x6e,
    0x69, 0x6e, 0x67, 0x4a, 0x6f, 0x62, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x12, 0x0e,
    0x0a, 0x02, 0x69, 0x64, 0x18, 0x01, 0x20, 0x01, 0x28, 0x09, 0x52, 0x02, 0x69, 0x64, 0x22, 0x27,
    0x0a, 0x15, 0x47, 0x65, 0x74, 0x54, 0x72, 0x61, 0x69, 0x6e, 0x69, 0x6e, 0x67, 0x4a, 0x6f, 0x62,
    0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x12, 0x0e, 0x0a, 0x02, 0x69, 0x64, 0x18, 0x01, 0x20,
    0x01, 0x28, 0x09, 0x52, 0x02, 0x69, 0x64, 0x22, 0x71, 0x0a, 0x16, 0x47, 0x65, 0x74, 0x54, 0x72,
    0x61, 0x69, 0x6e, 0x69, 0x6e, 0x67, 0x4a, 0x6f, 0x62, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73,
    0x65, 0x12, 0x0e, 0x0a, 0x02, 0x69, 0x64, 0x18, 0x01, 0x20, 0x01, 0x28, 0x09, 0x52, 0x02, 0x69,
    0x64, 0x12, 0x47, 0x0a, 0x08, 0x6d, 0x65, 0x74, 0x61, 0x64, 0x61, 0x74, 0x61, 0x18, 0x02, 0x20,
    0x01, 0x28, 0x0b, 0x32, 0x2b, 0x2e, 0x76, 0x69, 0x61, 0x6d, 0x2e, 0x61, 0x70, 0x70, 0x2e, 0x6d,
    0x6c, 0x74, 0x72, 0x61, 0x69, 0x6e, 0x69, 0x6e, 0x67, 0x2e, 0x76, 0x31, 0x2e, 0x54, 0x72, 0x61,
    0x69, 0x6e, 0x69, 0x6e, 0x67, 0x4a, 0x6f, 0x62, 0x4d, 0x65, 0x74, 0x61, 0x64, 0x61, 0x74, 0x61,
    0x52, 0x08, 0x6d, 0x65, 0x74, 0x61, 0x64, 0x61, 0x74, 0x61, 0x22, 0xc5, 0x02, 0x0a, 0x13, 0x54,
    0x72, 0x61, 0x69, 0x6e, 0x69, 0x6e, 0x67, 0x4a, 0x6f, 0x62, 0x4d, 0x65, 0x74, 0x61, 0x64, 0x61,
    0x74, 0x61, 0x12, 0x4a, 0x0a, 0x07, 0x72, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x18, 0x01, 0x20,
    0x01, 0x28, 0x0b, 0x32, 0x30, 0x2e, 0x76, 0x69, 0x61, 0x6d, 0x2e, 0x61, 0x70, 0x70, 0x2e, 0x6d,
    0x6c, 0x74, 0x72, 0x61, 0x69, 0x6e, 0x69, 0x6e, 0x67, 0x2e, 0x76, 0x31, 0x2e, 0x53, 0x75, 0x62,
    0x6d, 0x69, 0x74, 0x54, 0x72, 0x61, 0x69, 0x6e, 0x69, 0x6e, 0x67, 0x4a, 0x6f, 0x62, 0x52, 0x65,
    0x71, 0x75, 0x65, 0x73, 0x74, 0x52, 0x07, 0x72, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x12, 0x3e,
    0x0a, 0x06, 0x73, 0x74, 0x61, 0x74, 0x75, 0x73, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0e, 0x32, 0x26,
    0x2e, 0x76, 0x69, 0x61, 0x6d, 0x2e, 0x61, 0x70, 0x70, 0x2e, 0x6d, 0x6c, 0x74, 0x72, 0x61, 0x69,
    0x6e, 0x69, 0x6e, 0x67, 0x2e, 0x76, 0x31, 0x2e, 0x54, 0x72, 0x61, 0x69, 0x6e, 0x69, 0x6e, 0x67,
    0x53, 0x74, 0x61, 0x74, 0x75, 0x73, 0x52, 0x06, 0x73, 0x74, 0x61, 0x74, 0x75, 0x73, 0x12, 0x39,
    0x0a, 0x0a, 0x63, 0x72, 0x65, 0x61, 0x74, 0x65, 0x64, 0x5f, 0x6f, 0x6e, 0x18, 0x03, 0x20, 0x01,
    0x28, 0x0b, 0x32, 0x1a, 0x2e, 0x67, 0x6f, 0x6f, 0x67, 0x6c, 0x65, 0x2e, 0x70, 0x72, 0x6f, 0x74,
    0x6f, 0x62, 0x75, 0x66, 0x2e, 0x54, 0x69, 0x6d, 0x65, 0x73, 0x74, 0x61, 0x6d, 0x70, 0x52, 0x09,
    0x63, 0x72, 0x65, 0x61, 0x74, 0x65, 0x64, 0x4f, 0x6e, 0x12, 0x3f, 0x0a, 0x0d, 0x6c, 0x61, 0x73,
    0x74, 0x5f, 0x6d, 0x6f, 0x64, 0x69, 0x66, 0x69, 0x65, 0x64, 0x18, 0x04, 0x20, 0x01, 0x28, 0x0b,
    0x32, 0x1a, 0x2e, 0x67, 0x6f, 0x6f, 0x67, 0x6c, 0x65, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x62,
    0x75, 0x66, 0x2e, 0x54, 0x69, 0x6d, 0x65, 0x73, 0x74, 0x61, 0x6d, 0x70, 0x52, 0x0c, 0x6c, 0x61,
    0x73, 0x74, 0x4d, 0x6f, 0x64, 0x69, 0x66, 0x69, 0x65, 0x64, 0x12, 0x26, 0x0a, 0x0f, 0x73, 0x79,
    0x6e, 0x63, 0x65, 0x64, 0x5f, 0x6d, 0x6f, 0x64, 0x65, 0x6c, 0x5f, 0x69, 0x64, 0x18, 0x05, 0x20,
    0x01, 0x28, 0x09, 0x52, 0x0d, 0x73, 0x79, 0x6e, 0x63, 0x65, 0x64, 0x4d, 0x6f, 0x64, 0x65, 0x6c,
    0x49, 0x64, 0x22, 0xc4, 0x03, 0x0a, 0x0b, 0x54, 0x72, 0x61, 0x69, 0x6e, 0x69, 0x6e, 0x67, 0x4a,
    0x6f, 0x62, 0x12, 0x33, 0x0a, 0x02, 0x69, 0x64, 0x18, 0x01, 0x20, 0x01, 0x28, 0x09, 0x42, 0x23,
    0x9a, 0x84, 0x9e, 0x03, 0x1e, 0x62, 0x73, 0x6f, 0x6e, 0x3a, 0x22, 0x5f, 0x69, 0x64, 0x22, 0x20,
    0x6a, 0x73, 0x6f, 0x6e, 0x3a, 0x22, 0x69, 0x64, 0x2c, 0x6f, 0x6d, 0x69, 0x74, 0x65, 0x6d, 0x70,
    0x74, 0x79, 0x22, 0x52, 0x02, 0x69, 0x64, 0x12, 0x6d, 0x0a, 0x08, 0x6d, 0x65, 0x74, 0x61, 0x64,
    0x61, 0x74, 0x61, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x2b, 0x2e, 0x76, 0x69, 0x61, 0x6d,
    0x2e, 0x61, 0x70, 0x70, 0x2e, 0x6d, 0x6c, 0x74, 0x72, 0x61, 0x69, 0x6e, 0x69, 0x6e, 0x67, 0x2e,
    0x76, 0x31, 0x2e, 0x54, 0x72, 0x61, 0x69, 0x6e, 0x69, 0x6e, 0x67, 0x4a, 0x6f, 0x62, 0x4d, 0x65,
    0x74, 0x61, 0x64, 0x61, 0x74, 0x61, 0x42, 0x24, 0x9a, 0x84, 0x9e, 0x03, 0x1f, 0x62, 0x73, 0x6f,
    0x6e, 0x3a, 0x22, 0x6d, 0x65, 0x74, 0x61, 0x64, 0x61, 0x74, 0x61, 0x22, 0x20, 0x6a, 0x73, 0x6f,
    0x6e, 0x3a, 0x22, 0x6d, 0x65, 0x74, 0x61, 0x64, 0x61, 0x74, 0x61, 0x22, 0x52, 0x08, 0x6d, 0x65,
    0x74, 0x61, 0x64, 0x61, 0x74, 0x61, 0x12, 0x4b, 0x0a, 0x0b, 0x6f, 0x75, 0x74, 0x70, 0x75, 0x74,
    0x5f, 0x70, 0x61, 0x74, 0x68, 0x18, 0x03, 0x20, 0x01, 0x28, 0x09, 0x42, 0x2a, 0x9a, 0x84, 0x9e,
    0x03, 0x25, 0x62, 0x73, 0x6f, 0x6e, 0x3a, 0x22, 0x6f, 0x75, 0x74, 0x70, 0x75, 0x74, 0x5f, 0x70,
    0x61, 0x74, 0x68, 0x22, 0x20, 0x6a, 0x73, 0x6f, 0x6e, 0x3a, 0x22, 0x6f, 0x75, 0x74, 0x70, 0x75,
    0x74, 0x5f, 0x70, 0x61, 0x74, 0x68, 0x22, 0x52, 0x0a, 0x6f, 0x75, 0x74, 0x70, 0x75, 0x74, 0x50,
    0x61, 0x74, 0x68, 0x12, 0x52, 0x0a, 0x0d, 0x76, 0x65, 0x72, 0x74, 0x65, 0x78, 0x5f, 0x6a, 0x6f,
    0x62, 0x5f, 0x69, 0x64, 0x18, 0x04, 0x20, 0x01, 0x28, 0x09, 0x42, 0x2e, 0x9a, 0x84, 0x9e, 0x03,
    0x29, 0x62, 0x73, 0x6f, 0x6e, 0x3a, 0x22, 0x76, 0x65, 0x72, 0x74, 0x65, 0x78, 0x5f, 0x6a, 0x6f,
    0x62, 0x5f, 0x69, 0x64, 0x22, 0x20, 0x6a, 0x73, 0x6f, 0x6e, 0x3a, 0x22, 0x76, 0x65, 0x72, 0x74,
    0x65, 0x78, 0x5f, 0x6a, 0x6f, 0x62, 0x5f, 0x69, 0x64, 0x22, 0x52, 0x0b, 0x76, 0x65, 0x72, 0x74,
    0x65, 0x78, 0x4a, 0x6f, 0x62, 0x49, 0x64, 0x12, 0x70, 0x0a, 0x0e, 0x6d, 0x6f, 0x64, 0x65, 0x6c,
    0x5f, 0x6d, 0x65, 0x74, 0x61, 0x64, 0x61, 0x74, 0x61, 0x18, 0x05, 0x20, 0x01, 0x28, 0x0b, 0x32,
    0x17, 0x2e, 0x67, 0x6f, 0x6f, 0x67, 0x6c, 0x65, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x62, 0x75,
    0x66, 0x2e, 0x53, 0x74, 0x72, 0x75, 0x63, 0x74, 0x42, 0x30, 0x9a, 0x84, 0x9e, 0x03, 0x2b, 0x62,
    0x73, 0x6f, 0x6e, 0x3a, 0x22, 0x6d, 0x6f, 0x64, 0x65, 0x6c, 0x5f, 0x6d, 0x65, 0x74, 0x61, 0x64,
    0x61, 0x74, 0x61, 0x22, 0x20, 0x6a, 0x73, 0x6f, 0x6e, 0x3a, 0x22, 0x6d, 0x6f, 0x64, 0x65, 0x6c,
    0x5f, 0x6d, 0x65, 0x74, 0x61, 0x64, 0x61, 0x74, 0x61, 0x22, 0x52, 0x0d, 0x6d, 0x6f, 0x64, 0x65,
    0x6c, 0x4d, 0x65, 0x74, 0x61, 0x64, 0x61, 0x74, 0x61, 0x2a, 0x7e, 0x0a, 0x09, 0x4d, 0x6f, 0x64,
    0x65, 0x6c, 0x54, 0x79, 0x70, 0x65, 0x12, 0x1a, 0x0a, 0x16, 0x4d, 0x4f, 0x44, 0x45, 0x4c, 0x5f,
    0x54, 0x59, 0x50, 0x45, 0x5f, 0x55, 0x4e, 0x53, 0x50, 0x45, 0x43, 0x49, 0x46, 0x49, 0x45, 0x44,
    0x10, 0x00, 0x12, 0x2a, 0x0a, 0x26, 0x4d, 0x4f, 0x44, 0x45, 0x4c, 0x5f, 0x54, 0x59, 0x50, 0x45,
    0x5f, 0x53, 0x49, 0x4e, 0x47, 0x4c, 0x45, 0x5f, 0x4c, 0x41, 0x42, 0x45, 0x4c, 0x5f, 0x43, 0x4c,
    0x41, 0x53, 0x53, 0x49, 0x46, 0x49, 0x43, 0x41, 0x54, 0x49, 0x4f, 0x4e, 0x10, 0x01, 0x12, 0x29,
    0x0a, 0x25, 0x4d, 0x4f, 0x44, 0x45, 0x4c, 0x5f, 0x54, 0x59, 0x50, 0x45, 0x5f, 0x4d, 0x55, 0x4c,
    0x54, 0x49, 0x5f, 0x4c, 0x41, 0x42, 0x45, 0x4c, 0x5f, 0x43, 0x4c, 0x41, 0x53, 0x53, 0x49, 0x46,
    0x49, 0x43, 0x41, 0x54, 0x49, 0x4f, 0x4e, 0x10, 0x02, 0x2a, 0xca, 0x01, 0x0a, 0x0e, 0x54, 0x72,
    0x61, 0x69, 0x6e, 0x69, 0x6e, 0x67, 0x53, 0x74, 0x61, 0x74, 0x75, 0x73, 0x12, 0x1f, 0x0a, 0x1b,
    0x54, 0x52, 0x41, 0x49, 0x4e, 0x49, 0x4e, 0x47, 0x5f, 0x53, 0x54, 0x41, 0x54, 0x55, 0x53, 0x5f,
    0x55, 0x4e, 0x53, 0x50, 0x45, 0x43, 0x49, 0x46, 0x49, 0x45, 0x44, 0x10, 0x00, 0x12, 0x1b, 0x0a,
    0x17, 0x54, 0x52, 0x41, 0x49, 0x4e, 0x49, 0x4e, 0x47, 0x5f, 0x53, 0x54, 0x41, 0x54, 0x55, 0x53,
    0x5f, 0x50, 0x45, 0x4e, 0x44, 0x49, 0x4e, 0x47, 0x10, 0x01, 0x12, 0x1f, 0x0a, 0x1b, 0x54, 0x52,
    0x41, 0x49, 0x4e, 0x49, 0x4e, 0x47, 0x5f, 0x53, 0x54, 0x41, 0x54, 0x55, 0x53, 0x5f, 0x49, 0x4e,
    0x5f, 0x50, 0x52, 0x4f, 0x47, 0x52, 0x45, 0x53, 0x53, 0x10, 0x02, 0x12, 0x1d, 0x0a, 0x19, 0x54,
    0x52, 0x41, 0x49, 0x4e, 0x49, 0x4e, 0x47, 0x5f, 0x53, 0x54, 0x41, 0x54, 0x55, 0x53, 0x5f, 0x43,
    0x4f, 0x4d, 0x50, 0x4c, 0x45, 0x54, 0x45, 0x44, 0x10, 0x03, 0x12, 0x1a, 0x0a, 0x16, 0x54, 0x52,
    0x41, 0x49, 0x4e, 0x49, 0x4e, 0x47, 0x5f, 0x53, 0x54, 0x41, 0x54, 0x55, 0x53, 0x5f, 0x46, 0x41,
    0x49, 0x4c, 0x45, 0x44, 0x10, 0x04, 0x12, 0x1e, 0x0a, 0x1a, 0x54, 0x52, 0x41, 0x49, 0x4e, 0x49,
    0x4e, 0x47, 0x5f, 0x53, 0x54, 0x41, 0x54, 0x55, 0x53, 0x5f, 0x53, 0x55, 0x42, 0x4d, 0x49, 0x54,
    0x54, 0x49, 0x4e, 0x47, 0x10, 0x05, 0x32, 0xfe, 0x01, 0x0a, 0x11, 0x4d, 0x4c, 0x54, 0x72, 0x61,
    0x69, 0x6e, 0x69, 0x6e, 0x67, 0x53, 0x65, 0x72, 0x76, 0x69, 0x63, 0x65, 0x12, 0x78, 0x0a, 0x11,
    0x53, 0x75, 0x62, 0x6d, 0x69, 0x74, 0x54, 0x72, 0x61, 0x69, 0x6e, 0x69, 0x6e, 0x67, 0x4a, 0x6f,
    0x62, 0x12, 0x30, 0x2e, 0x76, 0x69, 0x61, 0x6d, 0x2e, 0x61, 0x70, 0x70, 0x2e, 0x6d, 0x6c, 0x74,
    0x72, 0x61, 0x69, 0x6e, 0x69, 0x6e, 0x67, 0x2e, 0x76, 0x31, 0x2e, 0x53, 0x75, 0x62, 0x6d, 0x69,
    0x74, 0x54, 0x72, 0x61, 0x69, 0x6e, 0x69, 0x6e, 0x67, 0x4a, 0x6f, 0x62, 0x52, 0x65, 0x71, 0x75,
    0x65, 0x73, 0x74, 0x1a, 0x31, 0x2e, 0x76, 0x69, 0x61, 0x6d, 0x2e, 0x61, 0x70, 0x70, 0x2e, 0x6d,
    0x6c, 0x74, 0x72, 0x61, 0x69, 0x6e, 0x69, 0x6e, 0x67, 0x2e, 0x76, 0x31, 0x2e, 0x53, 0x75, 0x62,
    0x6d, 0x69, 0x74, 0x54, 0x72, 0x61, 0x69, 0x6e, 0x69, 0x6e, 0x67, 0x4a, 0x6f, 0x62, 0x52, 0x65,
    0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x12, 0x6f, 0x0a, 0x0e, 0x47, 0x65, 0x74, 0x54, 0x72, 0x61,
    0x69, 0x6e, 0x69, 0x6e, 0x67, 0x4a, 0x6f, 0x62, 0x12, 0x2d, 0x2e, 0x76, 0x69, 0x61, 0x6d, 0x2e,
    0x61, 0x70, 0x70, 0x2e, 0x6d, 0x6c, 0x74, 0x72, 0x61, 0x69, 0x6e, 0x69, 0x6e, 0x67, 0x2e, 0x76,
    0x31, 0x2e, 0x47, 0x65, 0x74, 0x54, 0x72, 0x61, 0x69, 0x6e, 0x69, 0x6e, 0x67, 0x4a, 0x6f, 0x62,
    0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x1a, 0x2e, 0x2e, 0x76, 0x69, 0x61, 0x6d, 0x2e, 0x61,
    0x70, 0x70, 0x2e, 0x6d, 0x6c, 0x74, 0x72, 0x61, 0x69, 0x6e, 0x69, 0x6e, 0x67, 0x2e, 0x76, 0x31,
    0x2e, 0x47, 0x65, 0x74, 0x54, 0x72, 0x61, 0x69, 0x6e, 0x69, 0x6e, 0x67, 0x4a, 0x6f, 0x62, 0x52,
    0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x42, 0x23, 0x5a, 0x21, 0x67, 0x6f, 0x2e, 0x76, 0x69,
    0x61, 0x6d, 0x2e, 0x63, 0x6f, 0x6d, 0x2f, 0x61, 0x70, 0x69, 0x2f, 0x61, 0x70, 0x70, 0x2f, 0x6d,
    0x6c, 0x74, 0x72, 0x61, 0x69, 0x6e, 0x69, 0x6e, 0x67, 0x2f, 0x76, 0x31, 0x4a, 0xf4, 0x10, 0x0a,
    0x06, 0x12, 0x04, 0x00, 0x00, 0x45, 0x01, 0x0a, 0x08, 0x0a, 0x01, 0x0c, 0x12, 0x03, 0x00, 0x00,
    0x12, 0x0a, 0x08, 0x0a, 0x01, 0x02, 0x12, 0x03, 0x02, 0x00, 0x1f, 0x0a, 0x09, 0x0a, 0x02, 0x03,
    0x00, 0x12, 0x03, 0x04, 0x00, 0x20, 0x0a, 0x09, 0x0a, 0x02, 0x03, 0x01, 0x12, 0x03, 0x05, 0x00,
    0x26, 0x0a, 0x09, 0x0a, 0x02, 0x03, 0x02, 0x12, 0x03, 0x06, 0x00, 0x29, 0x0a, 0x09, 0x0a, 0x02,
    0x03, 0x03, 0x12, 0x03, 0x07, 0x00, 0x20, 0x0a, 0x08, 0x0a, 0x01, 0x08, 0x12, 0x03, 0x09, 0x00,
    0x38, 0x0a, 0x09, 0x0a, 0x02, 0x08, 0x0b, 0x12, 0x03, 0x09, 0x00, 0x38, 0x0a, 0x0a, 0x0a, 0x02,
    0x06, 0x00, 0x12, 0x04, 0x0b, 0x00, 0x0e, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x06, 0x00, 0x01, 0x12,
    0x03, 0x0b, 0x08, 0x19, 0x0a, 0x0b, 0x0a, 0x04, 0x06, 0x00, 0x02, 0x00, 0x12, 0x03, 0x0c, 0x02,
    0x56, 0x0a, 0x0c, 0x0a, 0x05, 0x06, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x0c, 0x06, 0x17, 0x0a,
    0x0c, 0x0a, 0x05, 0x06, 0x00, 0x02, 0x00, 0x02, 0x12, 0x03, 0x0c, 0x18, 0x30, 0x0a, 0x0c, 0x0a,
    0x05, 0x06, 0x00, 0x02, 0x00, 0x03, 0x12, 0x03, 0x0c, 0x3b, 0x54, 0x0a, 0x0b, 0x0a, 0x04, 0x06,
    0x00, 0x02, 0x01, 0x12, 0x03, 0x0d, 0x02, 0x4d, 0x0a, 0x0c, 0x0a, 0x05, 0x06, 0x00, 0x02, 0x01,
    0x01, 0x12, 0x03, 0x0d, 0x06, 0x14, 0x0a, 0x0c, 0x0a, 0x05, 0x06, 0x00, 0x02, 0x01, 0x02, 0x12,
    0x03, 0x0d, 0x15, 0x2a, 0x0a, 0x0c, 0x0a, 0x05, 0x06, 0x00, 0x02, 0x01, 0x03, 0x12, 0x03, 0x0d,
    0x35, 0x4b, 0x0a, 0x0a, 0x0a, 0x02, 0x05, 0x00, 0x12, 0x04, 0x10, 0x00, 0x14, 0x01, 0x0a, 0x0a,
    0x0a, 0x03, 0x05, 0x00, 0x01, 0x12, 0x03, 0x10, 0x05, 0x0e, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00,
    0x02, 0x00, 0x12, 0x03, 0x11, 0x02, 0x1d, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x00, 0x01,
    0x12, 0x03, 0x11, 0x02, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x00, 0x02, 0x12, 0x03,
    0x11, 0x1b, 0x1c, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x01, 0x12, 0x03, 0x12, 0x02, 0x2d,
    0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x01, 0x01, 0x12, 0x03, 0x12, 0x02, 0x28, 0x0a, 0x0c,
    0x0a, 0x05, 0x05, 0x00, 0x02, 0x01, 0x02, 0x12, 0x03, 0x12, 0x2b, 0x2c, 0x0a, 0x0b, 0x0a, 0x04,
    0x05, 0x00, 0x02, 0x02, 0x12, 0x03, 0x13, 0x02, 0x2c, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02,
    0x02, 0x01, 0x12, 0x03, 0x13, 0x02, 0x27, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x02, 0x02,
    0x12, 0x03, 0x13, 0x2a, 0x2b, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x00, 0x12, 0x04, 0x16, 0x00, 0x1d,
    0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x00, 0x01, 0x12, 0x03, 0x16, 0x08, 0x20, 0x0a, 0x0b, 0x0a,
    0x04, 0x04, 0x00, 0x02, 0x00, 0x12, 0x03, 0x17, 0x02, 0x25, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x00, 0x06, 0x12, 0x03, 0x17, 0x02, 0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00,
    0x01, 0x12, 0x03, 0x17, 0x1a, 0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x03, 0x12,
    0x03, 0x17, 0x23, 0x24, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x01, 0x12, 0x03, 0x18, 0x02,
    0x1d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x05, 0x12, 0x03, 0x18, 0x02, 0x08, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x01, 0x12, 0x03, 0x18, 0x09, 0x18, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x01, 0x03, 0x12, 0x03, 0x18, 0x1b, 0x1c, 0x0a, 0x0b, 0x0a, 0x04, 0x04,
    0x00, 0x02, 0x02, 0x12, 0x03, 0x19, 0x02, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02,
    0x05, 0x12, 0x03, 0x19, 0x02, 0x08, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x01, 0x12,
    0x03, 0x19, 0x09, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x03, 0x12, 0x03, 0x19,
    0x16, 0x17, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x03, 0x12, 0x03, 0x1a, 0x02, 0x1b, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x05, 0x12, 0x03, 0x1a, 0x02, 0x08, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x03, 0x01, 0x12, 0x03, 0x1a, 0x09, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x03, 0x03, 0x12, 0x03, 0x1a, 0x19, 0x1a, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02,
    0x04, 0x12, 0x03, 0x1b, 0x02, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x04, 0x06, 0x12,
    0x03, 0x1b, 0x02, 0x0b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x04, 0x01, 0x12, 0x03, 0x1b,
    0x0c, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x04, 0x03, 0x12, 0x03, 0x1b, 0x19, 0x1a,
    0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x05, 0x12, 0x03, 0x1c, 0x02, 0x1b, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x05, 0x04, 0x12, 0x03, 0x1c, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x05, 0x05, 0x12, 0x03, 0x1c, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x05, 0x01, 0x12, 0x03, 0x1c, 0x12, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x05, 0x03,
    0x12, 0x03, 0x1c, 0x19, 0x1a, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x01, 0x12, 0x04, 0x1f, 0x00, 0x21,
    0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x01, 0x01, 0x12, 0x03, 0x1f, 0x08, 0x21, 0x0a, 0x0b, 0x0a,
    0x04, 0x04, 0x01, 0x02, 0x00, 0x12, 0x03, 0x20, 0x02, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01,
    0x02, 0x00, 0x05, 0x12, 0x03, 0x20, 0x02, 0x08, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00,
    0x01, 0x12, 0x03, 0x20, 0x09, 0x0b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x03, 0x12,
    0x03, 0x20, 0x0e, 0x0f, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x02, 0x12, 0x04, 0x23, 0x00, 0x25, 0x01,
    0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x02, 0x01, 0x12, 0x03, 0x23, 0x08, 0x1d, 0x0a, 0x0b, 0x0a, 0x04,
    0x04, 0x02, 0x02, 0x00, 0x12, 0x03, 0x24, 0x02, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02,
    0x00, 0x05, 0x12, 0x03, 0x24, 0x02, 0x08, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x01,
    0x12, 0x03, 0x24, 0x09, 0x0b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x03, 0x12, 0x03,
    0x24, 0x0e, 0x0f, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x03, 0x12, 0x04, 0x27, 0x00, 0x2a, 0x01, 0x0a,
    0x0a, 0x0a, 0x03, 0x04, 0x03, 0x01, 0x12, 0x03, 0x27, 0x08, 0x1e, 0x0a, 0x0b, 0x0a, 0x04, 0x04,
    0x03, 0x02, 0x00, 0x12, 0x03, 0x28, 0x02, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00,
    0x05, 0x12, 0x03, 0x28, 0x02, 0x08, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x01, 0x12,
    0x03, 0x28, 0x09, 0x0b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x03, 0x12, 0x03, 0x28,
    0x0e, 0x0f, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x03, 0x02, 0x01, 0x12, 0x03, 0x29, 0x02, 0x23, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x01, 0x06, 0x12, 0x03, 0x29, 0x02, 0x15, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x03, 0x02, 0x01, 0x01, 0x12, 0x03, 0x29, 0x16, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x03, 0x02, 0x01, 0x03, 0x12, 0x03, 0x29, 0x21, 0x22, 0x0a, 0x0a, 0x0a, 0x02, 0x05, 0x01, 0x12,
    0x04, 0x2c, 0x00, 0x33, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x05, 0x01, 0x01, 0x12, 0x03, 0x2c, 0x05,
    0x13, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x01, 0x02, 0x00, 0x12, 0x03, 0x2d, 0x02, 0x22, 0x0a, 0x0c,
    0x0a, 0x05, 0x05, 0x01, 0x02, 0x00, 0x01, 0x12, 0x03, 0x2d, 0x02, 0x1d, 0x0a, 0x0c, 0x0a, 0x05,
    0x05, 0x01, 0x02, 0x00, 0x02, 0x12, 0x03, 0x2d, 0x20, 0x21, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x01,
    0x02, 0x01, 0x12, 0x03, 0x2e, 0x02, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x01, 0x02, 0x01, 0x01,
    0x12, 0x03, 0x2e, 0x02, 0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x01, 0x02, 0x01, 0x02, 0x12, 0x03,
    0x2e, 0x1c, 0x1d, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x01, 0x02, 0x02, 0x12, 0x03, 0x2f, 0x02, 0x22,
    0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x01, 0x02, 0x02, 0x01, 0x12, 0x03, 0x2f, 0x02, 0x1d, 0x0a, 0x0c,
    0x0a, 0x05, 0x05, 0x01, 0x02, 0x02, 0x02, 0x12, 0x03, 0x2f, 0x20, 0x21, 0x0a, 0x0b, 0x0a, 0x04,
    0x05, 0x01, 0x02, 0x03, 0x12, 0x03, 0x30, 0x02, 0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x01, 0x02,
    0x03, 0x01, 0x12, 0x03, 0x30, 0x02, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x01, 0x02, 0x03, 0x02,
    0x12, 0x03, 0x30, 0x1e, 0x1f, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x01, 0x02, 0x04, 0x12, 0x03, 0x31,
    0x02, 0x1d, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x01, 0x02, 0x04, 0x01, 0x12, 0x03, 0x31, 0x02, 0x18,
    0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x01, 0x02, 0x04, 0x02, 0x12, 0x03, 0x31, 0x1b, 0x1c, 0x0a, 0x0b,
    0x0a, 0x04, 0x05, 0x01, 0x02, 0x05, 0x12, 0x03, 0x32, 0x02, 0x21, 0x0a, 0x0c, 0x0a, 0x05, 0x05,
    0x01, 0x02, 0x05, 0x01, 0x12, 0x03, 0x32, 0x02, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x01, 0x02,
    0x05, 0x02, 0x12, 0x03, 0x32, 0x1f, 0x20, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x04, 0x12, 0x04, 0x35,
    0x00, 0x3b, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x04, 0x01, 0x12, 0x03, 0x35, 0x08, 0x1b, 0x0a,
    0x0b, 0x0a, 0x04, 0x04, 0x04, 0x02, 0x00, 0x12, 0x03, 0x36, 0x02, 0x27, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x04, 0x02, 0x00, 0x06, 0x12, 0x03, 0x36, 0x02, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04,
    0x02, 0x00, 0x01, 0x12, 0x03, 0x36, 0x1b, 0x22, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x00,
    0x03, 0x12, 0x03, 0x36, 0x25, 0x26, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x04, 0x02, 0x01, 0x12, 0x03,
    0x37, 0x02, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x01, 0x06, 0x12, 0x03, 0x37, 0x02,
    0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x01, 0x01, 0x12, 0x03, 0x37, 0x11, 0x17, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x01, 0x03, 0x12, 0x03, 0x37, 0x1a, 0x1b, 0x0a, 0x0b, 0x0a,
    0x04, 0x04, 0x04, 0x02, 0x02, 0x12, 0x03, 0x38, 0x02, 0x2b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04,
    0x02, 0x02, 0x06, 0x12, 0x03, 0x38, 0x02, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x02,
    0x01, 0x12, 0x03, 0x38, 0x1c, 0x26, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x02, 0x03, 0x12,
    0x03, 0x38, 0x29, 0x2a, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x04, 0x02, 0x03, 0x12, 0x03, 0x39, 0x02,
    0x2e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x03, 0x06, 0x12, 0x03, 0x39, 0x02, 0x1b, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x03, 0x01, 0x12, 0x03, 0x39, 0x1c, 0x29, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x04, 0x02, 0x03, 0x03, 0x12, 0x03, 0x39, 0x2c, 0x2d, 0x0a, 0x0b, 0x0a, 0x04, 0x04,
    0x04, 0x02, 0x04, 0x12, 0x03, 0x3a, 0x02, 0x1d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x04,
    0x05, 0x12, 0x03, 0x3a, 0x02, 0x08, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x04, 0x01, 0x12,
    0x03, 0x3a, 0x09, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x04, 0x03, 0x12, 0x03, 0x3a,
    0x1b, 0x1c, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x05, 0x12, 0x04, 0x3d, 0x00, 0x45, 0x01, 0x0a, 0x0a,
    0x0a, 0x03, 0x04, 0x05, 0x01, 0x12, 0x03, 0x3d, 0x08, 0x13, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x05,
    0x02, 0x00, 0x12, 0x03, 0x3e, 0x02, 0x4a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x00, 0x05,
    0x12, 0x03, 0x3e, 0x02, 0x08, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x00, 0x01, 0x12, 0x03,
    0x3e, 0x09, 0x0b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x00, 0x03, 0x12, 0x03, 0x3e, 0x0e,
    0x0f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x00, 0x08, 0x12, 0x03, 0x3e, 0x10, 0x49, 0x0a,
    0x0f, 0x0a, 0x08, 0x04, 0x05, 0x02, 0x00, 0x08, 0xc3, 0xe0, 0x33, 0x12, 0x03, 0x3e, 0x11, 0x48,
    0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x05, 0x02, 0x01, 0x12, 0x03, 0x3f, 0x02, 0x5e, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x05, 0x02, 0x01, 0x06, 0x12, 0x03, 0x3f, 0x02, 0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x05, 0x02, 0x01, 0x01, 0x12, 0x03, 0x3f, 0x16, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02,
    0x01, 0x03, 0x12, 0x03, 0x3f, 0x21, 0x22, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x01, 0x08,
    0x12, 0x03, 0x3f, 0x23, 0x5d, 0x0a, 0x0f, 0x0a, 0x08, 0x04, 0x05, 0x02, 0x01, 0x08, 0xc3, 0xe0,
    0x33, 0x12, 0x03, 0x3f, 0x24, 0x5c, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x05, 0x02, 0x02, 0x12, 0x03,
    0x40, 0x02, 0x5a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x02, 0x05, 0x12, 0x03, 0x40, 0x02,
    0x08, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x02, 0x01, 0x12, 0x03, 0x40, 0x09, 0x14, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x02, 0x03, 0x12, 0x03, 0x40, 0x17, 0x18, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x05, 0x02, 0x02, 0x08, 0x12, 0x03, 0x40, 0x19, 0x59, 0x0a, 0x0f, 0x0a, 0x08, 0x04,
    0x05, 0x02, 0x02, 0x08, 0xc3, 0xe0, 0x33, 0x12, 0x03, 0x40, 0x1a, 0x58, 0x0a, 0x76, 0x0a, 0x04,
    0x04, 0x05, 0x02, 0x03, 0x12, 0x03, 0x43, 0x02, 0x60, 0x1a, 0x69, 0x20, 0x54, 0x68, 0x65, 0x20,
    0x76, 0x65, 0x72, 0x74, 0x65, 0x78, 0x5f, 0x6a, 0x6f, 0x62, 0x5f, 0x69, 0x64, 0x20, 0x69, 0x73,
    0x20, 0x74, 0x68, 0x65, 0x20, 0x69, 0x64, 0x20, 0x6f, 0x66, 0x20, 0x74, 0x68, 0x65, 0x20, 0x56,
    0x65, 0x72, 0x74, 0x65, 0x78, 0x20, 0x41, 0x49, 0x20, 0x63, 0x75, 0x73, 0x74, 0x6f, 0x6d, 0x20,
    0x74, 0x72, 0x61, 0x69, 0x6e, 0x69, 0x6e, 0x67, 0x20, 0x6a, 0x6f, 0x62, 0x0a, 0x20, 0x62, 0x61,
    0x63, 0x6b, 0x69, 0x6e, 0x67, 0x20, 0x6f, 0x75, 0x72, 0x20, 0x63, 0x6f, 0x6e, 0x63, 0x65, 0x70,
    0x74, 0x20, 0x6f, 0x66, 0x20, 0x61, 0x20, 0x54, 0x72, 0x61, 0x69, 0x6e, 0x69, 0x6e, 0x67, 0x4a,
    0x6f, 0x62, 0x2e, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x03, 0x05, 0x12, 0x03, 0x43,
    0x02, 0x08, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x03, 0x01, 0x12, 0x03, 0x43, 0x09, 0x16,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x03, 0x03, 0x12, 0x03, 0x43, 0x19, 0x1a, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x05, 0x02, 0x03, 0x08, 0x12, 0x03, 0x43, 0x1b, 0x5f, 0x0a, 0x0f, 0x0a, 0x08,
    0x04, 0x05, 0x02, 0x03, 0x08, 0xc3, 0xe0, 0x33, 0x12, 0x03, 0x43, 0x1c, 0x5e, 0x0a, 0x0b, 0x0a,
    0x04, 0x04, 0x05, 0x02, 0x04, 0x12, 0x03, 0x44, 0x02, 0x73, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05,
    0x02, 0x04, 0x06, 0x12, 0x03, 0x44, 0x02, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x04,
    0x01, 0x12, 0x03, 0x44, 0x19, 0x27, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x04, 0x03, 0x12,
    0x03, 0x44, 0x2a, 0x2b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x04, 0x08, 0x12, 0x03, 0x44,
    0x2c, 0x72, 0x0a, 0x0f, 0x0a, 0x08, 0x04, 0x05, 0x02, 0x04, 0x08, 0xc3, 0xe0, 0x33, 0x12, 0x03,
    0x44, 0x2d, 0x71, 0x62, 0x06, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x33,
];
include!("viam.app.mltraining.v1.tonic.rs");
// @@protoc_insertion_point(module)