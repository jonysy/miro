use euclidean::Region;
use super::Classifier;

/// Supervised classification
pub trait Supervised<I>: Classifier {

	fn train<R, T>(&self, im: &I, region: R, label: T) -> Result<(), Self::Err>
		where R: Into<Option<Region>>,
			  T: Into<usize>;
}

pub trait SupervisedMut<I>: Classifier {

	fn train<R, T>(&mut self, im: &I, region: R, label: T) -> Result<(), Self::Err>
		where R: Into<Option<Region>>,
			  T: Into<usize>;
}