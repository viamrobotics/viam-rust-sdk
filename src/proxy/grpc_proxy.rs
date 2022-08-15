use http::uri::{Scheme, Uri};
use hyper::body::HttpBody;
use hyper::Request;
use std::task::{Context, Poll};
use tower::Service;
#[derive(Clone, Debug)]
pub struct GRPCProxy<T> {
    inner: T,
    uri: Uri,
}

impl<T> GRPCProxy<T> {
    pub fn new(inner: T, uri: Uri) -> Self {
        GRPCProxy { inner, uri }
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
        let mut to_uri = self.uri.clone().into_parts();
        to_uri.path_and_query = h.uri.into_parts().path_and_query;
        if to_uri.scheme.is_none() {
            to_uri.scheme = Some(Scheme::HTTPS)
        }
        let proxy_uri = Uri::from_parts(to_uri).unwrap();
        h.uri = proxy_uri;
        let req = Request::from_parts(h, b);
        self.inner.call(req)
    }
}
