use syn::Result;

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub enum Once<T> {
  None,
  Some(T),
}

impl<T> Once<T> {
  #[inline]
  pub fn try_once<F>(&mut self, init: F) -> Result<()>
  where
    F: FnOnce() -> Result<T>,
  {
    if let Self::None = self {
      *self = init().map(Self::Some)?;
    }

    Ok(())
  }

  #[inline]
  pub fn into_option(self) -> Option<T> {
    match self {
      Self::None => None,
      Self::Some(inner) => Some(inner),
    }
  }
}
