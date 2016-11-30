extern crate caribou_euclidean as euclidean;

pub mod motion;
pub mod tracking;
pub use tracking::{Tracker, TrackFn};
pub use motion::{OpticFlow, FlowFn};