//! Library errors.

use core::fmt::Display;
use core::fmt::Formatter;
use core::fmt::Result as FmtResult;
use std::error::Error as StdError;

use crate::result::CommandError;
use crate::types::CommandId;

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

  #[inline]
  pub(crate) fn bad_response(command_id: CommandId, command_err: Option<CommandError>) -> Self {
    Self::new(
      ErrorKind::CommandBadResponse,
      InvalidCommand::new(command_id, command_err),
    )
  }

  #[inline]
  pub(crate) fn incomplete(command_id: CommandId) -> Self {
    Self::new(
      ErrorKind::CommandIncomplete,
      InvalidCommand::new(command_id, None),
    )
  }
}

impl Display for Error {
  fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
    match self.kind {
      ErrorKind::DecodeJson => write!(f, "[decode json]: {}", self.source),
      ErrorKind::EncodeJson => write!(f, "[encode json]: {}", self.source),
      ErrorKind::Protobuf => write!(f, "[protobuf]: {}", self.source),
      ErrorKind::CommandBadResponse => write!(f, "[bad response]: {}", self.source),
      ErrorKind::CommandIncomplete => write!(f, "[incomplete]: {}", self.source),
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
  /// Command result was empty or invalid.
  CommandBadResponse,
  /// Command completed with incomplete response.
  CommandIncomplete,
}

// =============================================================================
// Command Result Error
// =============================================================================

/// Error caused by invalid command result.
#[derive(Debug)]
pub struct InvalidCommand {
  command_id: CommandId,
  command_err: Option<CommandError>,
}

impl InvalidCommand {
  #[inline]
  pub(crate) const fn new(command_id: CommandId, command_err: Option<CommandError>) -> Self {
    Self {
      command_id,
      command_err,
    }
  }
}

impl Display for InvalidCommand {
  fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
    let name: &str = self.command_id.as_str_name();

    if let Some(ref error) = self.command_err {
      let kind: &str = error.kind().as_str_name();
      let info: &str = error.message();

      write!(f, "`{name}` - {kind} - {info}")
    } else {
      write!(f, "`{name}`")
    }
  }
}

impl StdError for InvalidCommand {}
