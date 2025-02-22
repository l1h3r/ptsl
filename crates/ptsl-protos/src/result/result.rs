use core::fmt::Debug;
use core::fmt::Formatter;
use core::fmt::Result as FmtResult;
use ptsl_derive::delegate;

use crate::error::Error;
use crate::error::Result;
use crate::result::CommandError;
use crate::result::CommandHeader;
use crate::result::CommandStatus;
use crate::traits::Decode;
use crate::types::CommandId;
use crate::types::Response;
use crate::types::ResponseHeader;
use crate::types::TaskStatus;

// =============================================================================
// Command Result
// =============================================================================

/// Possible results from a gRPC command request.
#[derive(Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum CommandResult<T> {
  /// Success result.
  Pass(CommandPass<T>),
  /// Error result.
  Fail(CommandFail),
  /// Empty result.
  None(CommandNone),
}

impl<T> CommandResult<T> {
  /// Create a new command result from any response.
  ///
  /// # Errors
  ///
  /// Returns [`Err`] if any properties or `response` are not valid.
  pub fn try_new(command: CommandId, response: Response) -> Result<Self>
  where
    T: for<'de> Decode<'de>,
  {
    fn filter_empty(string: &str) -> Option<&str> {
      if !string.is_empty() {
        Some(string)
      } else {
        None
      }
    }

    if let Some(header) = response.header {
      if let Some(json) = filter_empty(&response.response_error_json) {
        CommandFail::new_json(header, json).map(Self::Fail)
      } else if let Some(json) = filter_empty(&response.response_body_json) {
        CommandPass::new_json(header, json).map(Self::Pass)
      } else {
        let status: TaskStatus = TaskStatus::try_from(header.status)?;

        if status.is_failed() || status.is_failed_invalid() || status.is_completed_invalid() {
          CommandFail::new(header).map(Self::Fail)
        } else if status.is_completed() {
          // TODO: Fix this hack
          CommandPass::new_json(header, "null").map(Self::Pass)
        } else {
          CommandPass::new(header).map(Self::Pass)
        }
      }
    } else {
      Ok(Self::None(CommandNone::new(command)))
    }
  }

  #[doc(hidden)]
  pub const fn empty(command: CommandId) -> Self {
    Self::None(CommandNone::new(command))
  }

  /// Returns `true` if the command was successful.
  #[inline]
  pub const fn is_pass(&self) -> bool {
    matches!(self, Self::Pass(_))
  }

  /// Returns `true` if the command failed.
  #[inline]
  pub const fn is_fail(&self) -> bool {
    matches!(self, Self::Fail(_))
  }

  /// Returns `true` if the command response was empty.
  #[inline]
  pub const fn is_none(&self) -> bool {
    matches!(self, Self::None(_))
  }

  /// Returns the command type.
  #[inline]
  pub const fn command(&self) -> CommandId {
    match self {
      Self::Pass(inner) => inner.header().command(),
      Self::Fail(inner) => inner.header().command(),
      Self::None(inner) => inner.command(),
    }
  }

  /// Returns a reference to the command header, or `None`.
  #[inline]
  pub const fn header(&self) -> Option<&CommandHeader> {
    match self {
      Self::Pass(inner) => Some(inner.header()),
      Self::Fail(inner) => Some(inner.header()),
      Self::None(_) => None,
    }
  }

  /// Returns a reference to the command status, or `None`.
  #[inline]
  pub const fn status(&self) -> Option<&CommandStatus> {
    match self {
      Self::Pass(inner) => Some(inner.status()),
      Self::Fail(inner) => Some(inner.status()),
      Self::None(_) => None,
    }
  }

  /// Converts the command result into an [`Result<T>`].
  ///
  /// # Errors
  ///
  /// Returns [`Err`] if the result is empty, represents an error, or is [`None`].
  #[inline]
  pub fn into_result(self) -> Result<T> {
    match self {
      Self::Pass(inner) => inner.try_into_result(),
      Self::Fail(inner) => Err(inner.into_error()),
      Self::None(inner) => Err(inner.into_error()),
    }
  }

  delegate! {
    #[delegate(doc = "See [`TaskStatus::$function`][TaskStatus::$function]")]
    #[delegate(map = false)]
    to self.status() => {
      pub const fn is_queued(&self) -> bool;
      pub const fn is_pending(&self) -> bool;
      pub const fn is_progress(&self) -> bool;
      pub const fn is_completed(&self) -> bool;
      pub const fn is_failed(&self) -> bool;
      pub const fn is_waiting(&self) -> bool;
      pub const fn is_completed_invalid(&self) -> bool;
      pub const fn is_failed_invalid(&self) -> bool;
      pub const fn is_completed_any(&self) -> bool;
      pub const fn is_failed_any(&self) -> bool;
      pub const fn is_done(&self) -> bool;
    }
  }
}

