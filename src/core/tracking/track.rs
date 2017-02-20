use euclidean::Region2D as Region;
use std::error::Error;

pub trait Track<I> {
    /// The type returned in the event of an error.
    type Err: Error;

    fn track(&self, &I, Region, &I) -> Result<Region, Self::Err>;
}

pub trait TrackMut<I> {
    /// The type returned in the event of an error.
    type Err: Error;

    fn track_mut(&mut self, &I, Region, &I) -> Result<Region, Self::Err>;
}