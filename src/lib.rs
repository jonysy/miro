//! Motion analysis
#![feature(non_ascii_idents)]

extern crate lychee_core as core;
extern crate lychee_euclidean as euclidean;
extern crate lychee_ext as ext;
extern crate lychee_linear as linear;
extern crate lychee_image as image;
extern crate extensions;

pub use lucas_kanade::PyrLk;

mod lucas_kanade;