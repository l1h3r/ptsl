use http::Uri;
use hyper::client::connect::Connection as HyperConnection;
use hyper::rt::Executor as _;
use std::fmt::Debug;
use std::fmt::Formatter;
use std::fmt::Result as FmtResult;
use std::task::Context;
use std::task::Poll;
use tokio::io::AsyncRead;
use tokio::io::AsyncWrite;
use tower::buffer::Buffer;
use tower::util::Either;
use tower::Service;
use tower::ServiceExt;

use crate::tonic::BoxChan;
use crate::tonic::Connection;
use crate::tonic::DynError;
use crate::tonic::Endpoint;
use crate::tonic::Error;
use crate::tonic::Executor;
use crate::tonic::Request;

#[derive(Clone)]
pub struct Channel {
  inner: BoxChan,
}

impl Channel {
  const BUFFER: usize = 1024;

  pub async fn connect<C>(connector: C, endpoint: Endpoint) -> Result<Self, Error>
  where
    C: Service<Uri> + Send + 'static,
    C::Error: Into<DynError> + Send,
    C::Future: Unpin + Send,
    C::Response: AsyncRead + AsyncWrite + HyperConnection + Unpin + Send + 'static,
  {
    let executor: Executor = endpoint.executor.clone();

    let connection: Connection = Connection::new(connector, endpoint)
      .ready_oneshot()
      .await
      .map_err(Error::from_source)?;

    let (connection, worker): (BoxChan, _) = Buffer::pair(Either::A(connection), Self::BUFFER);

    executor.execute(Box::pin(worker));

    Ok(Self { inner: connection })
  }
}

impl Debug for Channel {
  fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
    f.debug_struct("Channel").finish()
  }
}

impl Service<Request> for Channel {
  type Response = <BoxChan as Service<Request>>::Response;
  type Error = <BoxChan as Service<Request>>::Error;
  type Future = <BoxChan as Service<Request>>::Future;

  #[inline]
  fn poll_ready(&mut self, context: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
    self.inner.poll_ready(context)
  }

  #[inline]
  fn call(&mut self, request: Request) -> Self::Future {
    self.inner.call(request)
  }
}
