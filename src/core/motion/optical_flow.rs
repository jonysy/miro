use error::Result;
use utility::plane_euclidean::{OptPoint, Point};

/// [Optical flow] or optic flow is the pattern of apparent motion of objects, surfaces, and
/// edges in a visual scene caused by the relative motion between an
/// observer (an eye or a camera) and the scene.
///
/// [Optical flow]: https://en.wikipedia.org/wiki/Optical_flow
pub trait Flow<I> {    
    /// Calculate movement of selected `points` in a pair of images.
    ///
    /// Both `im_i` and `im_j` are expected to have the same dimensions. Every point in `points_i`
    /// is expected to be within the bounds of the images. A point located outside 
    /// the images' dimensions will not result in an error, but will return `None` for the 
    /// optional corresponding point at that index.
    ///
    /// # Arguments
    ///
    /// * `i` - The first image or image pyramid.
    /// * `points_i` - Tracked points from the first image or the bottom of the image pyramid.
    /// * `j` - The second image or image pyramid.
    ///
    /// # Returns
    ///
    /// Returns optional corresponding points/features on image `J`. If a point has not been lost, 
    /// the optional corresponding point is `Some` and contains a value, otherwise `None`.
    #[allow(non_snake_case)]
    fn flow(&self, i: &I, points_i: &[Point<f32>], j: &I) -> Result<Vec<OptPoint<f32>>>;
}

pub trait FlowMut<I> {    
    /// Calculate movement of selected `points` in a pair of images.
    ///
    /// # Arguments
    ///
    /// * `imI` - Previous image
    /// * `points` - Points to track
    /// * `imJ` - Current image
    ///
    /// # Returns
    ///
    /// Returns the corresponding points.
    #[allow(non_snake_case)]
    fn flow_mut(&mut self, i: &I, points_i: &[Point<f32>], j: &I) -> Result<Vec<OptPoint<f32>>>;
}