use super::base_stream::*;
use crate::gen::proto::rpc::webrtc::v1::{
    response::Type, Response, ResponseHeaders, ResponseMessage, ResponseTrailers,
};
use anyhow::Result;
use byteorder::{BigEndian, WriteBytesExt};
use bytes::Bytes;
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

    // processes a response message, returns true if and only if message was actually
    // processed
    fn process_message(&mut self, response: ResponseMessage) -> Result<bool> {
        let mut message_processed = false;
        if let Some(message) = response.packet_message {
            match self.base_stream.process_message(message) {
                Ok(mut data) => {
                    if !data.is_empty() {
                        message_processed = true;
                        let mut message_buf = vec![0u8];
                        let len: u32 = data.len().try_into()?;
                        message_buf.write_u32::<BigEndian>(len)?;
                        message_buf.append(&mut data);
                        let data = Bytes::from(message_buf);
                        // when sending data synchronously, we run the risk of sending
                        // too many messages too quickly, which results in messages being
                        // dropped. Unfortunately we cannot send asynchronously under the
                        // current code structure because mutexes don't play nicely with
                        // async. this loop is ugly, but ensures that we properly send
                        // all messages while still failing in cases where send is just
                        // not possible.
                        // TODO(RSDK-651) - we should refactor here so we can use the
                        // async `send_data(data)` fn, eliminating the need for this loop.
                        let now = std::time::SystemTime::now();
                        let timeout = std::time::Duration::from_secs(5);
                        loop {
                            if now.elapsed().unwrap() >= timeout {
                                return Err(anyhow::anyhow!("Error sending data: {data:?}"));
                            }
                            if let Ok(()) =
                                self.base_stream.message_sender.try_send_data(data.clone())
                            {
                                break;
                            }
                        }
                    }
                }

                Err(e) => {
                    log::error!("Error processing message: {e}");
                }
            }
        }
        Ok(message_processed)
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

                self.process_message(message.to_owned())
            }

            Some(Type::Trailers(trailers)) => {
                self.process_trailers(trailers.to_owned());
                Ok(true)
            }
            None => Ok(false),
        }
    }
}
