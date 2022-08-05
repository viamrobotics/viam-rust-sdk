use super::base_stream::*;
use crate::gen::proto::rpc::webrtc::v1::{
    response::Type, Response, ResponseHeaders, ResponseMessage, ResponseTrailers,
};
use anyhow::Result;
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc, Mutex,
};

pub struct ActiveWebrtcClientStream {
    pub client_stream: Arc<Mutex<WebrtcClientStream>>,
}

pub struct WebrtcClientStream {
    pub base_stream: WebrtcBaseStream,
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
            match self.base_stream.process_message(message) {
                Ok(data) => {
                    if !data.is_empty() {
                        self.base_stream.message_sender.send(data)?;
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

        self.base_stream.close_with_recv_error(&mut err.as_ref())
    }

    // processes response. returns true iff a message was sent
    pub fn on_response(&mut self, response: Response) -> Result<bool> {
        match &response.r#type {
            Some(Type::Headers(headers)) => {
                if self.headers_received.load(Ordering::SeqCst) {
                    let err = anyhow::format_err!("headers already received");
                    self.base_stream.close_with_recv_error(&mut Some(&err));
                    return Err(err);
                }

                if self.trailers_received.load(Ordering::SeqCst) {
                    let err = anyhow::format_err!("headers received after trailers");
                    self.base_stream.close_with_recv_error(&mut Some(&err));
                    return Err(err);
                }

                self.process_headers(headers.to_owned());
                Ok(false)
            }
            Some(Type::Message(message)) => {
                if !self.headers_received.load(Ordering::SeqCst) {
                    let err = anyhow::format_err!("headers not yet received");
                    self.base_stream.close_with_recv_error(&mut Some(&err));
                    return Err(err);
                }

                if self.trailers_received.load(Ordering::SeqCst) {
                    let err = anyhow::format_err!("new messages received after trailers");
                    self.base_stream.close_with_recv_error(&mut Some(&err));
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
}
