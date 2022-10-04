use anyhow::Result;
use viam::gen::proto::rpc::examples::echo::v1::echo_service_client::EchoServiceClient;
use viam::gen::proto::rpc::examples::echo::v1::{
    EchoBiDiRequest, EchoMultipleRequest, EchoRequest,
};
use viam::rpc::dial;

#[tokio::main]
/// Tests unary, server, and bidi streaming with simple echo requests. To run, simply
/// update the credentials and uri as necessary.
async fn main() -> Result<()> {
    let creds = dial::CredentialsExt::new(
        "robot-location-secret".to_string(),
        "ytexnwei4fu1xv9csoqxfv4ckl3htsb49mzzey5t15xo9swy".to_string(),
    );

    let c = dial::DialOptions::builder()
        .uri("webrtc-test-main.jkek76kqnh.viam.cloud")
        .with_credentials(creds)
        .allow_downgrade()
        .connect()
        .await?;

    let mut service = EchoServiceClient::new(c);
    let echo_request = EchoRequest {
        message: "hi".to_string(),
    };
    let resp = service.echo(echo_request).await?.into_inner();
    println!("resp: {resp:?}");

    let multi_echo_request = EchoMultipleRequest {
        message: "hello?".to_string(),
    };
    let mut resp = service
        .echo_multiple(multi_echo_request)
        .await?
        .into_inner();

    while let Some(resp) = resp.message().await? {
        println!("multiple response: {resp:?}");
    }

    let bidi_stream = async_stream::stream! {
        for i in 0..3 {
            let request =
            EchoBiDiRequest {
                message: i.to_string()
            };
            yield request;
        }
    };

    let mut bidi_resp = service.echo_bi_di(bidi_stream).await?.into_inner();
    while let Some(resp) = bidi_resp.message().await? {
        println!("Bidi response: {resp:?}");
    }

    Ok(())
}
