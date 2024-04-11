// @generated
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetSmartMachineStatusRequest {
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetSmartMachineStatusResponse {
    #[prost(message, optional, tag="1")]
    pub provisioning_info: ::core::option::Option<ProvisioningInfo>,
    #[prost(bool, tag="2")]
    pub has_smart_machine_credentials: bool,
    #[prost(bool, tag="3")]
    pub is_online: bool,
    #[prost(message, optional, tag="4")]
    pub latest_connection_attempt: ::core::option::Option<NetworkInfo>,
    #[prost(string, repeated, tag="5")]
    pub errors: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetNetworkCredentialsRequest {
    #[prost(string, tag="1")]
    pub r#type: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub ssid: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub psk: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetNetworkCredentialsResponse {
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetSmartMachineCredentialsRequest {
    #[prost(message, optional, tag="1")]
    pub cloud: ::core::option::Option<CloudConfig>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetSmartMachineCredentialsResponse {
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetNetworkListRequest {
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetNetworkListResponse {
    #[prost(message, repeated, tag="1")]
    pub networks: ::prost::alloc::vec::Vec<NetworkInfo>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProvisioningInfo {
    #[prost(string, tag="1")]
    pub fragment_id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub model: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub manufacturer: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NetworkInfo {
    #[prost(string, tag="1")]
    pub r#type: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub ssid: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub security: ::prost::alloc::string::String,
    #[prost(int32, tag="4")]
    pub signal: i32,
    #[prost(bool, tag="5")]
    pub connected: bool,
    #[prost(string, tag="6")]
    pub last_error: ::prost::alloc::string::String,
}
/// minimal CloudConfig to create /etc/viam.json
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CloudConfig {
    /// SmartMachine part id
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    /// SmartMachine part secret
    #[prost(string, tag="2")]
    pub secret: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub app_address: ::prost::alloc::string::String,
}
// @@protoc_insertion_point(module)
