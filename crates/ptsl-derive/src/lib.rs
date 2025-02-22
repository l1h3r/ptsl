//! PTSL Derive Macros
//!
//! Internal and not intended for your eyes.

#![deny(missing_docs)]

mod ast;
mod attrs;

use proc_macro::TokenStream;
use quote::ToTokens;

/// Proc macro for delegating function calls.
#[proc_macro]
pub fn delegate(input: TokenStream) -> TokenStream {
  match syn::parse::<ast::Delegate>(input) {
    Ok(input) => input.to_token_stream().into(),
    Err(error) => error.into_compile_error().into(),
  }
}
