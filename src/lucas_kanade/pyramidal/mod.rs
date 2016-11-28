use core::Error;
use euclidean::Dimensions;
use ext::OpticFlow;

/// [Pyramidal Implementation of the Lucas Kanade][1] Feature Tracker (Jean-Yves Bouguet).
///
/// Given a point in one image, the Lucas-Kanade tracking algorithm will attempt to locate the same 
/// point in the following image.
///
/// ## Example
///
/// ```
/// let pointsJ = LkPyramid::with_defaults().flow(imI, pointsI, imJ);
/// ```
///
/// [1]: (http://robots.stanford.edu/cs223b04/algo_tracking.pdf)
#[allow(non_snake_case)]
pub struct LkPyramid {
    /// The height (# of layers) of the image pyramid.
    ///
    /// ## Notes
    ///
    /// * Practical values are 2, 3, or 4. For typical image sizes, it makes no sense to go above 
    ///   a level 4. For example, for an image I of size 640×480, the images I1, I2, I3 and I4 
    ///   are of respective sizes 320×240, 160×120, 80×60 and 40×30. Going beyond level 4 does not 
    ///   make much sense in most cases. The central motivation behind pyramidal representation is 
    ///   to be able to handle large pixel motions (larger than the integration window 
    ///   sizes ωₓ₁ and ωₓ₂). Therefore the pyramid height should also be picked appropriately 
    ///   according to the maximum expected optical flow in the image.
    /// * See RFC [Range types for integers](https://github.com/rust-lang/rfcs/issues/671)
    nlayers: usize,
    
    /// Integration window size.
    ///
    /// Because of the *aperture problem*, it is essential to define the notion of similarity in 
    /// a 2D neighborhood sense. We define the image velocity as being the vector that minimizes 
    /// the residual function ε (see equation 1). The similarity function is measured on a image 
    /// neighborhood of size (2ωₓ₁ + 1) × (2ωₓ₂ + 1). This neighborhood will be also called 
    /// integration window. Typical values for ωₓ₁ and ωₓ₂ are 2,3,4,5,6,7 pixels.
    win: Dimensions<u16>,
}

impl OpticFlow for LkPyramid {
    type Err = Error;
}