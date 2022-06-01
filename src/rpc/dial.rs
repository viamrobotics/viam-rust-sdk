use crate::gen::proto::rpc::v1::{
    auth_service_client::AuthServiceClient, AuthenticateRequest, Credentials,
};
use anyhow::Context;
use anyhow::Result;
use core::fmt;
use http::uri::Parts;
use tonic::{transport::Channel, transport::Uri};
use tower::ServiceBuilder;
use tower_http::auth::AddAuthorization;
use tower_http::auth::AddAuthorizationLayer;

type SecretType = String;

pub trait CredentialsExt {
    fn new(r#type: String, payload: String) -> Self;
}

impl CredentialsExt for Credentials {
    fn new(r#type: SecretType, payload: String) -> Credentials {
        Credentials { r#type, payload }
    }
}

#[derive(Debug)]
pub struct DialOptions {
    credentials: Option<Credentials>,
    uri: Option<Parts>,
    allow_downgrade: bool,
    insecure: bool,
}
#[derive(Clone)]
pub struct WantsCredentials(());
#[derive(Clone)]
pub struct WantsUri(());
#[derive(Clone)]
pub struct WithCredentials(());
#[derive(Clone)]
pub struct WithoutCredentials(());

pub trait AuthMethod {}
impl AuthMethod for WithCredentials {}
impl AuthMethod for WithoutCredentials {}
#[allow(dead_code)]
pub struct DialBuilder<T> {
    state: T,
    config: DialOptions,
}

impl<T> fmt::Debug for DialBuilder<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Dial")
            .field("State", &format_args!("{}", &std::any::type_name::<T>()))
            .field("Opt", &format_args!("{:?}", self.config))
            .finish()
    }
}

impl DialOptions {
    pub fn builder() -> DialBuilder<WantsUri> {
        DialBuilder {
            state: WantsUri(()),
            config: DialOptions {
                credentials: None,
                uri: None,
                allow_downgrade: false,
                insecure: false,
            },
        }
    }
}

impl DialBuilder<WantsUri> {
    pub fn uri(self, uri: &str) -> DialBuilder<WantsCredentials> {
        let mut uri_parts = uri.parse::<Uri>().unwrap().into_parts();
        uri_parts.scheme = Some(http::uri::Scheme::HTTPS);
        uri_parts.path_and_query = Some(http::uri::PathAndQuery::from_static(""));
        DialBuilder {
            state: WantsCredentials(()),
            config: DialOptions {
                credentials: None,
                uri: Some(uri_parts),
                allow_downgrade: false,
                insecure: false,
            },
        }
    }
}
impl DialBuilder<WantsCredentials> {
    pub fn without_credentials(self) -> DialBuilder<WithoutCredentials> {
        DialBuilder {
            state: WithoutCredentials(()),
            config: DialOptions {
                credentials: None,
                uri: self.config.uri,
                allow_downgrade: false,
                insecure: false,
            },
        }
    }
    pub fn with_credentials(self, creds: Credentials) -> DialBuilder<WithCredentials> {
        DialBuilder {
            state: WithCredentials(()),
            config: DialOptions {
                credentials: Some(creds),
                uri: self.config.uri,
                allow_downgrade: false,
                insecure: false,
            },
        }
    }
}

impl<T: AuthMethod> DialBuilder<T> {
    pub fn insecure(mut self) -> Self {
        self.config.insecure = true;
        self
    }
    pub fn allow_downgrade(mut self) -> Self {
        self.config.allow_downgrade = true;
        self
    }
}

impl DialBuilder<WithoutCredentials> {
    pub async fn connect(self) -> Result<Channel> {
        let mut uri_parts = self.config.uri.unwrap();
        if self.config.insecure {
            uri_parts.scheme = Some(http::uri::Scheme::HTTP);
        }
        let uri = Uri::from_parts(uri_parts)?;
        let channel = Channel::builder(uri.clone())
            .connect()
            .await
            .with_context(|| format!("Connecting to {:?}", uri.clone()));
        match channel {
            Err(e) => {
                if self.config.allow_downgrade {
                    let mut uri_parts = uri.clone().into_parts();
                    uri_parts.scheme = Some(http::uri::Scheme::HTTP);
                    let uri = Uri::from_parts(uri_parts)?;
                    Ok(Channel::builder(uri.clone()).connect().await?)
                } else {
                    Err(anyhow::anyhow!(e))
                }
            }
            Ok(c) => Ok(c),
        }
    }
}
async fn get_auth_token(channel: Channel, creds: Credentials, entity: &str) -> Result<String> {
    let mut auth_service = AuthServiceClient::new(channel);
    let req = AuthenticateRequest {
        entity: entity.to_string(),
        credentials: Some(creds),
    };
    let rsp = auth_service.authenticate(req).await?;
    Ok(rsp.into_inner().access_token)
}

impl DialBuilder<WithCredentials> {
    pub async fn connect(self) -> Result<AddAuthorization<Channel>> {
        let mut uri_parts = self.config.uri.unwrap();
        if self.config.insecure {
            uri_parts.scheme = Some(http::uri::Scheme::HTTP);
        }
        let uri = Uri::from_parts(uri_parts)?;
        let real_channel;
        let channel = Channel::builder(uri.clone())
            .connect()
            .await
            .with_context(|| format!("Connecting to {:?}", uri.clone()));
        match channel {
            Err(e) => {
                if self.config.allow_downgrade {
                    let mut uri_parts = uri.clone().into_parts();
                    uri_parts.scheme = Some(http::uri::Scheme::HTTP);
                    let uri = Uri::from_parts(uri_parts)?;
                    real_channel = Channel::builder(uri.clone()).connect().await?;
                } else {
                    return Err(anyhow::anyhow!(e));
                }
            }
            Ok(c) => {
                real_channel = c;
            }
        }
        let token = get_auth_token(
            real_channel.clone(),
            self.config.credentials.unwrap(),
            uri.authority().unwrap().as_str(),
        )
        .await?;
        let channel = ServiceBuilder::new()
            .layer(AddAuthorizationLayer::bearer(&token))
            .service(real_channel);
        Ok(channel)
    }
}
