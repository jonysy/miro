#![feature(field_init_shorthand, inclusive_range_syntax, non_ascii_idents)]

extern crate lychee_core as core;
extern crate lychee_euclidean as euclidean;
extern crate lychee_extn as extn;
extern crate lychee_image as image;
extern crate extensions;

pub use self::median_flow::MedianFlow;
mod median_flow;