impl<T> Debug for CommandResult<T> {
  fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
    match self {
      Self::Pass(inner) => Debug::fmt(inner, f),
      Self::Fail(inner) => Debug::fmt(inner, f),
      Self::None(inner) => Debug::fmt(inner, f),
    }
  }
}

// =============================================================================
// Command Result (Pass)
// =============================================================================

/// Command result that contains a success response.
#[derive(Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct CommandPass<T> {
  header: CommandHeader,
  status: CommandStatus,
  result: Option<T>,
}

impl<T> CommandPass<T> {
  #[inline]
  fn new(header: ResponseHeader) -> Result<Self> {
    Ok(Self {
      header: CommandHeader::new(header.command, header.task_id)?,
      status: CommandStatus::new(header.progress, header.status)?,
      result: None,
    })
  }

  #[inline]
  fn new_json(header: ResponseHeader, result: &str) -> Result<Self>
  where
    T: for<'de> Decode<'de>,
  {
    Ok(Self {
      header: CommandHeader::new(header.command, header.task_id)?,
      status: CommandStatus::new(header.progress, header.status)?,
      result: T::decode(result).map(Some)?,
    })
  }

  /// Returns a reference to the command header.
  #[inline]
  pub const fn header(&self) -> &CommandHeader {
    &self.header
  }

  /// Returns a reference to the command status.
  #[inline]
  pub const fn status(&self) -> &CommandStatus {
    &self.status
  }

  /// Returns a reference to the command result, or [`None`].
  #[inline]
  pub const fn result(&self) -> Option<&T> {
    self.result.as_ref()
  }

  /// Returns the command result, or [`None`].
  #[inline]
  pub fn into_result(self) -> Option<T> {
    self.result
  }

  /// Returns the command result, or [`Err`].
  ///
  /// # Errors
  ///
  /// Returns [`Err`] if the underlying result is [`None`].
  #[inline]
  pub fn try_into_result(self) -> Result<T> {
    let Some(result) = self.result else {
      return Err(Error::incomplete(self.header.command()));
    };

    Ok(result)
  }
}

impl<T> Debug for CommandPass<T> {
  fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
    f.debug_struct("CommandPass")
      .field("header", &self.header)
      .field("status", &self.status)
      .finish_non_exhaustive()
  }
}

// =============================================================================
// Command Result (Fail)
// =============================================================================

/// Command result that contains an error response.
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct CommandFail {
  header: CommandHeader,
  status: CommandStatus,
  result: Option<CommandError>,
}

impl CommandFail {
  #[inline]
  fn new(header: ResponseHeader) -> Result<Self> {
    Ok(Self {
      header: CommandHeader::new(header.command, header.task_id)?,
      status: CommandStatus::new(header.progress, header.status)?,
      result: None,
    })
  }

  #[inline]
  fn new_json(header: ResponseHeader, result: &str) -> Result<Self> {
    Ok(Self {
      header: CommandHeader::new(header.command, header.task_id)?,
      status: CommandStatus::new(header.progress, header.status)?,
      result: CommandError::new(result).map(Some)?,
    })
  }

  /// Returns a reference to the command header.
  #[inline]
  pub const fn header(&self) -> &CommandHeader {
    &self.header
  }

  /// Returns a reference to the command status.
  #[inline]
  pub const fn status(&self) -> &CommandStatus {
    &self.status
  }

  /// Returns a reference to the command result, or [`None`].
  #[inline]
  pub const fn result(&self) -> Option<&CommandError> {
    self.result.as_ref()
  }

  /// Converts the command result into an [`Error`].
  #[inline]
  pub fn into_error(self) -> Error {
    Error::bad_response(self.header.command(), self.result)
  }
}

// =============================================================================
// Command Result (None)
// =============================================================================

/// Command result that contains no response info.
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct CommandNone {
  command: CommandId,
}

impl CommandNone {
  #[inline]
  const fn new(command: CommandId) -> Self {
    Self { command }
  }

  /// Returns the command type.
  #[inline]
  pub const fn command(&self) -> CommandId {
    self.command
  }

  /// Converts the command result into an [`Error`].
  #[inline]
  pub fn into_error(self) -> Error {
    Error::bad_response(self.command, None)
  }
}
