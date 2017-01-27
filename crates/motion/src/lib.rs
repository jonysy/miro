//! Motion analysis
#![feature(non_ascii_idents)]

extern crate image;
extern crate imageproc;
extern crate miro_core;
extern crate miro_euclidean;
extern crate miro_extn;
extern crate miro_misc;
extern crate nalgebra;

pub use lucas_kanade::PyramLucasKanade;

mod lucas_kanade;