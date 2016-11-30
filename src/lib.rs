//! Motion analysis
#![feature(non_ascii_idents)]

extern crate caribou_core as core;
extern crate caribou_euclidean as euclidean;
extern crate caribou_ext as ext;
extern crate caribou_linear as linear;
extern crate caribou_image as image;
extern crate extensions;

pub use lucas_kanade::PyrLk;

mod lucas_kanade;