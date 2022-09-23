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
Lastly you can run one of the examples by navigation to the examples folder and run `cargo run --bin test-dial`

## Usage
To use the SDK and connect to an RDK server you will need to setup a couple of things.

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
use viam::gen::proto::api::robot::v1::{robot_service_client, ResourceNamesRequest};
use viam::rpc::dial;
```
Replace the main function signature with :

``` rust
#[tokio::main]
async fn main() -> Result<()> {
    println!("Hello, world!");
}
```
### Echo Streaming Example
The echo example communicates with the goutils sample server. It demonstrates individual, streamed, and bidirectional communication. To test, navigate to your goutils clone and run

``` shell
go run rpc/examples/echo/server/cmd/main.go
```
Take note of the signaling port and replace the port value in examples/src/echo/main.rs with yours like this :

``` rust
let c = dial::DialOptions::builder()
    .uri("localhost:<your-port>")
    .without_credentials()
    .allow_downgrade()
    .connect
    .await?;
```
Then, from the `examples/` directory, run 

``` shell
cargo run --bin test-echo
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
        .uri("test-main.33vvxnbbw9.local.viam.cloud:8080") // Robot address
        .with_credentials(creds) // credentials
        .connect()
        .await?; // if the connection complete you will have a channel otherwise an error
```

Now if we want to get the metadata of the robots we just have to do :

``` rust
let mut service = robot_service_client::RobotServiceClient::new(c);
    let _rsp = service
        .resource_names(tonic::Request::new(ResourceNamesRequest {}))
        .await?;
    println!("Rsp {:?}", _rsp);
```
Note that after that the channel has been moved and therefore will not be usable again, to avoid this clone it first.

If you want to connect to a robot without credentials then just do :

``` rust
let c = dial::DialConfig::builder()
        .uri("localhost:8080") 
        .without_credentials()
        .insecure() // you can also do allow_downgrade()
        .connect()
        .await?; // if the connection complete you will have a channel otherwise an error
```

Note also that this will attempt to connect over webRTC by default. To override and only use direct gRPC calls, add a `disable_webrtc()` call to your dial builder, like so:

``` rust
let c = dial::DialConfig::builder()
        .uri("test-main.33vvxnbbw9.local.viam.cloud:8080") // Robot address
        .with_credentials(creds) // credentials
        .disable_webrtc() // forces gRPC connection
        .connect()
        .await?; // if the connection complete you will have a channel otherwise an error
```

## Using FFI (Foreign Functions Interface)
The rust sdk exposes a few functions conforming to the platform's C calling convention. Most languages support calling C code but their particular implementation is beyond the scope of this README. However we provide example in C++.

### Set up
For now we only support Unix-like systems. Before continuing make sure that GRPCCpp has been installed on your system.
Navigate to :

``` shell
cd examples/src/ffi/cpp
# Then
make buf
```

### Echo example
The echo example communicate with the goutils sample server, navigate to your goutils clone and run

``` shell
go run rpc/examples/echo/server/cmd/main.go
```

Then run 

``` shell
make ffi_echo && ./ffi_echo
```

### Robot example
The robot example communicate with a rdk server
Update the dial function with your address and secret in the file ffi_robot.cc

``` c++
dial("<robot-address>",
            "<robot-secret>",
            false, ptr);
```
Then run 

``` shell
make ffi_robot && ./ffi_robot
```

## Two Notes on Connectivity and webRTC Functionality
First: the rust SDK attempts to dial over webRTC by default. You can override this by calling `disable_webrtc()` on the dial builder.

Second: the rust webRTC implementation is still new, and liable to have bugs. At a minimum, we expect that calls to `ShellService::shell()` have a high likelihood of strange behavior. If you encounter any issues with streaming requests over webRTC, direct dial (by disabling webrtc as above) should resolve them. And please file a bug report! We will endeavor to be as responsive as possible, and resolve issues as quickly as possible.

## License 
Copyright 2021-2022 Viam Inc.

Apache 2.0 - See [LICENSE](https://github.com/viamrobotics/viam-rust-sdk/blob/main/LICENSE) file
