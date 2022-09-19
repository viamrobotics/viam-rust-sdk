use http::uri::Uri;
use std::ptr;
use tokio::runtime::Runtime;
use tokio::sync::oneshot;
use tokio::task::JoinHandle;
use tracing::Level;

use crate::rpc::dial::{
    CredentialsExt, DialBuilder, DialOptions, WithCredentials, WithoutCredentials,
};
use libc::c_char;

use crate::proxy;
use hyper::Server;
use std::ffi::{CStr, CString};
use tower::{make::Shared, ServiceBuilder};
use tower_http::{
    trace::{DefaultMakeSpan, DefaultOnRequest, DefaultOnResponse, TraceLayer},
    LatencyUnit,
};

use anyhow::Result;

use crate::proxy::grpc_proxy::GRPCProxy;

pub struct Ffi {
    runtime: Runtime,
    jhs: Option<Vec<JoinHandle<()>>>,
    sigs: Option<Vec<oneshot::Sender<()>>>,
}

impl Ffi {
    fn new() -> Self {
        Self {
            runtime: Runtime::new().unwrap(),
            jhs: None,
            sigs: None,
        }
    }
    fn push_handle(&mut self, jh: JoinHandle<()>) {
        match self.jhs {
            Some(ref mut v) => v.push(jh),
            None => {
                let v: Vec<JoinHandle<()>> = vec![jh];
                self.jhs = Some(v);
            }
        }
    }
    fn push_signal(&mut self, sig: oneshot::Sender<()>) {
        match self.sigs {
            Some(ref mut v) => v.push(sig),
            None => {
                let v: Vec<oneshot::Sender<()>> = vec![sig];
                self.sigs = Some(v);
            }
        }
    }
}

#[no_mangle]
pub extern "C" fn init_rust_runtime() -> Box<Ffi> {
    tracing_subscriber::fmt::init();
    Box::new(Ffi::new())
}

fn dial_without_cred(
    uri: String,
    allow_insec: bool,
    disable_webrtc: bool,
) -> Result<DialBuilder<WithoutCredentials>> {
    let c = DialOptions::builder().uri(&uri).without_credentials();
    let c = if disable_webrtc {
        c.disable_webrtc()
    } else {
        c
    };
    let c = if allow_insec { c.allow_downgrade() } else { c };
    Ok(c)
}

fn dial_with_cred(
    uri: String,
    payload: &str,
    allow_insec: bool,
    disable_webrtc: bool,
) -> Result<DialBuilder<WithCredentials>> {
    let creds = CredentialsExt::new(String::from("robot-location-secret"), String::from(payload));
    let c = DialOptions::builder().uri(&uri).with_credentials(creds);
    let c = if disable_webrtc {
        c.disable_webrtc()
    } else {
        c
    };
    let c = if allow_insec { c.allow_downgrade() } else { c };
    Ok(c)
}
/// Returns a path to a UDS proxy to a robot
/// # Safety
///
/// This function should be called from another language. See rpc::dial for dial from rust
/// # Arguments
/// * `c_uri` a C-style string representing the robot your are proxiying to
/// * `c_payload` a C-style string that is the robot's secret, set to NULL if you don't need authentication
/// * `c_allow_insecure` a bool, set to true when allowing insecure connection to your robot
/// * `rt_ptr` a pointer to a rust runtime previously obtained with init_rust_runtime
#[no_mangle]
pub unsafe extern "C" fn dial(
    c_uri: *const c_char,
    c_payload: *const c_char,
    c_allow_insec: bool,
    rt_ptr: Option<&mut Ffi>,
) -> *mut c_char {
    let uri = {
        if c_uri.is_null() {
            return ptr::null_mut();
        }
        let ur = match Uri::from_maybe_shared(CStr::from_ptr(c_uri).to_bytes()) {
            Ok(ur) => ur,
            Err(e) => {
                println!("Sorry {e:?} is not a valid URI");
                return ptr::null_mut();
            }
        };
        ur
    };
    let allow_insec = c_allow_insec;
    let payload = {
        match c_payload.is_null() {
            true => None,
            false => Some(CStr::from_ptr(c_payload)),
        }
    };
    let ctx = match rt_ptr {
        Some(rt) => rt,
        None => {
            return ptr::null_mut();
        }
    };
    let conn = match ctx
        .runtime
        .block_on(async { proxy::uds::UDSConnector::new_random() })
    {
        Ok(conn) => conn,
        Err(e) => {
            println!("Error creating the UDS proxy {e:?}");
            return ptr::null_mut();
        }
    };
    let path = match CString::new(conn.get_path()) {
        Ok(s) => s,
        Err(e) => {
            println!("Error getting the path {e:?}");
            return ptr::null_mut();
        }
    };
    let (tx, rx) = oneshot::channel::<()>();
    let uri_str = uri.to_string();
    // if the uri is local then we can connect directly.
    let disable_webrtc = uri_str.contains(".local");
    let server = match ctx.runtime.block_on(async move {
        let dial = match payload {
            Some(p) => tower::util::Either::A(
                dial_with_cred(uri_str, p.to_str()?, allow_insec, disable_webrtc)?
                    .connect()
                    .await?,
            ),
            None => {
                let c = dial_without_cred(uri_str, allow_insec, disable_webrtc)?;
                tower::util::Either::B(c.connect().await?)
            }
        };
        let g = GRPCProxy::new(dial, uri);
        let service = ServiceBuilder::new()
            .layer(
                TraceLayer::new_for_http()
                    .make_span_with(DefaultMakeSpan::new().include_headers(true))
                    .on_request(DefaultOnRequest::new().level(Level::INFO))
                    .on_response(
                        DefaultOnResponse::new()
                            .level(Level::INFO)
                            .latency_unit(LatencyUnit::Micros),
                    ),
            )
            .service(g);
        let server = Server::builder(conn)
            .http2_only(true)
            .serve(Shared::new(service));
        Ok::<_, Box<dyn std::error::Error>>(server)
    }) {
        Ok(s) => s,
        Err(e) => {
            println!("Error building GRPC proxy reason : {e:?}");
            return ptr::null_mut();
        }
    };
    let server = server.with_graceful_shutdown(async {
        rx.await.ok();
    });
    ctx.push_signal(tx);
    let h = ctx.runtime.spawn(async {
        let _ = server.await;
    });
    ctx.push_handle(h);
    path.into_raw()
}

/// This function must be used to free the path returned by the dial_direct function
/// # Safety
///
/// This function must be use from outside a rust context
/// # Arguments
/// * `c_char` a pointer to the string returned by dial_direct
#[no_mangle]
pub unsafe extern "C" fn free_string(s: *mut c_char) {
    if s.is_null() {
        return;
    }
    let _ = CString::from_raw(s);
}

#[no_mangle]
pub extern "C" fn free_rust_runtime(rt_ptr: Option<Box<Ffi>>) -> i32 {
    let ctx = match rt_ptr {
        Some(ctx) => ctx,
        None => {
            return -1;
        }
    };
    match ctx.sigs {
        Some(sigs) => {
            for sig in sigs {
                let _ = sig.send(());
            }
        }
        None => {}
    }

    match ctx.jhs {
        Some(jhs) => {
            for js in jhs {
                let _ = ctx.runtime.block_on(async move { js.await });
            }
        }
        None => {}
    }
    0
}
