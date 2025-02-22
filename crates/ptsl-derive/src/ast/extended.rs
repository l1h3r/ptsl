use syn::parse_quote;
use syn::Expr;
use syn::GenericArgument;
use syn::PathArguments;
use syn::PathSegment;
use syn::Type;

// =============================================================================
// Extended Type
// =============================================================================

pub struct ExtType<'a> {
  inner: &'a Type,
}

impl<'a> ExtType<'a> {
  pub const fn new(inner: &'a Type) -> Self {
    Self { inner }
  }

  pub fn trailer(&self) -> &'a Type {
    self.generic().unwrap_or(self.inner)
  }

  pub fn initializer(&self) -> Expr {
    let Type::Path(path) = self.inner else {
      panic!("Invalid Type");
    };

    for segment in path.path.segments.iter() {
      if segment.ident == "String" {
        return parse_quote!(String::new());
      } else if segment.ident == "bool" {
        return parse_quote!(false);
      } else if segment.ident == "i32" {
        return parse_quote!(0);
      } else if segment.ident == "Vec" {
        return parse_quote!(Vec::new());
      } else if segment.ident == "Option" {
        return parse_quote!(None);
      }
    }

    panic!("Invalid Path");
  }

  fn generic(&self) -> Option<&'a Type> {
    let Some(segment) = self.segment() else {
      return None;
    };

    if segment.ident == "Option" {
      return None;
    }

    let PathArguments::AngleBracketed(ref arguments) = segment.arguments else {
      return None;
    };

    let Some(GenericArgument::Type(generic)) = arguments.args.first() else {
      return None;
    };

    Some(generic)
  }

  fn segment(&self) -> Option<&'a PathSegment> {
    let Type::Path(path) = self.inner else {
      return None;
    };

    path.path.segments.last()
  }
}
