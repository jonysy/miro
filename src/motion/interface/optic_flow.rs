use euclidean::Coordinates2D as Coordinates;
use std::error::Error;

/// [Optical flow] or optic flow is the pattern of apparent motion of objects, surfaces, and
/// edges in a visual scene caused by the relative motion between an
/// observer (an eye or a camera) and the scene.
///
/// [Optical flow]: https://en.wikipedia.org/wiki/Optical_flow
pub trait OpticFlow<I> {
    /// The type returned in the event of an error.
    type Err: Error;
    
    /// Calculate movement of selected `points` in a pair of images.
    ///
    /// # Arguments
    ///
    /// * `imI` - Previous image
    /// * `points` - Points to track
    /// * `imJ` - Current image
    #[allow(non_snake_case)]
    fn flow(&self, imI: &I, points: &[Coordinates<f32>], imJ: &I) 
        -> Result<Vec<Option<Coordinates<f32>>>, Self::Err>;
}

pub trait OpticFlowMut<I> {
    /// The type returned in the event of an error.
    type Err: Error;
    
    /// Calculate movement of selected `points` in a pair of images.
    ///
    /// # Arguments
    ///
    /// * `imI` - Previous image
    /// * `points` - Points to track
    /// * `imJ` - Current image
    #[allow(non_snake_case)]
    fn flow_mut(&mut self, imI: &I, points: &[Coordinates<f32>], imJ: &I) 
        -> Result<Vec<Option<Coordinates<f32>>>, Self::Err>;
}