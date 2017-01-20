#[macro_use]
extern crate log;

extern crate high;
extern crate miro;

#[no_mangle]
pub fn dyn_func() -> Result<(), String> {

	include!(concat!(env!("OUT_DIR"), "/main.rs"))
}