// @generated
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DataRequest {
    #[prost(message, optional, tag="1")]
    pub filter: ::core::option::Option<Filter>,
    #[prost(uint64, tag="2")]
    pub limit: u64,
    #[prost(string, tag="3")]
    pub last: ::prost::alloc::string::String,
    #[prost(enumeration="Order", tag="4")]
    pub sort_order: i32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Filter {
    #[prost(string, tag="1")]
    pub component_name: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub component_type: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub method: ::prost::alloc::string::String,
    #[prost(string, tag="6")]
    pub robot_name: ::prost::alloc::string::String,
    #[prost(string, tag="7")]
    pub robot_id: ::prost::alloc::string::String,
    #[prost(string, tag="8")]
    pub part_name: ::prost::alloc::string::String,
    #[prost(string, tag="9")]
    pub part_id: ::prost::alloc::string::String,
    #[prost(string, repeated, tag="10")]
    pub location_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, repeated, tag="11")]
    pub organization_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, repeated, tag="12")]
    pub mime_type: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(message, optional, tag="13")]
    pub interval: ::core::option::Option<CaptureInterval>,
    #[prost(message, optional, tag="14")]
    pub tags_filter: ::core::option::Option<TagsFilter>,
    #[prost(string, repeated, tag="15")]
    pub bbox_labels: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TagsFilter {
    #[prost(enumeration="TagsFilterType", tag="1")]
    pub r#type: i32,
    /// Tags are used to match documents if `type` is UNSPECIFIED or MATCH_BY_ORG
    #[prost(string, repeated, tag="2")]
    pub tags: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// CaptureMetadata contains information on the settings used for the data capture
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CaptureMetadata {
    #[prost(string, tag="1")]
    pub organization_id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub location_id: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub robot_name: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub robot_id: ::prost::alloc::string::String,
    #[prost(string, tag="5")]
    pub part_name: ::prost::alloc::string::String,
    #[prost(string, tag="6")]
    pub part_id: ::prost::alloc::string::String,
    #[prost(string, tag="7")]
    pub component_type: ::prost::alloc::string::String,
    #[prost(string, tag="9")]
    pub component_name: ::prost::alloc::string::String,
    #[prost(string, tag="10")]
    pub method_name: ::prost::alloc::string::String,
    #[prost(map="string, message", tag="11")]
    pub method_parameters: ::std::collections::HashMap<::prost::alloc::string::String, ::prost_types::Any>,
    #[prost(string, repeated, tag="12")]
    pub tags: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, tag="13")]
    pub mime_type: ::prost::alloc::string::String,
}
/// CaptureInterval describes the start and end time of the capture in this file
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CaptureInterval {
    #[prost(message, optional, tag="1")]
    pub start: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(message, optional, tag="2")]
    pub end: ::core::option::Option<::prost_types::Timestamp>,
}
/// TabularDataByFilterRequest requests tabular data based on filter values
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TabularDataByFilterRequest {
    #[prost(message, optional, tag="1")]
    pub data_request: ::core::option::Option<DataRequest>,
    #[prost(bool, tag="2")]
    pub count_only: bool,
}
/// TabularDataByFilterResponse provides the data and metadata of tabular data
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TabularDataByFilterResponse {
    #[prost(message, repeated, tag="1")]
    pub metadata: ::prost::alloc::vec::Vec<CaptureMetadata>,
    #[prost(message, repeated, tag="2")]
    pub data: ::prost::alloc::vec::Vec<TabularData>,
    #[prost(uint64, tag="3")]
    pub count: u64,
    #[prost(string, tag="4")]
    pub last: ::prost::alloc::string::String,
    #[prost(uint64, tag="5")]
    pub total_size_bytes: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TabularData {
    #[prost(message, optional, tag="1")]
    pub data: ::core::option::Option<::prost_types::Struct>,
    #[prost(uint32, tag="2")]
    pub metadata_index: u32,
    #[prost(message, optional, tag="3")]
    pub time_requested: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(message, optional, tag="4")]
    pub time_received: ::core::option::Option<::prost_types::Timestamp>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BinaryData {
    #[prost(bytes="vec", tag="1")]
    pub binary: ::prost::alloc::vec::Vec<u8>,
    #[prost(message, optional, tag="2")]
    pub metadata: ::core::option::Option<BinaryMetadata>,
}
/// BinaryDataByFilterRequest requests the data and metadata of binary (image + file) data when a filter is provided
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BinaryDataByFilterRequest {
    #[prost(message, optional, tag="1")]
    pub data_request: ::core::option::Option<DataRequest>,
    #[prost(bool, tag="2")]
    pub include_binary: bool,
    #[prost(bool, tag="3")]
    pub count_only: bool,
}
/// BinaryDataByFilterResponse provides the data and metadata of binary (image + file) data when a filter is provided
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BinaryDataByFilterResponse {
    #[prost(message, repeated, tag="1")]
    pub data: ::prost::alloc::vec::Vec<BinaryData>,
    #[prost(uint64, tag="2")]
    pub count: u64,
    #[prost(string, tag="3")]
    pub last: ::prost::alloc::string::String,
    #[prost(uint64, tag="4")]
    pub total_size_bytes: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BinaryId {
    #[prost(string, tag="1")]
    pub file_id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub organization_id: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub location_id: ::prost::alloc::string::String,
}
/// BinaryDataByFilterRequest requests the data and metadata of binary (image + file) data by binary ids
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BinaryDataByIDsRequest {
    #[prost(bool, tag="2")]
    pub include_binary: bool,
    #[prost(message, repeated, tag="3")]
    pub binary_ids: ::prost::alloc::vec::Vec<BinaryId>,
    /// Replaced by binary_ids
    #[deprecated]
    #[prost(string, repeated, tag="1")]
    pub file_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// BinaryDataByIDsResponse provides the data and metadata of binary (image + file) data when a filter is provided
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BinaryDataByIDsResponse {
    #[prost(message, repeated, tag="1")]
    pub data: ::prost::alloc::vec::Vec<BinaryData>,
    #[prost(uint64, tag="2")]
    pub count: u64,
}
/// BoundingBox represents a labeled bounding box on an image.
/// x and y values are normalized ratios between 0 and 1.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BoundingBox {
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub label: ::prost::alloc::string::String,
    #[prost(double, tag="3")]
    pub x_min_normalized: f64,
    #[prost(double, tag="4")]
    pub y_min_normalized: f64,
    #[prost(double, tag="5")]
    pub x_max_normalized: f64,
    #[prost(double, tag="6")]
    pub y_max_normalized: f64,
}
/// Annotations are data annotations used for machine learning.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Annotations {
    #[prost(message, repeated, tag="1")]
    pub bboxes: ::prost::alloc::vec::Vec<BoundingBox>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BinaryMetadata {
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub capture_metadata: ::core::option::Option<CaptureMetadata>,
    #[prost(message, optional, tag="3")]
    pub time_requested: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(message, optional, tag="4")]
    pub time_received: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(string, tag="5")]
    pub file_name: ::prost::alloc::string::String,
    #[prost(string, tag="6")]
    pub file_ext: ::prost::alloc::string::String,
    #[prost(string, tag="7")]
    pub uri: ::prost::alloc::string::String,
    #[prost(message, optional, tag="8")]
    pub annotations: ::core::option::Option<Annotations>,
}
/// DeleteTabularDataByFilterRequest deletes the data and metadata of tabular data when a filter is provided
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteTabularDataByFilterRequest {
    #[prost(message, optional, tag="1")]
    pub filter: ::core::option::Option<Filter>,
}
/// DeleteBinaryDataByFilterResponse returns the number of tabular datapoints deleted when a filter is provided
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteTabularDataByFilterResponse {
    #[prost(uint64, tag="1")]
    pub deleted_count: u64,
}
/// DeleteBinaryDataByFilterRequest deletes the data and metadata of binary data when a filter is provided
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteBinaryDataByFilterRequest {
    #[prost(message, optional, tag="1")]
    pub filter: ::core::option::Option<Filter>,
}
/// DeleteBinaryDataByFilterResponse returns the number of binary files deleted when a filter is provided
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteBinaryDataByFilterResponse {
    #[prost(uint64, tag="1")]
    pub deleted_count: u64,
}
/// DeleteBinaryDataByIDsRequest deletes the data and metadata of binary data when binary ids are provided
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteBinaryDataByIDsRequest {
    #[prost(message, repeated, tag="2")]
    pub binary_ids: ::prost::alloc::vec::Vec<BinaryId>,
    /// Replaced by binary_ids
    #[deprecated]
    #[prost(string, repeated, tag="1")]
    pub file_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// DeleteBinaryDataByIDsResponse returns the number of binary files deleted when binary ids are provided
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteBinaryDataByIDsResponse {
    #[prost(uint64, tag="1")]
    pub deleted_count: u64,
}
/// AddTagsToBinaryDataByIDsRequest requests adding all specified tags to each of the files when binary ids are provided
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddTagsToBinaryDataByIDsRequest {
    #[prost(message, repeated, tag="3")]
    pub binary_ids: ::prost::alloc::vec::Vec<BinaryId>,
    #[prost(string, repeated, tag="2")]
    pub tags: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Replaced by binary_ids
    #[deprecated]
    #[prost(string, repeated, tag="1")]
    pub file_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddTagsToBinaryDataByIDsResponse {
}
/// AddTagsToBinaryDataByFilterRequest requests adding all specified tags to each of the files when a filter is provided
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddTagsToBinaryDataByFilterRequest {
    #[prost(message, optional, tag="1")]
    pub filter: ::core::option::Option<Filter>,
    #[prost(string, repeated, tag="2")]
    pub tags: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddTagsToBinaryDataByFilterResponse {
}
/// RemoveTagsFromBinaryDataByIDsRequest requests removing the given tags value from each file when binary ids are provided
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveTagsFromBinaryDataByIDsRequest {
    #[prost(message, repeated, tag="3")]
    pub binary_ids: ::prost::alloc::vec::Vec<BinaryId>,
    #[prost(string, repeated, tag="2")]
    pub tags: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Replaced by binary_ids
    #[deprecated]
    #[prost(string, repeated, tag="1")]
    pub file_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// RemoveTagsFromBinaryDataByIDsResponse returns the number of binary files which had tags removed
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveTagsFromBinaryDataByIDsResponse {
    #[prost(uint64, tag="1")]
    pub deleted_count: u64,
}
/// RemoveTagsFromBinaryDataByFilterRequest requests removing the given tags value from each file when a filter is provided
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveTagsFromBinaryDataByFilterRequest {
    #[prost(message, optional, tag="1")]
    pub filter: ::core::option::Option<Filter>,
    #[prost(string, repeated, tag="2")]
    pub tags: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// RemoveTagsFromBinaryDataByFilterResponse returns the number of binary files which had tags removed
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveTagsFromBinaryDataByFilterResponse {
    #[prost(uint64, tag="1")]
    pub deleted_count: u64,
}
/// TagsByFilterRequest requests the unique tags from data based on given filter
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TagsByFilterRequest {
    #[prost(message, optional, tag="1")]
    pub filter: ::core::option::Option<Filter>,
}
/// TagsByFilterResponse returns the unique tags from data based on given filter
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TagsByFilterResponse {
    #[prost(string, repeated, tag="1")]
    pub tags: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddBoundingBoxToImageByIdRequest {
    #[prost(message, optional, tag="7")]
    pub binary_id: ::core::option::Option<BinaryId>,
    #[prost(string, tag="2")]
    pub label: ::prost::alloc::string::String,
    #[prost(double, tag="3")]
    pub x_min_normalized: f64,
    #[prost(double, tag="4")]
    pub y_min_normalized: f64,
    #[prost(double, tag="5")]
    pub x_max_normalized: f64,
    #[prost(double, tag="6")]
    pub y_max_normalized: f64,
    /// Replaced by binary_id
    #[deprecated]
    #[prost(string, tag="1")]
    pub file_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddBoundingBoxToImageByIdResponse {
    #[prost(string, tag="1")]
    pub bbox_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveBoundingBoxFromImageByIdRequest {
    #[prost(message, optional, tag="3")]
    pub binary_id: ::core::option::Option<BinaryId>,
    #[prost(string, tag="2")]
    pub bbox_id: ::prost::alloc::string::String,
    /// Replaced by binary_id
    #[deprecated]
    #[prost(string, tag="1")]
    pub file_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveBoundingBoxFromImageByIdResponse {
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BoundingBoxLabelsByFilterRequest {
    #[prost(message, optional, tag="1")]
    pub filter: ::core::option::Option<Filter>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BoundingBoxLabelsByFilterResponse {
    #[prost(string, repeated, tag="1")]
    pub labels: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum Order {
    Unspecified = 0,
    Descending = 1,
    Ascending = 2,
}
impl Order {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Order::Unspecified => "ORDER_UNSPECIFIED",
            Order::Descending => "ORDER_DESCENDING",
            Order::Ascending => "ORDER_ASCENDING",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "ORDER_UNSPECIFIED" => Some(Self::Unspecified),
            "ORDER_DESCENDING" => Some(Self::Descending),
            "ORDER_ASCENDING" => Some(Self::Ascending),
            _ => None,
        }
    }
}
/// TagsFilterType specifies how data can be filtered based on tags
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum TagsFilterType {
    Unspecified = 0,
    /// TAGS_FILTER_TYPE_MATCH_BY_OR specifies documents matched (using logical OR) on the tags field in the TagsFilter
    MatchByOr = 1,
    /// TAGS_FILTER_TYPE_TAGGED specifies that all tagged documents should be returned
    Tagged = 2,
    /// TAGS_FILTER_TYPE_UNTAGGED specifes that all untagged documents should be returned
    Untagged = 3,
}
impl TagsFilterType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            TagsFilterType::Unspecified => "TAGS_FILTER_TYPE_UNSPECIFIED",
            TagsFilterType::MatchByOr => "TAGS_FILTER_TYPE_MATCH_BY_OR",
            TagsFilterType::Tagged => "TAGS_FILTER_TYPE_TAGGED",
            TagsFilterType::Untagged => "TAGS_FILTER_TYPE_UNTAGGED",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "TAGS_FILTER_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
            "TAGS_FILTER_TYPE_MATCH_BY_OR" => Some(Self::MatchByOr),
            "TAGS_FILTER_TYPE_TAGGED" => Some(Self::Tagged),
            "TAGS_FILTER_TYPE_UNTAGGED" => Some(Self::Untagged),
            _ => None,
        }
    }
}
// @@protoc_insertion_point(module)
