use serde::Deserialize;
use serde_json::from_str;

use crate::error::Error;
use crate::error::ErrorKind;
use crate::error::Result;

mod private {
  pub trait Sealed {}
}

// =============================================================================
// Decode JSON
// =============================================================================

/// Convenience wrapper around [`serde_json::from_str`][from_str].
pub trait Decode<'de>: private::Sealed + Deserialize<'de> {
  /// Deserializes `Self` from the given JSON `text`.
  ///
  /// # Errors
  ///
  /// Returns [`Err`] if deserialization fails.
  #[inline]
  fn decode(text: &'de str) -> Result<Self> {
    from_str(text).map_err(|error| Error::new(ErrorKind::DecodeJson, error))
  }
}

impl<'de, T: Deserialize<'de>> private::Sealed for T {}
impl<'de, T: Deserialize<'de>> Decode<'de> for T {}
