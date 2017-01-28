use std::error::Error;

/// A trait implemented by all classifiers.
pub trait Classifier {
	
    type Err: Error;
}