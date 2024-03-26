// @generated
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InferRequest {
    /// name of the model service
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// the input data is provided as set of named flat tensors
    #[prost(message, optional, tag="3")]
    pub input_tensors: ::core::option::Option<FlatTensors>,
    /// Additional arguments to the method
    #[prost(message, optional, tag="99")]
    pub extra: ::core::option::Option<::prost_types::Struct>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InferResponse {
    /// the output data is provided as a set of named flat tensors
    #[prost(message, optional, tag="3")]
    pub output_tensors: ::core::option::Option<FlatTensors>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MetadataRequest {
    /// name of the model service
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// Additional arguments to the method
    #[prost(message, optional, tag="99")]
    pub extra: ::core::option::Option<::prost_types::Struct>,
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
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FlatTensorDataInt8 {
    #[prost(bytes="vec", tag="1")]
    pub data: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FlatTensorDataUInt8 {
    #[prost(bytes="vec", tag="1")]
    pub data: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FlatTensorDataInt16 {
    /// packs two 16-bit numbers per entry - explicitly little-endian
    /// so big-endian producers/consumers must compensate
    #[prost(fixed32, repeated, tag="1")]
    pub data: ::prost::alloc::vec::Vec<u32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FlatTensorDataUInt16 {
    /// packs two 16-bit numbers per entry - explicitly little-endian
    /// so big-endian producers/consumers must compensate
    #[prost(fixed32, repeated, tag="1")]
    pub data: ::prost::alloc::vec::Vec<u32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FlatTensorDataInt32 {
    #[prost(sfixed32, repeated, tag="1")]
    pub data: ::prost::alloc::vec::Vec<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FlatTensorDataUInt32 {
    #[prost(fixed32, repeated, tag="1")]
    pub data: ::prost::alloc::vec::Vec<u32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FlatTensorDataInt64 {
    #[prost(sfixed64, repeated, tag="1")]
    pub data: ::prost::alloc::vec::Vec<i64>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FlatTensorDataUInt64 {
    #[prost(fixed64, repeated, tag="1")]
    pub data: ::prost::alloc::vec::Vec<u64>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FlatTensorDataFloat {
    #[prost(float, repeated, tag="1")]
    pub data: ::prost::alloc::vec::Vec<f32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FlatTensorDataDouble {
    #[prost(double, repeated, tag="1")]
    pub data: ::prost::alloc::vec::Vec<f64>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FlatTensor {
    /// the shape of the provided tensor as a list of integer extents
    #[prost(fixed64, repeated, tag="1")]
    pub shape: ::prost::alloc::vec::Vec<u64>,
    /// the flat data to be interpreted per the above shape information
    #[prost(oneof="flat_tensor::Tensor", tags="2, 3, 4, 5, 6, 7, 8, 9, 10, 11")]
    pub tensor: ::core::option::Option<flat_tensor::Tensor>,
}
/// Nested message and enum types in `FlatTensor`.
pub mod flat_tensor {
    /// the flat data to be interpreted per the above shape information
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Tensor {
        #[prost(message, tag="2")]
        Int8Tensor(super::FlatTensorDataInt8),
        #[prost(message, tag="3")]
        Uint8Tensor(super::FlatTensorDataUInt8),
        #[prost(message, tag="4")]
        Int16Tensor(super::FlatTensorDataInt16),
        #[prost(message, tag="5")]
        Uint16Tensor(super::FlatTensorDataUInt16),
        #[prost(message, tag="6")]
        Int32Tensor(super::FlatTensorDataInt32),
        #[prost(message, tag="7")]
        Uint32Tensor(super::FlatTensorDataUInt32),
        #[prost(message, tag="8")]
        Int64Tensor(super::FlatTensorDataInt64),
        #[prost(message, tag="9")]
        Uint64Tensor(super::FlatTensorDataUInt64),
        #[prost(message, tag="10")]
        FloatTensor(super::FlatTensorDataFloat),
        #[prost(message, tag="11")]
        DoubleTensor(super::FlatTensorDataDouble),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FlatTensors {
    /// A name-indexed collection of flat tensor objects
    #[prost(map="string, message", tag="1")]
    pub tensors: ::std::collections::HashMap<::prost::alloc::string::String, FlatTensor>,
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
