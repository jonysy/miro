use miro_euclidean::Region;
use std::error::Error;

/// Supervised classification
pub trait Supervised<I> {
	/// The type returned in the event of an error.
	type Err: Error;

	fn train<R, T>(&self, im: &I, region: R, label: T) -> Result<(), Self::Err>
		where R: Into<Option<Region>>,
			  T: Into<usize>;
}

pub trait SupervisedMut<I> {
	/// The type returned in the event of an error.
	type Err: Error;

	fn train_mut<R, T>(&mut self, im: &I, region: R, label: T) -> Result<(), Self::Err>
		where R: Into<Option<Region>>,
			  T: Into<usize>;
}

/// Unsupervised classification
pub trait Unsupervised<I> {
	/// The type returned in the event of an error.
	type Err: Error;

	fn learn<R, T>(&self, im: &I, region: R) -> Result<(), Self::Err>
		where R: Into<Option<Region>>,
			  T: Into<usize>;
}

pub trait UnsupervisedMut<I> {
	/// The type returned in the event of an error.
	type Err: Error;
	
	fn learn_mut<R, T>(&mut self, im: &I, region: R) -> Result<(), Self::Err>
		where R: Into<Option<Region>>,
			  T: Into<usize>;
}