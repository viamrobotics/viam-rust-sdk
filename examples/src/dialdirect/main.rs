use anyhow::Result;
use viam::gen::proto::api::robot::v1::{robot_service_client, ResourceNamesRequest};
use viam::rpc::dial;

#[derive(Clone)]
pub struct DynCodec;

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();
    //let creds = dial::CredentialsExt::new(
    //"robot-location-secret".to_string(),
    //"huwbahasm4zt3un5hfag7oy1d0lseie5uzr9camkolxh81g4".to_string(),
    ////"ytexnwei4fu1xv9csoqxfv4ckl3htsb49mzzey5t15xo9swy".to_string(),
    //);
    let creds = dial::CredentialsExt::new(
        String::from("robot-location-secret"),
        //String::from("huwbahasm4zt3un5hfag7oy1d0lseie5uzr9camkolxh81g4"),
        //String::from("67uwqotidsyb2guk0qz514qxjefdtjvwthnr2vbqz2qa9tc5"),
        String::from("ytexnwei4fu1xv9csoqxfv4ckl3htsb49mzzey5t15xo9swy"), // webrtc
                                                                          //String::from("og3d31ekzzxyopx44m37o4wcg1tz2oyisl4kxu4gbqiyvpfe"), // franke
    );
    let c = dial::DialOptions::builder()
        //.uri("vg2105m04.6099bd9d94.viam.cloud")
        .uri("webrtc-test-main.jkek76kqnh.viam.cloud")
        //.uri("my-cool-robot-main.ue1p0073ug.local.robot.viaminternal:8080")
        //.uri("my-cool-robot-main.ue1p0073ug.robot.viaminternal")
        //.uri("my-cool-robot-main.ue1p0073ug.viam.cloud")
        //.uri("webrtc-test-main.jkek76kqnh.local.viam.cloud:8080")
        //.uri("franke-main.6xs7zv3bxz.viam.cloud")
        //.uri("https://app.viam.com:443")
        //.uri("remote-1-main.33vvxnbbw9.local.viam.cloud:8080")
        .with_credentials(creds)
        .connect()
        .await?;

    let mut service = robot_service_client::RobotServiceClient::new(c);
    println!("Get the robot's metadata");
    let _rsp = service
        .resource_names(tonic::Request::new(ResourceNamesRequest {}))
        .await?;
    println!("Rsp {:?}", _rsp);
    let _rsp = service
        .resource_names(tonic::Request::new(ResourceNamesRequest {}))
        .await?;
    println!("Rsp {:?}", _rsp);
    Ok(())
}
