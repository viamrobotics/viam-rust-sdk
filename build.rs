fn main() -> Result<(), Box<dyn std::error::Error>> {
    let iface_files = &[
        "proto/api/component/forcematrix/v1/force_matrix.proto",
        "proto/api/component/gps/v1/gps.proto",
        "proto/api/component/board/v1/board.proto",
        "proto/api/component/camera/v1/camera.proto",
        "proto/api/component/servo/v1/servo.proto",
        "proto/api/component/inputcontroller/v1/input_controller.proto",
        "proto/api/component/gripper/v1/gripper.proto",
        "proto/api/component/imu/v1/imu.proto",
        "proto/api/component/motor/v1/motor.proto",
        "proto/api/component/sensor/v1/sensor.proto",
        "proto/api/component/posetracker/v1/pose_tracker.proto",
        "proto/api/component/arm/v1/arm.proto",
        "proto/api/component/gantry/v1/gantry.proto",
        "proto/api/component/base/v1/base.proto",
        "proto/api/robot/v1/robot.proto",
        "proto/api/common/v1/common.proto",
        "proto/api/service/sensors/v1/sensors.proto",
        "proto/api/service/objectmanipulation/v1/object_manipulation.proto",
        "proto/api/service/objectsegmentation/v1/object_segmentation.proto",
        "proto/api/service/navigation/v1/navigation.proto",
        "proto/api/service/status/v1/status.proto",
        "proto/api/service/framesystem/v1/frame_system.proto",
        "proto/api/service/metadata/v1/metadata.proto",
        "proto/rpc/v1/auth.proto",
        "proto/rpc/examples/echo/v1/echo.proto",
    ];
    let dirs = &[".", "proto/"];
    tonic_build::configure()
        .build_client(true)
        .include_file("mod.rs")
        .out_dir("src/gen")
        .compile(iface_files, dirs)?;
    Ok(())
}
