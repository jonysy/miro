use super::{CorrespondingPoints, Points};

pub trait Flow<I>: super::OpticFlow {
    
    /// Calculate movement of selected points in a pair of images.
    fn flow(&self, &I, &Points, &I) -> Result<CorrespondingPoints, Self::Err>;
}