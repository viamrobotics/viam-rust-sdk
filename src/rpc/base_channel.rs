use anyhow::Result;
use std::sync::{
    atomic::{AtomicBool, AtomicPtr, Ordering},
    Arc,
};
use webrtc::{data_channel::RTCDataChannel, peer_connection::RTCPeerConnection};

// see golang/client_stream.go
pub struct WebRTCBaseChannel {
    pub peer_connection: Arc<RTCPeerConnection>,
    pub data_channel: Arc<RTCDataChannel>,
    closed_reason: AtomicPtr<Option<anyhow::Error>>,
    closed: AtomicBool,
}

impl WebRTCBaseChannel {
    pub async fn new(
        peer_connection: Arc<RTCPeerConnection>,
        data_channel: Arc<RTCDataChannel>,
    ) -> Arc<Self> {
        let dc = data_channel.clone();
        let pc = peer_connection.clone();
        peer_connection
            .on_ice_connection_state_change(Box::new(move |conn_state| {
                let pc = pc.clone();
                Box::pin(async move {
                    let sctp = pc.sctp();
                    let transport = sctp.transport();
                    let transport = transport.ice_transport();
                    let candidate_pair = transport.get_selected_candidate_pair().await;
                    log::info!(
                        "Selected candidate pair. Pair: {candidate_pair:?}. ID: {}. Current connection state: {conn_state}",
                        pc.get_stats_id()
                    );
                })
            }))
            .await;

        let channel = Arc::new(Self {
            peer_connection,
            data_channel,
            closed_reason: AtomicPtr::new(&mut None),
            closed: AtomicBool::new(false),
        });

        let c = channel.clone();
        dc.on_error(Box::new(move |err: webrtc::Error| {
            let c = c.clone();
            Box::pin(async move {
                if let Err(e) = c.close_with_reason(Some(anyhow::Error::from(err))).await {
                    log::error!("error closing channel: {e}")
                }
            })
        }))
        .await;

        channel
    }

    async fn close_with_reason(&self, err: Option<anyhow::Error>) -> Result<()> {
        let mut err = err;
        if self.closed.load(Ordering::Acquire) {
            return Ok(());
        }
        self.closed.store(true, Ordering::Release);
        self.closed_reason.store(&mut err, Ordering::Release);

        self.peer_connection
            .close()
            .await
            .map_err(anyhow::Error::from)
    }

    pub async fn close(&self) -> Result<()> {
        self.close_with_reason(None).await
    }

    pub fn is_closed(&self) -> bool {
        self.closed.load(Ordering::Acquire)
    }

    pub fn closed_reason(&self) -> *mut Option<anyhow::Error> {
        self.closed_reason.load(Ordering::Acquire)
    }
}
