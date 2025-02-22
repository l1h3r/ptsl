//! Library errors.

use std::convert::Infallible;
use std::error::Error as StdError;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result as FmtResult;
use std::process::ExitStatusError;

/// Alias for [`core::result::Result`].
pub type Result<T, E = Error> = core::result::Result<T, E>;

/// Errors returned from any `ptsl` operation.
#[derive(Debug)]
pub enum Error {
  /// Process error.
  OsProcess(OsProcessError),
  /// gRPC error.
  Transport(TransportError),
  /// Protobuf error.
  Protobufs(ptsl_protos::error::Error),
}

impl From<OsProcessError> for Error {
  #[inline]
  fn from(other: OsProcessError) -> Self {
    Self::OsProcess(other)
  }
}

impl From<TransportError> for Error {
  #[inline]
  fn from(other: TransportError) -> Self {
    Self::Transport(other)
  }
}

impl From<ptsl_protos::error::Error> for Error {
  #[inline]
  fn from(other: ptsl_protos::error::Error) -> Self {
    Self::Protobufs(other)
  }
}

impl From<Infallible> for Error {
  fn from(_: Infallible) -> Self {
    unreachable!()
  }
}

impl Display for Error {
  fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
    match self {
      Self::OsProcess(inner) => Display::fmt(inner, f),
      Self::Transport(inner) => Display::fmt(inner, f),
      Self::Protobufs(inner) => Display::fmt(inner, f),
    }
  }
}

impl StdError for Error {
  #[inline]
  fn source(&self) -> Option<&(dyn StdError + 'static)> {
    match self {
      Self::OsProcess(inner) => Some(inner),
      Self::Transport(inner) => Some(inner),
      Self::Protobufs(inner) => Some(inner),
    }
  }
}

// =============================================================================
// OS Process Error
// =============================================================================

/// Errors returned from process interactions.
#[derive(Debug)]
pub enum OsProcessError {
  /// Error returned attempting to launch Pro Tools.
  Launch(std::io::Error),
  /// Error returned from launching Pro Tools.
  Status(ExitStatusError),
}

impl Display for OsProcessError {
  fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
    match self {
      Self::Launch(inner) => write!(f, "[launch]: {inner}"),
      Self::Status(inner) => write!(f, "[status]: {inner}"),
    }
  }
}

impl StdError for OsProcessError {
  #[inline]
  fn source(&self) -> Option<&(dyn StdError + 'static)> {
    match self {
      Self::Launch(inner) => Some(inner),
      Self::Status(inner) => Some(inner),
    }
  }
}

// =============================================================================
// gRPC Transport Error
// =============================================================================

/// Errors returned from gRPC operations.
#[derive(Debug)]
pub enum TransportError {
  /// Error returned connecting to gRPC endpoint.
  Connect(crate::tonic::Error),
  /// Error returned from attempting gRPC request.
  Request(tonic::Status),
  /// Error returned from attempting gRPC streaming request.
  Stream(tonic::Status),
}

impl Display for TransportError {
  fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
    match self {
      Self::Connect(inner) => write!(f, "[connect]: {inner}"),
      Self::Request(inner) => write!(f, "[request]: {inner}"),
      Self::Stream(inner) => write!(f, "[stream]: {inner}"),
    }
  }
}

impl StdError for TransportError {
  #[inline]
  fn source(&self) -> Option<&(dyn StdError + 'static)> {
    match self {
      Self::Connect(inner) => Some(inner),
      Self::Request(inner) => Some(inner),
      Self::Stream(inner) => Some(inner),
    }
  }
}
