//! Motion analysis
#![feature(non_ascii_idents)]

extern crate image;
extern crate imageproc;
extern crate lychee_core;
extern crate lychee_euclidean;
extern crate lychee_extn;
extern crate lychee_misc;
extern crate nalgebra;

pub use lucas_kanade::PyramLucasKanade;

mod lucas_kanade;