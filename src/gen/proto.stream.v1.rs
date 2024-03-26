// @generated
/// ListStreamsRequest requests all streams registered.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListStreamsRequest {
}
/// A ListStreamsResponse details streams registered.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListStreamsResponse {
    #[prost(string, repeated, tag="1")]
    pub names: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// A AddStreamRequest requests the given stream be added to the connection.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddStreamRequest {
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
}
/// AddStreamResponse is returned after a successful AddStreamRequest.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddStreamResponse {
}
/// A RemoveStreamRequest requests the given stream be removed from the connection.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveStreamRequest {
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
}
/// RemoveStreamResponse is returned after a successful RemoveStreamRequest.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveStreamResponse {
}
// @@protoc_insertion_point(module)
