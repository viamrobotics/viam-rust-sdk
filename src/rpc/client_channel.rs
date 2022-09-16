use super::{base_channel::*, base_stream::*, client_stream::*};
use crate::gen::proto::rpc::webrtc::v1::{
    request::Type, response::Type as RespType, PacketMessage, Request, RequestHeaders,
    RequestMessage, Response, Stream,
};
use anyhow::Result;
use chashmap::CHashMap;
use hyper::Body;
use prost::Message;
use std::{
    collections::HashMap,
    sync::{
        atomic::{AtomicBool, AtomicPtr, AtomicU64, Ordering},
        Arc, Mutex,
    },
};
use webrtc::{
    data_channel::{data_channel_message::DataChannelMessage, RTCDataChannel},
    peer_connection::RTCPeerConnection,
};

// see golang/client_stream.go
const MAX_REQUEST_MESSAGE_PACKET_DATA_SIZE: usize = 16373;

pub struct WebRTCClientChannel {
    pub base_channel: Arc<WebRTCBaseChannel>,
    stream_id_counter: AtomicU64,
    message_ready: Arc<AtomicBool>,
    pub streams: Mutex<HashMap<u64, ActiveWebRTCClientStream>>,
    pub receiver_bodies: CHashMap<u64, hyper::Body>,
}

impl WebRTCClientChannel {
    pub async fn new(
        peer_connection: Arc<RTCPeerConnection>,
        data_channel: Arc<RTCDataChannel>,
    ) -> Arc<Self> {
        let base_channel = WebRTCBaseChannel::new(peer_connection, data_channel.clone()).await;
        let channel = Self {
            base_channel,
            message_ready: Arc::new(AtomicBool::new(false)),
            streams: Mutex::new(HashMap::new()),
            stream_id_counter: AtomicU64::new(0),
            receiver_bodies: CHashMap::new(),
        };

        let channel = Arc::new(channel);
        let ret_channel = channel.clone();

        data_channel
            .on_message(Box::new(move |msg: DataChannelMessage| {
                let channel = channel.clone();
                Box::pin(async move {
                    if let Err(e) = channel.on_channel_message(msg) {
                        log::error!("error deserializing message: {e}");
                    }
                })
            }))
            .await;
        ret_channel
    }

    pub fn new_stream(&self) -> Arc<Mutex<WebRTCClientStream>> {
        let id = self.stream_id_counter.fetch_add(1, Ordering::AcqRel);
        let stream = Stream { id };

        let (message_sender, receiver_body) = hyper::Body::channel();

        let base_stream = WebRTCBaseStream {
            stream,
            message_sender,
            closed: AtomicBool::new(false),
            packet_buffer: Vec::new(),
            closed_reason: AtomicPtr::new(&mut None),
        };

        let client_stream = Arc::new(Mutex::new(WebRTCClientStream {
            base_stream,
            message_sent: AtomicBool::new(false),
            headers_received: AtomicBool::new(false),
            trailers_received: AtomicBool::new(false),
        }));

        let stream = ActiveWebRTCClientStream {
            client_stream: client_stream.clone(),
        };
        let _ = self.streams.lock().unwrap().insert(id, stream);
        let _ = self.receiver_bodies.insert(id, receiver_body);
        client_stream
    }

    fn on_channel_message(&self, msg: DataChannelMessage) -> Result<()> {
        let streams = self.streams.lock().unwrap();
        let response = Response::decode(&*msg.data.to_vec())?;
        let should_drop_stream = match response.r#type {
            Some(RespType::Trailers(_)) => true,
            _ => false,
        };
        let (active_stream, stream_id) = match response.stream.as_ref() {
            None => {
                log::error!(
                    "no stream associated with response {:?}: discarding response",
                    response
                );
                return Ok(());
            }
            Some(stream) => {
                let id: u64 = stream.id;
                let stream = streams.get(&stream.id).ok_or_else(|| {
                    anyhow::anyhow!(
                        "No stream found for id {}: discarding response {:?}",
                        &stream.id,
                        response
                    )
                });
                (stream, id)
            }
        };

