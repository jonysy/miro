use super::{CorrespondingPoints, Points};

pub trait Flow<I>: super::OpticFlow {
    
    /// Calculate movement of selected `points` in a pair of images.
    ///
    /// # Arguments
    ///
    /// * `imI` - Previous image
    /// * `points` - Points to track
    /// * `imJ` - Current image
    #[allow(non_snake_case)]
    fn flow(&self, imI: &I, points: &Points, imJ: &I) -> Result<CorrespondingPoints, Self::Err>;
}