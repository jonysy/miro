extern crate miro_euclidean as euclidean;

pub use classification::{Classifier, Supervised, SupervisedMut, Unsupervised, UnsupervisedMut};
pub use motion::{CorrespondingPoints, Flow, OpticFlow, Points};
pub use tracking::{Tracker, Track};

mod classification;
mod motion;
mod tracking;