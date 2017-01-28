use euclidean::Region;
use super::Classifier;

/// Unsupervised classification
pub trait Unsupervised<I>: Classifier {

	fn learn<R, T>(&self, im: &I, region: R) -> Result<(), Self::Err>
		where R: Into<Option<Region>>,
			  T: Into<usize>;
}

pub trait UnsupervisedMut<I>: Classifier {

	fn learn<R, T>(&mut self, im: &I, region: R) -> Result<(), Self::Err>
		where R: Into<Option<Region>>,
			  T: Into<usize>;
}