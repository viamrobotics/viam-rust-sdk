use crate::gen::proto::rpc::webrtc::v1::{PacketMessage, Stream};
use anyhow::Result;
use bytes::BufMut;
use hyper::body::Sender;
use std::sync::atomic::{AtomicBool, AtomicPtr, Ordering};

const MAX_MESSAGE_SIZE: usize = 1 << 25;

/// Base elements of a webRTC stream, used in both client and server implementations
#[allow(dead_code)]
pub struct WebRTCBaseStream {
    pub(crate) stream: Stream,
    pub(crate) message_sender: Sender,
    pub(crate) closed: AtomicBool,
    pub(crate) packet_buffer: Vec<u8>,
    pub(crate) closed_reason: AtomicPtr<Option<anyhow::Error>>,
}

impl WebRTCBaseStream {
    pub(crate) fn close_with_recv_error(&self, err: &mut Option<&anyhow::Error>) {
        if self.closed.load(Ordering::Acquire) {
            return;
        }
        let mut err = err.map(|e| anyhow::anyhow!(e.to_string()));
        self.closed.store(true, Ordering::Release);
        self.closed_reason.store(&mut err, Ordering::Release);
    }

    pub(crate) fn process_message(&mut self, message: PacketMessage) -> Result<Option<Vec<u8>>> {
        if message.data.is_empty() && message.eom {
            return Ok(Some(Vec::new()));
        }
        if message.data.len() + self.packet_buffer.len() > MAX_MESSAGE_SIZE {
            let e = Err(anyhow::anyhow!(
                "message size larger than max {}, discarding",
                MAX_MESSAGE_SIZE
            ));
            self.reset_packet_buffer();
            return e;
        }

        self.packet_buffer.put_slice(&message.data);
        if message.eom {
            let ret = self.packet_buffer.clone();
            self.reset_packet_buffer();
            return Ok(Some(ret));
        }
        Ok(None)
    }

    fn reset_packet_buffer(&mut self) {
        self.packet_buffer = vec![]
    }
}
