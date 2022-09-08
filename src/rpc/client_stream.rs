use super::base_stream::*;
use crate::gen::proto::rpc::webrtc::v1::{
    response::Type, Response, ResponseHeaders, ResponseMessage, ResponseTrailers,
};
use anyhow::Result;
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc, Mutex,
};

pub struct ActiveWebRTCClientStream {
    pub client_stream: Arc<Mutex<WebRTCClientStream>>,
}

pub struct WebRTCClientStream {
    pub base_stream: WebRTCBaseStream,
    pub headers_received: AtomicBool,
    pub message_sent: AtomicBool,
    pub trailers_received: AtomicBool,
}

impl WebRTCClientStream {
    fn process_headers(&mut self, _headers: ResponseHeaders) {
        self.headers_received.store(true, Ordering::Release)
    }

    // processes a response message
    fn process_message(&mut self, response: ResponseMessage) -> Result<()> {
        if let Some(message) = response.packet_message {
            match self.base_stream.process_message(message) {
                Ok(data) => {
                    if !data.is_empty() {
                        self.base_stream.message_sender.try_send(data)?;
                        self.message_sent.store(true, Ordering::Release);
                    }
                }

                Err(e) => {
                    log::error!("Error processing message: {e}");
                }
            }
        }
        Ok(())
    }

    fn process_trailers(&mut self, trailers: ResponseTrailers) {
        self.trailers_received.store(true, Ordering::Release);

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

    // processes response. returns true if and only if a message was sent and we're done
    // processing (i.e., trailers were processed)
    pub fn on_response(&mut self, response: Response) -> Result<bool> {
        match &response.r#type {
            Some(Type::Headers(headers)) => {
                if self.headers_received.load(Ordering::Acquire) {
                    let err = anyhow::format_err!("headers already received");
                    self.base_stream.close_with_recv_error(&mut Some(&err));
                    return Err(err);
                }

                if self.trailers_received.load(Ordering::Acquire) {
                    let err = anyhow::format_err!("headers received after trailers");
                    self.base_stream.close_with_recv_error(&mut Some(&err));
                    return Err(err);
                }

                self.process_headers(headers.to_owned());
                Ok(false)
            }
            Some(Type::Message(message)) => {
                if !self.headers_received.load(Ordering::Acquire) {
                    let err = anyhow::format_err!("headers not yet received");
                    self.base_stream.close_with_recv_error(&mut Some(&err));
                    return Err(err);
                }

                if self.trailers_received.load(Ordering::Acquire) {
                    let err = anyhow::format_err!("new messages received after trailers");
                    self.base_stream.close_with_recv_error(&mut Some(&err));
                    return Err(err);
                }

                self.process_message(message.to_owned()).map(|()| false)
            }

            Some(Type::Trailers(trailers)) => {
                self.process_trailers(trailers.to_owned());
                Ok(true)
            }
            None => Ok(false),
        }
    }
}
