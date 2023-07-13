// @generated
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ShellRequest {
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub data_in: ::prost::alloc::string::String,
    /// Additional arguments to the method
    #[prost(message, optional, tag="99")]
    pub extra: ::core::option::Option<::prost_types::Struct>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ShellResponse {
    #[prost(string, tag="1")]
    pub data_out: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub data_err: ::prost::alloc::string::String,
    #[prost(bool, tag="3")]
    pub eof: bool,
}
// @@protoc_insertion_point(module)
