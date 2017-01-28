//! Motion analysis
#![feature(non_ascii_idents)]

extern crate miro_core;
extern crate miro_euclidean;
extern crate miro_extn;
extern crate miro_image;
extern crate miro_misc;
extern crate nalgebra;

pub use lucas_kanade::PyramLucasKanade;

mod lucas_kanade;