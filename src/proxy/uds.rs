use hyper::server::accept::Accept;
use rand::distributions::{Alphanumeric, DistString};
use std::io::Error;
use std::pin::Pin;
use std::task::{Context, Poll};
use tokio::net::{UnixListener, UnixStream};

pub struct UDSConnector {
    inner: UnixListener,
    path: String,
}

impl UDSConnector {
    pub fn new(path: String) -> Result<Self, Error> {
        let uds = UnixListener::bind(&path)?;
        Ok(UDSConnector { inner: uds, path })
    }
    pub fn new_random() -> Result<Self, Error> {
        let mut rname = Alphanumeric.sample_string(&mut rand::thread_rng(), 8);
        rname = format!("/tmp/proxy-{}.sock", rname);
        Self::new(rname)
    }
    pub fn get_path(&self) -> &str {
        &self.path
    }
}

impl Accept for UDSConnector {
    type Conn = UnixStream;
    type Error = Error;

    fn poll_accept(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<Option<Result<Self::Conn, Self::Error>>> {
        match self.inner.poll_accept(cx) {
            Poll::Pending => Poll::Pending,
            Poll::Ready(Ok((socket, _addr))) => Poll::Ready(Some(Ok(socket))),
            Poll::Ready(Err(err)) => Poll::Ready(Some(Err(err))),
        }
    }
}

impl Drop for UDSConnector {
    fn drop(&mut self) {
        std::fs::remove_file(&self.path).unwrap();
    }
}
