// @generated
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Robot {
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub location: ::prost::alloc::string::String,
    #[prost(message, optional, tag="4")]
    pub last_access: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(message, optional, tag="5")]
    pub created_on: ::core::option::Option<::prost_types::Timestamp>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RobotPart {
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub name: ::prost::alloc::string::String,
    /// dns_name part name used for fqdn and local fqdn. Anytime the Name is updated this should be sanitized and updated as well.
    #[prost(string, tag="10")]
    pub dns_name: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub secret: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub robot: ::prost::alloc::string::String,
    /// Store the location_id to allow for unique indexes across parts and locations. This filed MUST be updated each time the robots location
    /// changes.
    #[prost(string, tag="12")]
    pub location_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag="5")]
    pub robot_config: ::core::option::Option<::prost_types::Struct>,
    #[prost(message, optional, tag="6")]
    pub last_access: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(message, optional, tag="7")]
    pub user_supplied_info: ::core::option::Option<::prost_types::Struct>,
    #[prost(bool, tag="8")]
    pub main_part: bool,
    #[prost(string, tag="9")]
    pub fqdn: ::prost::alloc::string::String,
    #[prost(string, tag="11")]
    pub local_fqdn: ::prost::alloc::string::String,
    #[prost(message, optional, tag="13")]
    pub created_on: ::core::option::Option<::prost_types::Timestamp>,
    /// List of secrets allowed for authentication.
    #[prost(message, repeated, tag="14")]
    pub secrets: ::prost::alloc::vec::Vec<SharedSecret>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RobotPartHistoryEntry {
    #[prost(string, tag="1")]
    pub part: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub robot: ::prost::alloc::string::String,
    #[prost(message, optional, tag="3")]
    pub when: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(message, optional, tag="4")]
    pub old: ::core::option::Option<RobotPart>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListOrganizationsRequest {
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Organization {
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub name: ::prost::alloc::string::String,
    #[prost(message, optional, tag="3")]
    pub created_on: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(string, tag="4")]
    pub public_namespace: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OrganizationMember {
    #[prost(string, tag="1")]
    pub user_id: ::prost::alloc::string::String,
    #[prost(string, repeated, tag="2")]
    pub emails: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(message, optional, tag="3")]
    pub date_added: ::core::option::Option<::prost_types::Timestamp>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListOrganizationsResponse {
    #[prost(message, repeated, tag="1")]
    pub organizations: ::prost::alloc::vec::Vec<Organization>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OrganizationInvite {
    #[prost(string, tag="1")]
    pub organization_id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub email: ::prost::alloc::string::String,
    #[prost(message, optional, tag="3")]
    pub created_on: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(int64, tag="4")]
    pub robot_count: i64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateOrganizationRequest {
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateOrganizationResponse {
    #[prost(message, optional, tag="1")]
    pub organization: ::core::option::Option<Organization>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetOrganizationRequest {
    #[prost(string, tag="1")]
    pub organization_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetOrganizationResponse {
    #[prost(message, optional, tag="1")]
    pub organization: ::core::option::Option<Organization>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateOrganizationRequest {
    #[prost(string, tag="1")]
    pub organization_id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, optional, tag="3")]
    pub public_namespace: ::core::option::Option<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateOrganizationResponse {
    #[prost(message, optional, tag="1")]
    pub organization: ::core::option::Option<Organization>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteOrganizationRequest {
    #[prost(string, tag="1")]
    pub organization_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteOrganizationResponse {
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListOrganizationMembersRequest {
    #[prost(string, tag="1")]
    pub organization_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListOrganizationMembersResponse {
    #[prost(string, tag="1")]
    pub organization_id: ::prost::alloc::string::String,
    #[prost(message, repeated, tag="2")]
    pub members: ::prost::alloc::vec::Vec<OrganizationMember>,
    #[prost(message, repeated, tag="3")]
    pub invites: ::prost::alloc::vec::Vec<OrganizationInvite>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateOrganizationInviteRequest {
    #[prost(string, tag="1")]
    pub organization_id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub email: ::prost::alloc::string::String,
    #[prost(message, repeated, tag="3")]
    pub authorizations: ::prost::alloc::vec::Vec<Authorization>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateOrganizationInviteResponse {
    #[prost(message, optional, tag="1")]
    pub invite: ::core::option::Option<OrganizationInvite>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteOrganizationInviteRequest {
    #[prost(string, tag="1")]
    pub organization_id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub email: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteOrganizationInviteResponse {
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResendOrganizationInviteRequest {
    #[prost(string, tag="1")]
    pub organization_id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub email: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResendOrganizationInviteResponse {
    #[prost(message, optional, tag="1")]
    pub invite: ::core::option::Option<OrganizationInvite>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteOrganizationMemberRequest {
    #[prost(string, tag="1")]
    pub organization_id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub user_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteOrganizationMemberResponse {
}
// Location
//

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LocationOrganization {
    /// Organization ID the location is shared with.
    #[prost(string, tag="1")]
    pub organization_id: ::prost::alloc::string::String,
    /// Whether the organization is the primary owner or not.
    #[prost(bool, tag="2")]
    pub primary: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LocationAuth {
    /// Deprecated: use secrets field.
    #[deprecated]
    #[prost(string, tag="1")]
    pub secret: ::prost::alloc::string::String,
    /// Location ID containing this LocationAuth.
    #[prost(string, tag="2")]
    pub location_id: ::prost::alloc::string::String,
    /// List of secrets used to authenticate to the Location.
    #[prost(message, repeated, tag="3")]
    pub secrets: ::prost::alloc::vec::Vec<SharedSecret>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Location {
    /// Location ID.
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    /// Location name.
    #[prost(string, tag="2")]
    pub name: ::prost::alloc::string::String,
    /// Location ID of the parent location.
    #[prost(string, tag="4")]
    pub parent_location_id: ::prost::alloc::string::String,
    /// Location authentication secrets.
    #[prost(message, optional, tag="5")]
    pub auth: ::core::option::Option<LocationAuth>,
    /// Organizations that the location is shared with.
    #[prost(message, repeated, tag="6")]
    pub organizations: ::prost::alloc::vec::Vec<LocationOrganization>,
    /// Location creation timestamp.
    #[prost(message, optional, tag="3")]
    pub created_on: ::core::option::Option<::prost_types::Timestamp>,
    ///
    #[prost(int32, tag="7")]
    pub robot_count: i32,
}
/// SharedSecret is a secret used for LocationAuth and RobotParts.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SharedSecret {
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    /// The payload of the secret. Used during authentication to the rpc framework.
    #[prost(string, tag="2")]
    pub secret: ::prost::alloc::string::String,
    /// Date/time the secret was first created.
    #[prost(message, optional, tag="3")]
    pub created_on: ::core::option::Option<::prost_types::Timestamp>,
    /// State of the shared secret. In most cases it should be enabled. We may support
    /// disabling a specific secret while keeping it in the database.
    #[prost(enumeration="shared_secret::State", tag="4")]
    pub state: i32,
}
/// Nested message and enum types in `SharedSecret`.
pub mod shared_secret {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum State {
        Unspecified = 0,
        /// Secret is enabled and can be used in authentication.
        Enabled = 1,
        /// Secret is disabled and must not be used to authenticate to rpc.
        Disabled = 2,
    }
    impl State {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                State::Unspecified => "STATE_UNSPECIFIED",
                State::Enabled => "STATE_ENABLED",
                State::Disabled => "STATE_DISABLED",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "STATE_UNSPECIFIED" => Some(Self::Unspecified),
                "STATE_ENABLED" => Some(Self::Enabled),
                "STATE_DISABLED" => Some(Self::Disabled),
                _ => None,
            }
        }
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateLocationRequest {
    /// Organization ID to create the location under.
    #[prost(string, tag="1")]
    pub organization_id: ::prost::alloc::string::String,
    /// Name of the location.
    #[prost(string, tag="2")]
    pub name: ::prost::alloc::string::String,
    /// The new parent location to move the location under.
    #[prost(string, optional, tag="3")]
    pub parent_location_id: ::core::option::Option<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateLocationResponse {
    /// Location object is returned.
    #[prost(message, optional, tag="1")]
    pub location: ::core::option::Option<Location>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetLocationRequest {
    /// Location ID of location to get.
    #[prost(string, tag="1")]
    pub location_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetLocationResponse {
    /// Location object is returned.
    #[prost(message, optional, tag="1")]
    pub location: ::core::option::Option<Location>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateLocationRequest {
    /// Location ID of location to update.
    #[prost(string, tag="1")]
    pub location_id: ::prost::alloc::string::String,
    /// The new to be updated on location.
    #[prost(string, optional, tag="2")]
    pub name: ::core::option::Option<::prost::alloc::string::String>,
    /// The new parent location to move the location under.
    #[prost(string, optional, tag="3")]
    pub parent_location_id: ::core::option::Option<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateLocationResponse {
    /// Location object is returned.
    #[prost(message, optional, tag="1")]
    pub location: ::core::option::Option<Location>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteLocationRequest {
    /// Location ID of location to delete.
    #[prost(string, tag="1")]
    pub location_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteLocationResponse {
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListLocationsRequest {
    /// Organization ID under which to list all locations.
    #[prost(string, tag="1")]
    pub organization_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ShareLocationRequest {
    /// Location ID to be shared.
    #[prost(string, tag="1")]
    pub location_id: ::prost::alloc::string::String,
    /// Organization ID to share the location with.
    #[prost(string, tag="2")]
    pub organization_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ShareLocationResponse {
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UnshareLocationRequest {
    /// Location ID to be unshared.
    #[prost(string, tag="1")]
    pub location_id: ::prost::alloc::string::String,
    /// Organization ID to unshare the location with.
    #[prost(string, tag="2")]
    pub organization_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UnshareLocationResponse {
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListLocationsResponse {
    #[prost(message, repeated, tag="1")]
    pub locations: ::prost::alloc::vec::Vec<Location>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateLocationSecretRequest {
    /// Location ID to create the secret in.
    #[prost(string, tag="1")]
    pub location_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateLocationSecretResponse {
    /// Location's auth after updates.
    #[prost(message, optional, tag="1")]
    pub auth: ::core::option::Option<LocationAuth>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteLocationSecretRequest {
    #[prost(string, tag="1")]
    pub location_id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub secret_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteLocationSecretResponse {
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LocationAuthRequest {
    #[prost(string, tag="1")]
    pub location_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LocationAuthResponse {
    #[prost(message, optional, tag="1")]
    pub auth: ::core::option::Option<LocationAuth>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetRobotRequest {
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetRoverRentalRobotsRequest {
    #[prost(string, tag="1")]
    pub org_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RoverRentalRobot {
    #[prost(string, tag="1")]
    pub robot_id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub location_id: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub robot_name: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub robot_main_part_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetRoverRentalRobotsResponse {
    #[prost(message, repeated, tag="1")]
    pub robots: ::prost::alloc::vec::Vec<RoverRentalRobot>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetRobotResponse {
    #[prost(message, optional, tag="1")]
    pub robot: ::core::option::Option<Robot>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetRobotPartsRequest {
    #[prost(string, tag="1")]
    pub robot_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetRobotPartsResponse {
    #[prost(message, repeated, tag="1")]
    pub parts: ::prost::alloc::vec::Vec<RobotPart>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetRobotPartRequest {
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetRobotPartResponse {
    #[prost(message, optional, tag="1")]
    pub part: ::core::option::Option<RobotPart>,
    #[prost(string, tag="2")]
    pub config_json: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetRobotPartLogsRequest {
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    #[prost(bool, tag="2")]
    pub errors_only: bool,
    #[prost(string, optional, tag="3")]
    pub filter: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag="4")]
    pub page_token: ::core::option::Option<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LogEntry {
    #[prost(string, tag="1")]
    pub host: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub level: ::prost::alloc::string::String,
    #[prost(message, optional, tag="3")]
    pub time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(string, tag="4")]
    pub logger_name: ::prost::alloc::string::String,
    #[prost(string, tag="5")]
    pub message: ::prost::alloc::string::String,
    #[prost(message, optional, tag="6")]
    pub caller: ::core::option::Option<::prost_types::Struct>,
    #[prost(string, tag="7")]
    pub stack: ::prost::alloc::string::String,
    #[prost(message, repeated, tag="8")]
    pub fields: ::prost::alloc::vec::Vec<::prost_types::Struct>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetRobotPartLogsResponse {
    #[prost(message, repeated, tag="1")]
    pub logs: ::prost::alloc::vec::Vec<LogEntry>,
    #[prost(string, tag="2")]
    pub next_page_token: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TailRobotPartLogsRequest {
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    #[prost(bool, tag="2")]
    pub errors_only: bool,
    #[prost(string, optional, tag="3")]
    pub filter: ::core::option::Option<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TailRobotPartLogsResponse {
    #[prost(message, repeated, tag="1")]
    pub logs: ::prost::alloc::vec::Vec<LogEntry>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetRobotPartHistoryRequest {
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetRobotPartHistoryResponse {
    #[prost(message, repeated, tag="1")]
    pub history: ::prost::alloc::vec::Vec<RobotPartHistoryEntry>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateRobotPartRequest {
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub name: ::prost::alloc::string::String,
    #[prost(message, optional, tag="3")]
    pub robot_config: ::core::option::Option<::prost_types::Struct>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateRobotPartResponse {
    #[prost(message, optional, tag="1")]
    pub part: ::core::option::Option<RobotPart>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NewRobotPartRequest {
    #[prost(string, tag="1")]
    pub robot_id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub part_name: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NewRobotPartResponse {
    #[prost(string, tag="1")]
    pub part_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteRobotPartRequest {
    #[prost(string, tag="1")]
    pub part_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteRobotPartResponse {
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Fragment {
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub name: ::prost::alloc::string::String,
    #[prost(message, optional, tag="3")]
    pub fragment: ::core::option::Option<::prost_types::Struct>,
    #[prost(string, tag="4")]
    pub organization_owner: ::prost::alloc::string::String,
    #[prost(bool, tag="5")]
    pub public: bool,
    #[prost(message, optional, tag="6")]
    pub created_on: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(string, tag="7")]
    pub organization_name: ::prost::alloc::string::String,
    /// number of robot parts using this fragment
    #[prost(int32, tag="9")]
    pub robot_part_count: i32,
    /// number of organizations using this fragment
    #[prost(int32, tag="10")]
    pub organization_count: i32,
    /// whether the organization(s) using this fragment is the same as the fragment org
    #[prost(bool, tag="11")]
    pub only_used_by_owner: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListFragmentsRequest {
    #[prost(string, tag="1")]
    pub organization_id: ::prost::alloc::string::String,
    #[prost(bool, tag="2")]
    pub show_public: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListFragmentsResponse {
    #[prost(message, repeated, tag="1")]
    pub fragments: ::prost::alloc::vec::Vec<Fragment>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetFragmentRequest {
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetFragmentResponse {
    #[prost(message, optional, tag="1")]
    pub fragment: ::core::option::Option<Fragment>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateFragmentRequest {
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub config: ::core::option::Option<::prost_types::Struct>,
    #[prost(string, tag="3")]
    pub organization_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateFragmentResponse {
    #[prost(message, optional, tag="1")]
    pub fragment: ::core::option::Option<Fragment>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateFragmentRequest {
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub name: ::prost::alloc::string::String,
    #[prost(message, optional, tag="3")]
    pub config: ::core::option::Option<::prost_types::Struct>,
    #[prost(bool, optional, tag="4")]
    pub public: ::core::option::Option<bool>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateFragmentResponse {
    #[prost(message, optional, tag="1")]
    pub fragment: ::core::option::Option<Fragment>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteFragmentRequest {
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteFragmentResponse {
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListRobotsRequest {
    #[prost(string, tag="1")]
    pub location_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListRobotsResponse {
    #[prost(message, repeated, tag="1")]
    pub robots: ::prost::alloc::vec::Vec<Robot>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NewRobotRequest {
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub location: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NewRobotResponse {
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateRobotRequest {
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub location: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateRobotResponse {
    #[prost(message, optional, tag="1")]
    pub robot: ::core::option::Option<Robot>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteRobotRequest {
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteRobotResponse {
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MarkPartAsMainRequest {
    #[prost(string, tag="1")]
    pub part_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MarkPartAsMainResponse {
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MarkPartForRestartRequest {
    #[prost(string, tag="1")]
    pub part_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MarkPartForRestartResponse {
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateRobotPartSecretRequest {
    /// Robot Part ID to create the secret in.
    #[prost(string, tag="1")]
    pub part_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateRobotPartSecretResponse {
    /// Location's auth after updates.
    #[prost(message, optional, tag="1")]
    pub part: ::core::option::Option<RobotPart>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteRobotPartSecretRequest {
    #[prost(string, tag="1")]
    pub part_id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub secret_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteRobotPartSecretResponse {
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Authorization {
    #[prost(string, tag="1")]
    pub authorization_type: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub authorization_id: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub resource_type: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub resource_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddRoleRequest {
    #[prost(string, tag="1")]
    pub identity_id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub role_id: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub resource_type: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub resource_id: ::prost::alloc::string::String,
    #[prost(string, tag="5")]
    pub organization_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddRoleResponse {
    #[prost(message, optional, tag="1")]
    pub authorization: ::core::option::Option<Authorization>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveRoleRequest {
    #[prost(string, tag="1")]
    pub identity_id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub role_id: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub resource_type: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub resource_id: ::prost::alloc::string::String,
    #[prost(string, tag="5")]
    pub organization_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveRoleResponse {
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListAuthorizationsRequest {
    #[prost(string, tag="1")]
    pub resource_type: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub resource_id: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub organization_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Authorizations {
    #[prost(string, tag="1")]
    pub identity_id: ::prost::alloc::string::String,
    #[prost(message, repeated, tag="2")]
    pub authorizations: ::prost::alloc::vec::Vec<Authorization>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListAuthorizationsResponse {
    #[prost(message, repeated, tag="1")]
    pub authorizations: ::prost::alloc::vec::Vec<Authorizations>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CheckPermissionsRequest {
    #[prost(message, repeated, tag="1")]
    pub permissions: ::prost::alloc::vec::Vec<AuthorizedPermissions>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AuthorizedPermissions {
    #[prost(string, tag="1")]
    pub resource_type: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub resource_id: ::prost::alloc::string::String,
    #[prost(string, repeated, tag="3")]
    pub permissions: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CheckPermissionsResponse {
    #[prost(message, repeated, tag="1")]
    pub authorized_permissions: ::prost::alloc::vec::Vec<AuthorizedPermissions>,
}
/// Modules
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateModuleRequest {
    /// The organization to create the module under
    #[prost(string, tag="1")]
    pub organization_id: ::prost::alloc::string::String,
    /// The name of the module, which must be unique within your org
    #[prost(string, tag="2")]
    pub name: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateModuleResponse {
    /// The id of the module containing the namespace and name
    #[prost(string, tag="1")]
    pub module_id: ::prost::alloc::string::String,
    /// The detail page of the module
    #[prost(string, tag="2")]
    pub url: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateModuleRequest {
    /// The id of the module being updated, containing module name or namespace and module name
    #[prost(string, tag="1")]
    pub module_id: ::prost::alloc::string::String,
    /// The organization of the module being updated, required if no namespace exists in the module_id
    #[prost(string, optional, tag="7")]
    pub organization_id: ::core::option::Option<::prost::alloc::string::String>,
    /// The visibility that should be set for the module
    #[prost(enumeration="Visibility", tag="2")]
    pub visibility: i32,
    /// The url to reference for documentation, code, etc.
    #[prost(string, tag="3")]
    pub url: ::prost::alloc::string::String,
    /// A short description of the module that explains its purpose
    #[prost(string, tag="4")]
    pub description: ::prost::alloc::string::String,
    /// A list of models that are available in the module
    #[prost(message, repeated, tag="5")]
    pub models: ::prost::alloc::vec::Vec<Model>,
    /// The executable to run to start the module program
    #[prost(string, tag="6")]
    pub entrypoint: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateModuleResponse {
    /// The detail page of the module
    #[prost(string, tag="1")]
    pub url: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Model {
    /// The colon-delimited-triplet of the api implemented by the model
    #[prost(string, tag="1")]
    pub api: ::prost::alloc::string::String,
    /// The colon-delimited-triplet of the model
    #[prost(string, tag="2")]
    pub model: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ModuleFileInfo {
    /// The id of the module being uploaded, containing module name or namespace and module name
    #[prost(string, tag="1")]
    pub module_id: ::prost::alloc::string::String,
    /// The organization of the module being updated, required if no namespace exists in the module_id
    #[prost(string, optional, tag="4")]
    pub organization_id: ::core::option::Option<::prost::alloc::string::String>,
    /// The semver string that represents the new major/minor/patch version of the module
    #[prost(string, tag="2")]
    pub version: ::prost::alloc::string::String,
    /// The platform that the file is built to run on
    #[prost(string, tag="3")]
    pub platform: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UploadModuleFileRequest {
    #[prost(oneof="upload_module_file_request::ModuleFile", tags="1, 2")]
    pub module_file: ::core::option::Option<upload_module_file_request::ModuleFile>,
}
/// Nested message and enum types in `UploadModuleFileRequest`.
pub mod upload_module_file_request {
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum ModuleFile {
        /// The information about the module file being uploaded
        #[prost(message, tag="1")]
        ModuleFileInfo(super::ModuleFileInfo),
        /// The file contents to be uploaded
        #[prost(bytes, tag="2")]
        File(::prost::alloc::vec::Vec<u8>),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UploadModuleFileResponse {
    /// The detail page of the module
    #[prost(string, tag="1")]
    pub url: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetModuleRequest {
    /// The id of the module being retrieved, containing module name or namespace and module name
    #[prost(string, tag="1")]
    pub module_id: ::prost::alloc::string::String,
    /// The organization of the module being updated, required if no namespace exists in the module_id
    #[prost(string, optional, tag="2")]
    pub organization_id: ::core::option::Option<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetModuleResponse {
    /// The module object
    #[prost(message, optional, tag="1")]
    pub module: ::core::option::Option<Module>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Module {
    /// The id of the module, containing module name or namespace and module name
    #[prost(string, tag="1")]
    pub module_id: ::prost::alloc::string::String,
    /// The id of the organization that owns the module
    #[prost(string, tag="10")]
    pub organization_id: ::prost::alloc::string::String,
    /// The name of the module
    #[prost(string, tag="2")]
    pub name: ::prost::alloc::string::String,
    /// The visibility of the module
    #[prost(enumeration="Visibility", tag="3")]
    pub visibility: i32,
    /// The versions of the module that are available
    #[prost(message, repeated, tag="4")]
    pub versions: ::prost::alloc::vec::Vec<VersionHistory>,
    /// The url to reference for documentation, code, etc.
    #[prost(string, tag="5")]
    pub url: ::prost::alloc::string::String,
    /// A short description of the module that explains its purpose
    #[prost(string, tag="6")]
    pub description: ::prost::alloc::string::String,
    /// A list of models that are available in the module
    #[prost(message, repeated, tag="7")]
    pub models: ::prost::alloc::vec::Vec<Model>,
    /// The executable to run to start the module program
    #[prost(string, tag="11")]
    pub entrypoint: ::prost::alloc::string::String,
    /// The total number of robots using this module
    #[prost(int64, tag="8")]
    pub total_robot_usage: i64,
    /// The total number of organizations using this module
    #[prost(int64, tag="9")]
    pub total_organization_usage: i64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VersionHistory {
    /// The semver string that represents the major/minor/patch version of the module
    #[prost(string, tag="1")]
    pub version: ::prost::alloc::string::String,
    /// The uploads that are available for this module version
    #[prost(message, repeated, tag="2")]
    pub files: ::prost::alloc::vec::Vec<Uploads>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Uploads {
    /// The OS and architecture the module is built to run on
    #[prost(string, tag="1")]
    pub platform: ::prost::alloc::string::String,
    /// The time when the file was uploaded
    #[prost(message, optional, tag="2")]
    pub uploaded_at: ::core::option::Option<::prost_types::Timestamp>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListModulesRequest {
    /// The id of the organization to return private modules for.
    #[prost(string, optional, tag="1")]
    pub organization_id: ::core::option::Option<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListModulesResponse {
    /// A listed of modules. When authenticated, this API will return modules that are private for this org. Public modules are always returned.
    #[prost(message, repeated, tag="1")]
    pub modules: ::prost::alloc::vec::Vec<Module>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetUserIdByEmailRequest {
    #[prost(string, tag="1")]
    pub email: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetUserIdByEmailResponse {
    #[prost(string, tag="1")]
    pub user_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListOrganizationsByUserRequest {
    #[prost(string, tag="1")]
    pub user_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OrgDetails {
    #[prost(string, tag="1")]
    pub org_id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub org_name: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListOrganizationsByUserResponse {
    #[prost(message, repeated, tag="1")]
    pub orgs: ::prost::alloc::vec::Vec<OrgDetails>,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum Visibility {
    Unspecified = 0,
    /// Private modules are visible only within your org
    Private = 1,
    /// Public modules are visible to everyone
    Public = 2,
}
impl Visibility {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Visibility::Unspecified => "VISIBILITY_UNSPECIFIED",
            Visibility::Private => "VISIBILITY_PRIVATE",
            Visibility::Public => "VISIBILITY_PUBLIC",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "VISIBILITY_UNSPECIFIED" => Some(Self::Unspecified),
            "VISIBILITY_PRIVATE" => Some(Self::Private),
            "VISIBILITY_PUBLIC" => Some(Self::Public),
            _ => None,
        }
    }
}
/// TODO(APP-1865) should be deprecated/removed in favor of GetCurrentMonthUsage
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CurrentMonthUsageSummary {
    #[prost(double, tag="1")]
    pub cloud_storage_usage: f64,
    #[prost(double, tag="2")]
    pub cloud_storage_usage_cost: f64,
    #[prost(double, tag="3")]
    pub data_upload_usage_cost: f64,
    #[prost(double, tag="4")]
    pub data_upload_usage_quantity: f64,
    #[prost(double, tag="5")]
    pub data_egres_usage_cost: f64,
    #[prost(double, tag="6")]
    pub data_egres_usage_quantity: f64,
    #[prost(double, tag="7")]
    pub standard_compute_usage_cost: f64,
    #[prost(double, tag="8")]
    pub standard_compute_usage_quantity: f64,
    #[prost(double, tag="9")]
    pub total_usage_quantity: f64,
    /// returns amt with any discounts applied
    #[prost(double, tag="10")]
    pub total_usage_with_discount: f64,
    /// returns amt without any discounts applied
    #[prost(double, tag="11")]
    pub total_usage_without_discount: f64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InvoiceSummary {
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub invoice_date: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(double, tag="3")]
    pub invoice_amount: f64,
    #[prost(string, tag="4")]
    pub status: ::prost::alloc::string::String,
    #[prost(message, optional, tag="5")]
    pub due_date: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(message, optional, tag="6")]
    pub paid_date: ::core::option::Option<::prost_types::Timestamp>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BillableResourceEvent {
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub r#type: ::prost::alloc::string::String,
    #[prost(double, tag="3")]
    pub usage_quantity: f64,
    #[prost(string, tag="4")]
    pub usage_quantity_unit: ::prost::alloc::string::String,
    #[prost(string, tag="5")]
    pub usage_cost: ::prost::alloc::string::String,
    #[prost(message, optional, tag="6")]
    pub occurred_at: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(string, tag="7")]
    pub user_name: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Invoice {
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub invoice_date: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(double, tag="3")]
    pub invoice_amount: f64,
    #[prost(string, tag="4")]
    pub status: ::prost::alloc::string::String,
    #[prost(message, optional, tag="5")]
    pub due_date: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(message, repeated, tag="6")]
    pub items: ::prost::alloc::vec::Vec<BillableResourceEvent>,
    #[prost(string, tag="7")]
    pub emailed_to: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PaymentMethodCard {
    #[prost(string, tag="1")]
    pub brand: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub last_four_digits: ::prost::alloc::string::String,
}
/// TODO(APP-1865) should be deprecated/removed in favor of GetCurrentMonthUsage
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetCurrentMonthUsageSummaryRequest {
    #[prost(string, tag="1")]
    pub org_id: ::prost::alloc::string::String,
}
/// TODO(APP-1865) should be deprecated/removed in favor of GetCurrentMonthUsage
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetCurrentMonthUsageSummaryResponse {
    #[prost(double, tag="1")]
    pub cloud_storage_usage: f64,
    #[prost(double, tag="2")]
    pub cloud_storage_usage_cost: f64,
    #[prost(double, tag="3")]
    pub data_upload_usage_cost: f64,
    #[prost(double, tag="4")]
    pub data_upload_usage_quantity: f64,
    #[prost(double, tag="5")]
    pub data_egres_usage_cost: f64,
    #[prost(double, tag="6")]
    pub data_egres_usage_quantity: f64,
    #[prost(double, tag="7")]
    pub standard_compute_usage_cost: f64,
    #[prost(double, tag="8")]
    pub standard_compute_usage_quantity: f64,
    #[prost(double, tag="9")]
    pub total_usage_quantity: f64,
    /// returns amt with any discounts applied
    #[prost(double, tag="10")]
    pub total_usage_with_discount: f64,
    /// returns amt without any discounts applied
    #[prost(double, tag="11")]
    pub total_usage_without_discount: f64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetCurrentMonthUsageRequest {
    #[prost(string, tag="1")]
    pub org_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetCurrentMonthUsageResponse {
    #[prost(message, optional, tag="1")]
    pub start_date: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(message, optional, tag="2")]
    pub end_date: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(double, tag="3")]
    pub cloud_storage_usage_cost: f64,
    #[prost(double, tag="4")]
    pub data_upload_usage_cost: f64,
    #[prost(double, tag="5")]
    pub data_egres_usage_cost: f64,
    #[prost(double, tag="6")]
    pub remote_control_usage_cost: f64,
    #[prost(double, tag="7")]
    pub standard_compute_usage_cost: f64,
    #[prost(double, tag="8")]
    pub discount_amount: f64,
    #[prost(double, tag="9")]
    pub total_usage_with_discount: f64,
    #[prost(double, tag="10")]
    pub total_usage_without_discount: f64,
}
/// TODO(APP-1865) may want to remove
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetUnpaidBalanceRequest {
    #[prost(string, tag="1")]
    pub org_id: ::prost::alloc::string::String,
}
/// TODO(APP-1865) may want to remove
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetUnpaidBalanceResponse {
    #[prost(double, tag="1")]
    pub unpaid_balance: f64,
    #[prost(message, optional, tag="2")]
    pub unpaid_balance_due_date: ::core::option::Option<::prost_types::Timestamp>,
}
/// TODO(APP-1865) may want to remove
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetInvoiceHistoryRequest {
    #[prost(string, tag="1")]
    pub org_id: ::prost::alloc::string::String,
}
/// TODO(APP-1865) may want to remove
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetInvoiceHistoryResponse {
    #[prost(message, repeated, tag="1")]
    pub invoices: ::prost::alloc::vec::Vec<InvoiceSummary>,
}
/// TODO(APP-1865) may want to remove
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetItemizedInvoiceRequest {
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
}
/// TODO(APP-1865) may want to remove
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetItemizedInvoiceResponse {
    #[prost(message, optional, tag="1")]
    pub invoice: ::core::option::Option<Invoice>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetOrgBillingInformationRequest {
    #[prost(string, tag="1")]
    pub org_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetOrgBillingInformationResponse {
    #[prost(enumeration="PaymentMethodType", tag="1")]
    pub r#type: i32,
    #[prost(string, tag="2")]
    pub billing_email: ::prost::alloc::string::String,
    /// defined if type is PAYMENT_METHOD_TYPE_CARD
    #[prost(message, optional, tag="3")]
    pub method: ::core::option::Option<PaymentMethodCard>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetInvoicesSummaryRequest {
    #[prost(string, tag="1")]
    pub org_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetInvoicesSummaryResponse {
    /// all unpaid balances at the end of the last billing cycle
    #[prost(double, tag="1")]
    pub outstanding_balance: f64,
    /// all previous invoices
    #[prost(message, repeated, tag="2")]
    pub invoices: ::prost::alloc::vec::Vec<InvoiceSummary>,
}
/// TODO(APP-1865) should be deprecated/removed
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetBillingSummaryRequest {
    #[prost(string, tag="1")]
    pub org_id: ::prost::alloc::string::String,
}
/// TODO(APP-1865) should be deprecated/removed
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetBillingSummaryResponse {
    #[prost(message, optional, tag="1")]
    pub usage_summary: ::core::option::Option<CurrentMonthUsageSummary>,
    #[prost(message, repeated, tag="2")]
    pub invoices: ::prost::alloc::vec::Vec<InvoiceSummary>,
    /// all unpaid balances at the end of the last billing cycle
    #[prost(double, tag="3")]
    pub statement_balance: f64,
    /// statement_balance + current_month_usage
    #[prost(double, tag="4")]
    pub current_balance: f64,
    /// current_month_usage_cost
    #[prost(double, tag="5")]
    pub current_month_balance: f64,
    /// due date for current month usage
    #[prost(message, optional, tag="7")]
    pub current_month_due_date: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(string, tag="8")]
    pub invoice_email: ::prost::alloc::string::String,
    #[prost(message, optional, tag="9")]
    pub payment_method: ::core::option::Option<PaymentMethodCard>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetInvoicePdfRequest {
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub org_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetInvoicePdfResponse {
    #[prost(bytes="vec", tag="1")]
    pub chunk: ::prost::alloc::vec::Vec<u8>,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum PaymentMethodType {
    Unspecified = 0,
    Card = 1,
}
impl PaymentMethodType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            PaymentMethodType::Unspecified => "PAYMENT_METHOD_TYPE_UNSPECIFIED",
            PaymentMethodType::Card => "PAYMENT_METHOD_TYPE_CARD",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "PAYMENT_METHOD_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
            "PAYMENT_METHOD_TYPE_CARD" => Some(Self::Card),
            _ => None,
        }
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RobotConfig {
    #[prost(message, optional, tag="1")]
    pub cloud: ::core::option::Option<CloudConfig>,
    #[prost(message, repeated, tag="2")]
    pub remotes: ::prost::alloc::vec::Vec<RemoteConfig>,
    #[prost(message, repeated, tag="3")]
    pub components: ::prost::alloc::vec::Vec<ComponentConfig>,
    #[prost(message, repeated, tag="4")]
    pub processes: ::prost::alloc::vec::Vec<ProcessConfig>,
    #[prost(message, repeated, tag="5")]
    pub services: ::prost::alloc::vec::Vec<ServiceConfig>,
    #[prost(message, optional, tag="6")]
    pub network: ::core::option::Option<NetworkConfig>,
    #[prost(message, optional, tag="7")]
    pub auth: ::core::option::Option<AuthConfig>,
    /// Turns on debug mode for robot, adding an echo server and more logging and tracing. Only works after restart
    #[prost(bool, optional, tag="8")]
    pub debug: ::core::option::Option<bool>,
    #[prost(message, repeated, tag="9")]
    pub modules: ::prost::alloc::vec::Vec<ModuleConfig>,
    #[prost(bool, optional, tag="10")]
    pub disable_partial_start: ::core::option::Option<bool>,
    #[prost(message, repeated, tag="11")]
    pub packages: ::prost::alloc::vec::Vec<PackageConfig>,
}
/// Valid location secret that can be used for authentication to the robot.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LocationSecret {
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    /// secret payload
    #[prost(string, tag="2")]
    pub secret: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CloudConfig {
    /// Robot part id.
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub fqdn: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub local_fqdn: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub managed_by: ::prost::alloc::string::String,
    #[prost(string, tag="5")]
    pub signaling_address: ::prost::alloc::string::String,
    #[prost(bool, tag="6")]
    pub signaling_insecure: bool,
    /// Deprecated use location_secrets
    #[deprecated]
    #[prost(string, tag="7")]
    pub location_secret: ::prost::alloc::string::String,
    /// Robot part secret
    #[prost(string, tag="8")]
    pub secret: ::prost::alloc::string::String,
    /// All valid location secrets.
    #[prost(message, repeated, tag="9")]
    pub location_secrets: ::prost::alloc::vec::Vec<LocationSecret>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ComponentConfig {
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// deprecated; use api
    #[prost(string, tag="2")]
    pub namespace: ::prost::alloc::string::String,
    /// deprecated; use api
    #[prost(string, tag="3")]
    pub r#type: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub model: ::prost::alloc::string::String,
    #[prost(message, optional, tag="5")]
    pub frame: ::core::option::Option<Frame>,
    #[prost(string, repeated, tag="6")]
    pub depends_on: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(message, repeated, tag="7")]
    pub service_configs: ::prost::alloc::vec::Vec<ResourceLevelServiceConfig>,
    #[prost(message, optional, tag="8")]
    pub attributes: ::core::option::Option<::prost_types::Struct>,
    #[prost(string, tag="9")]
    pub api: ::prost::alloc::string::String,
}
/// A ResourceLevelServiceConfig describes component or remote configuration for a service.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResourceLevelServiceConfig {
    #[prost(string, tag="1")]
    pub r#type: ::prost::alloc::string::String,
    /// TODO(adam): Should this be move to a structured type as defined in the typescript frontend.
    #[prost(message, optional, tag="2")]
    pub attributes: ::core::option::Option<::prost_types::Struct>,
}
/// A ProcessConfig describes how to manage a system process.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProcessConfig {
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, repeated, tag="3")]
    pub args: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, tag="4")]
    pub cwd: ::prost::alloc::string::String,
    #[prost(bool, tag="5")]
    pub one_shot: bool,
    #[prost(bool, tag="6")]
    pub log: bool,
    #[prost(int32, tag="7")]
    pub stop_signal: i32,
    #[prost(message, optional, tag="8")]
    pub stop_timeout: ::core::option::Option<::prost_types::Duration>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ServiceConfig {
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// deprecated; use api
    #[prost(string, tag="2")]
    pub namespace: ::prost::alloc::string::String,
    /// deprecated; use api
    #[prost(string, tag="3")]
    pub r#type: ::prost::alloc::string::String,
    #[prost(message, optional, tag="4")]
    pub attributes: ::core::option::Option<::prost_types::Struct>,
    #[prost(string, repeated, tag="5")]
    pub depends_on: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, tag="6")]
    pub model: ::prost::alloc::string::String,
    #[prost(string, tag="9")]
    pub api: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NetworkConfig {
    #[prost(string, tag="1")]
    pub fqdn: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub bind_address: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub tls_cert_file: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub tls_key_file: ::prost::alloc::string::String,
    #[prost(message, optional, tag="5")]
    pub sessions: ::core::option::Option<SessionsConfig>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SessionsConfig {
    #[prost(message, optional, tag="1")]
    pub heartbeat_window: ::core::option::Option<::prost_types::Duration>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AuthConfig {
    #[prost(message, repeated, tag="1")]
    pub handlers: ::prost::alloc::vec::Vec<AuthHandlerConfig>,
    #[prost(string, repeated, tag="2")]
    pub tls_auth_entities: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(message, optional, tag="3")]
    pub external_auth_config: ::core::option::Option<ExternalAuthConfig>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct JwksFile {
    /// JSON Web Keys (JWKS) file as arbitary json.
    /// See <https://www.rfc-editor.org/rfc/rfc7517>
    #[prost(message, optional, tag="1")]
    pub json: ::core::option::Option<::prost_types::Struct>,
}
/// ExternalAuthConfig describes how a viam managed robot can accept
/// credentials signed by the cloud app.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExternalAuthConfig {
    #[prost(message, optional, tag="1")]
    pub jwks: ::core::option::Option<JwksFile>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AuthHandlerConfig {
    #[prost(enumeration="CredentialsType", tag="1")]
    pub r#type: i32,
    #[prost(message, optional, tag="5")]
    pub config: ::core::option::Option<::prost_types::Struct>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Frame {
    #[prost(string, tag="1")]
    pub parent: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub translation: ::core::option::Option<Translation>,
    #[prost(message, optional, tag="3")]
    pub orientation: ::core::option::Option<Orientation>,
    #[prost(message, optional, tag="4")]
    pub geometry: ::core::option::Option<super::super::common::v1::Geometry>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Translation {
    #[prost(double, tag="1")]
    pub x: f64,
    #[prost(double, tag="2")]
    pub y: f64,
    #[prost(double, tag="3")]
    pub z: f64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Orientation {
    #[prost(oneof="orientation::Type", tags="1, 2, 3, 4, 5, 6")]
    pub r#type: ::core::option::Option<orientation::Type>,
}
/// Nested message and enum types in `Orientation`.
pub mod orientation {
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
    pub struct NoOrientation {
    }
    /// OrientationVector containing ox, oy, oz, theta represents an orientation vector
    /// Structured similarly to an angle axis, an orientation vector works differently. Rather than representing an orientation
    /// with an arbitrary axis and a rotation around it from an origin, an orientation vector represents orientation
    /// such that the ox/oy/oz components represent the point on the cartesian unit sphere at which your end effector is pointing
    /// from the origin, and that unit vector forms an axis around which theta rotates. This means that incrementing/decrementing
    /// theta will perform an in-line rotation of the end effector.
    /// Theta is defined as rotation between two planes: the plane defined by the origin, the point (0,0,1), and the rx,ry,rz
    /// point, and the plane defined by the origin, the rx,ry,rz point, and the new local Z axis. So if theta is kept at
    /// zero as the north/south pole is circled, the Roll will correct itself to remain in-line.
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
    pub struct OrientationVectorRadians {
        #[prost(double, tag="1")]
        pub theta: f64,
        #[prost(double, tag="2")]
        pub x: f64,
        #[prost(double, tag="3")]
        pub y: f64,
        #[prost(double, tag="4")]
        pub z: f64,
    }
    /// OrientationVectorDegrees is the orientation vector between two objects, but expressed in degrees rather than radians.
    /// Because protobuf Pose is in degrees, this is necessary.
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
    pub struct OrientationVectorDegrees {
        #[prost(double, tag="1")]
        pub theta: f64,
        #[prost(double, tag="2")]
        pub x: f64,
        #[prost(double, tag="3")]
        pub y: f64,
        #[prost(double, tag="4")]
        pub z: f64,
    }
    /// EulerAngles are three angles (in radians) used to represent the rotation of an object in 3D Euclidean space
    /// The Tait–Bryan angle formalism is used, with rotations around three distinct axes in the z-y′-x″ sequence.
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
    pub struct EulerAngles {
        #[prost(double, tag="1")]
        pub roll: f64,
        #[prost(double, tag="2")]
        pub pitch: f64,
        #[prost(double, tag="3")]
        pub yaw: f64,
    }
    /// See here for a thorough explanation: <https://en.wikipedia.org/wiki/Axis%E2%80%93angle_representation>
    /// Basic explanation: Imagine a 3d cartesian grid centered at 0,0,0, and a sphere of radius 1 centered at
    /// that same point. An orientation can be expressed by first specifying an axis, i.e. a line from the origin
    /// to a point on that sphere, represented by (rx, ry, rz), and a rotation around that axis, theta.
    /// These four numbers can be used as-is (R4), or they can be converted to R3, where theta is multiplied by each of
    /// the unit sphere components to give a vector whose length is theta and whose direction is the original axis.
    /// AxisAngles represents an R4 axis angle.
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
    pub struct AxisAngles {
        #[prost(double, tag="1")]
        pub theta: f64,
        #[prost(double, tag="2")]
        pub x: f64,
        #[prost(double, tag="3")]
        pub y: f64,
        #[prost(double, tag="4")]
        pub z: f64,
    }
    /// Quaternion is a float64 precision quaternion.
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Quaternion {
        #[prost(double, tag="1")]
        pub w: f64,
        #[prost(double, tag="2")]
        pub x: f64,
        #[prost(double, tag="3")]
        pub y: f64,
        #[prost(double, tag="4")]
        pub z: f64,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Type {
        #[prost(message, tag="1")]
        NoOrientation(NoOrientation),
        #[prost(message, tag="2")]
        VectorRadians(OrientationVectorRadians),
        #[prost(message, tag="3")]
        VectorDegrees(OrientationVectorDegrees),
        #[prost(message, tag="4")]
        EulerAngles(EulerAngles),
        #[prost(message, tag="5")]
        AxisAngles(AxisAngles),
        #[prost(message, tag="6")]
        Quaternion(Quaternion),
    }
}
/// A RemoteConfig describes a remote robot that should be integrated.
/// The Frame field defines how the "world" node of the remote robot should be reconciled with the "world" node of the
/// the current robot. All components of the remote robot who have Parent as "world" will be attached to the parent defined
/// in Frame, and with the given offset as well.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoteConfig {
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub address: ::prost::alloc::string::String,
    #[prost(message, optional, tag="3")]
    pub frame: ::core::option::Option<Frame>,
    #[prost(message, optional, tag="4")]
    pub auth: ::core::option::Option<RemoteAuth>,
    #[prost(string, tag="5")]
    pub managed_by: ::prost::alloc::string::String,
    #[prost(bool, tag="6")]
    pub insecure: bool,
    #[prost(message, optional, tag="7")]
    pub connection_check_interval: ::core::option::Option<::prost_types::Duration>,
    #[prost(message, optional, tag="8")]
    pub reconnect_interval: ::core::option::Option<::prost_types::Duration>,
    #[prost(message, repeated, tag="9")]
    pub service_configs: ::prost::alloc::vec::Vec<ResourceLevelServiceConfig>,
    /// Secret is a helper for a robot location secret.
    #[prost(string, tag="10")]
    pub secret: ::prost::alloc::string::String,
}
/// RemoteAuth specifies how to authenticate against a remote. If no credentials are
/// specified, authentication does not happen. If an entity is specified, the
/// authentication request will specify it.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoteAuth {
    #[prost(message, optional, tag="1")]
    pub credentials: ::core::option::Option<remote_auth::Credentials>,
    #[prost(string, tag="2")]
    pub entity: ::prost::alloc::string::String,
}
/// Nested message and enum types in `RemoteAuth`.
pub mod remote_auth {
    /// Credentials packages up both a type of credential along with its payload which
    /// is formatted specific to the type.
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Credentials {
        #[prost(enumeration="super::CredentialsType", tag="1")]
        pub r#type: i32,
        #[prost(string, tag="2")]
        pub payload: ::prost::alloc::string::String,
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AgentInfo {
    #[prost(string, tag="1")]
    pub host: ::prost::alloc::string::String,
    /// Will soon be deprecated, use platform instead
    #[prost(string, tag="2")]
    pub os: ::prost::alloc::string::String,
    /// list of all ipv4 ips.
    #[prost(string, repeated, tag="3")]
    pub ips: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// RDK version
    #[prost(string, tag="4")]
    pub version: ::prost::alloc::string::String,
    #[prost(string, tag="5")]
    pub git_revision: ::prost::alloc::string::String,
    /// The platform the RDK is running on. For example linux/amd64
    #[prost(string, optional, tag="6")]
    pub platform: ::core::option::Option<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ConfigRequest {
    /// Robot part id.
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    /// Details about the RDK (os, version) are updated during this request.
    #[prost(message, optional, tag="2")]
    pub agent_info: ::core::option::Option<AgentInfo>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ConfigResponse {
    #[prost(message, optional, tag="1")]
    pub config: ::core::option::Option<RobotConfig>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CertificateRequest {
    /// Robot part id.
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CertificateResponse {
    /// Robot part id.
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub tls_certificate: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub tls_private_key: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LogRequest {
    /// Robot part id.
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    #[prost(message, repeated, tag="2")]
    pub logs: ::prost::alloc::vec::Vec<LogEntry>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LogResponse {
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NeedsRestartRequest {
    /// Robot part id.
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NeedsRestartResponse {
    /// Robot part id.
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    #[prost(bool, tag="2")]
    pub must_restart: bool,
    #[prost(message, optional, tag="3")]
    pub restart_check_interval: ::core::option::Option<::prost_types::Duration>,
}
/// ModuleConfig is the configuration for a module.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ModuleConfig {
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// path to the executable
    #[prost(string, tag="2")]
    pub path: ::prost::alloc::string::String,
    /// log level for module
    #[prost(string, tag="3")]
    pub log_level: ::prost::alloc::string::String,
}
/// PackageConfig is the configration for deployed Packages.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PackageConfig {
    /// Name is the local name of the package on the RDK. Must be unique across Packages. Must not be empty.
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// Package is the unique package name hosted by Viam. Must not be empty.
    #[prost(string, tag="2")]
    pub package: ::prost::alloc::string::String,
    /// version of the package ID hosted by Viam. If not specified "latest" is assumed.
    #[prost(string, tag="3")]
    pub version: ::prost::alloc::string::String,
    /// type of the package
    #[prost(string, tag="4")]
    pub r#type: ::prost::alloc::string::String,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum CredentialsType {
    Unspecified = 0,
    Internal = 1,
    ApiKey = 2,
    RobotSecret = 3,
    RobotLocationSecret = 4,
}
impl CredentialsType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            CredentialsType::Unspecified => "CREDENTIALS_TYPE_UNSPECIFIED",
            CredentialsType::Internal => "CREDENTIALS_TYPE_INTERNAL",
            CredentialsType::ApiKey => "CREDENTIALS_TYPE_API_KEY",
            CredentialsType::RobotSecret => "CREDENTIALS_TYPE_ROBOT_SECRET",
            CredentialsType::RobotLocationSecret => "CREDENTIALS_TYPE_ROBOT_LOCATION_SECRET",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "CREDENTIALS_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
            "CREDENTIALS_TYPE_INTERNAL" => Some(Self::Internal),
            "CREDENTIALS_TYPE_API_KEY" => Some(Self::ApiKey),
            "CREDENTIALS_TYPE_ROBOT_SECRET" => Some(Self::RobotSecret),
            "CREDENTIALS_TYPE_ROBOT_LOCATION_SECRET" => Some(Self::RobotLocationSecret),
            _ => None,
        }
    }
}
// @@protoc_insertion_point(module)
