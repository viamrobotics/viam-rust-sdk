use anyhow::Result;
use std::sync::{
    atomic::{AtomicBool, AtomicPtr, Ordering},
    Arc,
};
use webrtc::{data_channel::RTCDataChannel, peer_connection::RTCPeerConnection};

// see golang/client_stream.go
pub struct WebrtcBaseChannel {
    pub peer_connection: Arc<RTCPeerConnection>,
    pub data_channel: Arc<RTCDataChannel>,
    closed_reason: AtomicPtr<Option<anyhow::Error>>,
    closed: AtomicBool,
}

impl WebrtcBaseChannel {
    pub async fn new(
        peer_connection: Arc<RTCPeerConnection>,
        data_channel: Arc<RTCDataChannel>,
    ) -> Arc<Self> {
        let dc = data_channel.clone();
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
        if self.closed.load(Ordering::SeqCst) {
            return Ok(());
        }
        self.closed.store(true, Ordering::SeqCst);
        self.closed_reason.store(&mut err, Ordering::SeqCst);

        self.peer_connection
            .close()
            .await
            .map_err(anyhow::Error::from)
    }

    pub async fn close(&self) -> Result<()> {
        self.close_with_reason(None).await
    }

    pub fn is_closed(&self) -> bool {
        self.closed.load(Ordering::SeqCst)
    }

    pub fn closed_reason(&self) -> *mut Option<anyhow::Error> {
        self.closed_reason.load(Ordering::SeqCst)
    }
}
