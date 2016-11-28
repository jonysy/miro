#![feature(field_init_shorthand)]

pub mod error;
pub use error::{Error, Kind as ErrorKind};
pub use result::Result;

mod result;