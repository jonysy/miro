#![feature(pub_restricted)]

#[macro_use]
extern crate log;
#[macro_use]
extern crate objc;

extern crate av_foundation;
extern crate current;
extern crate dispatch;
extern crate gfx_device_gl;
extern crate image;
extern crate objc_foundation;
extern crate piston_window;

pub mod capture;
pub mod piston;

pub use self::currentize::currentize;
mod currentize;

const WIDTH: usize = 640;
const HEIGHT: usize = 360;
const NCH: usize = 4;