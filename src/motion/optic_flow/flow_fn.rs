use super::{CorrespondingPoints, Points};

pub trait FlowFn<I>: super::OpticFlow {
    
    /// Computes the optical flow.
    fn flow(&self, &I, &Points, &I) -> Result<CorrespondingPoints, Self::Err>;
}