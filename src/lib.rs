//! A modern, open, high-performance computer vision library

#![deny(/*missing_docs,*/
        missing_debug_implementations, missing_copy_implementations,
        trivial_casts, trivial_numeric_casts,
        unsafe_code,
        /*unstable_features,*/
        unused_import_braces, unused_qualifications)]
        
#![feature(field_init_shorthand, non_ascii_idents)]

extern crate euclidean;
extern crate image as piston_image;
extern crate imageproc as piston_imageproc;

pub mod image;
pub use self::classification::{Supervised, SupervisedMut, Unsupervised, UnsupervisedMut};
pub use self::error::{Error, ErrorKind, Result};
pub use self::motion::{OpticFlow, OpticFlowMut};
pub use self::tracking::{Track, TrackMut};

mod classification;
mod error;
mod motion;
mod tracking;