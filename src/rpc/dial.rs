use super::{client_channel::*, webrtc::Options};
use crate::gen::google;
use crate::gen::proto::rpc::v1::{
    auth_service_client::AuthServiceClient, AuthenticateRequest, Credentials,
};
use crate::gen::proto::rpc::webrtc::v1::{
    call_response::Stage, call_update_request::Update,
    signaling_service_client::SignalingServiceClient, CallUpdateRequest,
    OptionalWebRtcConfigRequest,
};
use crate::gen::proto::rpc::webrtc::v1::{
    CallRequest, IceCandidate, Metadata, RequestHeaders, Strings,
};
use crate::rpc::webrtc;
use ::http::header::HeaderName;
use ::http::{
    uri::{Authority, Parts, PathAndQuery, Scheme},
    HeaderValue, StatusCode, Version,
};
use ::webrtc::ice_transport::ice_candidate::{RTCIceCandidate, RTCIceCandidateInit};
use ::webrtc::peer_connection::sdp::session_description::RTCSessionDescription;
use anyhow::{Context, Result};
use core::fmt;
use std::hint;
use std::{
    collections::HashMap,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc, RwLock,
    },
    task::{Context as TaskContext, Poll},
};
use tonic::body::BoxBody;
use tonic::codegen::{http, BoxFuture};
use tonic::transport::{Body, Channel, Uri};
use tower::{Service, ServiceBuilder};
use tower_http::auth::AddAuthorization;
use tower_http::auth::AddAuthorizationLayer;
use tower_http::set_header::{SetRequestHeader, SetRequestHeaderLayer};

type SecretType = String;

#[derive(Clone)]
pub enum ViamChannel {
    Direct(Channel),
    Webrtc(Arc<WebrtcClientChannel>),
}

pub trait CredentialsExt {
    fn new(r#type: String, payload: String) -> Self;
}

impl CredentialsExt for Credentials {
    fn new(r#type: SecretType, payload: String) -> Credentials {
        Credentials { r#type, payload }
    }
}

impl Service<http::Request<BoxBody>> for ViamChannel {
    type Response = http::Response<Body>;
    type Error = tonic::transport::Error;
    type Future = BoxFuture<Self::Response, Self::Error>;

    fn poll_ready(&mut self, cx: &mut TaskContext<'_>) -> Poll<Result<(), Self::Error>> {
        match self {
            Self::Direct(channel) => channel.poll_ready(cx),
            Self::Webrtc(_channel) => Poll::Ready(Ok(())),
        }
    }

    fn call(&mut self, request: http::Request<BoxBody>) -> Self::Future {
        match self {
            Self::Direct(channel) => Box::pin(channel.call(request)),
            Self::Webrtc(channel) => {
                let channel = channel.clone();
                let fut = async move {
                    let (parts, body) = request.into_parts();

                    let stream = channel
                        .new_stream()
                        .lock()
                        .unwrap()
                        .base_stream
                        .stream
                        .clone();
                    let stream_id = stream.id;
                    let metadata = Some(metadata_from_parts(&parts));
                    let headers = RequestHeaders {
                        method: parts.uri.to_string(),
                        metadata,
                        timeout: None,
                    };

                    if let Err(e) = channel.write_headers(&stream, headers).await {
                        log::error!("error writing headers: {e}");
                        channel.close_stream_with_recv_error(stream_id, e);
                    }

                    let data = hyper::body::to_bytes(body).await.unwrap().to_vec();
                    if let Err(e) = channel.write_message(false, Some(stream), data).await {
                        log::error!("error sending message: {e}");
                        channel.close_stream_with_recv_error(stream_id, e);
                    };
                    let recv = match channel.recv_from_stream(stream_id) {
                        Ok(recv) => recv,
                        Err(e) => {
                            log::error!("error receiving response from stream: {e}");
                            channel.close_stream_with_recv_error(stream_id, e);
                            Vec::new()
                        }
                    };

                    let response = http::response::Response::builder()
                        .status(StatusCode::OK)
                        .header("content-type", "application/grpc+tonic")
                        .version(Version::HTTP_2)
                        .body(Body::from(recv))
                        .unwrap();
                    Ok(response)
                };
                Box::pin(fut)
            }
        }
    }
}

#[derive(Debug)]
pub struct DialOptions {
    credentials: Option<Credentials>,
    webrtc_options: Option<Options>,
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
                webrtc_options: None,
            },
        }
    }
}

