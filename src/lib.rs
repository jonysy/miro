#![feature(field_init_shorthand)]

pub use error::{Error, Kind as ErrorKind};
pub use result::Result;

mod error;
mod result;