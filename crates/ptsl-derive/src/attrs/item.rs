use syn::Meta;
use syn::Result;

use crate::attrs::ParseItem;

pub struct Item {
  name: &'static str,
  meta: Meta,
}

impl Item {
  #[inline]
  pub(super) const fn new(name: &'static str, meta: Meta) -> Self {
    Self { name, meta }
  }

  #[inline]
  pub const fn name(&self) -> &'static str {
    self.name
  }

  #[inline]
  pub fn parse<T: ParseItem>(&self) -> Result<T> {
    T::from_meta(&self.meta)
  }
}
