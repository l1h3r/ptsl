use serde::Serialize;
use serde_json::to_string;

use crate::error::Error;
use crate::error::ErrorKind;
use crate::error::Result;

mod private {
  pub trait Sealed {}
}

// =============================================================================
// Encode JSON
// =============================================================================

/// Convenience wrapper around [`serde_json::to_string`][to_string].
pub trait Encode: private::Sealed + Serialize {
  /// Serializes `self` as a string of JSON.
  ///
  /// # Errors
  ///
  /// Returns [`Err`] if serialization fails.
  #[inline]
  fn encode(&self) -> Result<String> {
    to_string(self).map_err(|error| Error::new(ErrorKind::EncodeJson, error))
  }
}

impl<T: Serialize> private::Sealed for T {}
impl<T: Serialize> Encode for T {}
