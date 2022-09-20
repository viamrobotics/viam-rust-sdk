use crate::gen::proto::rpc::webrtc::v1::{IceServer, ResponseTrailers, WebRtcConfig};
use anyhow::Result;
use bytes::Bytes;
use core::fmt;
use futures::Future;
use http::{header::HeaderName, HeaderMap, HeaderValue, Uri};
use std::{
    hint,
    pin::Pin,
    str::FromStr,
    sync::{atomic::AtomicBool, Arc},
    task::{Context, Poll},
    time::Duration,
};
use webrtc::{
    api::{
        interceptor_registry, media_engine::MediaEngine, setting_engine::SettingEngine, APIBuilder,
        API,
    },
    data_channel::{
        data_channel_init::RTCDataChannelInit, data_channel_message::DataChannelMessage,
        RTCDataChannel,
    },
    ice::mdns::MulticastDnsMode,
    ice_transport::ice_server::RTCIceServer,
    interceptor::registry::Registry,
    peer_connection::{
        configuration::RTCConfiguration, peer_connection_state::RTCPeerConnectionState,
        sdp::session_description::RTCSessionDescription, signaling_state::RTCSignalingState,
        RTCPeerConnection,
    },
};

// set to 20sec to match _defaultOfferDeadline in goutils/rpc/wrtc_call_queue.go
const WEBRTC_TIMEOUT: Duration = Duration::from_secs(20);

#[derive(Default)]
pub struct Options {
    pub disable_webrtc: bool,
    pub disable_trickle_ice: bool,
    pub config: RTCConfiguration,
    pub signaling_insecure: bool,
    pub signaling_server_address: String,
}

// an Arc<AtomicBool> that is pollable as a future. Pending when false, Ready(()) when true
pub(crate) struct PollableAtomicBool {
    // TODO(RSDK-598): expand the PollableAtomicBool to include load and store methods, and
    // to keep a Waker field that awakens when the value is true.
    inner: Arc<AtomicBool>,
}

impl PollableAtomicBool {
    pub(crate) fn new(inner: Arc<AtomicBool>) -> Self {
        Self { inner }
    }
}

// implementing Future for AtomicBool allows us to use an AtomicBool in a `tokio::select!`
// statement
impl Future for PollableAtomicBool {
    type Output = ();

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        match self.inner.load(std::sync::atomic::Ordering::Acquire) {
            false => {
                cx.waker().wake_by_ref();
                Poll::Pending
            }
            true => Poll::Ready(()),
        }
    }
}

impl fmt::Debug for Options {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Options")
            .field("disable_webrtc", &format_args!("{}", self.disable_webrtc))
            .field(
                "disable_trickle_ice",
                &format_args!("{}", self.disable_trickle_ice),
            )
            // RTCConfiguration does not derive Debug
            .field("config", &format_args!("{}", "<Opaque>"))
            .field(
                "signaling_insecure",
                &format_args!("{}", self.signaling_insecure),
            )
            .field(
                "signaling_server_address",
                &format_args!("{}", self.signaling_server_address),
            )
            .finish()
    }
}

impl Options {
    pub fn infer_signaling_server_address(uri: &Uri) -> Option<(String, bool)> {
        // TODO(RSDK-235): remove hard coding of signaling server
        // address and prefer SRV lookup instead
        let path = uri.to_string();
        if path.contains(".viam.cloud") {
            Some(("app.viam.com:443".to_string(), true))
        } else if path.contains(".robot.viaminternal") {
            Some(("app.viaminternal:8089".to_string(), false))
        } else {
            None
        }
    }

    pub fn infer_from_uri(uri: Uri) -> Self {
        match Self::infer_signaling_server_address(&uri) {
            None => Options {
                config: default_configuration(),
                ..Default::default()
            },
            Some((signaling_server_address, secure)) => Options {
                config: default_configuration(),
                signaling_server_address,
                signaling_insecure: !secure,
                ..Default::default()
            },
        }
    }

    pub fn disable_webrtc(mut self) -> Self {
        self.disable_webrtc = true;
        self
    }
}

pub fn default_configuration() -> RTCConfiguration {
    let ice_server = RTCIceServer {
        urls: vec!["stun:global.stun.twilio.com:3478?transport=udp".to_string()],
        ..Default::default()
    };

    RTCConfiguration {
        ice_servers: vec![ice_server],
        ..Default::default()
    }
}

fn ice_server_from_proto(ice_server: IceServer) -> RTCIceServer {
    RTCIceServer {
        urls: ice_server.urls,
        username: ice_server.username,
        credential: ice_server.credential,
        ..Default::default()
    }
}

pub fn extend_webrtc_config(
    original: RTCConfiguration,
    optional: Option<WebRtcConfig>,
) -> RTCConfiguration {
    match optional {
        None => original,
        Some(optional) => {
            let mut new_ice_servers = original.ice_servers;
            for additional_server in optional.additional_ice_servers {
                let additional_server = ice_server_from_proto(additional_server);
                new_ice_servers.push(additional_server);
            }

            RTCConfiguration {
                ice_servers: new_ice_servers,
                ..original
            }
        }
    }
}

