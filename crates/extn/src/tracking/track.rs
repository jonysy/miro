use euclidean::Region;

pub trait Track<I>: super::Tracker {
    
    fn track(&self, &I, Region, &I) -> Result<Region, Self::Err>;
}