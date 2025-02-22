//! PTSL Derive Macros
//!
//! Internal and not intended for your eyes.

#![deny(missing_docs)]

mod ast;
mod attrs;

use proc_macro::TokenStream;
use quote::ToTokens;

/// Derive macro for implementing extensions to generated protobuf types.
#[proc_macro_derive(ProtoType)]
pub fn prototype(input: TokenStream) -> TokenStream {
  match syn::parse::<ast::ProtoType>(input) {
    Ok(input) => input.to_token_stream().into(),
    Err(error) => error.into_compile_error().into(),
  }
}

/// Proc macro for delegating function calls.
#[proc_macro]
pub fn delegate(input: TokenStream) -> TokenStream {
  match syn::parse::<ast::Delegate>(input) {
    Ok(input) => input.to_token_stream().into(),
    Err(error) => error.into_compile_error().into(),
  }
}

/// Proc macro for implementing unified command API.
#[proc_macro]
pub fn commands(input: TokenStream) -> TokenStream {
  match syn::parse::<ast::CommandList>(input) {
    Ok(input) => input.to_token_stream().into(),
    Err(error) => error.into_compile_error().into(),
  }
}