fn new_webrtc_api() -> Result<API> {
    let mut media_engine = MediaEngine::default();
    media_engine.register_default_codecs()?;
    let registry = Registry::new();
    let interceptor =
        interceptor_registry::register_default_interceptors(registry, &mut media_engine)?;

    let mut setting_engine = SettingEngine::default();
    setting_engine.set_ice_multicast_dns_mode(MulticastDnsMode::QueryAndGather);

    Ok(APIBuilder::new()
        .with_media_engine(media_engine)
        .with_interceptor_registry(interceptor)
        .with_setting_engine(setting_engine)
        .build())
}

fn create_invalid_sdp_err(err: serde_json::error::Error) -> webrtc::Error {
    webrtc::Error::Sdp(webrtc::sdp::Error::SdpInvalidValue(err.to_string()))
}

pub async fn new_peer_connection_for_client(
    config: RTCConfiguration,
    disable_trickle_ice: bool,
) -> Result<(Arc<RTCPeerConnection>, Arc<RTCDataChannel>)> {
    let web_api = new_webrtc_api()?;
    let peer_connection = Arc::new(web_api.new_peer_connection(config).await?);

    let data_channel_init = RTCDataChannelInit {
        negotiated: Some(0),
        ordered: Some(true),
        ..Default::default()
    };

    let negotiation_channel_init = RTCDataChannelInit {
        negotiated: Some(1),
        ordered: Some(true),
        ..Default::default()
    };

    peer_connection
        .on_peer_connection_state_change(Box::new(move |connection: RTCPeerConnectionState| {
            log::info!("peer connection state change: {connection}");
            Box::pin(async move {})
        }))
        .await;

    peer_connection
        .on_signaling_state_change(Box::new(move |ssc: RTCSignalingState| {
            log::info!("new signaling state: {ssc}");
            Box::pin(async move {})
        }))
        .await;

    let data_channel = peer_connection
        .create_data_channel("data", Some(data_channel_init))
        .await?;

    let negotiation_channel = peer_connection
        .create_data_channel("negotiation", Some(negotiation_channel_init))
        .await?;

    let nc = negotiation_channel.clone();
    let pc = peer_connection.clone();

    negotiation_channel
        .on_message(Box::new(move |msg: DataChannelMessage| {
            let pc = pc.clone();
            let nc = nc.clone();
            Box::pin(async move {
                let sdp_vec = msg.data.to_vec();
                let maybe_err = async move {
                    let sdp = serde_json::from_slice::<RTCSessionDescription>(&sdp_vec)
                        .map_err(create_invalid_sdp_err)?;
                    pc.set_remote_description(sdp).await?;
                    let answer = pc.create_answer(None).await?;
                    pc.set_local_description(answer).await?;
                    let local_description = pc
                        .local_description()
                        .await
                        .ok_or("No local description set");
                    let desc =
                        serde_json::to_vec(&local_description).map_err(create_invalid_sdp_err)?;
                    let desc = Bytes::copy_from_slice(&desc);
                    nc.send(&desc).await
                }
                .await;

                if let Err(e) = maybe_err {
                    log::error!("Error processing sdp in negotiation channel: {e}");
                }
            })
        }))
        .await;

    if disable_trickle_ice {
        let offer = peer_connection.create_offer(None).await?;
        let mut receiver = peer_connection.gathering_complete_promise().await;
        peer_connection.set_local_description(offer).await?;

        // TODO(RSDK-596): impl future here so we don't spin loop, which prevents this
        // from actually timing out.
        let promise_gathering_completed = async move {
            // Block until ICE gathering is complete since we signal back one complete SDP and
            // do not want to wait on trickle ice
            while receiver.recv().await.is_some() {
                hint::spin_loop();
            }
        };

        webrtc_action_with_timeout(promise_gathering_completed).await?;
    }

    Ok((peer_connection, data_channel))
}

pub async fn webrtc_action_with_timeout<T>(f: impl Future<Output = T>) -> Result<T> {
    tokio::pin! {
        let timeout = tokio::time::sleep(WEBRTC_TIMEOUT);
        let f = f;
    }

    loop {
        tokio::select! {
            res = &mut f => {
                return Ok(res);
            }
            _ = &mut timeout => {
                return Err(anyhow::anyhow!("Action timed out"));
            }
        }
    }
}

pub fn trailers_from_proto(proto: ResponseTrailers) -> HeaderMap {
    let mut trailers = HeaderMap::new();
    if let Some(metadata) = proto.metadata {
        let mut vals = metadata.md.iter();
        while let Some((k, v)) = vals.next() {
            let k = HeaderName::from_str(k);
            let v = HeaderValue::from_str(&v.values.concat());
            let (k, v) = match (k, v) {
                (Ok(k), Ok(v)) => (k, v),
                (Err(e), _) => {
                    log::error!("Error converting proto trailer key: [{e}]");
                    continue;
                }
                (_, Err(e)) => {
                    log::error!("Error converting proto trailer value: [{e}]");
                    continue;
                }
            };
            trailers.insert(k, v);
        }
    };

    let status_name = "grpc-status";
    let status_code = match proto.status {
        Some(ref status) => status.code.to_string(),
        None => "0".to_string(),
    };

    let k = match HeaderName::from_str(status_name) {
        Ok(k) => k,
        Err(e) => {
            log::error!("Error parsing HeaderName: {e}");
            return trailers;
        }
    };
    let v = match HeaderValue::from_str(&status_code) {
        Ok(v) => v,
        Err(e) => {
            log::error!("Error parsing HeaderValue: {e}");
            return trailers;
        }
    };
    trailers.insert(k, v);
    trailers
}
