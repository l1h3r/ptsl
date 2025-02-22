use ptsl_future::retry::Retry;
use ptsl_protos::types::ptsl_client::PtslClient;
use ptsl_protos::types::Request;
use ptsl_protos::types::Response;
use tonic::Streaming;

use crate::client::Config;
use crate::error::TransportError;
use crate::tonic::Channel;
use crate::tonic::Endpoint;

// =============================================================================
// gRPC
//
// # Tonic Notes
//
// https://github.com/hyperium/tonic/issues/959
// https://github.com/hyperium/tonic/issues/1014#issuecomment-1156611566
// =============================================================================

/// gRPC core API.
#[derive(Clone, Debug)]
pub struct Rpc {
  inner: PtslClient<Channel>,
}

impl Rpc {
  const MAX_DECODE: usize = 1024 * 1024 * 2; // 2MB
  const MAX_ENCODE: usize = 1024 * 1024 * 2; // 2MB

  /// Connect to the PTSL gRPC endpoint.
  ///
  /// # Errors
  ///
  /// Returns [`Err`] if connecting to the endpoint fails.
  pub async fn connect(config: &Config) -> Result<Self, TransportError> {
    Ok(Self {
      inner: connect(config).await?,
    })
  }

  /// Send a gRPC request to the PTSL server.
  ///
  /// # Errors
  ///
  /// Returns [`Err`] if the gRPC request fails.
  pub async fn send_request(&mut self, request: Request) -> Result<Response, TransportError> {
    self
      .inner
      .send_grpc_request(request)
      .await
      .map_err(TransportError::Request)
      .map(tonic::Response::into_inner)
  }

  /// Send a gRPC streaming request to the PTSL server.
  ///
  /// # Errors
  ///
  /// Returns [`Err`] if the gRPC request fails.
  pub async fn send_streaming_request(
    &mut self,
    request: Request,
  ) -> Result<Stream, TransportError> {
    self
      .inner
      .send_grpc_streaming_request(request)
      .await
      .map_err(TransportError::Request)
      .map(tonic::Response::into_inner)
      .map(Stream::new)
  }
}

// =============================================================================
// Connect to gRPC Server
// =============================================================================

#[allow(clippy::significant_drop_in_scrutinee)]
async fn connect(config: &Config) -> Result<PtslClient<Channel>, TransportError> {
  let endpoint: Endpoint = create_endpoint(config)?;
  let create = || async { endpoint.connect().await };

  Retry::with_config(create, config.connection_retry)
    .await
    .map(create_client)
    .map_err(TransportError::Connect)
}

fn create_endpoint(config: &Config) -> Result<Endpoint, TransportError> {
  Endpoint::new(config.address.clone())
    .map(|endpoint| endpoint.connect_timeout(config.connect_timeout))
    .map(|endpoint| endpoint.request_timeout(config.request_timeout))
    .map_err(TransportError::Connect)
}

fn create_client(channel: Channel) -> PtslClient<Channel> {
  PtslClient::new(channel)
    .max_decoding_message_size(Rpc::MAX_DECODE)
    .max_encoding_message_size(Rpc::MAX_ENCODE)
}

// =============================================================================
// Response Stream
// =============================================================================

/// A stream of gRPC responses.
#[derive(Debug)]
pub struct Stream {
  inner: Streaming<Response>,
}

impl Stream {
  const fn new(inner: Streaming<Response>) -> Self {
    Self { inner }
  }

  /// Returns the next message in the response stream, or `None`.
  ///
  /// # Errors
  ///
  /// Returns [`Err`] if polling the next message fails.
  #[inline]
  pub async fn message(&mut self) -> Result<Option<Response>, TransportError> {
    self.inner.message().await.map_err(TransportError::Stream)
  }
}
