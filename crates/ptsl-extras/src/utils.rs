#![allow(dead_code)]

use ptsl_client::error::Result;
use ptsl_protos::error::Error;
use ptsl_protos::types::TripleBool;

pub fn try_from_proto<T, U>(value: T) -> Result<U>
where
  U: TryFrom<T>,
  U::Error: Into<Error>,
{
  U::try_from(value).map_err(Into::into).map_err(Into::into)
}

pub const fn triple_bool(value: Option<bool>) -> TripleBool {
  match value {
    None => TripleBool::TbNone,
    Some(true) => TripleBool::TbTrue,
    Some(false) => TripleBool::TbFalse,
  }
}
