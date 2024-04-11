// @generated
/// Moves any component on the robot to a specified destination which can be from the reference frame of any other component on the robot.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MoveRequest {
    /// Name of the motion service
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// Destination to move to, which can a pose in the reference frame of any frame in the robot's frame system
    #[prost(message, optional, tag="2")]
    pub destination: ::core::option::Option<super::super::super::common::v1::PoseInFrame>,
    /// Component on the robot to move to the specified destination
    #[prost(message, optional, tag="3")]
    pub component_name: ::core::option::Option<super::super::super::common::v1::ResourceName>,
    /// Avoid obstacles by specifying their geometries in the world state
    /// Augment the frame system of the robot by specifying additional transforms to add to it for the duration of the Move
    #[prost(message, optional, tag="4")]
    pub world_state: ::core::option::Option<super::super::super::common::v1::WorldState>,
    /// Constrain the way the robot will move
    #[prost(message, optional, tag="5")]
    pub constraints: ::core::option::Option<Constraints>,
    /// Additional arguments to the method
    #[prost(message, optional, tag="99")]
    pub extra: ::core::option::Option<::prost_types::Struct>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MoveResponse {
    #[prost(bool, tag="1")]
    pub success: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MoveOnMapRequest {
    /// Name of the motion service
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// Specify a destination to, which can be any pose with respect to the SLAM map's origin
    #[prost(message, optional, tag="2")]
    pub destination: ::core::option::Option<super::super::super::common::v1::Pose>,
    /// Component on the robot to move to the specified destination
    #[prost(message, optional, tag="3")]
    pub component_name: ::core::option::Option<super::super::super::common::v1::ResourceName>,
    /// Name of the slam service from which the SLAM map is requested
    #[prost(message, optional, tag="4")]
    pub slam_service_name: ::core::option::Option<super::super::super::common::v1::ResourceName>,
    /// Optional set of motion configuration options
    #[prost(message, optional, tag="5")]
    pub motion_configuration: ::core::option::Option<MotionConfiguration>,
    /// Obstacles to be considered for motion planning
    #[prost(message, repeated, tag="6")]
    pub obstacles: ::prost::alloc::vec::Vec<super::super::super::common::v1::Geometry>,
    /// Additional arguments to the method
    #[prost(message, optional, tag="99")]
    pub extra: ::core::option::Option<::prost_types::Struct>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MoveOnMapResponse {
    /// The unique ID which identifies the execution.
    /// Multiple plans will share the same execution_id if they were
    /// generated due to replanning.
    #[prost(string, tag="1")]
    pub execution_id: ::prost::alloc::string::String,
}
/// Pairs a vision service with a camera, informing the service about which camera it may use
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ObstacleDetector {
    #[prost(message, optional, tag="1")]
    pub vision_service: ::core::option::Option<super::super::super::common::v1::ResourceName>,
    #[prost(message, optional, tag="2")]
    pub camera: ::core::option::Option<super::super::super::common::v1::ResourceName>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MotionConfiguration {
    /// The ObstacleDetectors that will be used for transient obstacle avoidance
    #[prost(message, repeated, tag="1")]
    pub obstacle_detectors: ::prost::alloc::vec::Vec<ObstacleDetector>,
    /// Sets the frequency to poll for the position of the robot
    #[prost(double, optional, tag="2")]
    pub position_polling_frequency_hz: ::core::option::Option<f64>,
    /// Sets the frequency to poll the vision service(s) for new obstacles
    #[prost(double, optional, tag="3")]
    pub obstacle_polling_frequency_hz: ::core::option::Option<f64>,
    /// Sets the distance in meters that a robot is allowed to deviate from the motion plan
    #[prost(double, optional, tag="4")]
    pub plan_deviation_m: ::core::option::Option<f64>,
    /// Optional linear velocity to target when moving
    #[prost(double, optional, tag="5")]
    pub linear_m_per_sec: ::core::option::Option<f64>,
    /// Optional angular velocity to target when turning
    #[prost(double, optional, tag="6")]
    pub angular_degs_per_sec: ::core::option::Option<f64>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MoveOnGlobeRequest {
    /// Name of the motion service
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// Destination, encoded as a GeoPoint
    #[prost(message, optional, tag="2")]
    pub destination: ::core::option::Option<super::super::super::common::v1::GeoPoint>,
    /// Optional compass heading to achieve at the destination, in degrees [0-360)
    #[prost(double, optional, tag="3")]
    pub heading: ::core::option::Option<f64>,
    /// Component on the robot to move to the specified destination
    #[prost(message, optional, tag="4")]
    pub component_name: ::core::option::Option<super::super::super::common::v1::ResourceName>,
    /// Name of the movement sensor which will be used to check robot location
    #[prost(message, optional, tag="5")]
    pub movement_sensor_name: ::core::option::Option<super::super::super::common::v1::ResourceName>,
    /// Obstacles to be considered for motion planning
    #[prost(message, repeated, tag="6")]
    pub obstacles: ::prost::alloc::vec::Vec<super::super::super::common::v1::GeoObstacle>,
    /// Optional set of motion configuration options
    #[prost(message, optional, tag="7")]
    pub motion_configuration: ::core::option::Option<MotionConfiguration>,
    /// Additional arguments to the method
    #[prost(message, optional, tag="99")]
    pub extra: ::core::option::Option<::prost_types::Struct>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MoveOnGlobeResponse {
    /// The unique ID which identifies the execution.
    /// Multiple plans will share the same execution_id if they were
    /// generated due to replanning.
    #[prost(string, tag="1")]
    pub execution_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetPoseRequest {
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// the component whose pose is being requested
    #[prost(message, optional, tag="2")]
    pub component_name: ::core::option::Option<super::super::super::common::v1::ResourceName>,
    /// the reference frame in which the component's pose
    /// should be provided, if unset this defaults
    /// to the "world" reference frame
    #[prost(string, tag="3")]
    pub destination_frame: ::prost::alloc::string::String,
    /// pose information on any additional reference frames that are needed
    /// to compute the component's pose
    #[prost(message, repeated, tag="4")]
    pub supplemental_transforms: ::prost::alloc::vec::Vec<super::super::super::common::v1::Transform>,
    /// Additional arguments to the method
    #[prost(message, optional, tag="99")]
    pub extra: ::core::option::Option<::prost_types::Struct>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetPoseResponse {
    #[prost(message, optional, tag="1")]
    pub pose: ::core::option::Option<super::super::super::common::v1::PoseInFrame>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StopPlanRequest {
    /// The name of the motion service
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// The component of the currently executing plan to stop
    #[prost(message, optional, tag="2")]
    pub component_name: ::core::option::Option<super::super::super::common::v1::ResourceName>,
    /// Additional arguments to the method
    #[prost(message, optional, tag="99")]
    pub extra: ::core::option::Option<::prost_types::Struct>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StopPlanResponse {
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListPlanStatusesRequest {
    /// The name of the motion service
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// If supplied, the response will filter the
    /// plan results for the supplied state
    #[prost(bool, tag="2")]
    pub only_active_plans: bool,
    /// Additional arguments to the method
    #[prost(message, optional, tag="99")]
    pub extra: ::core::option::Option<::prost_types::Struct>,
}
/// Status of all executed / executing plan statuses with associated IDs within the 24 hour TTL
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListPlanStatusesResponse {
    /// List of last known statuses with the associated IDs of all plans within the TTL
    /// ordered by timestamp in ascending order
    #[prost(message, repeated, tag="1")]
    pub plan_statuses_with_ids: ::prost::alloc::vec::Vec<PlanStatusWithId>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetPlanRequest {
    /// The name of the motion service
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// The name of the component which was requested to be moved.
    #[prost(message, optional, tag="2")]
    pub component_name: ::core::option::Option<super::super::super::common::v1::ResourceName>,
    /// If supplied, the response will only return
    /// the the last plan for the component / execution
    #[prost(bool, tag="3")]
    pub last_plan_only: bool,
    /// If you want to know about the plans of a previous execution
    #[prost(string, optional, tag="4")]
    pub execution_id: ::core::option::Option<::prost::alloc::string::String>,
    /// Additional arguments to the method
    #[prost(message, optional, tag="99")]
    pub extra: ::core::option::Option<::prost_types::Struct>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetPlanResponse {
    /// The current plan and status that matches the request query
    #[prost(message, optional, tag="1")]
    pub current_plan_with_status: ::core::option::Option<PlanWithStatus>,
    /// Returns the history of all previous plans that were
    /// generated in ascending order.
    /// This field will be empty if the motion service
    /// did not need to re-plan.
    #[prost(message, repeated, tag="2")]
    pub replan_history: ::prost::alloc::vec::Vec<PlanWithStatus>,
}
/// Constraints specifies all enumerated constraints to be passed to Viam's motion planning, along with any optional parameters
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Constraints {
    /// Typed message for a specific constraint
    #[prost(message, repeated, tag="1")]
    pub linear_constraint: ::prost::alloc::vec::Vec<LinearConstraint>,
    #[prost(message, repeated, tag="2")]
    pub orientation_constraint: ::prost::alloc::vec::Vec<OrientationConstraint>,
    /// Arc constraint, Time constraint, and others will be added here when they are supported
    #[prost(message, repeated, tag="3")]
    pub collision_specification: ::prost::alloc::vec::Vec<CollisionSpecification>,
}
/// LinearConstraint specifies that the component being moved should move linearly relative to its goal.
/// It does not constrain the motion of components other than the `component_name` specified in motion.Move
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LinearConstraint {
    /// Max linear deviation from straight-line between start and goal, in mm.
    #[prost(float, optional, tag="1")]
    pub line_tolerance_mm: ::core::option::Option<f32>,
    /// Max allowable orientation deviation, in degrees, while on the shortest path between start / goal states
    #[prost(float, optional, tag="2")]
    pub orientation_tolerance_degs: ::core::option::Option<f32>,
}
/// OrientationConstraint specifies that the component being moved will not deviate its orientation beyond some threshold relative
/// to the goal. It does not constrain the motion of components other than the `component_name` specified in motion.Move
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OrientationConstraint {
    /// Max allowable orientation deviation, in degrees, while on the shortest path between start / goal states
    #[prost(float, optional, tag="1")]
    pub orientation_tolerance_degs: ::core::option::Option<f32>,
}
/// CollisionSpecification is used to selectively apply obstacle avoidance to specific parts of the robot
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CollisionSpecification {
    /// Pairs of frame which should be allowed to collide with one another
    #[prost(message, repeated, tag="1")]
    pub allows: ::prost::alloc::vec::Vec<collision_specification::AllowedFrameCollisions>,
}
/// Nested message and enum types in `CollisionSpecification`.
pub mod collision_specification {
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
    pub struct AllowedFrameCollisions {
        #[prost(string, tag="1")]
        pub frame1: ::prost::alloc::string::String,
        #[prost(string, tag="2")]
        pub frame2: ::prost::alloc::string::String,
    }
}
/// Describes a plan, its current status & all status changes
/// that have occured previously on that plan
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PlanWithStatus {
    /// The plan
    #[prost(message, optional, tag="1")]
    pub plan: ::core::option::Option<Plan>,
    /// The current status of the plan
    #[prost(message, optional, tag="2")]
    pub status: ::core::option::Option<PlanStatus>,
    /// The prior status changes that have happened during plan execution
    #[prost(message, repeated, tag="3")]
    pub status_history: ::prost::alloc::vec::Vec<PlanStatus>,
}
/// PlanStatusWithID describes the state of a given plan at a
/// point in time plus the plan_id, component_name and execution_id
/// the status is associated with
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PlanStatusWithId {
    /// The unique ID of the plan
    #[prost(string, tag="1")]
    pub plan_id: ::prost::alloc::string::String,
    /// The component to be moved.
    /// Used for tracking & stopping.
    /// NOTE: A plan may move more components than just the
    /// component_name.
    #[prost(message, optional, tag="2")]
    pub component_name: ::core::option::Option<super::super::super::common::v1::ResourceName>,
    /// The unique ID which identifies the plan execution.
    /// Multiple plans will share the same execution_id if they were
    /// generated due to replanning.
    #[prost(string, tag="3")]
    pub execution_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag="4")]
    pub status: ::core::option::Option<PlanStatus>,
}
/// Plan status describes the state of a given plan at a
/// point in time
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PlanStatus {
    /// The state of the plan execution
    #[prost(enumeration="PlanState", tag="1")]
    pub state: i32,
    /// The time the executing plan transtioned to the state
    #[prost(message, optional, tag="2")]
    pub timestamp: ::core::option::Option<::prost_types::Timestamp>,
    /// The reason for the state change. If motion plan failed
    /// this will return the error message.
    /// If motion needed to re-plan, this will return
    /// the re-plan reason.
    #[prost(string, optional, tag="3")]
    pub reason: ::core::option::Option<::prost::alloc::string::String>,
}
/// A plan describes a motion plan
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Plan {
    /// The plan's unique ID
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    /// The component requested to be moved.
    /// Used for tracking & stopping.
    /// NOTE: A plan may move more components than just the
    /// root component.
    #[prost(message, optional, tag="2")]
    pub component_name: ::core::option::Option<super::super::super::common::v1::ResourceName>,
    /// The unique ID which identifies the execution.
    /// Multiple plans will share the same execution_id if they were
    /// generated due to replanning
    #[prost(string, tag="3")]
    pub execution_id: ::prost::alloc::string::String,
    /// The steps of a plan is an ordered list of plan steps
    #[prost(message, repeated, tag="4")]
    pub steps: ::prost::alloc::vec::Vec<PlanStep>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PlanStep {
    /// A step is the component state each
    /// component resource should reach while executing
    /// that step of the plan.
    /// Keys are the fully qualified component name.
    #[prost(map="string, message", tag="1")]
    pub step: ::std::collections::HashMap<::prost::alloc::string::String, ComponentState>,
}
/// A pose
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ComponentState {
    #[prost(message, optional, tag="1")]
    pub pose: ::core::option::Option<super::super::super::common::v1::Pose>,
}
/// The states that a plan can be in.
/// InProgress if the plan is executing.
/// Stopped if the plan was stopped.
/// Suceeded if the robot reached its destination successfully.
/// Failed if the robot did not reach its destination.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum PlanState {
    Unspecified = 0,
    InProgress = 1,
    Stopped = 2,
    Succeeded = 3,
    Failed = 4,
}
impl PlanState {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            PlanState::Unspecified => "PLAN_STATE_UNSPECIFIED",
            PlanState::InProgress => "PLAN_STATE_IN_PROGRESS",
            PlanState::Stopped => "PLAN_STATE_STOPPED",
            PlanState::Succeeded => "PLAN_STATE_SUCCEEDED",
            PlanState::Failed => "PLAN_STATE_FAILED",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "PLAN_STATE_UNSPECIFIED" => Some(Self::Unspecified),
            "PLAN_STATE_IN_PROGRESS" => Some(Self::InProgress),
            "PLAN_STATE_STOPPED" => Some(Self::Stopped),
            "PLAN_STATE_SUCCEEDED" => Some(Self::Succeeded),
            "PLAN_STATE_FAILED" => Some(Self::Failed),
            _ => None,
        }
    }
}
// @@protoc_insertion_point(module)
