use http::Uri;
use hyper::client::conn::Builder;
use hyper::client::connect::Connection as HyperConnection;
use hyper::client::service::Connect as HyperConnect;
use std::fmt::Debug;
use std::fmt::Formatter;
use std::fmt::Result as FmtResult;
use std::task::Context;
use std::task::Poll;
use std::time::Duration;
use tokio::io::AsyncRead;
use tokio::io::AsyncWrite;
use tower::reconnect::Reconnect;
use tower::util::BoxService;
use tower::Service;

use crate::tonic::middleware::Origin;
use crate::tonic::BoxBody;
use crate::tonic::BoxConn;
use crate::tonic::DynError;
use crate::tonic::Endpoint;
use crate::tonic::Request;

pub struct Connection {
  inner: BoxConn,
}

impl Connection {
  pub fn new<C>(connector: C, endpoint: Endpoint) -> Self
  where
    C: Service<Uri> + Send + 'static,
    C::Error: Into<DynError> + Send,
    C::Future: Unpin + Send,
    C::Response: AsyncRead + AsyncWrite + HyperConnection + Unpin + Send + 'static,
  {
    let mut builder: Builder = Builder::new();

    builder.executor(endpoint.executor.clone());
    builder.http2_adaptive_window(false);
    builder.http2_initial_connection_window_size(None);
    builder.http2_initial_stream_window_size(None);
    builder.http2_keep_alive_interval(None);
    builder.http2_keep_alive_timeout(Duration::from_secs(20));
    builder.http2_keep_alive_while_idle(false);
    builder.http2_max_concurrent_reset_streams(10);
    builder.http2_max_frame_size(None);
    builder.http2_max_send_buf_size(1024 * 1024);
    builder.http2_only(true);

    let connector: HyperConnect<C, BoxBody, Uri> = HyperConnect::new(connector, builder);
    let connector: Reconnect<_, Uri> = Reconnect::new::<(), ()>(connector, endpoint.uri.clone());
    let connector: Origin<Reconnect<_, Uri>> = Origin::new(connector, endpoint.uri.clone());

    Self {
      inner: BoxService::new(connector),
    }
  }
}

impl Debug for Connection {
  fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
    f.debug_struct("Connection").finish()
  }
}

impl Service<Request> for Connection {
  type Response = <BoxConn as Service<Request>>::Response;
  type Error = <BoxConn as Service<Request>>::Error;
  type Future = <BoxConn as Service<Request>>::Future;

  #[inline]
  fn poll_ready(&mut self, context: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
    self.inner.poll_ready(context)
  }

  #[inline]
  fn call(&mut self, request: Request) -> Self::Future {
    self.inner.call(request)
  }
}
