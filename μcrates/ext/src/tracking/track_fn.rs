use euclidean::Region;

pub trait TrackFn<I>: super::Tracker {
    
    fn track(&self, &I, Region, &I) -> Result<Region, Self::Err>;
}