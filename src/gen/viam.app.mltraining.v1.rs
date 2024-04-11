// @generated
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SubmitTrainingJobRequest {
    #[prost(string, tag="7")]
    pub dataset_id: ::prost::alloc::string::String,
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
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SubmitTrainingJobResponse {
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetTrainingJobRequest {
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetTrainingJobResponse {
    #[prost(message, optional, tag="1")]
    pub metadata: ::core::option::Option<TrainingJobMetadata>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListTrainingJobsRequest {
    #[prost(string, tag="1")]
    pub organization_id: ::prost::alloc::string::String,
    #[prost(enumeration="TrainingStatus", tag="2")]
    pub status: i32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListTrainingJobsResponse {
    #[prost(message, repeated, tag="1")]
    pub jobs: ::prost::alloc::vec::Vec<TrainingJobMetadata>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TrainingJobMetadata {
    #[prost(message, optional, tag="1")]
    pub request: ::core::option::Option<SubmitTrainingJobRequest>,
    #[prost(string, tag="7")]
    pub id: ::prost::alloc::string::String,
    #[prost(string, tag="11")]
    pub dataset_id: ::prost::alloc::string::String,
    #[prost(string, tag="12")]
    pub organization_id: ::prost::alloc::string::String,
    #[prost(string, tag="13")]
    pub model_name: ::prost::alloc::string::String,
    #[prost(string, tag="14")]
    pub model_version: ::prost::alloc::string::String,
    #[prost(enumeration="ModelType", tag="15")]
    pub model_type: i32,
    #[prost(enumeration="TrainingStatus", tag="2")]
    pub status: i32,
    #[prost(message, optional, tag="8")]
    pub error_status: ::core::option::Option<super::super::super::super::google::rpc::Status>,
    #[prost(message, optional, tag="3")]
    pub created_on: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(message, optional, tag="4")]
    pub last_modified: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(message, optional, tag="9")]
    pub training_started: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(message, optional, tag="10")]
    pub training_ended: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(string, tag="5")]
    pub synced_model_id: ::prost::alloc::string::String,
    #[prost(string, repeated, tag="16")]
    pub tags: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CancelTrainingJobRequest {
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CancelTrainingJobResponse {
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteCompletedTrainingJobRequest {
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteCompletedTrainingJobResponse {
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ModelType {
    Unspecified = 0,
    SingleLabelClassification = 1,
    MultiLabelClassification = 2,
    ObjectDetection = 3,
}
impl ModelType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            ModelType::Unspecified => "MODEL_TYPE_UNSPECIFIED",
            ModelType::SingleLabelClassification => "MODEL_TYPE_SINGLE_LABEL_CLASSIFICATION",
            ModelType::MultiLabelClassification => "MODEL_TYPE_MULTI_LABEL_CLASSIFICATION",
            ModelType::ObjectDetection => "MODEL_TYPE_OBJECT_DETECTION",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "MODEL_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
            "MODEL_TYPE_SINGLE_LABEL_CLASSIFICATION" => Some(Self::SingleLabelClassification),
            "MODEL_TYPE_MULTI_LABEL_CLASSIFICATION" => Some(Self::MultiLabelClassification),
            "MODEL_TYPE_OBJECT_DETECTION" => Some(Self::ObjectDetection),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ModelFramework {
    Unspecified = 0,
    Tflite = 1,
    Tensorflow = 2,
    Pytorch = 3,
    Onnx = 4,
}
impl ModelFramework {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            ModelFramework::Unspecified => "MODEL_FRAMEWORK_UNSPECIFIED",
            ModelFramework::Tflite => "MODEL_FRAMEWORK_TFLITE",
            ModelFramework::Tensorflow => "MODEL_FRAMEWORK_TENSORFLOW",
            ModelFramework::Pytorch => "MODEL_FRAMEWORK_PYTORCH",
            ModelFramework::Onnx => "MODEL_FRAMEWORK_ONNX",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "MODEL_FRAMEWORK_UNSPECIFIED" => Some(Self::Unspecified),
            "MODEL_FRAMEWORK_TFLITE" => Some(Self::Tflite),
            "MODEL_FRAMEWORK_TENSORFLOW" => Some(Self::Tensorflow),
            "MODEL_FRAMEWORK_PYTORCH" => Some(Self::Pytorch),
            "MODEL_FRAMEWORK_ONNX" => Some(Self::Onnx),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum TrainingStatus {
    Unspecified = 0,
    Pending = 1,
    InProgress = 2,
    Completed = 3,
    Failed = 4,
    Canceled = 5,
    Canceling = 6,
}
impl TrainingStatus {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            TrainingStatus::Unspecified => "TRAINING_STATUS_UNSPECIFIED",
            TrainingStatus::Pending => "TRAINING_STATUS_PENDING",
            TrainingStatus::InProgress => "TRAINING_STATUS_IN_PROGRESS",
            TrainingStatus::Completed => "TRAINING_STATUS_COMPLETED",
            TrainingStatus::Failed => "TRAINING_STATUS_FAILED",
            TrainingStatus::Canceled => "TRAINING_STATUS_CANCELED",
            TrainingStatus::Canceling => "TRAINING_STATUS_CANCELING",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "TRAINING_STATUS_UNSPECIFIED" => Some(Self::Unspecified),
            "TRAINING_STATUS_PENDING" => Some(Self::Pending),
            "TRAINING_STATUS_IN_PROGRESS" => Some(Self::InProgress),
            "TRAINING_STATUS_COMPLETED" => Some(Self::Completed),
            "TRAINING_STATUS_FAILED" => Some(Self::Failed),
            "TRAINING_STATUS_CANCELED" => Some(Self::Canceled),
            "TRAINING_STATUS_CANCELING" => Some(Self::Canceling),
            _ => None,
        }
    }
}
// @@protoc_insertion_point(module)
