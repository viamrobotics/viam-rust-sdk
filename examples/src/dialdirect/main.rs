use anyhow::Result;
use viam::gen::proto::api::robot::v1::{robot_service_client, ResourceNamesRequest};
use viam::rpc::dial;

#[tokio::main]
async fn main() -> Result<()> {
    let creds = dial::CredentialsExt::new(
        String::from("robot-location-secret"),
        String::from("9x375brdv1f7u2v6vi42a21cbzo0xuauov025wox5mg9grd5"),
    );
    let c = dial::DialOptions::builder()
        .uri("test-main.33vvxnbbw9.local.viam.cloud:8080")
        .with_credentials(creds)
        .connect()
        .await?;
    println!("Get the robot's metadata");
    let mut service = robot_service_client::RobotServiceClient::new(c);
    let _rsp = service
        .resource_names(tonic::Request::new(ResourceNamesRequest {}))
        .await?;
    println!("Rsp {:?}", _rsp);
    Ok(())
}
