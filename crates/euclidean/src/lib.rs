#![feature(slice_patterns)]

extern crate num;

pub mod plane;
pub use plane::{Coordinates, Dimensions, Region, RegionIter};

pub type Rectangle<T = f64> = Region<T>;