#![feature(slice_patterns)]

#[macro_use]
extern crate log;

extern crate high;
extern crate miro;

use std::borrow::Cow;

#[no_mangle]
pub fn dyn_func() -> Result<(), Cow<'static, str>> {

	include!(concat!(env!("OUT_DIR"), "/main.rs"))
}