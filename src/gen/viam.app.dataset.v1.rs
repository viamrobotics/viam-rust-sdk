// @generated
/// Dataset stores the metadata of a dataset.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Dataset {
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub organization_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag="4")]
    pub time_created: ::core::option::Option<::prost_types::Timestamp>,
}
/// CreateDatasetRequest defines the name and organization ID of a dataset.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateDatasetRequest {
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub organization_id: ::prost::alloc::string::String,
}
/// CreateDatasetResponse returns the dataset ID of the created dataset.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateDatasetResponse {
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
}
/// DeleteDatasetRequest deletes the dataset specified by the dataset ID.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteDatasetRequest {
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteDatasetResponse {
}
/// RenameDatasetRequest applies the new name to the dataset specified by the dataset ID.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RenameDatasetRequest {
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub name: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RenameDatasetResponse {
}
/// ListDatasetsByOrganizationIDRequest requests all of the datasets for an organization.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListDatasetsByOrganizationIdRequest {
    #[prost(string, tag="1")]
    pub organization_id: ::prost::alloc::string::String,
}
/// ListDatasetsByOrganizationIDResponse returns all the dataset metadata for the organization.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListDatasetsByOrganizationIdResponse {
    #[prost(message, repeated, tag="1")]
    pub datasets: ::prost::alloc::vec::Vec<Dataset>,
}
/// ListDatasetsByIDsRequest requests all of the datasets by their dataset IDs.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListDatasetsByIDsRequest {
    #[prost(string, repeated, tag="1")]
    pub ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// ListDatasetsByIDsResponse returns all the dataset metadata for the associated dataset IDs.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListDatasetsByIDsResponse {
    #[prost(message, repeated, tag="1")]
    pub datasets: ::prost::alloc::vec::Vec<Dataset>,
}
// @@protoc_insertion_point(module)
