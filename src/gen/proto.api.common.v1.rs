#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResourceName {
    #[prost(string, tag = "1")]
    pub uuid: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub namespace: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub r#type: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub subtype: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub name: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BoardStatus {
    #[prost(map = "string, message", tag = "1")]
    pub analogs: ::std::collections::HashMap<::prost::alloc::string::String, AnalogStatus>,
    #[prost(map = "string, message", tag = "2")]
    pub digital_interrupts:
        ::std::collections::HashMap<::prost::alloc::string::String, DigitalInterruptStatus>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AnalogStatus {
    #[prost(int32, tag = "1")]
    pub value: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DigitalInterruptStatus {
    #[prost(int64, tag = "1")]
    pub value: i64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Pose {
    /// millimeters of the end effector from the base
    #[prost(double, tag = "1")]
    pub x: f64,
    #[prost(double, tag = "2")]
    pub y: f64,
    #[prost(double, tag = "3")]
    pub z: f64,
    /// ox, oy, oz, theta represents an orientation vector
    /// Structured similarly to an angle axis, an orientation vector works differently. Rather than representing an orientation
    /// with an arbitrary axis and a rotation around it from an origin, an orientation vector represents orientation
    /// such that the ox/oy/oz components represent the point on the cartesian unit sphere at which your end effector is pointing
    /// from the origin, and that unit vector forms an axis around which theta rotates. This means that incrementing/decrementing
    /// theta will perform an in-line rotation of the end effector.
    /// Theta is defined as rotation between two planes: the plane defined by the origin, the point (0,0,1), and the rx,ry,rz
    /// point, and the plane defined by the origin, the rx,ry,rz point, and the new local Z axis. So if theta is kept at
    /// zero as the north/south pole is circled, the Roll will correct itself to remain in-line.
    /// Theta in pb.Pose should be degrees. It will be converted to radians in the internal OrientationVec.
    #[prost(double, tag = "4")]
    pub o_x: f64,
    #[prost(double, tag = "5")]
    pub o_y: f64,
    #[prost(double, tag = "6")]
    pub o_z: f64,
    #[prost(double, tag = "7")]
    pub theta: f64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PoseInFrame {
    #[prost(string, tag = "1")]
    pub reference_frame: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub pose: ::core::option::Option<Pose>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Vector3 {
    #[prost(double, tag = "1")]
    pub x: f64,
    #[prost(double, tag = "2")]
    pub y: f64,
    #[prost(double, tag = "3")]
    pub z: f64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Sphere {
    #[prost(double, tag = "1")]
    pub radius_mm: f64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RectangularPrism {
    #[prost(double, tag = "1")]
    pub width_mm: f64,
    #[prost(double, tag = "2")]
    pub length_mm: f64,
    #[prost(double, tag = "3")]
    pub depth_mm: f64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Geometry {
    #[prost(message, optional, tag = "1")]
    pub center: ::core::option::Option<Pose>,
    #[prost(oneof = "geometry::GeometryType", tags = "2, 3")]
    pub geometry_type: ::core::option::Option<geometry::GeometryType>,
}
/// Nested message and enum types in `Geometry`.
pub mod geometry {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum GeometryType {
        #[prost(message, tag = "2")]
        Sphere(super::Sphere),
        #[prost(message, tag = "3")]
        Box(super::RectangularPrism),
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GeometriesInFrame {
    #[prost(string, tag = "1")]
    pub reference_frame: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "2")]
    pub geometries: ::prost::alloc::vec::Vec<Geometry>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PointCloudObject {
    /// image frame expressed in bytes
    #[prost(bytes = "vec", tag = "1")]
    pub point_cloud: ::prost::alloc::vec::Vec<u8>,
    /// volume of a given geometry
    #[prost(message, optional, tag = "2")]
    pub geometries: ::core::option::Option<GeometriesInFrame>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GeoPoint {
    #[prost(double, tag = "1")]
    pub latitude: f64,
    #[prost(double, tag = "2")]
    pub longitude: f64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WorldState {
    #[prost(message, repeated, tag = "1")]
    pub obstacles: ::prost::alloc::vec::Vec<GeometriesInFrame>,
}
