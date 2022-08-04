use crate::gen::proto::rpc::webrtc::v1::{
    response::Type, PacketMessage, Response, ResponseHeaders, ResponseMessage, ResponseTrailers,
    Stream,
};
use anyhow::Result;
use bytes::BufMut;
use std::sync::{
    atomic::{AtomicBool, AtomicPtr, Ordering},
    mpsc::{Receiver, Sender},
    Arc, Mutex,
};

const MAX_MESSAGE_SIZE: usize = 1 << 25;

pub struct ActiveWebrtcClientStream {
    pub client_stream: Arc<Mutex<WebrtcClientStream>>,
}

pub struct WebrtcClientStream {
    pub stream: Stream,
    pub message_sender: Sender<Vec<u8>>,
    pub message_receiver: Receiver<Vec<u8>>,
    pub closed: AtomicBool,
    pub packet_buffer: Vec<u8>,
    pub closed_reason: AtomicPtr<Option<anyhow::Error>>,
    pub headers_received: AtomicBool,
    pub trailers_received: AtomicBool,
}

impl WebrtcClientStream {
    fn process_headers(&mut self, _headers: ResponseHeaders) {
        self.headers_received.store(true, Ordering::SeqCst)
    }

    // processes a response message. Return value is true iff a message was sent.
    fn process_message(&mut self, response: ResponseMessage) -> Result<bool> {
        let mut message_sent = false;
        if self.trailers_received.load(Ordering::SeqCst) {
            return Ok(message_sent);
        };

        if let Some(message) = response.packet_message {
            match self.pack_message(message) {
                Ok(data) => {
                    if !data.is_empty() {
                        self.message_sender.send(data)?;
                        message_sent = true;
                    }
                }

                Err(e) => {
                    log::error!("Error processing message: {e}");
                }
            }
        }
        Ok(message_sent)
    }

    fn process_trailers(&mut self, trailers: ResponseTrailers) {
        self.trailers_received.store(true, Ordering::SeqCst);

        let err = match trailers.status {
            None => None,
            Some(status) => {
                let status_code = status.code;
                let message = status.message;
                if status_code == 0 {
                    None
                } else {
                    Some(anyhow::anyhow!("Code={status_code} message={message}"))
                }
            }
        };

        self.close_with_recv_error(&mut err.as_ref())
    }

    pub fn close_with_recv_error(&self, err: &mut Option<&anyhow::Error>) {
        if self.closed.load(Ordering::SeqCst) {
            return;
        }
        let mut err = err.map(|e| anyhow::anyhow!(e.to_string()));
        self.closed.store(true, Ordering::SeqCst);
        self.closed_reason.store(&mut err, Ordering::SeqCst);
    }

    // processes response. returns true iff a message was sent
    pub fn on_response(&mut self, response: Response) -> Result<bool> {
        match &response.r#type {
            Some(Type::Headers(headers)) => {
                if self.headers_received.load(Ordering::SeqCst) {
                    let err = anyhow::format_err!("headers already received");
                    self.close_with_recv_error(&mut Some(&err));
                    return Err(err);
                }

                if self.trailers_received.load(Ordering::SeqCst) {
                    let err = anyhow::format_err!("headers received after trailers");
                    self.close_with_recv_error(&mut Some(&err));
                    return Err(err);
                }

                self.process_headers(headers.to_owned());
                Ok(false)
            }
            Some(Type::Message(message)) => {
                if !self.headers_received.load(Ordering::SeqCst) {
                    let err = anyhow::format_err!("headers not yet received");
                    self.close_with_recv_error(&mut Some(&err));
                    return Err(err);
                }

                if self.trailers_received.load(Ordering::SeqCst) {
                    let err = anyhow::format_err!("new messages received after trailers");
                    self.close_with_recv_error(&mut Some(&err));
                    return Err(err);
                }

                self.process_message(message.to_owned())
            }

            Some(Type::Trailers(trailers)) => {
                self.process_trailers(trailers.to_owned());
                Ok(false)
            }
            None => Ok(false),
        }
    }

    fn reset_packet_buffer(&mut self) {
        self.packet_buffer = vec![]
    }

    fn pack_message(&mut self, message: PacketMessage) -> Result<Vec<u8>> {
        if message.data.is_empty() && message.eom {
            return Ok(Vec::new());
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
            return Ok(ret);
        }
        Ok(Vec::new())
    }
}
