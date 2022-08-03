use super::{client_channel::*, webrtc::Options};
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
use std::str::FromStr;
use std::sync::Once;
use std::{
    collections::HashMap,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc, RwLock,
    },
    task::{Context as TaskContext, Poll},
    time::Duration,
};
use tonic::body::BoxBody;
use tonic::codegen::{http, BoxFuture, InterceptedService};
use tonic::metadata::{Ascii, Binary, BinaryMetadataValue, MetadataValue};
use tonic::{
    metadata::AsciiMetadataValue,
    transport::{Body, Channel, Uri},
    Request, Status,
};
use tower::{Service, ServiceBuilder};
use tower_http::auth::AddAuthorization;
use tower_http::auth::AddAuthorizationLayer;
use tower_http::sensitive_headers::SetSensitiveHeaders;
use tower_http::set_header::{SetRequestHeader, SetRequestHeaderLayer};
use tower_http::ServiceBuilderExt;

type SecretType = String;

#[derive(Clone)]
// CR erodkin: rename
pub enum GenericChannel {
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

impl Service<http::Request<BoxBody>> for GenericChannel {
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

                    let stream = channel.new_stream().lock().unwrap().stream.clone();
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
        let uri_parts = uri_parts_with_defaults(uri);
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
    pub async fn connect(self) -> Result<GenericChannel> {
        // CR erodkin: delete me. but first test me.
        println!("doing anything without credentials la la la");
        let mut uri_parts = self.config.uri.unwrap();
        if self.config.insecure {
            uri_parts.scheme = Some(Scheme::HTTP);
        }
        let uri = Uri::from_parts(uri_parts)?;
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
        let intercepted_channel = ServiceBuilder::new()
            //.layer(tonic::service::interceptor(Ok))
            // CR erodkin: sad that we have this here at all. can we avoid it?
            // CR erodkin: can we use the AddAuthorizationLayer again, or are we sol on that?
            .layer(AddAuthorizationLayer::basic(
                "fake username",
                "fake password",
            ))
            .layer(SetRequestHeaderLayer::overriding(
                HeaderName::from_str("fake")?,
                HeaderValue::from_str("fake")?,
            ))
            .service(channel.clone());

        match maybe_connect_via_webrtc(uri, intercepted_channel.clone()).await {
            Ok(webrtc_channel) => Ok(GenericChannel::Webrtc(webrtc_channel)),
            Err(e) => {
                log::error!("error connecting via webrtc: {e}. Attempting to connect directly");
                Ok(GenericChannel::Direct(channel.clone()))
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

    let rsp = auth_service.authenticate(req).await;
    match rsp {
        Err(e) => Err(anyhow::anyhow!("Err acquiring auth token: {}", e)),
        Ok(rsp) => Ok(rsp.into_inner().access_token),
    }
}

impl DialBuilder<WithCredentials> {
    pub async fn connect(self) -> Result<AddAuthorization<GenericChannel>> {
        let mut uri_parts = self.config.uri.unwrap();
        if self.config.insecure {
            uri_parts.scheme = Some(Scheme::HTTP);
        }

        let uri = Uri::from_parts(uri_parts)?;
        let uri2 = uri.clone();
        let domain = uri2.authority().to_owned().unwrap().as_str();

        let uri = infer_remote_uri_from_domain(uri);

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
        // CR erodkin: this sure is ugly! see if we can't fix it up. looks like nicolas already
        // figured out how, check out the main branch
        let metadata = AsciiMetadataValue::try_from(&*format!("Bearer {}", token))?;
        let host = AsciiMetadataValue::try_from(domain)?;
        //let host = BinaryMetadataValue::fro(domain);
        // CR erodkin: see if we need webrtc layer at all
        //let webrtc_layer = tonic::service::interceptor(move |mut req: Request<()>| {
        //req.metadata_mut().insert("authorization", metadata.clone());
        //req.metadata_mut().insert("rpc-host", host.clone());
        //req.metadata_mut()
        //.insert("content-type", "application/grpc+tonic".parse().unwrap());
        //Ok(req)
        //});

        let channel = ServiceBuilder::new()
            //.override_request_header(HeaderName::from_static("rpc-host"), host.clone())
            .layer(AddAuthorizationLayer::bearer(&token))
            .layer(SetRequestHeaderLayer::overriding(
                HeaderName::from_static("rpc-host"),
                HeaderValue::from_str(domain)?, //domain,
                                                //host.clone(),
            ))
            //.layer(SetRequestHeader::overriding("rpc-host", host.clone())
            //.layer(webrtc_layer)
            .service(real_channel.clone());
        //Ok(channel)

        //let channel = ServiceBuilder::new()
        //.layer(&layer)
        //.service(real_channel.clone());

        let channel = match maybe_connect_via_webrtc(uri.clone(), channel.clone()).await {
            Ok(webrtc_channel) => GenericChannel::Webrtc(webrtc_channel),
            Err(e) => {
                log::error!(
                    "Unable to establish webrtc connection due to error {e}. 
                    Attempting direct connection."
                );
                GenericChannel::Direct(real_channel.clone())
            }
        };

        Ok(ServiceBuilder::new()
            .layer(AddAuthorizationLayer::bearer(&token))
            .service(channel))
    }
}

async fn send_done_once(
    sent_done: Arc<AtomicBool>,
    uuid: String,
    channel: AddAuthorization<SetRequestHeader<Channel, HeaderValue>>,
) -> Result<()> {
    if sent_done.load(Ordering::SeqCst) {
        return Ok(());
    }
    sent_done.store(true, Ordering::SeqCst);
    let update_request = CallUpdateRequest {
        uuid,
        update: Some(Update::Done(true)),
    };
    let mut signaling_client = SignalingServiceClient::new(channel.clone());

    signaling_client
        .call_update(update_request)
        .await
        .map_err(anyhow::Error::from)
        .map(|_| ())
}

async fn maybe_connect_via_webrtc(
    uri: Uri,
    channel: AddAuthorization<SetRequestHeader<Channel, HeaderValue>>,
) -> Result<Arc<WebrtcClientChannel>> {
    let webrtc_options = Options::infer_from_uri(uri.clone());
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

    // CR erodkin: handle the "send error" cases!
    let sent_done_or_error = Arc::new(AtomicBool::new(false));
    // CR erodkin: do we need this clone?
    let sent_done2 = sent_done_or_error.clone();
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
        let channel_clone = channel.clone();
        let uuid_lock_clone = uuid_lock.clone();
        let sent_done_or_error_clone = sent_done_or_error.clone();

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
                let channel = channel_clone.clone();
                let sent_done_or_error = sent_done_or_error_clone.clone();
                let ice_done = ice_done.clone();
                let uuid_lock_clone = uuid_lock_clone.clone();
                Box::pin(async move {
                    if ice_done.load(Ordering::SeqCst) {
                        return;
                    }
                    let uuid = uuid_lock_clone.read().unwrap().to_string();
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
                            if let Err(e) =
                                send_done_once(sent_done_or_error, uuid, channel.clone()).await
                            {
                                log::error!("Error in sending done update: {e}");
                            }
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
    tokio::spawn(
        // CR erodkin: so many unwraps, I bet we can make this better
        async move {
            // CR erodkin: probably we'd want here to do something here to exit
            // the loop when we're done. maybe something like
            // tokio:sync:mpsc:chanel ala offer-answer/offer.rs, line 286? also so much
            // nesting! less indenting would be cool.
            // CR erodkin: exit once we finish ice connecting probably
            //CR erodkin: this fixes everything... but why?
            tokio::time::sleep(Duration::from_secs(1)).await;

            loop {
                let call_response = match call_client.message().await {
                    Ok(cr) => cr,
                    Err(e) => {
                        log::error!("Error processing call response: {e}");
                        continue;
                    }
                };

                if let Some(response) = call_response {
                    log::info!("got call_response: {:?}", response);
                    match response.stage {
                        Some(Stage::Init(init)) => {
                            {
                                let mut uuid_s = uuid_lock.write().unwrap();
                                *uuid_s = response.uuid.clone();
                            }

                            let answer = match decode_sdp(init.sdp) {
                                Ok(a) => a,
                                Err(e) => {
                                    log::error!("error decoding sdp answer: {e}");
                                    continue;
                                }
                            };

                            if let Err(e) = client_channel2
                                .base_channel
                                .peer_connection
                                .set_remote_description(answer)
                                .await
                            {
                                // CR erodkin: probably send error message and be done here?
                                log::error!("Error setting remote description: {e}");
                                break;
                            }
                            remote_description_set.store(true, Ordering::SeqCst);
                            if let Err(e) = send_done_once(
                                sent_done_or_error.clone(),
                                response.uuid,
                                channel2.clone(),
                            )
                            .await
                            {
                                log::error!("Error calling update : {e}");
                                break;
                            }
                        }

                        Some(Stage::Update(update)) => {
                            if response.uuid != *uuid_lock.read().unwrap() {
                                return Err(anyhow::anyhow!(
                                    "uuid mismatch: have {}, want {}",
                                    response.uuid,
                                    *uuid_lock.read().unwrap()
                                ));
                            }
                            if let Some(candidate) = update.candidate {
                                let candidate = ice_candidate_from_proto(candidate).unwrap();
                                if let Err(e) = client_channel2
                                    .base_channel
                                    .peer_connection
                                    .add_ice_candidate(candidate)
                                    .await
                                {
                                    log::error!(
                                        "error adding ice candidate to peer connection: {e}"
                                    )
                                }
                            }
                        }
                        None => continue,
                    }
                }
            }
            Ok(())
        },
    );

    // CR erodkin: make a ticket for this
    // TODO: create separate authorization if external_auth_addr and/or creds.Type is `Some`

    while !is_open_read.load(Ordering::SeqCst) {
        hint::spin_loop();
    }

    exchange_done.store(true, Ordering::SeqCst);
    let uuid = uuid2.read().unwrap().to_string();
    if let Err(e) = send_done_once(sent_done2, uuid, channel.clone()).await {
        log::error!("error sending Done update: {e}");
    }
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

fn ice_candidate_from_proto(proto: IceCandidate) -> Result<RTCIceCandidateInit> {
    let proto_sdpm: usize = proto.sdpm_line_index().try_into()?;
    let sdp_mline_index: Option<u16> = proto_sdpm.try_into().ok();

    Ok(RTCIceCandidateInit {
        candidate: proto.candidate.clone(),
        sdp_mid: Some(proto.sdp_mid().to_string()),
        sdp_mline_index,
        username_fragment: Some(proto.username_fragment().to_string()),
    })
}

fn decode_sdp(sdp: String) -> Result<RTCSessionDescription> {
    let sdp = String::from_utf8(base64::decode(sdp)?)?;
    Ok(serde_json::from_str::<RTCSessionDescription>(&sdp)?)
}

fn encode_sdp(sdp: RTCSessionDescription) -> Result<String> {
    let sdp = serde_json::to_vec(&sdp)?;
    Ok(base64::encode(sdp))
}

pub fn infer_remote_uri_from_domain(uri: Uri) -> Uri {
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

pub(crate) fn uri_parts_with_defaults(uri: &str) -> Parts {
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
