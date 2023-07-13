// @generated
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResourceName {
    #[prost(string, tag="1")]
    pub namespace: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub r#type: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub subtype: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub name: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BoardStatus {
    #[prost(map="string, message", tag="1")]
    pub analogs: ::std::collections::HashMap<::prost::alloc::string::String, AnalogStatus>,
    #[prost(map="string, message", tag="2")]
    pub digital_interrupts: ::std::collections::HashMap<::prost::alloc::string::String, DigitalInterruptStatus>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AnalogStatus {
    /// Current value of the analog reader of a robot's board
    #[prost(int32, tag="1")]
    pub value: i32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DigitalInterruptStatus {
    /// Current value of the digital interrupt of a robot's board
    #[prost(int64, tag="1")]
    pub value: i64,
}
/// Pose is a combination of location and orientation.
/// Location is expressed as distance which is represented by x , y, z coordinates. Orientation is expressed as an orientation vector which
/// is represented by o_x, o_y, o_z and theta. The o_x, o_y, o_z coordinates represent the point on the cartesian unit sphere that the end of
/// the arm is pointing to (with the origin as reference). That unit vector forms an axis around which theta rotates. This means that
/// incrementing / decrementing theta will perform an inline rotation of the end effector.
/// Theta is defined as rotation between two planes: the first being defined by the origin, the point (0,0,1), and the rx, ry, rz point, and the
/// second being defined by the origin, the rx, ry, rz point and the local Z axis. Therefore, if theta is kept at zero as the north/south pole
/// is circled, the Roll will correct itself to remain in-line. 
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Pose {
    /// millimeters from the origin
    #[prost(double, tag="1")]
    pub x: f64,
    /// millimeters from the origin
    #[prost(double, tag="2")]
    pub y: f64,
    /// millimeters from the origin
    #[prost(double, tag="3")]
    pub z: f64,
    /// z component of a vector defining axis of rotation
    #[prost(double, tag="4")]
    pub o_x: f64,
    /// x component of a vector defining axis of rotation
    #[prost(double, tag="5")]
    pub o_y: f64,
    /// y component of a vector defining axis of rotation
    #[prost(double, tag="6")]
    pub o_z: f64,
    /// degrees
    #[prost(double, tag="7")]
    pub theta: f64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Orientation {
    /// x component of a vector defining axis of rotation
    #[prost(double, tag="1")]
    pub o_x: f64,
    /// y component of a vector defining axis of rotation
    #[prost(double, tag="2")]
    pub o_y: f64,
    /// z component of a vector defining axis of rotation
    #[prost(double, tag="3")]
    pub o_z: f64,
    /// degrees
    #[prost(double, tag="4")]
    pub theta: f64,
}
/// PoseInFrame contains a pose and the and the reference frame in which it was observed
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PoseInFrame {
    #[prost(string, tag="1")]
    pub reference_frame: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub pose: ::core::option::Option<Pose>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Vector3 {
    #[prost(double, tag="1")]
    pub x: f64,
    #[prost(double, tag="2")]
    pub y: f64,
    #[prost(double, tag="3")]
    pub z: f64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Sphere {
    #[prost(double, tag="1")]
    pub radius_mm: f64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Capsule {
    #[prost(double, tag="1")]
    pub radius_mm: f64,
    #[prost(double, tag="2")]
    pub length_mm: f64,
}
/// RectangularPrism contains a Vector3 field corresponding to the X, Y, Z dimensions of the prism in mms
/// These dimensions are with respect to the referenceframe in which the RectangularPrism is defined
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RectangularPrism {
    #[prost(message, optional, tag="1")]
    pub dims_mm: ::core::option::Option<Vector3>,
}
/// Geometry contains the dimensions of a given geometry and the pose of its center. The geometry is one of either a sphere or a box.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Geometry {
    /// Pose of a geometries center point
    #[prost(message, optional, tag="1")]
    pub center: ::core::option::Option<Pose>,
    /// Label of the geometry. If none supplied, will be an empty string.
    #[prost(string, tag="4")]
    pub label: ::prost::alloc::string::String,
    /// Dimensions of a given geometry. This can be a sphere or box
    #[prost(oneof="geometry::GeometryType", tags="2, 3, 5")]
    pub geometry_type: ::core::option::Option<geometry::GeometryType>,
}
/// Nested message and enum types in `Geometry`.
pub mod geometry {
    /// Dimensions of a given geometry. This can be a sphere or box
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum GeometryType {
        #[prost(message, tag="2")]
        Sphere(super::Sphere),
        #[prost(message, tag="3")]
        Box(super::RectangularPrism),
        #[prost(message, tag="5")]
        Capsule(super::Capsule),
    }
}
/// GeometriesinFrame contains the dimensions of a given geometry, pose of its center point, and the reference frame by which it was
/// observed.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GeometriesInFrame {
    /// Reference frame of the observer of the geometry
    #[prost(string, tag="1")]
    pub reference_frame: ::prost::alloc::string::String,
    /// Dimensional type
    #[prost(message, repeated, tag="2")]
    pub geometries: ::prost::alloc::vec::Vec<Geometry>,
}
/// PointCloudObject contains an image in bytes with point cloud data of all of the objects captured by a given observer as well as a
/// repeated list of geometries which respresents the center point and geometry of each of the objects within the point cloud
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PointCloudObject {
    /// image frame expressed in bytes
    #[prost(bytes="vec", tag="1")]
    pub point_cloud: ::prost::alloc::vec::Vec<u8>,
    /// volume of a given geometry
    #[prost(message, optional, tag="2")]
    pub geometries: ::core::option::Option<GeometriesInFrame>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GeoPoint {
    #[prost(double, tag="1")]
    pub latitude: f64,
    #[prost(double, tag="2")]
    pub longitude: f64,
}
/// GeoObstacle contains information about the geometric structure of an obstacle and the location of the obstacle,
/// captured in latitude and longitude.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GeoObstacle {
    /// Location of the obstacle
    #[prost(message, optional, tag="1")]
    pub location: ::core::option::Option<GeoPoint>,
    /// Geometries that describe the obstacle, where embedded Pose data is with respect to the specified location
    #[prost(message, repeated, tag="2")]
    pub geometries: ::prost::alloc::vec::Vec<Geometry>,
}
/// Transform contains a pose and two reference frames. The first reference frame is the starting reference frame, and the second reference
/// frame is the observer reference frame. The second reference frame has a pose which represents the pose of an object in the first
/// reference frame as observed within the second reference frame.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Transform {
    /// the name of a given reference frame
    #[prost(string, tag="1")]
    pub reference_frame: ::prost::alloc::string::String,
    /// the pose of the above reference frame with respect to a different observer reference frame
    #[prost(message, optional, tag="2")]
    pub pose_in_observer_frame: ::core::option::Option<PoseInFrame>,
    #[prost(message, optional, tag="3")]
    pub physical_object: ::core::option::Option<Geometry>,
}
/// WorldState contains information about the physical environment around a given robot. All of the fields within this message are optional,
/// they can include information about the physical dimensions of an obstacle, the freespace of a robot, and any desired transforms between a
/// given reference frame and a new target reference frame.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WorldState {
    /// a list of obstacles expressed as a geometry and the reference frame in which it was observed; this field is optional
    #[prost(message, repeated, tag="1")]
    pub obstacles: ::prost::alloc::vec::Vec<GeometriesInFrame>,
    /// a list of Transforms, optionally with geometries. Used as supplemental transforms to transform a pose from one reference frame to another, or to attach moving geometries to the frame system. This field is optional
    #[prost(message, repeated, tag="3")]
    pub transforms: ::prost::alloc::vec::Vec<Transform>,
}
/// ActuatorStatus is a generic status for resources that only need to return actuator status.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ActuatorStatus {
    #[prost(bool, tag="1")]
    pub is_moving: bool,
}
/// DoCommandRequest represents a generic DoCommand input
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DoCommandRequest {
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub command: ::core::option::Option<::prost_types::Struct>,
}
/// DoCommandResponse represents a generic DoCommand output
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DoCommandResponse {
    #[prost(message, optional, tag="1")]
    pub result: ::core::option::Option<::prost_types::Struct>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetKinematicsRequest {
    /// The component name
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetKinematicsResponse {
    /// The kinematics of the component, in either URDF format or in Viam’s kinematic parameter format (spatial vector algebra)
    /// <https://docs.viam.com/internals/kinematic-chain-config/#kinematic-parameters>
    #[prost(enumeration="KinematicsFileFormat", tag="1")]
    pub format: i32,
    /// The byte contents of the file
    #[prost(bytes="vec", tag="2")]
    pub kinematics_data: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetGeometriesRequest {
    /// The component name
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetGeometriesResponse {
    /// All geometries associated with the component, in their current configuration, in the frame of that component.
    #[prost(message, repeated, tag="1")]
    pub geometries: ::prost::alloc::vec::Vec<Geometry>,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum KinematicsFileFormat {
    Unspecified = 0,
    Sva = 1,
    Urdf = 2,
}
impl KinematicsFileFormat {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            KinematicsFileFormat::Unspecified => "KINEMATICS_FILE_FORMAT_UNSPECIFIED",
            KinematicsFileFormat::Sva => "KINEMATICS_FILE_FORMAT_SVA",
            KinematicsFileFormat::Urdf => "KINEMATICS_FILE_FORMAT_URDF",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "KINEMATICS_FILE_FORMAT_UNSPECIFIED" => Some(Self::Unspecified),
            "KINEMATICS_FILE_FORMAT_SVA" => Some(Self::Sva),
            "KINEMATICS_FILE_FORMAT_URDF" => Some(Self::Urdf),
            _ => None,
        }
    }
}
// @@protoc_insertion_point(module)
