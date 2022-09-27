use super::{base_stream::*, webrtc::trailers_from_proto};
use crate::gen::proto::rpc::webrtc::v1::{
    response::Type, Response, ResponseHeaders, ResponseMessage, ResponseTrailers,
};
use anyhow::Result;
use byteorder::{BigEndian, WriteBytesExt};
use bytes::Bytes;
use std::sync::atomic::{AtomicBool, Ordering};

/// The client-specific parts of a webRTC stream.
pub struct WebRTCClientStream {
    pub(crate) base_stream: WebRTCBaseStream,
    pub(crate) headers_received: AtomicBool,
    pub(crate) trailers_received: AtomicBool,
}

impl WebRTCClientStream {
    fn process_headers(&mut self, _headers: ResponseHeaders) {
        self.headers_received.store(true, Ordering::Release)
    }

    // processes a response message, returns true if and only if message was actually
    // processed
    async fn process_message(&mut self, response: ResponseMessage) -> Result<()> {
        if let Some(message) = response.packet_message {
            match self.base_stream.process_message(message) {
                Ok(data) => {
                    if data.is_some() {
                        let mut data = data.unwrap();
                        let mut message_buf = vec![0u8];
                        let len: u32 = data.len().try_into()?;
                        message_buf.write_u32::<BigEndian>(len)?;
                        message_buf.append(&mut data);
                        let data = Bytes::from(message_buf);
                        self.base_stream
                            .message_sender
                            .send_data(data.clone())
                            .await?;
                    }
                }
                Err(e) => {
                    log::error!("Error processing message: {e}");
                }
            }
        }
        Ok(())
    }

    async fn process_trailers(&mut self, trailers: ResponseTrailers) {
        let trailers_to_send = trailers_from_proto(trailers.clone());
        if let Err(e) = self
            .base_stream
            .message_sender
            .send_trailers(trailers_to_send)
            .await
        {
            log::error!("Error sending trailers to http response: {e}");
        }

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

    // processes response.
    pub(crate) async fn on_response(&mut self, response: Response) -> Result<()> {
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
                Ok(())
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

                self.process_message(message.to_owned()).await
            }

            Some(Type::Trailers(trailers)) => {
                self.process_trailers(trailers.to_owned()).await;
                Ok(())
            }
            None => Ok(()),
        }
    }
}
