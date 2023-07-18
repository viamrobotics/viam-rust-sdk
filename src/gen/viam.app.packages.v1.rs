// @generated
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FileInfo {
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    #[prost(uint64, tag="2")]
    pub size: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PackageInfo {
    #[prost(string, tag="1")]
    pub organization_id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub version: ::prost::alloc::string::String,
    #[prost(enumeration="PackageType", tag="4")]
    pub r#type: i32,
    #[prost(string, optional, tag="7")]
    pub platform: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(message, repeated, tag="5")]
    pub files: ::prost::alloc::vec::Vec<FileInfo>,
    #[prost(message, optional, tag="6")]
    pub metadata: ::core::option::Option<::prost_types::Struct>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreatePackageRequest {
    #[prost(oneof="create_package_request::Package", tags="1, 2")]
    pub package: ::core::option::Option<create_package_request::Package>,
}
/// Nested message and enum types in `CreatePackageRequest`.
pub mod create_package_request {
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Package {
        #[prost(message, tag="1")]
        Info(super::PackageInfo),
        /// .tar.gz file
        #[prost(bytes, tag="2")]
        Contents(::prost::alloc::vec::Vec<u8>),
    }
}
/// Returns the package ID and version which are populated in GetPackageRequest and DeletePackageRequest to
/// retrieve or delete this package.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreatePackageResponse {
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub version: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeletePackageRequest {
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub version: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeletePackageResponse {
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Package {
    #[prost(message, optional, tag="1")]
    pub info: ::core::option::Option<PackageInfo>,
    #[prost(string, tag="2")]
    pub url: ::prost::alloc::string::String,
    #[prost(message, optional, tag="3")]
    pub created_on: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(string, tag="4")]
    pub checksum: ::prost::alloc::string::String,
    #[prost(string, tag="5")]
    pub id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetPackageRequest {
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub version: ::prost::alloc::string::String,
    #[prost(bool, optional, tag="3")]
    pub include_url: ::core::option::Option<bool>,
    #[prost(enumeration="PackageType", optional, tag="4")]
    pub r#type: ::core::option::Option<i32>,
    #[prost(string, optional, tag="5")]
    pub platform: ::core::option::Option<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetPackageResponse {
    #[prost(message, optional, tag="1")]
    pub package: ::core::option::Option<Package>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListPackagesRequest {
    #[prost(string, tag="1")]
    pub organization_id: ::prost::alloc::string::String,
    #[prost(string, optional, tag="2")]
    pub name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="3")]
    pub version: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(enumeration="PackageType", optional, tag="4")]
    pub r#type: ::core::option::Option<i32>,
    #[prost(bool, optional, tag="5")]
    pub include_url: ::core::option::Option<bool>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListPackagesResponse {
    #[prost(message, repeated, tag="1")]
    pub packages: ::prost::alloc::vec::Vec<Package>,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum PackageType {
    Unspecified = 0,
    Archive = 1,
    MlModel = 2,
    Module = 3,
    SlamMap = 4,
}
impl PackageType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            PackageType::Unspecified => "PACKAGE_TYPE_UNSPECIFIED",
            PackageType::Archive => "PACKAGE_TYPE_ARCHIVE",
            PackageType::MlModel => "PACKAGE_TYPE_ML_MODEL",
            PackageType::Module => "PACKAGE_TYPE_MODULE",
            PackageType::SlamMap => "PACKAGE_TYPE_SLAM_MAP",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "PACKAGE_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
            "PACKAGE_TYPE_ARCHIVE" => Some(Self::Archive),
            "PACKAGE_TYPE_ML_MODEL" => Some(Self::MlModel),
            "PACKAGE_TYPE_MODULE" => Some(Self::Module),
            "PACKAGE_TYPE_SLAM_MAP" => Some(Self::SlamMap),
            _ => None,
        }
    }
}
// @@protoc_insertion_point(module)
