use http::Uri;
use ptsl_future::retry::Config as RetryConfig;
use std::time::Duration;

use crate::consts::CONNECT_TIMEOUT;
use crate::consts::ENDPOINT;
use crate::consts::REQUEST_TIMEOUT;
use crate::consts::RETRY_ATTEMPTS;
use crate::consts::RETRY_INTERVAL;

/// Configuration for gRPC [`client`][crate::client::Client].
#[derive(Clone, Debug)]
pub struct Config {
  pub(crate) address: Uri,
  pub(crate) launch: bool,
  pub(crate) locate: bool,
  pub(crate) connection_retry: RetryConfig,
  pub(crate) connect_timeout: Duration,
  pub(crate) request_timeout: Duration,
}

impl Config {
  /// Create a new `Config` with default settings.
  #[inline]
  pub fn new() -> Self {
    Self {
      address: Uri::from_static(ENDPOINT),
      launch: true,
      locate: false,
      connection_retry: RetryConfig::new(RETRY_ATTEMPTS).fixed(RETRY_INTERVAL),
      connect_timeout: CONNECT_TIMEOUT,
      request_timeout: REQUEST_TIMEOUT,
    }
  }

  /// Set the gRPC endpoint to handle client requests.
  #[inline]
  pub fn address(mut self, value: impl Into<Uri>) -> Self {
    self.address = value.into();
    self
  }

  /// Enable launching Pro Tools when initializing client.
  #[inline]
  pub fn launch(mut self, value: bool) -> Self {
    self.launch = value;
    self
  }

  /// TODO
  #[inline]
  pub fn locate(mut self, value: bool) -> Self {
    self.locate = value;
    self
  }

  /// Set configuration for retrying connection attempts.
  #[inline]
  pub fn connection_retry(mut self, value: RetryConfig) -> Self {
    self.connection_retry = value;
    self
  }

  /// Set the timeout applied when connecting to `address`.
  #[inline]
  pub fn connect_timeout(mut self, value: Duration) -> Self {
    self.connect_timeout = value;
    self
  }

  /// Set the timeout applied to each gRPC request.
  #[inline]
  pub fn request_timeout(mut self, value: Duration) -> Self {
    self.request_timeout = value;
    self
  }
}

impl Default for Config {
  #[inline]
  fn default() -> Self {
    Self::new()
  }
}
