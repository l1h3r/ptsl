use proc_macro2::TokenStream;
use quote::quote;
use quote::ToTokens;
use syn::braced;
use syn::parse::Parse;
use syn::parse::ParseBuffer;
use syn::parse::ParseStream;
use syn::parse_quote;
use syn::Attribute;
use syn::Expr;
use syn::FnArg;
use syn::Ident;
use syn::Pat;
use syn::Result;
use syn::Signature;
use syn::Token;
use syn::Visibility;

use crate::attrs::Once;
use crate::attrs::ParseAttr;

// =============================================================================
// Custom Keywords
// =============================================================================

mod kw {
  syn::custom_keyword!(to);
}

// =============================================================================
// Delegate
// =============================================================================

/// A set of delegated functions.
pub struct Delegate {
  attributes: Attributes,
  expression: Expr,
  functions: Vec<Function>,
}

impl Delegate {
  fn parse_functions(input: ParseStream<'_>) -> Result<Vec<Function>> {
    let mut output: Vec<Function> = Vec::new();

    while !input.is_empty() {
      let function: Function = input.parse()?;
      let _unused: Token![;] = input.parse()?;

      output.push(function);
    }

    Ok(output)
  }
}

impl Parse for Delegate {
  fn parse(input: ParseStream<'_>) -> Result<Self> {
    let functions: ParseBuffer<'_>;

    let attributes: Attributes = Attributes::parse_outer(input)?;
    let _unused: kw::to = input.parse()?;
    let expression: Expr = input.parse()?;
    let _unused: Token![=>] = input.parse()?;

    braced!(functions in input);

    let functions: Vec<Function> = Self::parse_functions(&functions)?;

    Ok(Self {
      attributes,
      expression,
      functions,
    })
  }
}

impl ToTokens for Delegate {
  fn to_tokens(&self, tokens: &mut TokenStream) {
    for function in self.functions.iter() {
      function.to_tokens(self, tokens);
    }
  }
}

// =============================================================================
// Delegate Function
// =============================================================================

struct Function {
  vis: Visibility,
  sig: Signature,
}

impl Function {
  const fn ident(&self) -> &Ident {
    &self.sig.ident
  }

  fn name(&self) -> String {
    self.ident().to_string()
  }

  fn params(&self) -> Vec<&Ident> {
    self.sig.inputs.iter().filter_map(Self::param).collect()
  }

  fn param(argument: &FnArg) -> Option<&Ident> {
    // Ok to silently drop FnArg::Receiver
    let FnArg::Typed(ref typed) = argument else {
      return None;
    };

    // Only Ident is supported
    let Pat::Ident(ident) = &*typed.pat else {
      panic!("delegate! only supports method with identifier arguments");
    };

    Some(&ident.ident)
  }

  fn docstr(&self, delegate: &Delegate) -> Option<Attribute> {
    delegate
      .attributes
      .doc
      .as_ref()
      .map(|template| template.replace("$function", &self.name()))
      .map(|string| parse_quote!(#[doc = #string]))
  }

  fn method(&self, delegate: &Delegate) -> TokenStream {
    let identifier: &Ident = self.ident();
    let expression: &Expr = &delegate.expression;
    let parameters: Vec<&Ident> = self.params();

    if let Some(ref default) = delegate.attributes.map {
      quote! {
        match #expression {
          Some(ref inner) => inner.#identifier(#(#parameters),*),
          None => #default,
        }
      }
    } else {
      quote! {
        #expression.#identifier(#(#parameters),*)
      }
    }
  }

  fn to_tokens(&self, delegate: &Delegate, tokens: &mut TokenStream) {
    let visibility: &Visibility = &self.vis;
    let signature: &Signature = &self.sig;
    let docstr: Option<Attribute> = self.docstr(delegate);
    let method: TokenStream = self.method(delegate);

    tokens.extend(quote! {
      #docstr
      #[inline]
      #visibility #signature {
        #method
      }
    });
  }
}

impl Parse for Function {
  fn parse(input: ParseStream<'_>) -> Result<Self> {
    Ok(Self {
      vis: input.parse()?,
      sig: input.parse()?,
    })
  }
}

// =============================================================================
// Delegate Attributes
// =============================================================================

struct Attributes {
  doc: Option<String>,
  map: Option<Expr>,
}

impl ParseAttr for Attributes {
  const NAME: &'static str = "delegate";
  const DATA: &'static [&'static str] = &["doc", "map"];

  fn parse(attributes: &[Attribute]) -> Result<Self> {
    let mut doc: Once<String> = Once::None;
    let mut map: Once<Expr> = Once::None;

    Self::parse_inner(attributes, |item| match item.name() {
      "doc" => doc.try_once(|| item.parse()),
      "map" => map.try_once(|| item.parse()),
      _ => unreachable!(),
    })?;

    Ok(Self {
      doc: doc.into_option(),
      map: map.into_option(),
    })
  }
}
