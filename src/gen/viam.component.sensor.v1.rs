// @generated
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetReadingsRequest {
    /// Name of a sensor
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// Additional arguments to the method
    #[prost(message, optional, tag="99")]
    pub extra: ::core::option::Option<::prost_types::Struct>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetReadingsResponse {
    #[prost(map="string, message", tag="1")]
    pub readings: ::std::collections::HashMap<::prost::alloc::string::String, ::prost_types::Value>,
}
// @@protoc_insertion_point(module)
