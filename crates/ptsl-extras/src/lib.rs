//! PTSL Extra Utilities.

#![feature(iterator_try_collect)]
#![feature(variant_count)]
#![allow(async_fn_in_trait)]
#![allow(clippy::module_inception)]
#![deny(missing_docs)]

#[macro_use]
extern crate ptsl_protos;

mod utils;

pub mod path;
pub mod property;
pub mod session;
