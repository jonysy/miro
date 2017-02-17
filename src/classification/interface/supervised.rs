use euclidean::Region2D as Region;
use std::error::Error;

/// Supervised classification
pub trait Supervised<I> {
    /// The type returned in the event of an error.
    type Err: Error;

    fn train<L>(&self, im: &I, region: Option<Region>, label: L) -> Result<(), Self::Err>
        where L: Into<usize>;
}

pub trait SupervisedMut<I> {
    /// The type returned in the event of an error.
    type Err: Error;

    fn train_mut<L>(&mut self, im: &I, region: Option<Region>, label: L) -> Result<(), Self::Err>
        where L: Into<usize>;
}