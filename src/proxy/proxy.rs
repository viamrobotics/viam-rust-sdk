use http::uri::Uri;
use hyper::body::HttpBody;
use hyper::Request;
use std::task::{Context, Poll};
use tower::Service;
#[derive(Clone, Debug)]
pub struct GRPCProxy<T> {
    inner: T,
    uri_parts: Uri,
}

impl<T> GRPCProxy<T> {
    pub fn new(inner: T, uri: Uri) -> Self {
        GRPCProxy {
            inner,
            uri_parts: uri,
        }
    }
}

impl<T, ReqBody> Service<Request<ReqBody>> for GRPCProxy<T>
where
    T: Service<Request<tonic::body::BoxBody>> + Clone,
    ReqBody: http_body::Body<Data = hyper::body::Bytes> + Send + 'static,
    ReqBody::Error: ToString + Send + 'static,
{
    type Response = T::Response;
    type Error = T::Error;
    type Future = T::Future;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx).map_err(Into::into)
    }
    fn call(&mut self, request: Request<ReqBody>) -> Self::Future {
        let (mut h, b) = request.into_parts();
        let b = b
            .map_err(|e| tonic::Status::new(tonic::Code::Unknown, e.to_string()))
            .boxed_unsync();
        let from_uri = h.uri.into_parts();
        let proxy_uri = Uri::builder()
            .scheme(self.uri_parts.scheme_str().unwrap().clone())
            .authority(self.uri_parts.authority().unwrap().clone())
            .path_and_query(from_uri.path_and_query.unwrap().clone())
            .build()
            .unwrap();
        h.uri = proxy_uri;
        let req = Request::from_parts(h, b);
        self.inner.call(req)
    }
}
