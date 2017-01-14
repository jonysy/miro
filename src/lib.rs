extern crate high;

#[no_mangle]
pub fn dyn_func() -> Result<(), String> {

	include!(concat!(env!("OUT_DIR"), "/main.rs"))
}