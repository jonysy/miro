//! A modern, open, high-performance computer vision library

#![deny(/*missing_docs,*/
        missing_debug_implementations, /*missing_copy_implementations,*/
        trivial_casts, trivial_numeric_casts,
        /*unsafe_code,*/
        /*unstable_features,*/
        unused_import_braces, unused_qualifications)]
        
#![feature(field_init_shorthand, non_ascii_idents, slice_patterns)]

#[macro_use]
extern crate log;

#[cfg(feature = "pilot")]
extern crate float;

extern crate euclidean;
extern crate image as piston_image;
extern crate imageproc as piston_imageproc;
extern crate nalgebra;
extern crate num;
extern crate rand;

pub mod core;
pub mod image;
pub mod modules;
pub mod utility;

pub use self::error::{Error, ErrorKind, Result};

mod error;