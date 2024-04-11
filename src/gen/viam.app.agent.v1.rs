// @generated
/// App side
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetAgentConfigRequest {
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetAgentConfigResponse {
    #[prost(message, optional, tag="1")]
    pub agent_config: ::core::option::Option<AppAgentConfig>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateAgentConfigRequest {
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub agent_config: ::core::option::Option<AppAgentConfig>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateAgentConfigResponse {
    #[prost(message, optional, tag="1")]
    pub agent_config: ::core::option::Option<AppAgentConfig>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AppAgentConfig {
    #[prost(map="string, message", tag="1")]
    pub subsystem_configs: ::std::collections::HashMap<::prost::alloc::string::String, AppSubsystemConfig>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AppSubsystemConfig {
    #[prost(string, tag="1")]
    pub release_channel: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub pin_version: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub pin_url: ::prost::alloc::string::String,
    #[prost(bool, tag="4")]
    pub disable_subsystem: bool,
    #[prost(message, optional, tag="5")]
    pub attributes: ::core::option::Option<::prost_types::Struct>,
}
/// Device side
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeviceAgentConfigRequest {
    /// robot partID
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    /// info about the host system
    #[prost(message, optional, tag="2")]
    pub host_info: ::core::option::Option<HostInfo>,
    /// current subsystems and versions
    #[prost(map="string, string", tag="3")]
    pub subsystem_versions: ::std::collections::HashMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeviceAgentConfigResponse {
    /// subsystems to be installed/configured/updated
    /// note: previously installed subsystems will be removed from the system if removed from this list
    #[prost(map="string, message", tag="1")]
    pub subsystem_configs: ::std::collections::HashMap<::prost::alloc::string::String, DeviceSubsystemConfig>,
    /// how often this request should be repeated
    #[prost(message, optional, tag="2")]
    pub check_interval: ::core::option::Option<::prost_types::Duration>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeviceSubsystemConfig {
    /// data needed to download/validate the subsystem
    #[prost(message, optional, tag="1")]
    pub update_info: ::core::option::Option<SubsystemUpdateInfo>,
    /// if this subsystem is disabled and should not be started by the agent
    #[prost(bool, tag="2")]
    pub disable: bool,
    /// force_restart will restart the subsystem, even if no updates are available
    #[prost(bool, tag="3")]
    pub force_restart: bool,
    /// arbitrary config sections
    #[prost(message, optional, tag="4")]
    pub attributes: ::core::option::Option<::prost_types::Struct>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HostInfo {
    /// platform is the docker styled combination of kernel and architecture. Ex: linux/amd64, darwin/arm64
    #[prost(string, tag="1")]
    pub platform: ::prost::alloc::string::String,
    /// ID and VERSION_ID fields from /etc/os-release, colon seperated. Ex: ubuntu:22.04, debian:11
    #[prost(string, tag="2")]
    pub distro: ::prost::alloc::string::String,
    /// additional tags for specific hardware or software that's present and may affect software selection
    /// ex: "jetson", "rpi4", "systemd", etc.
    #[prost(string, repeated, tag="3")]
    pub tags: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SubsystemUpdateInfo {
    /// unpacked filename as it is expected on disk (regardless of url)
    #[prost(string, tag="1")]
    pub filename: ::prost::alloc::string::String,
    /// url to download from
    #[prost(string, tag="2")]
    pub url: ::prost::alloc::string::String,
    /// version expected at the url
    #[prost(string, tag="3")]
    pub version: ::prost::alloc::string::String,
    /// sha256 sum of file as downloaded
    #[prost(bytes="vec", tag="4")]
    pub sha256: ::prost::alloc::vec::Vec<u8>,
    /// determines if decompression or executable permissions are needed
    #[prost(enumeration="PackageFormat", tag="5")]
    pub format: i32,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum PackageFormat {
    /// unknown/unset (autodetection may be attempted)
    Unspecified = 0,
    /// do nothing
    Raw = 1,
    /// decompress .xz file
    Xz = 2,
    /// set executable permissions
    Executable = 3,
    /// decompress and set executable
    XzExecutable = 4,
}
impl PackageFormat {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            PackageFormat::Unspecified => "PACKAGE_FORMAT_UNSPECIFIED",
            PackageFormat::Raw => "PACKAGE_FORMAT_RAW",
            PackageFormat::Xz => "PACKAGE_FORMAT_XZ",
            PackageFormat::Executable => "PACKAGE_FORMAT_EXECUTABLE",
            PackageFormat::XzExecutable => "PACKAGE_FORMAT_XZ_EXECUTABLE",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "PACKAGE_FORMAT_UNSPECIFIED" => Some(Self::Unspecified),
            "PACKAGE_FORMAT_RAW" => Some(Self::Raw),
            "PACKAGE_FORMAT_XZ" => Some(Self::Xz),
            "PACKAGE_FORMAT_EXECUTABLE" => Some(Self::Executable),
            "PACKAGE_FORMAT_XZ_EXECUTABLE" => Some(Self::XzExecutable),
            _ => None,
        }
    }
}
// @@protoc_insertion_point(module)
