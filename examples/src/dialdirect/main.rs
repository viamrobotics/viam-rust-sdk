use anyhow::Result;
use viam::gen::proto::api::service::metadata::v1::{metadata_service_client, ResourcesRequest};
use viam::rpc::dial;

#[tokio::main]
async fn main() -> Result<()> {
    let creds = dial::CredentialsExt::new(
        String::from("robot-location-secret"),
        String::from("9x375brdv1f7u2v6vi42a21cbzo0xuauov025wox5mg9grd5"),
    );
    let c = dial::DialConfig::builder()
        .uri(&"test-main.33vvxnbbw9.local.viam.cloud:8080".to_string())
        .with_credentials(creds)
        .connect()
        .await?;
    println!("Get the robot's metadata");
    let mut service = metadata_service_client::MetadataServiceClient::new(c);
    let _rsp = service
        .resources(tonic::Request::new(ResourcesRequest {}))
        .await?;
    println!("Rsp {:?}", _rsp);
    Ok(())
}
