use http::request::Parts as HttpParts;
use http::uri::Authority;
use http::uri::Parts;
use http::uri::Scheme;
use http::Request;
use http::Uri;
use std::task::Context;
use std::task::Poll;
use tower::Service;

#[derive(Debug)]
pub struct Origin<T> {
  inner: T,
  scheme: Option<Scheme>,
  authority: Option<Authority>,
}

impl<T> Origin<T> {
  pub fn new(inner: T, origin: Uri) -> Self {
    Self::from_parts(inner, origin.into())
  }

  fn from_parts(inner: T, parts: Parts) -> Self {
    Self {
      inner,
      scheme: parts.scheme,
      authority: parts.authority,
    }
  }
}

impl<T, B> Service<Request<B>> for Origin<T>
where
  T: Service<Request<B>>,
{
  type Response = T::Response;
  type Error = T::Error;
  type Future = T::Future;

  fn poll_ready(&mut self, context: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
    self.inner.poll_ready(context)
  }

  fn call(&mut self, request: Request<B>) -> Self::Future {
    let mut req_parts: (HttpParts, B) = request.into_parts();
    let mut uri_parts: Parts = req_parts.0.uri.into();

    uri_parts.scheme = self.scheme.clone();
    uri_parts.authority = self.authority.clone();
    req_parts.0.uri = Uri::from_parts(uri_parts).expect("valid uri");

    self
      .inner
      .call(Request::from_parts(req_parts.0, req_parts.1))
  }
}
