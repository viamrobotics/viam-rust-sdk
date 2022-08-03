use crate::gen::proto::rpc::{
    v1::Credentials,
    webrtc::v1::{IceServer, WebRtcConfig},
};
use anyhow::Result;
use bytes::Bytes;
use http::Uri;
use interceptor::registry::Registry;
use std::{hint, sync::Arc};
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
    peer_connection::{
        configuration::RTCConfiguration, peer_connection_state::RTCPeerConnectionState,
        sdp::session_description::RTCSessionDescription, signaling_state::RTCSignalingState,
        RTCPeerConnection,
    },
};

#[derive(Default)]
pub struct Options {
    pub disable: bool,
    pub signaling_insecure: bool,
    pub signaling_server_address: String,
    pub signaling_auth_entity: String,
    pub signaling_external_auth_address: String,
    pub signaling_external_auth_to_entity: String,
    pub signaling_external_auth_insecure: String,
    pub signaling_credentials: Credentials,
    pub disable_trickle_ice: bool,
    pub allow_auto_detect_auth_options: bool,
    pub config: RTCConfiguration,
}

impl Options {
    pub(crate) fn infer_signaling_server_address(uri: &Uri) -> Option<(String, bool)> {
        let path = uri.to_string();
        if path.contains(".viam.cloud") {
            Some(("app.viam.com:443".to_string(), true))
        } else if path.contains(".robot.viaminternal") {
            Some(("app.viaminternal:8089".to_string(), false))
        } else {
            None
        }
    }

    pub(crate) fn infer_from_uri(uri: Uri) -> Self {
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
        negotiated: Some(true),
        ordered: Some(true),
        id: Some(0),
        ..Default::default()
    };

    let renegotiation_channel_init = RTCDataChannelInit {
        negotiated: Some(true),
        ordered: Some(true),
        id: Some(1),
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
        .create_data_channel("negotiation", Some(renegotiation_channel_init))
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

        // Block until ICE gathering is complete since we signal back one complete SDP and
        // do not want to wait on trickle ice
        while receiver.recv().await.is_some() {
            hint::spin_loop();
        }
    }

    Ok((peer_connection, data_channel))
}
