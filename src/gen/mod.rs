pub mod proto {
    pub mod api {
        pub mod common {
            // @@protoc_insertion_point(attribute:proto.api.common.v1)
            pub mod v1 {
                include!("proto.api.common.v1.rs");
                // @@protoc_insertion_point(proto.api.common.v1)
            }
        }
        pub mod component {
            pub mod arm {
                // @@protoc_insertion_point(attribute:proto.api.component.arm.v1)
                pub mod v1 {
                    include!("proto.api.component.arm.v1.rs");
                    // @@protoc_insertion_point(proto.api.component.arm.v1)
                }
            }
            pub mod base {
                // @@protoc_insertion_point(attribute:proto.api.component.base.v1)
                pub mod v1 {
                    include!("proto.api.component.base.v1.rs");
                    // @@protoc_insertion_point(proto.api.component.base.v1)
                }
            }
            pub mod board {
                // @@protoc_insertion_point(attribute:proto.api.component.board.v1)
                pub mod v1 {
                    include!("proto.api.component.board.v1.rs");
                    // @@protoc_insertion_point(proto.api.component.board.v1)
                }
            }
            pub mod camera {
                // @@protoc_insertion_point(attribute:proto.api.component.camera.v1)
                pub mod v1 {
                    include!("proto.api.component.camera.v1.rs");
                    // @@protoc_insertion_point(proto.api.component.camera.v1)
                }
            }
            pub mod generic {
                // @@protoc_insertion_point(attribute:proto.api.component.generic.v1)
                pub mod v1 {
                    include!("proto.api.component.generic.v1.rs");
                    // @@protoc_insertion_point(proto.api.component.generic.v1)
                }
            }
            pub mod gantry {
                // @@protoc_insertion_point(attribute:proto.api.component.gantry.v1)
                pub mod v1 {
                    include!("proto.api.component.gantry.v1.rs");
                    // @@protoc_insertion_point(proto.api.component.gantry.v1)
                }
            }
            pub mod gripper {
                // @@protoc_insertion_point(attribute:proto.api.component.gripper.v1)
                pub mod v1 {
                    include!("proto.api.component.gripper.v1.rs");
                    // @@protoc_insertion_point(proto.api.component.gripper.v1)
                }
            }
            pub mod inputcontroller {
                // @@protoc_insertion_point(attribute:proto.api.component.inputcontroller.v1)
                pub mod v1 {
                    include!("proto.api.component.inputcontroller.v1.rs");
                    // @@protoc_insertion_point(proto.api.component.inputcontroller.v1)
                }
            }
            pub mod motor {
                // @@protoc_insertion_point(attribute:proto.api.component.motor.v1)
                pub mod v1 {
                    include!("proto.api.component.motor.v1.rs");
                    // @@protoc_insertion_point(proto.api.component.motor.v1)
                }
            }
            pub mod movementsensor {
                // @@protoc_insertion_point(attribute:proto.api.component.movementsensor.v1)
                pub mod v1 {
                    include!("proto.api.component.movementsensor.v1.rs");
                    // @@protoc_insertion_point(proto.api.component.movementsensor.v1)
                }
            }
            pub mod posetracker {
                // @@protoc_insertion_point(attribute:proto.api.component.posetracker.v1)
                pub mod v1 {
                    include!("proto.api.component.posetracker.v1.rs");
                    // @@protoc_insertion_point(proto.api.component.posetracker.v1)
                }
            }
            pub mod sensor {
                // @@protoc_insertion_point(attribute:proto.api.component.sensor.v1)
                pub mod v1 {
                    include!("proto.api.component.sensor.v1.rs");
                    // @@protoc_insertion_point(proto.api.component.sensor.v1)
                }
            }
            pub mod servo {
                // @@protoc_insertion_point(attribute:proto.api.component.servo.v1)
                pub mod v1 {
                    include!("proto.api.component.servo.v1.rs");
                    // @@protoc_insertion_point(proto.api.component.servo.v1)
                }
            }
        }
        pub mod robot {
            // @@protoc_insertion_point(attribute:proto.api.robot.v1)
            pub mod v1 {
                include!("proto.api.robot.v1.rs");
                // @@protoc_insertion_point(proto.api.robot.v1)
            }
        }
        pub mod service {
            pub mod datamanager {
                // @@protoc_insertion_point(attribute:proto.api.service.datamanager.v1)
                pub mod v1 {
                    include!("proto.api.service.datamanager.v1.rs");
                    // @@protoc_insertion_point(proto.api.service.datamanager.v1)
                }
            }
            pub mod shell {
                // @@protoc_insertion_point(attribute:proto.api.service.shell.v1)
                pub mod v1 {
                    include!("proto.api.service.shell.v1.rs");
                    // @@protoc_insertion_point(proto.api.service.shell.v1)
                }
            }
            pub mod slam {
                // @@protoc_insertion_point(attribute:proto.api.service.slam.v1)
                pub mod v1 {
                    include!("proto.api.service.slam.v1.rs");
                    // @@protoc_insertion_point(proto.api.service.slam.v1)
                }
            }
            pub mod motion {
                // @@protoc_insertion_point(attribute:proto.api.service.motion.v1)
                pub mod v1 {
                    include!("proto.api.service.motion.v1.rs");
                    // @@protoc_insertion_point(proto.api.service.motion.v1)
                }
            }
            pub mod navigation {
                // @@protoc_insertion_point(attribute:proto.api.service.navigation.v1)
                pub mod v1 {
                    include!("proto.api.service.navigation.v1.rs");
                    // @@protoc_insertion_point(proto.api.service.navigation.v1)
                }
            }
            pub mod sensors {
                // @@protoc_insertion_point(attribute:proto.api.service.sensors.v1)
                pub mod v1 {
                    include!("proto.api.service.sensors.v1.rs");
                    // @@protoc_insertion_point(proto.api.service.sensors.v1)
                }
            }
            pub mod vision {
                // @@protoc_insertion_point(attribute:proto.api.service.vision.v1)
                pub mod v1 {
                    include!("proto.api.service.vision.v1.rs");
                    // @@protoc_insertion_point(proto.api.service.vision.v1)
                }
            }
        }
    }
    pub mod rpc {
        pub mod webrtc {
            pub mod v1 {
                include!("proto.rpc.webrtc.v1.rs");
            }
        }
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
    pub mod rpc {
        include!("google.rpc.rs");
    }
    pub mod api {
        include!("google.api.rs");
    }
}
