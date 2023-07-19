// @generated
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InferRequest {
    /// name of the model service
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// this is a struct of input arrays/tensors as specified in the metadata
    #[prost(message, optional, tag="2")]
    pub input_data: ::core::option::Option<::prost_types::Struct>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InferResponse {
    /// this is a struct of output arrays/tensors as specified in the metadata
    #[prost(message, optional, tag="2")]
    pub output_data: ::core::option::Option<::prost_types::Struct>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MetadataRequest {
    /// name of the model service
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MetadataResponse {
    /// this is the metadata associated with the ML model
    #[prost(message, optional, tag="1")]
    pub metadata: ::core::option::Option<Metadata>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Metadata {
    /// name of the model
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// type of model e.g. object_detector, text_classifier
    #[prost(string, tag="2")]
    pub r#type: ::prost::alloc::string::String,
    /// description of the model
    #[prost(string, tag="3")]
    pub description: ::prost::alloc::string::String,
    /// the necessary input arrays/tensors for an inference, order matters
    #[prost(message, repeated, tag="4")]
    pub input_info: ::prost::alloc::vec::Vec<TensorInfo>,
    /// the output arrays/tensors of the model, order matters
    #[prost(message, repeated, tag="5")]
    pub output_info: ::prost::alloc::vec::Vec<TensorInfo>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TensorInfo {
    /// name of the data in the array/tensor
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// description of the data in the array/tensor
    #[prost(string, tag="2")]
    pub description: ::prost::alloc::string::String,
    /// data type of the array/tensor, e.g. float32, float64, uint8
    #[prost(string, tag="3")]
    pub data_type: ::prost::alloc::string::String,
    /// shape of the array/tensor (-1 for unknown)
    #[prost(int32, repeated, tag="4")]
    pub shape: ::prost::alloc::vec::Vec<i32>,
    /// files associated with the array/tensor, like for category labels
    #[prost(message, repeated, tag="5")]
    pub associated_files: ::prost::alloc::vec::Vec<File>,
    /// anything else you want to say
    #[prost(message, optional, tag="99")]
    pub extra: ::core::option::Option<::prost_types::Struct>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct File {
    /// name of the file, with file extension
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// description of what the file contains
    #[prost(string, tag="2")]
    pub description: ::prost::alloc::string::String,
    /// How to associate the arrays/tensors to the labels in the file
    #[prost(enumeration="LabelType", tag="3")]
    pub label_type: i32,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum LabelType {
    Unspecified = 0,
    /// the value of the arrays/tensor is the label index
    TensorValue = 1,
    /// the position of the tensor value in the axis is the label index
    TensorAxis = 2,
}
impl LabelType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            LabelType::Unspecified => "LABEL_TYPE_UNSPECIFIED",
            LabelType::TensorValue => "LABEL_TYPE_TENSOR_VALUE",
            LabelType::TensorAxis => "LABEL_TYPE_TENSOR_AXIS",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "LABEL_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
            "LABEL_TYPE_TENSOR_VALUE" => Some(Self::TensorValue),
            "LABEL_TYPE_TENSOR_AXIS" => Some(Self::TensorAxis),
            _ => None,
        }
    }
}
// @@protoc_insertion_point(module)
