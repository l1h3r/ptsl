//! Library errors.

use core::fmt::Display;
use core::fmt::Formatter;
use core::fmt::Result as FmtResult;
use std::error::Error as StdError;

/// Alias for [`core::result::Result`].
pub type Result<T, E = Error> = core::result::Result<T, E>;

// =============================================================================
// Error
// =============================================================================

/// Errors returned from protobuf-related operations.
#[derive(Debug)]
pub struct Error {
  kind: ErrorKind,
  source: Box<dyn StdError + 'static>,
}

impl Error {
  #[inline]
  pub(crate) fn new(kind: ErrorKind, source: impl StdError + 'static) -> Self {
    Self {
      kind,
      source: Box::new(source),
    }
  }
}

impl Display for Error {
  fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
    match self.kind {
      ErrorKind::DecodeJson => write!(f, "[decode json]: {}", self.source),
      ErrorKind::EncodeJson => write!(f, "[encode json]: {}", self.source),
      ErrorKind::Protobuf => write!(f, "[protobuf]: {}", self.source),
    }
  }
}

impl StdError for Error {
  #[inline]
  fn source(&self) -> Option<&(dyn StdError + 'static)> {
    Some(&*self.source)
  }
}

impl From<prost::DecodeError> for Error {
  #[inline]
  fn from(other: prost::DecodeError) -> Self {
    Self::new(ErrorKind::Protobuf, other)
  }
}

// =============================================================================
// Error Kind
// =============================================================================

/// A list of the general categories of library errors.
#[derive(Clone, Copy, Debug)]
pub enum ErrorKind {
  /// JSON deserialization error.
  DecodeJson,
  /// JSON serialization error.
  EncodeJson,
  /// Protobuf library error.
  Protobuf,
}
