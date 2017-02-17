use euclidean::Region2D as Region;
use std::error::Error;

/// Unsupervised classification
pub trait Unsupervised<I> {
    /// The type returned in the event of an error.
    type Err: Error;

    fn learn(&self, im: &I, region: Option<Region>) -> Result<(), Self::Err>;
}

pub trait UnsupervisedMut<I> {
    /// The type returned in the event of an error.
    type Err: Error;
    
    fn learn_mut(&mut self, im: &I, region: Option<Region>) -> Result<(), Self::Err>;
}