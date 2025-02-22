use std::error::Error as StdError;
use std::fmt::Debug;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result;

use crate::tonic::DynError;
use crate::tonic::DynErrorStatic;

#[derive(Debug)]
pub struct Error {
  inner: DynErrorStatic,
}

impl Error {
  #[inline]
  pub(crate) fn from_source(from: impl Into<DynError>) -> Self {
    Self { inner: from.into() }
  }
}

impl From<DynError> for Error {
  #[inline]
  fn from(other: DynError) -> Self {
    Self::from_source(other)
  }
}

impl Display for Error {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result {
    write!(f, "transport error - {}", self.inner)
  }
}

impl StdError for Error {
  #[inline]
  fn source(&self) -> Option<&(dyn StdError + 'static)> {
    Some(&*self.inner)
  }
}