impl DialBuilder<WantsUri> {
    pub fn uri(self, uri: &str) -> DialBuilder<WantsCredentials> {
        let uri_parts = uri_parts_with_defaults(uri);
        DialBuilder {
            state: WantsCredentials(()),
            config: DialOptions {
                credentials: None,
                uri: Some(uri_parts),
                allow_downgrade: false,
                insecure: false,
                webrtc_options: None,
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
                webrtc_options: None,
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
                webrtc_options: None,
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

    pub fn disable_webrtc(mut self) -> Self {
        let webrtc_options = Options::default().disable_webrtc();
        self.config.webrtc_options = Some(webrtc_options);
        self
    }
}

impl DialBuilder<WithoutCredentials> {
    pub async fn connect(self) -> Result<ViamChannel> {
        // TODO: test me
        let webrtc_options = self.config.webrtc_options;
        let disable_webrtc = match &webrtc_options {
            Some(options) => options.disable_webrtc,
            None => false,
        };
        let mut uri_parts = self.config.uri.unwrap();
        if self.config.insecure {
            uri_parts.scheme = Some(Scheme::HTTP);
        }
        let uri = Uri::from_parts(uri_parts)?;
        let uri2 = uri.clone();
        let uri = infer_remote_uri_from_authority(uri);
        let domain = uri2.authority().to_owned().unwrap().as_str();
        let channel = Channel::builder(uri.clone())
            .connect()
            .await
            .with_context(|| format!("Connecting to {:?}", uri.clone()));
        let channel = match channel {
            Err(e) => {
                if self.config.allow_downgrade {
                    let mut uri_parts = uri.clone().into_parts();
                    uri_parts.scheme = Some(Scheme::HTTP);
                    let uri = Uri::from_parts(uri_parts)?;
                    let c = Channel::builder(uri.clone()).connect().await?;
                    c
                } else {
                    return Err(anyhow::anyhow!(e));
                }
            }
            Ok(c) => c,
        };

        // TODO (RSDK-517) make maybe_connect_via_webrtc take a more generic type so we don't
        // need to add these dummy layers.
        let intercepted_channel = ServiceBuilder::new()
            .layer(AddAuthorizationLayer::basic(
                "fake username",
                "fake password",
            ))
            .layer(SetRequestHeaderLayer::overriding(
                HeaderName::from_static("rpc-host"),
                HeaderValue::from_str(domain)?,
            ))
            .service(channel.clone());

        if disable_webrtc {
            Ok(ViamChannel::Direct(channel.clone()))
        } else {
            match maybe_connect_via_webrtc(uri, intercepted_channel.clone(), webrtc_options).await {
                Ok(webrtc_channel) => Ok(ViamChannel::Webrtc(webrtc_channel)),
                Err(e) => {
                    log::error!("error connecting via webrtc: {e}. Attempting to connect directly");
                    Ok(ViamChannel::Direct(channel.clone()))
                }
            }
        }
    }
}

async fn get_auth_token(channel: &mut Channel, creds: Credentials, entity: &str) -> Result<String> {
    let mut auth_service = AuthServiceClient::new(channel);
    let req = AuthenticateRequest {
        entity: entity.to_string(),
        credentials: Some(creds),
    };

    let rsp = auth_service.authenticate(req).await?;
    Ok(rsp.into_inner().access_token)
}

impl DialBuilder<WithCredentials> {
    pub async fn connect(self) -> Result<AddAuthorization<ViamChannel>> {
        let webrtc_options = self.config.webrtc_options;
        let disable_webrtc = match &webrtc_options {
            Some(options) => options.disable_webrtc,
            None => false,
        };
        let mut uri_parts = self.config.uri.unwrap();
        if self.config.insecure {
            uri_parts.scheme = Some(Scheme::HTTP);
        }

        let uri = Uri::from_parts(uri_parts)?;
        let uri2 = uri.clone();
        let domain = uri2.authority().to_owned().unwrap().as_str();

        let uri = infer_remote_uri_from_authority(uri);

        let real_channel = match Channel::builder(uri.clone())
            .connect()
            .await
            .with_context(|| format!("Connecting to {:?}", uri.clone()))
        {
            Ok(c) => c,
            Err(e) => {
                if self.config.allow_downgrade {
                    let mut uri_parts = uri.clone().into_parts();
                    uri_parts.scheme = Some(Scheme::HTTP);
                    let uri = Uri::from_parts(uri_parts)?;
                    Channel::builder(uri.clone()).connect().await?
                } else {
                    return Err(anyhow::anyhow!(e));
                }
            }
        };

        let token = get_auth_token(
            &mut real_channel.clone(),
            self.config.credentials.unwrap(),
            domain,
        )
        .await?;

        let channel = ServiceBuilder::new()
            .layer(AddAuthorizationLayer::bearer(&token))
            .layer(SetRequestHeaderLayer::overriding(
                HeaderName::from_static("rpc-host"),
                HeaderValue::from_str(domain)?,
            ))
            .service(real_channel.clone());

        let channel = if disable_webrtc {
            ViamChannel::Direct(real_channel.clone())
        } else {
            match maybe_connect_via_webrtc(uri.clone(), channel.clone(), webrtc_options).await {
                Ok(webrtc_channel) => ViamChannel::Webrtc(webrtc_channel),
                Err(e) => {
                    log::error!(
                    "Unable to establish webrtc connection due to error {e}. Attempting direct connection."
                );
                    ViamChannel::Direct(real_channel.clone())
                }
            }
        };

        Ok(ServiceBuilder::new()
            .layer(AddAuthorizationLayer::bearer(&token))
            .service(channel))
    }
}

async fn send_done_or_error_update(
    update: CallUpdateRequest,
    channel: AddAuthorization<SetRequestHeader<Channel, HeaderValue>>,
) {
    let mut signaling_client = SignalingServiceClient::new(channel.clone());

    if let Err(e) = signaling_client
        .call_update(update)
        .await
        .map_err(anyhow::Error::from)
        .map(|_| ())
    {
        log::error!("Error sending done or error update: {e}");
    }
}

async fn send_error_once(
    sent_error: Arc<AtomicBool>,
    uuid: &String,
    err: &anyhow::Error,
    channel: AddAuthorization<SetRequestHeader<Channel, HeaderValue>>,
) {
    if sent_error.load(Ordering::SeqCst) {
        return;
    }

    let err = google::rpc::Status {
        code: google::rpc::Code::Unknown.into(),
        message: err.to_string(),
        details: Vec::new(),
    };
    sent_error.store(true, Ordering::SeqCst);
    let update_request = CallUpdateRequest {
        uuid: uuid.to_string(),
        update: Some(Update::Error(err)),
    };

    send_done_or_error_update(update_request, channel).await
}

async fn send_done_once(
    sent_done: Arc<AtomicBool>,
    uuid: &String,
    channel: AddAuthorization<SetRequestHeader<Channel, HeaderValue>>,
) {
    if sent_done.load(Ordering::SeqCst) {
        return;
    }
    sent_done.store(true, Ordering::SeqCst);
    let update_request = CallUpdateRequest {
        uuid: uuid.to_string(),
        update: Some(Update::Done(true)),
    };

    send_done_or_error_update(update_request, channel).await
}

async fn maybe_connect_via_webrtc(
    uri: Uri,
    channel: AddAuthorization<SetRequestHeader<Channel, HeaderValue>>,
    webrtc_options: Option<Options>,
) -> Result<Arc<WebrtcClientChannel>> {
    let webrtc_options = webrtc_options.unwrap_or_else(|| Options::infer_from_uri(uri.clone()));
    let mut signaling_client = SignalingServiceClient::new(channel.clone());
    let response = match signaling_client
        .optional_web_rtc_config(OptionalWebRtcConfigRequest::default())
        .await
    {
        Ok(resp) => resp,
        Err(e) => {
            return Err(anyhow::anyhow!(e));
        }
    };

    let optional_config = response.into_inner().config;
    let config = webrtc::extend_webrtc_config(webrtc_options.config, optional_config);

    let (peer_connection, data_channel) =
        webrtc::new_peer_connection_for_client(config, webrtc_options.disable_trickle_ice).await?;

    let sent_done_or_error = Arc::new(AtomicBool::new(false));
    let uuid_lock = Arc::new(RwLock::new("".to_string()));
    let uuid2 = uuid_lock.clone();
    let is_open = Arc::new(AtomicBool::new(false));
    let is_open_read = is_open.clone();

    data_channel
        .on_open(Box::new(move || {
            is_open.store(true, Ordering::SeqCst);
            Box::pin(async move {})
        }))
        .await;

    let exchange_done = Arc::new(AtomicBool::new(false));
    let remote_description_set = Arc::new(AtomicBool::new(false));

    if !webrtc_options.disable_trickle_ice {
        let offer = peer_connection.create_offer(None).await?;
        let channel2 = channel.clone();
        let uuid_lock2 = uuid_lock.clone();
        let sent_done_or_error2 = sent_done_or_error.clone();

        let ice_done = Arc::new(AtomicBool::new(false));
        let exchange_done = exchange_done.clone();
        let remote_description_set = remote_description_set.clone();
        peer_connection
            .on_ice_candidate(Box::new(move |ice_candidate: Option<RTCIceCandidate>| {
                let remote_description_set = remote_description_set.clone();
                while !remote_description_set.load(Ordering::SeqCst) {
                    hint::spin_loop();
                }

                if exchange_done.load(Ordering::SeqCst) {
                    return Box::pin(async move {});
                }
                let channel = channel2.clone();
                let sent_done_or_error = sent_done_or_error2.clone();
                let ice_done = ice_done.clone();
                let uuid_lock = uuid_lock2.clone();
                Box::pin(async move {
                    if ice_done.load(Ordering::SeqCst) {
                        return;
                    }
                    let uuid = uuid_lock.read().unwrap().to_string();
                    let mut signaling_client = SignalingServiceClient::new(channel.clone());
                    match ice_candidate {
                        Some(ice_candidate) => {
                            if sent_done_or_error.load(Ordering::SeqCst) {
                                return;
                            }
                            let proto_candidate = ice_candidate_to_proto(ice_candidate).await;
                            match proto_candidate {
                                Ok(proto_candidate) => {
                                    let update_request = CallUpdateRequest {
                                        uuid: uuid.clone(),
                                        update: Some(Update::Candidate(proto_candidate)),
                                    };
                                    if let Err(e) =
                                        signaling_client.call_update(update_request).await
                                    {
                                        log::error!("Error sending ice candidate: {e}");
                                    }
                                }
                                Err(e) => log::error!("Error parsing ice candidate: {e}"),
                            }
                        }
                        None => {
                            ice_done.store(true, Ordering::SeqCst);
                            send_done_once(sent_done_or_error, &uuid, channel.clone()).await;
                        }
                    }
                })
            }))
            .await;

        peer_connection.set_local_description(offer).await?;
    }

    let local_description = peer_connection.local_description().await.unwrap();
    let sdp = encode_sdp(local_description)?;
    let call_request = CallRequest {
        sdp,
        disable_trickle: webrtc_options.disable_trickle_ice,
    };

    let client_channel = WebrtcClientChannel::new(peer_connection, data_channel).await;
    let client_channel2 = client_channel.clone();
    let mut signaling_client = SignalingServiceClient::new(channel.clone());
    let mut call_client = signaling_client.call(call_request).await?.into_inner();

    let channel2 = channel.clone();
    let sent_done_or_error2 = sent_done_or_error.clone();
    tokio::spawn(async move {
        let init_received = AtomicBool::new(false);
        let sent_done = sent_done_or_error2;

        loop {
            let response = match call_client.message().await {
                Ok(cr) => match cr {
                    Some(cr) => cr,
                    None => {
                        let uuid = uuid2.read().unwrap().to_string();
                        send_done_once(sent_done.clone(), &uuid, channel2.clone()).await;
                        break;
                    }
                },
                Err(e) => {
                    log::error!("Error processing call response: {e}");
                    continue;
                }
            };

            match response.stage {
                Some(Stage::Init(init)) => {
                    if init_received.load(Ordering::SeqCst) {
                        let uuid = uuid2.read().unwrap().to_string();
                        let e = anyhow::anyhow!("Init received more than once");
                        send_error_once(sent_done.clone(), &uuid, &e, channel2.clone()).await;
                        break;
                    }
                    init_received.store(true, Ordering::SeqCst);
                    {
                        let mut uuid_s = uuid2.write().unwrap();
                        *uuid_s = response.uuid.clone();
                    }

                    let answer = match decode_sdp(init.sdp) {
                        Ok(a) => a,
                        Err(e) => {
                            send_error_once(
                                sent_done.clone(),
                                &response.uuid,
                                &e,
                                channel2.clone(),
                            )
                            .await;
                            break;
                        }
                    };

                    if let Err(e) = client_channel2
                        .base_channel
                        .peer_connection
                        .set_remote_description(answer)
                        .await
                    {
                        let e = anyhow::Error::from(e);
                        send_error_once(sent_done.clone(), &response.uuid, &e, channel2.clone())
                            .await;
                        break;
                    }
                    remote_description_set.store(true, Ordering::SeqCst);
                    if webrtc_options.disable_trickle_ice {
                        send_done_once(sent_done.clone(), &response.uuid, channel2.clone()).await;
                        break;
                    }
                }

                Some(Stage::Update(update)) => {
                    let uuid = uuid2.read().unwrap().to_string();
                    if !init_received.load(Ordering::SeqCst) {
                        let e = anyhow::anyhow!("Got update before init stage");
                        send_error_once(sent_done.clone(), &uuid, &e, channel2.clone()).await;
                        break;
                    }

                    if response.uuid != *uuid2.read().unwrap() {
                        let e = anyhow::anyhow!(
                            "uuid mismatch: have {}, want {}",
                            response.uuid,
                            uuid,
                        );
                        send_error_once(sent_done.clone(), &uuid, &e, channel2.clone()).await;
                        break;
                    }
                    match ice_candidate_from_proto(update.candidate) {
                        Ok(candidate) => {
                            if let Err(e) = client_channel2
                                .base_channel
                                .peer_connection
                                .add_ice_candidate(candidate)
                                .await
                            {
                                let e = anyhow::Error::from(e);
                                send_error_once(sent_done.clone(), &uuid, &e, channel2.clone())
                                    .await;
                                break;
                            }
                        }
                        Err(e) => log::error!("Error adding ice candidate: {e}"),
                    }
                }
                None => continue,
            }
        }
    });

    // TODO (GOUT-11): create separate authorization if external_auth_addr and/or creds.Type is `Some`

    // Delay returning the client channel until data channel is open, so we don't lose messages
    while !is_open_read.load(Ordering::SeqCst) {
        hint::spin_loop();
    }

    exchange_done.store(true, Ordering::SeqCst);
    let uuid = uuid_lock.read().unwrap().to_string();
    send_done_once(sent_done_or_error, &uuid, channel.clone()).await;

    Ok(client_channel)
}

async fn ice_candidate_to_proto(ice_candidate: RTCIceCandidate) -> Result<IceCandidate> {
    let ice_candidate = ice_candidate.to_json().await?;
    Ok(IceCandidate {
        candidate: ice_candidate.candidate,
        sdp_mid: ice_candidate.sdp_mid,
        sdpm_line_index: ice_candidate.sdp_mline_index.map(u32::from),
        username_fragment: ice_candidate.username_fragment,
    })
}

fn ice_candidate_from_proto(proto: Option<IceCandidate>) -> Result<RTCIceCandidateInit> {
    match proto {
        Some(proto) => {
            let proto_sdpm: usize = proto.sdpm_line_index().try_into()?;
            let sdp_mline_index: Option<u16> = proto_sdpm.try_into().ok();

            Ok(RTCIceCandidateInit {
                candidate: proto.candidate.clone(),
                sdp_mid: Some(proto.sdp_mid().to_string()),
                sdp_mline_index,
                username_fragment: Some(proto.username_fragment().to_string()),
            })
        }
        None => Err(anyhow::anyhow!("No ice candidate provided")),
    }
}

fn decode_sdp(sdp: String) -> Result<RTCSessionDescription> {
    let sdp = String::from_utf8(base64::decode(sdp)?)?;
    Ok(serde_json::from_str::<RTCSessionDescription>(&sdp)?)
}

fn encode_sdp(sdp: RTCSessionDescription) -> Result<String> {
    let sdp = serde_json::to_vec(&sdp)?;
    Ok(base64::encode(sdp))
}

pub fn infer_remote_uri_from_authority(uri: Uri) -> Uri {
    let is_local_connection = uri
        .authority()
        .map(Authority::as_str)
        .unwrap_or_default()
        .contains(".local");

    if !is_local_connection {
        if let Some((new_uri, _)) = Options::infer_signaling_server_address(&uri) {
            return Uri::from_parts(uri_parts_with_defaults(&new_uri)).unwrap_or(uri);
        }
    }
    uri
}

fn uri_parts_with_defaults(uri: &str) -> Parts {
    let mut uri_parts = uri.parse::<Uri>().unwrap().into_parts();
    uri_parts.scheme = Some(Scheme::HTTPS);
    uri_parts.path_and_query = Some(PathAndQuery::from_static(""));
    uri_parts
}

fn metadata_from_parts(parts: &http::request::Parts) -> Metadata {
    let mut md = HashMap::new();
    for (k, v) in parts.headers.iter() {
        let k = k.to_string();
        let v = Strings {
            values: vec![HeaderValue::to_str(v).unwrap().to_string()],
        };
        md.insert(k, v);
    }
    Metadata { md }
}
