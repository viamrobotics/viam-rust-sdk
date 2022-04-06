pub mod proto {
    pub mod api {
        pub mod service {
            pub mod sensors {
                pub mod v1 {
                    include!("proto.api.service.sensors.v1.rs");
                }
            }
            pub mod navigation {
                pub mod v1 {
                    include!("proto.api.service.navigation.v1.rs");
                }
            }
            pub mod status {
                pub mod v1 {
                    include!("proto.api.service.status.v1.rs");
                }
            }
            pub mod objectsegmentation {
                pub mod v1 {
                    include!("proto.api.service.objectsegmentation.v1.rs");
                }
            }
            pub mod framesystem {
                pub mod v1 {
                    include!("proto.api.service.framesystem.v1.rs");
                }
            }
            pub mod objectmanipulation {
                pub mod v1 {
                    include!("proto.api.service.objectmanipulation.v1.rs");
                }
            }
            pub mod metadata {
                pub mod v1 {
                    include!("proto.api.service.metadata.v1.rs");
                }
            }
        }
        pub mod robot {
            pub mod v1 {
                include!("proto.api.robot.v1.rs");
            }
        }
        pub mod component {
            pub mod arm {
                pub mod v1 {
                    include!("proto.api.component.arm.v1.rs");
                }
            }
            pub mod board {
                pub mod v1 {
                    include!("proto.api.component.board.v1.rs");
                }
            }
            pub mod gripper {
                pub mod v1 {
                    include!("proto.api.component.gripper.v1.rs");
                }
            }
            pub mod posetracker {
                pub mod v1 {
                    include!("proto.api.component.posetracker.v1.rs");
                }
            }
            pub mod inputcontroller {
                pub mod v1 {
                    include!("proto.api.component.inputcontroller.v1.rs");
                }
            }
            pub mod gps {
                pub mod v1 {
                    include!("proto.api.component.gps.v1.rs");
                }
            }
            pub mod camera {
                pub mod v1 {
                    include!("proto.api.component.camera.v1.rs");
                }
            }
            pub mod gantry {
                pub mod v1 {
                    include!("proto.api.component.gantry.v1.rs");
                }
            }
            pub mod servo {
                pub mod v1 {
                    include!("proto.api.component.servo.v1.rs");
                }
            }
            pub mod motor {
                pub mod v1 {
                    include!("proto.api.component.motor.v1.rs");
                }
            }
            pub mod sensor {
                pub mod v1 {
                    include!("proto.api.component.sensor.v1.rs");
                }
            }
            pub mod forcematrix {
                pub mod v1 {
                    include!("proto.api.component.forcematrix.v1.rs");
                }
            }
            pub mod base {
                pub mod v1 {
                    include!("proto.api.component.base.v1.rs");
                }
            }
            pub mod imu {
                pub mod v1 {
                    include!("proto.api.component.imu.v1.rs");
                }
            }
        }
        pub mod common {
            pub mod v1 {
                include!("proto.api.common.v1.rs");
            }
        }
    }
    pub mod rpc {
        pub mod examples {
            pub mod echo {
                pub mod v1 {
                    include!("proto.rpc.examples.echo.v1.rs");
                }
            }
        }
        pub mod v1 {
            include!("proto.rpc.v1.rs");
        }
    }
}
pub mod google {
    pub mod protobuf {
        include!("google.protobuf.rs");
    }
    pub mod api {
        include!("google.api.rs");
    }
}
