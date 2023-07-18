// @generated
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetPosesRequest {
    /// Name of the pose tracker
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// Names of the bodies whose poses are being requested. In the event
    /// this parameter is not supplied or is an empty list, all available
    /// poses are returned
    #[prost(string, repeated, tag="2")]
    pub body_names: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Additional arguments to the method
    #[prost(message, optional, tag="99")]
    pub extra: ::core::option::Option<::prost_types::Struct>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetPosesResponse {
    /// Mapping of each body name to the pose representing the center of the body.
    #[prost(map="string, message", tag="1")]
    pub body_poses: ::std::collections::HashMap<::prost::alloc::string::String, super::super::super::common::v1::PoseInFrame>,
}
// @@protoc_insertion_point(module)