        let message_sent = match active_stream {
            Ok(active_stream) => active_stream
                .client_stream
                .lock()
                .unwrap()
                .on_response(response),
            Err(e) => {
                log::error!("{e}");
                return Ok(());
            }
        }?;
        drop(streams);
        if should_drop_stream {
            self.streams.lock().unwrap().remove(&stream_id);
        }
        self.message_ready.store(message_sent, Ordering::Release);
        Ok(())
    }

    pub async fn resp_body_from_stream(&self, stream_id: u64) -> Result<Body> {
        let body = self
            .receiver_bodies
            .remove(&stream_id)
            .ok_or(anyhow::anyhow!(
                "Tried to receive stream {stream_id} but it didn't exist!"
            ))?;
        self.message_ready.store(false, Ordering::Release);
        Ok(body)
    }

    pub async fn write_headers(&self, stream: &Stream, headers: RequestHeaders) -> Result<()> {
        let headers = Request {
            stream: Some(stream.clone()),
            r#type: Some(Type::Headers(headers)),
        };
        let header_vec = Message::encode_to_vec(&headers);
        self.send(&header_vec).await
    }

    pub async fn write_message(
        &self,
        eos: bool,
        stream: Option<Stream>,
        mut data: Vec<u8>,
    ) -> Result<()> {
        // even if no meaningful data, any actual message will include at least frame header bytes
        let has_message = !data.is_empty();

        // rust libraries are munging streamed client requests into a single http request.
        // we can look at the gRPC header bytes to determine the length of the first message
        // and compare it to the length of the data to determine whether this http request
        // is a single unary call, or a streaming call.
        // TODO(RSDK-654) The munging of streaming requests into a single http request is
        // likely going to cause problems for us when we encounter a need for bidi streaming
        // in the real world. Look into how we can fix it, and hopefully get rid of this
        // header math in the process.
        let mut length_of_first_message = 0usize;
        // 1-5 because those are the length header bytes for gRPC
        for i in 1..5 {
            let to_add = data.get(i).unwrap_or(&0).to_owned();
            length_of_first_message += usize::from(to_add);
        }
        // if this is all streaming calls we need to tell the server when we're done with
        // the stream, otherwise neither side will know we're done, trailers will never be
        // sent/processed, and we'll hang on the strream.
        let it_was_all_a_stream = length_of_first_message + 5 < data.len();

        // always run the loop at least once, check at completion if we've sent all data and
        // break the loop accordingly
        loop {
            if 0 < data.len() && data.len() < 5 {
                return Err(anyhow::anyhow!(
                    "Attempted to process message with irregular length"
                ));
            }

            let mut message_length = 0usize;
            // because we might have multiple requests contained within our data, we have
            // to do the manual work of breaking apart the body into separate requests.
            for i in 1..5 {
                let to_add = data.get(i).unwrap_or(&0).to_owned();
                message_length += usize::from(to_add);
            }
            data = data.split_off(5);
            let split_at = MAX_REQUEST_MESSAGE_PACKET_DATA_SIZE
                .min(data.len())
                .min(message_length);
            let (to_send, remaining) = data.split_at(split_at);
            let stream = stream.clone();
            let request = Request {
                stream,
                r#type: Some(Type::Message(RequestMessage {
                    has_message,
                    eos: if remaining.len() > 0 {
                        eos
                    } else {
                        it_was_all_a_stream
                    },
                    packet_message: Some(PacketMessage {
                        eom: to_send.len() == message_length || remaining.len() == 0,
                        data: to_send.to_vec(),
                    }),
                })),
            };

            let request = Message::encode_to_vec(&request);
            if let Err(e) = self.send(&request).await {
                log::error!("error sending message: {e}");
                return Err(e);
            }

            data = remaining.to_vec();
            if data.is_empty() {
                break;
            }
        }
        Ok(())
    }

    async fn send(&self, data: &[u8]) -> Result<()> {
        let data = &bytes::Bytes::copy_from_slice(data);
        self.base_channel
            .data_channel
            .send(data)
            .await
            .map_err(anyhow::Error::from)
            .map(|_: usize| ())
    }

    pub fn close_stream_with_recv_error(&self, stream_id: u64, error: anyhow::Error) {
        let mut stream_lock = self.streams.lock().unwrap();
        match stream_lock.remove(&stream_id) {
            Some(stream) => stream
                .client_stream
                .lock()
                .unwrap()
                .base_stream
                .close_with_recv_error(&mut Some(&error)),
            None => {
                log::error!("attempted to close stream with id {stream_id}, but it wasn't found!")
            }
        }
    }
}
