use super::{CorrespondingPoints, Points};

pub trait FlowFn<I>: super::OpticFlow {
    
    /// Calculate movement of selected points in a pair of images.
    fn flow(&self, &I, &Points, &I) -> Result<CorrespondingPoints, Self::Err>;
}