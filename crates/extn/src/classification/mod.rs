pub use self::classifier::Classifier;
pub use self::supervised::{Supervised, SupervisedMut};
pub use self::unsupervised::{Unsupervised, UnsupervisedMut};

mod classifier;
mod supervised;
mod unsupervised;