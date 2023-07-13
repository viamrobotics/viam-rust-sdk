// @generated
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RecordRequest {
    /// Name of an audio input
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub duration: ::core::option::Option<::prost_types::Duration>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AudioChunkInfo {
    /// Actual sample encoding format of the response
    #[prost(enumeration="SampleFormat", tag="1")]
    pub sample_format: i32,
    #[prost(uint32, tag="2")]
    pub channels: u32,
    #[prost(int64, tag="3")]
    pub sampling_rate: i64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AudioChunk {
    /// Data is PCM data that is organized according to the sample format
    /// along with its possible interleaving. Data in each format is
    /// Little Endian.
    #[prost(bytes="vec", tag="1")]
    pub data: ::prost::alloc::vec::Vec<u8>,
    /// Length is the number of samples
    #[prost(uint32, tag="2")]
    pub length: u32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChunksRequest {
    /// Name of an audio input
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// Requested sample encoding format of the response
    #[prost(enumeration="SampleFormat", tag="2")]
    pub sample_format: i32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChunksResponse {
    #[prost(oneof="chunks_response::Type", tags="1, 2")]
    pub r#type: ::core::option::Option<chunks_response::Type>,
}
/// Nested message and enum types in `ChunksResponse`.
pub mod chunks_response {
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Type {
        #[prost(message, tag="1")]
        Info(super::AudioChunkInfo),
        #[prost(message, tag="2")]
        Chunk(super::AudioChunk),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PropertiesRequest {
    /// Name of an audio input
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PropertiesResponse {
    #[prost(uint32, tag="1")]
    pub channel_count: u32,
    #[prost(message, optional, tag="2")]
    pub latency: ::core::option::Option<::prost_types::Duration>,
    #[prost(uint32, tag="3")]
    pub sample_rate: u32,
    #[prost(uint32, tag="4")]
    pub sample_size: u32,
    #[prost(bool, tag="5")]
    pub is_big_endian: bool,
    #[prost(bool, tag="6")]
    pub is_float: bool,
    #[prost(bool, tag="7")]
    pub is_interleaved: bool,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum SampleFormat {
    Unspecified = 0,
    Int16Interleaved = 1,
    Float32Interleaved = 2,
}
impl SampleFormat {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            SampleFormat::Unspecified => "SAMPLE_FORMAT_UNSPECIFIED",
            SampleFormat::Int16Interleaved => "SAMPLE_FORMAT_INT16_INTERLEAVED",
            SampleFormat::Float32Interleaved => "SAMPLE_FORMAT_FLOAT32_INTERLEAVED",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "SAMPLE_FORMAT_UNSPECIFIED" => Some(Self::Unspecified),
            "SAMPLE_FORMAT_INT16_INTERLEAVED" => Some(Self::Int16Interleaved),
            "SAMPLE_FORMAT_FLOAT32_INTERLEAVED" => Some(Self::Float32Interleaved),
            _ => None,
        }
    }
}
// @@protoc_insertion_point(module)
