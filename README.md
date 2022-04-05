# Viam Rust SDK

## Prerequisites

### Installing Rust for Mac and Unix-Like
Prior completing these steps make sure no other installations of Rust are present, for example for Mac you want to run `brew uninstall rust`

Next run the following command `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh` in your favorite terminal, if you have installed rustup in the past you can just run `rustup update`. The PATH environment variable should be updated by the installer, running `which rustc` you should see something like `~/.cargo/bin/rustc` if not you may need to reload your terminal and if it still doesn't work then you should add `~/.cargo/bin` in your PATH environment variable (more info [here](https://www.rust-lang.org/tools/install))


## Repository Layout
- `proto` All the google and viam api proto files
- `src` The implementation of the Rust sdk library
- `examples` A list of examples

## Getting Started
First you may want to build the SDK library, to do so navigate to the root of the repository and run `cargo build` this should pull all the dependencies and hopefully build the library!
You can locally build the library documentation by running `cargo doc --open`, it will open your browser pointing to the documentation
Lastly you can run one of the examples by navigation to the examples folder and run `cargo run --bin test-dialdirect`

## Usage
Tu use the SDK and connect to an RDK server you will need to setup a couple of things.

### Create a new project with Cargo
Outside of the SDK repository simply run `cargo new my-robot` this will create a new project named my-robot as well as a couple of files

### Configure Cargo.toml
The details of Cargo.toml are beyond the scope of this README please refer to [this](https://doc.rust-lang.org/cargo/reference/manifest.html) for more info
Add the following to your Cargo.toml :
``` toml
[dependencies]
viam = {path = "../"} #this path should point to the Rust SDK repository
anyhow = { version = "1.0", features = ["backtrace"]}
tokio = { version = "1.0", features = [ "rt-multi-thread", "time", "fs", "macros", "net",] }
tonic = {version = "0.6.2",features = ["tls", "compression", "tls-roots"]}
```
### Change main.rs
At the top of the file add :

``` rust
use anyhow::Result;
use viam::gen::proto::api::service::metadata::v1::{metadata_service_client, ResourcesRequest};
use viam::rpc::dial;
```
Replace the main function signature with :

``` rust
#[tokio::main]
async fn main() -> Result<()> {
    println!("Hello, world!");
}
```
### Connecting to a robot

If you are connecting to a robot with authentication you will need to create credentials :

``` rust
let creds = dial::CredentialsExt::new(
        String::from("robot-location-secret"),
        String::from("secret"),
    );
```
Then to obtain a channel do :

``` rust
let c = dial::DialConfig::builder()
        .uri(&"test-main.33vvxnbbw9.local.viam.cloud:8080".to_string()) // Robot address
        .with_credentials(creds) // credentials
        .connect()
        .await?; // if the connection complete you will have a channel otherwise an error
```

Now if we want to get the metadata of the robots we just have to do :

``` rust
let mut service = metadata_service_client::MetadataServiceClient::new(c);
    let _rsp = service
        .resources(tonic::Request::new(ResourcesRequest {}))
        .await?;
    println!("Rsp {:?}", _rsp);
```
Note that after that the channel has been moved and therefore will not be usable again, to avoid this clone it first.

If you want to connect to a robot without credentials then just do :

``` rust
let c = dial::DialConfig::builder()
        .uri(&"localhost:8080".to_string()) 
        .without_credentials()
        .insecure() // you can also do allow_downgrade()
        .connect()
        .await?; // if the connection complete you will have a channel otherwise an error
```
