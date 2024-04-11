// @generated
/// DataCaptureUploadRequest requests to upload the contents and metadata for tabular data.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DataCaptureUploadRequest {
    #[prost(message, optional, tag="1")]
    pub metadata: ::core::option::Option<UploadMetadata>,
    #[prost(message, repeated, tag="2")]
    pub sensor_contents: ::prost::alloc::vec::Vec<SensorData>,
}
/// DataCaptureUploadResponse returns the file id of the uploaded contents and metadata for tabular data.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DataCaptureUploadResponse {
    #[prost(string, tag="1")]
    pub file_id: ::prost::alloc::string::String,
}
/// FileUploadRequest requests to upload the contents and metadata for binary (image + file) data.
/// The first packet must be the UploadMetadata associated with the binary data.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FileUploadRequest {
    #[prost(oneof="file_upload_request::UploadPacket", tags="1, 2")]
    pub upload_packet: ::core::option::Option<file_upload_request::UploadPacket>,
}
/// Nested message and enum types in `FileUploadRequest`.
pub mod file_upload_request {
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum UploadPacket {
        #[prost(message, tag="1")]
        Metadata(super::UploadMetadata),
        #[prost(message, tag="2")]
        FileContents(super::FileData),
    }
}
/// FileUploadResponse returns the file id of the uploaded contents and metadata for binary (image + file) data.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FileUploadResponse {
    #[prost(string, tag="1")]
    pub file_id: ::prost::alloc::string::String,
}
/// StreamingDataCaptureUploadRequest requests to upload the contents and metadata for streaming binary (image + file) data.
/// The first packet must be the DataCaptureUploadMetadata associated with the data.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StreamingDataCaptureUploadRequest {
    #[prost(oneof="streaming_data_capture_upload_request::UploadPacket", tags="1, 2")]
    pub upload_packet: ::core::option::Option<streaming_data_capture_upload_request::UploadPacket>,
}
/// Nested message and enum types in `StreamingDataCaptureUploadRequest`.
pub mod streaming_data_capture_upload_request {
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum UploadPacket {
        #[prost(message, tag="1")]
        Metadata(super::DataCaptureUploadMetadata),
        #[prost(bytes, tag="2")]
        Data(::prost::alloc::vec::Vec<u8>),
    }
}
/// StreamingDataCaptureUploadResponse returns the file id of the uploaded contents and metadata for streaming binary (image + file) data.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StreamingDataCaptureUploadResponse {
    #[prost(string, tag="1")]
    pub file_id: ::prost::alloc::string::String,
}
/// SensorMetadata contains the time the sensor data was requested and was received.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SensorMetadata {
    #[prost(message, optional, tag="1")]
    pub time_requested: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(message, optional, tag="2")]
    pub time_received: ::core::option::Option<::prost_types::Timestamp>,
}
/// SensorData contains the contents and metadata for tabular data.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SensorData {
    #[prost(message, optional, tag="1")]
    pub metadata: ::core::option::Option<SensorMetadata>,
    #[prost(oneof="sensor_data::Data", tags="2, 3")]
    pub data: ::core::option::Option<sensor_data::Data>,
}
/// Nested message and enum types in `SensorData`.
pub mod sensor_data {
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Data {
        #[prost(message, tag="2")]
        Struct(::prost_types::Struct),
        #[prost(bytes, tag="3")]
        Binary(::prost::alloc::vec::Vec<u8>),
    }
}
/// FileData contains the contents of binary (image + file) data.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FileData {
    #[prost(bytes="vec", tag="1")]
    pub data: ::prost::alloc::vec::Vec<u8>,
}
/// UploadMetadata contains the metadata for binary (image + file) data.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UploadMetadata {
    #[prost(string, tag="1")]
    pub part_id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub component_type: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub component_name: ::prost::alloc::string::String,
    #[prost(string, tag="5")]
    pub method_name: ::prost::alloc::string::String,
    #[prost(enumeration="DataType", tag="6")]
    pub r#type: i32,
    #[prost(string, tag="7")]
    pub file_name: ::prost::alloc::string::String,
    #[prost(map="string, message", tag="8")]
    pub method_parameters: ::std::collections::HashMap<::prost::alloc::string::String, ::prost_types::Any>,
    #[prost(string, tag="9")]
    pub file_extension: ::prost::alloc::string::String,
    #[prost(string, repeated, tag="10")]
    pub tags: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// CaptureInterval specifies the start and end times of the data capture.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CaptureInterval {
    #[prost(message, optional, tag="1")]
    pub start: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(message, optional, tag="2")]
    pub end: ::core::option::Option<::prost_types::Timestamp>,
}
/// DataCaptureMetadata contains the metadata for data captured by collectors.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DataCaptureMetadata {
    #[prost(string, tag="1")]
    pub component_type: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub component_name: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub method_name: ::prost::alloc::string::String,
    #[prost(enumeration="DataType", tag="5")]
    pub r#type: i32,
    #[prost(map="string, message", tag="6")]
    pub method_parameters: ::std::collections::HashMap<::prost::alloc::string::String, ::prost_types::Any>,
    #[prost(string, tag="7")]
    pub file_extension: ::prost::alloc::string::String,
    #[prost(string, repeated, tag="8")]
    pub tags: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// DataCaptureUploadMetadata contains the metadata for streaming binary (image + file) data.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DataCaptureUploadMetadata {
    #[prost(message, optional, tag="1")]
    pub upload_metadata: ::core::option::Option<UploadMetadata>,
    #[prost(message, optional, tag="2")]
    pub sensor_metadata: ::core::option::Option<SensorMetadata>,
}
/// DataType specifies the type of data uploaded.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum DataType {
    Unspecified = 0,
    BinarySensor = 1,
    TabularSensor = 2,
    File = 3,
}
impl DataType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            DataType::Unspecified => "DATA_TYPE_UNSPECIFIED",
            DataType::BinarySensor => "DATA_TYPE_BINARY_SENSOR",
            DataType::TabularSensor => "DATA_TYPE_TABULAR_SENSOR",
            DataType::File => "DATA_TYPE_FILE",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "DATA_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
            "DATA_TYPE_BINARY_SENSOR" => Some(Self::BinarySensor),
            "DATA_TYPE_TABULAR_SENSOR" => Some(Self::TabularSensor),
            "DATA_TYPE_FILE" => Some(Self::File),
            _ => None,
        }
    }
}
// @@protoc_insertion_point(module)
