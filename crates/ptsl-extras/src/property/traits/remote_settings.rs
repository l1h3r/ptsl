use ptsl_client::error::Error;
use ptsl_client::error::Result;

/// Shared behaviour for types that can store properties.
pub trait RemoteSettings: Sized {
  /// The type of property stored.
  type Data;

  /// An iterator over all valid property values.
  type Iter: IntoIterator<Item = Self::Data>;

  /// Expand `self` into current/allowed property values.
  fn expand(self) -> (Self::Data, Self::Iter);

  #[doc(hidden)]
  #[inline]
  fn convert<T>(self) -> Result<(T, Vec<T>)>
  where
    T: TryFrom<Self::Data>,
    T::Error: Into<Error>,
  {
    let (data, iter): (Self::Data, Self::Iter) = self.expand();

    let data: T = Self::convert_data(data)?;
    let list: Vec<T> = Self::convert_iter(iter)?;

    Ok((data, list))
  }

  #[doc(hidden)]
  #[inline]
  fn convert_data<T>(one: Self::Data) -> Result<T>
  where
    T: TryFrom<Self::Data>,
    T::Error: Into<Error>,
  {
    T::try_from(one).map_err(Into::into)
  }

  #[doc(hidden)]
  #[inline]
  fn convert_iter<T>(all: Self::Iter) -> Result<Vec<T>>
  where
    T: TryFrom<Self::Data>,
    T::Error: Into<Error>,
  {
    all.into_iter().map(Self::convert_data).collect()
  }
}
