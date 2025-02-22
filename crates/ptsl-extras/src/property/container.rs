use core::ops::Deref;

/// A generic property container - stores current and possible values.
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Container<T> {
  pub(crate) current: T,
  pub(crate) allowed: Vec<T>,
}

impl<T> Container<T> {
  /// Returns the current value stored in the container.
  #[inline]
  pub const fn get(&self) -> T
  where
    T: Copy,
  {
    self.current
  }

  /// Returns a reference to the current value stored in the container.
  #[inline]
  pub const fn get_ref(&self) -> &T {
    &self.current
  }

  /// Returns a mutable reference to the current value stored in the container.
  #[inline]
  pub fn get_mut(&mut self) -> &mut T {
    &mut self.current
  }

  /// Returns a slice of all supported property values.
  #[inline]
  pub fn allowed(&self) -> &[T] {
    self.allowed.as_slice()
  }

  /// Consumes `self` and returns the current value.
  #[inline]
  pub fn into_inner(self) -> T {
    self.current
  }
}

impl<T> Deref for Container<T> {
  type Target = T;

  #[inline]
  fn deref(&self) -> &Self::Target {
    self.get_ref()
  }
}

impl<T> From<(T, Vec<T>)> for Container<T> {
  #[inline]
  fn from((current, allowed): (T, Vec<T>)) -> Self {
    Self { current, allowed }
  }
}
