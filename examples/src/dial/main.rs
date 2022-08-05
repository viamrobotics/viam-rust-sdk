use anyhow::Result;
use viam::gen::proto::api::robot::v1::{
    robot_service_client, ResourceNamesRequest, ResourceNamesResponse,
};
use viam::rpc::dial;

async fn dial_direct() -> Result<tonic::Response<ResourceNamesResponse>> {
    let creds = dial::CredentialsExt::new(
        "robot-location-secret".to_string(),
        "9x375brdv1f7u2v6vi42a21cbzo0xuauov025wox5mg9grd5".to_string(),
    );

    let c = dial::DialOptions::builder()
        .uri("test-main.33vvxnbbw9.local.viam.cloud:8080")
        .with_credentials(creds)
        .connect()
        .await?;

    let mut service = robot_service_client::RobotServiceClient::new(c);
    println!("Getting direct robot's metadata");
    service
        .resource_names(tonic::Request::new(ResourceNamesRequest {}))
        .await
        .map_err(anyhow::Error::from)
}

async fn dial_webrtc() -> Result<tonic::Response<ResourceNamesResponse>> {
    let creds = dial::CredentialsExt::new(
        "robot-location-secret".to_string(),
        "ytexnwei4fu1xv9csoqxfv4ckl3htsb49mzzey5t15xo9swy".to_string(),
    );

    let c = dial::DialOptions::builder()
        .uri("webrtc-test-main.jkek76kqnh.viam.cloud")
        .with_credentials(creds)
        .connect()
        .await?;

    let mut service = robot_service_client::RobotServiceClient::new(c);
    service
        .resource_names(tonic::Request::new(ResourceNamesRequest {}))
        .await
        .map_err(anyhow::Error::from)
}

#[tokio::main]
async fn main() {
    env_logger::init();
    println!("Testing direct dial...");
    match dial_direct().await {
        Ok(rsp) => println!("rsp: {rsp:?}"),
        Err(e) => println!("Error connecting directly: {e}"),
    }

    println!("Testing webrtc dial...");
    match dial_webrtc().await {
        Ok(rsp) => println!("rsp: {rsp:?}"),
        Err(e) => println!("Error connecting via webrtc: {e}"),
    };
}
