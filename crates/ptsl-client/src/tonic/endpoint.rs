use http::Uri;
use hyper::client::HttpConnector;
use std::time::Duration;

use crate::tonic::Channel;
use crate::tonic::DynError;
use crate::tonic::Error;
use crate::tonic::Executor;

#[derive(Clone)]
pub struct Endpoint {
  pub(crate) uri: Uri,
  pub(crate) connect_timeout: Option<Duration>,
  pub(crate) request_timeout: Option<Duration>,
  pub(crate) executor: Executor,
}

impl Endpoint {
  #[inline]
  pub fn new<E>(endpoint: E) -> Result<Self, Error>
  where
    E: TryInto<Self>,
    E::Error: Into<DynError>,
  {
    endpoint
      .try_into()
      .map_err(|error| Error::from_source(error.into()))
  }

  #[inline]
  pub fn connect_timeout(mut self, duration: Duration) -> Self {
    self.connect_timeout = Some(duration);
    self
  }

  #[inline]
  pub fn request_timeout(mut self, duration: Duration) -> Self {
    self.request_timeout = Some(duration);
    self
  }

  pub async fn connect(&self) -> Result<Channel, Error> {
    let mut http: HttpConnector = HttpConnector::new();

    http.enforce_http(false);
    http.set_connect_timeout(self.connect_timeout);
    http.set_happy_eyeballs_timeout(Some(Duration::from_millis(300)));
    http.set_keepalive(None);
    http.set_nodelay(true);
    http.set_recv_buffer_size(None);
    http.set_reuse_address(false);
    http.set_send_buffer_size(None);

    Channel::connect(http, self.clone()).await
  }
}

impl From<Uri> for Endpoint {
  #[inline]
  fn from(other: Uri) -> Self {
    Self {
      uri: other,
      connect_timeout: None,
      request_timeout: None,
      executor: Executor::tokio(),
    }
  }
}
