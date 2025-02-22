use proc_macro2::Span;
use syn::Error;
use syn::Expr;
use syn::Lit;
use syn::Meta;
use syn::Path;
use syn::Result;

use crate::attrs::parse_meta;

pub trait ParseItem: Sized {
  #[inline]
  fn from_meta(meta: &Meta) -> Result<Self> {
    match meta {
      Meta::Path(meta) => Self::from_path(meta),
      Meta::List(meta) => Self::from_list(&parse_meta(meta.tokens.clone())?),
      Meta::NameValue(meta) => Self::from_expr(&meta.value),
    }
  }

  #[inline]
  fn from_path(_meta: &Path) -> Result<Self> {
    unimplemented!("from_path")
  }

  #[inline]
  fn from_list(_meta: &[Meta]) -> Result<Self> {
    unimplemented!("from_list")
  }

  #[inline]
  fn from_expr(meta: &Expr) -> Result<Self> {
    match meta {
      Expr::Lit(meta) => Self::from_data(&meta.lit),
      Expr::Group(meta) => Self::from_expr(&meta.expr),
      _ => unimplemented!("from_expr"),
    }
  }

  #[inline]
  fn from_data(meta: &Lit) -> Result<Self> {
    match meta {
      Lit::Str(meta) => Self::from_str(meta.value()),
      Lit::Char(meta) => Self::from_char(meta.value()),
      Lit::Bool(meta) => Self::from_bool(meta.value),
      _ => unimplemented!("from_data"),
    }
  }

  #[inline]
  fn from_str(_meta: String) -> Result<Self> {
    unimplemented!("from_str")
  }

  #[inline]
  fn from_char(_meta: char) -> Result<Self> {
    unimplemented!("from_char")
  }

  #[inline]
  fn from_bool(_meta: bool) -> Result<Self> {
    unimplemented!("from_bool")
  }
}

// =============================================================================
// Impls
// =============================================================================

impl ParseItem for String {
  #[inline]
  fn from_str(meta: String) -> Result<Self> {
    Ok(meta)
  }
}

impl ParseItem for bool {
  #[inline]
  fn from_path(_meta: &syn::Path) -> Result<Self> {
    Ok(true)
  }

  #[inline]
  fn from_str(meta: String) -> Result<Self> {
    meta
      .parse()
      .map_err(|error| Error::new(Span::call_site(), error))
  }

  #[inline]
  fn from_bool(meta: bool) -> Result<Self> {
    Ok(meta)
  }
}

impl ParseItem for Expr {
  #[inline]
  fn from_expr(meta: &Expr) -> Result<Self> {
    Ok(meta.clone())
  }
}
