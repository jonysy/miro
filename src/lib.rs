#![feature(field_init_shorthand, inclusive_range_syntax, non_ascii_idents)]

extern crate caribou_core as core;
extern crate caribou_euclidean as euclidean;
extern crate caribou_ext as ext;
extern crate caribou_image as image;
extern crate extensions;

pub use self::median_flow::MedianFlow;
mod median_flow;