use proc_macro2::extra::DelimSpan;
use proc_macro2::Delimiter;
use proc_macro2::Group;
use proc_macro2::TokenStream;
use syn::parse::Parser;
use syn::punctuated::Punctuated;
use syn::spanned::Spanned;
use syn::token::Paren;
use syn::Attribute;
use syn::Error;
use syn::MacroDelimiter;
use syn::Meta;
use syn::MetaList;
use syn::Path;
use syn::Result;
use syn::Token;

pub fn parse_meta(input: TokenStream) -> Result<Vec<Meta>> {
  Punctuated::<Meta, Token![,]>::parse_terminated
    .parse2(input)
    .map(|punctuated| punctuated.into_iter().collect())
}

pub fn parse_attr(attr: &Attribute) -> Result<MetaList> {
  match attr.meta {
    Meta::List(ref meta) => Ok(meta.clone()),
    Meta::NameValue(_) => Err(Error::new(attr.span(), "NameValue Not Supported")),
    Meta::Path(ref meta) => Ok(metalist(attr, meta)),
  }
}

fn metalist(attr: &Attribute, path: &Path) -> MetaList {
  MetaList {
    path: path.clone(),
    delimiter: MacroDelimiter::Paren(paren(attr)),
    tokens: Default::default(),
  }
}

fn paren(attr: &Attribute) -> Paren {
  Paren { span: span(attr) }
}

fn span(attr: &Attribute) -> DelimSpan {
  let mut group: Group = Group::new(Delimiter::None, TokenStream::new());
  group.set_span(attr.span());
  group.delim_span()
}
