use error::Result;
use euclidean::Region2D as Region;

pub trait Track<I> {

    fn track(&self, &I, Region, &I) -> Result<Region>;
}

pub trait TrackMut<I> {

    fn track_mut(&mut self, &I, Region, &I) -> Result<Region>;
}