extern crate miro_euclidean;

pub use classification::{Supervised, SupervisedMut, Unsupervised, UnsupervisedMut};
pub use motion::{OpticFlow, OpticFlowMut};
pub use tracking::{Track, TrackMut};

mod classification;
mod motion;
mod tracking;