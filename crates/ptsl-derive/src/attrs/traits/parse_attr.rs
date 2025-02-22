use syn::parse::ParseStream;
use syn::Attribute;
use syn::Meta;
use syn::MetaList;
use syn::Result;

use crate::attrs::parse_attr;
use crate::attrs::parse_meta;
use crate::attrs::Item;

pub trait ParseAttr: Sized {
  const NAME: &'static str;
  const DATA: &'static [&'static str];

  fn parse(attributes: &[Attribute]) -> Result<Self>;

  fn parse_outer(input: ParseStream<'_>) -> Result<Self> {
    input
      .call(Attribute::parse_outer)
      .and_then(|attributes| Self::parse(&attributes))
  }

  fn parse_inner<F>(attributes: &[Attribute], mut write: F) -> Result<()>
  where
    F: FnMut(Item) -> Result<()>,
  {
    for attribute in attributes {
      if !attribute.path().is_ident(Self::NAME) {
        continue;
      }

      let meta: MetaList = parse_attr(attribute)?;
      let list: Vec<Meta> = parse_meta(meta.tokens)?;

      for meta in list {
        'scan: for name in Self::DATA {
          if meta.path().is_ident(name) {
            write(Item::new(name, meta))?;
            break 'scan;
          }
        }
      }
    }

    Ok(())
  }
}
