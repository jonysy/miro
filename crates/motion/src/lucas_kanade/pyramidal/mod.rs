mod util;

use miro_error::{Error, MotionCategory};
use miro_euclidean::{Coordinates, Dimensions};
use miro_extn::OpticFlow;
use miro_image::{GrayImage, GrayPyramid};
use miro_image::math::{Mat2, Vec2};
use miro_misc::parcmp;
use miro_misc::range::RangeInc;
use nalgebra::Inv;

/// Pyramidal Implementation of the Lucas Kanade Feature Tracker
///
/// Given a point in one image, the Lucas-Kanade tracking algorithm will attempt to locate the same 
/// point in the following image.
///
/// ## References
///
/// * [Jean-Yves Bouguet. Pyramidal Implementation of the Lucas-Kanade Feature Tracker. Intel Corp., 2001][1]
/// 
/// [1]: (http://robots.stanford.edu/cs223b04/algo_tracking.pdf)
#[allow(non_snake_case)]
pub struct PyramLucasKanade {
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
    
    k: usize,
}

impl PyramLucasKanade {

    pub fn new(nlayers: usize, win: u16, k: usize) -> PyramLucasKanade {

        PyramLucasKanade {
            nlayers: nlayers,
            win: [win, win].into(),
            k: k,
        }
    }
}

impl Default for PyramLucasKanade {
    
    /// Constructs a new `PyramLucasKanade` using the default `nlayers` and `win` parameters.
    fn default() -> Self {
        PyramLucasKanade {
            nlayers: 4,
            win: [7, 7].into(),
            k: 4,
        }
    }
}

impl OpticFlow<GrayImage> for PyramLucasKanade {
    type Err = Error;

    /// Computes the optical flow.
    ///
    /// Processes an image `imI` containing points `u ∈ U` and returns the corresponding
    /// points `v ∈ V` on image `imJ`, where `imI(u)` and `imJ(v)` are similar grayscaled values.
    ///
    /// `v = u + d` where `d` is the optical flow at `u`.
    ///
    /// ## Arguments
    ///
    /// * `imI` - The first grayscale image.
    /// * `pointsI` - Tracked points from the first grayscale image.
    /// * `imJ` - The second grayscale image.
    ///
    /// ## Returns
    ///
    /// Returns optional corresponding points/features on image `J`. If a point has not been lost, 
    /// the optional corresponding point is `Some` and contains a value, otherwise `None`.
    ///
    /// ## "Lost" features/points
    ///
    /// There are two cases that should give rise to  a “lost” feature.
    ///
    /// 1) The point falls outside of the image. 
    /// 2) The image patch around the tracked point varies too much between image `I` and 
    ///    image `J` (the point disappears due to occlusion).
    ///
    /// ## Helpful Resources
    ///
    /// * [Optical Flow](http://docs.opencv.org/trunk/d7/d8b/tutorial_py_lucas_kanade.html)
    /// * [Optical Flow Measurement using Lucas kanade Method][1]
    ///
    /// [1]: https://pdfs.semanticscholar.org/6841/d3368d4dfb52548cd0ed5fef29199d14c014.pdf
    #[allow(non_snake_case)]
    fn flow(&self, imI: &GrayImage, pointsI: &[Coordinates<f32>], imJ: &GrayImage)
        -> Result<Vec<Option<Coordinates<f32>>>, Self::Err>
    {
        let pyrI = util::pyramid(imI, self.nlayers);
            
        let pyrJ = util::pyramid(imJ, self.nlayers);
        
        self.flow(&pyrI, pointsI, &pyrJ)
    }
}


impl OpticFlow<GrayPyramid> for PyramLucasKanade {
    type Err = Error;

    #[allow(non_snake_case)]
    fn flow(&self, pyrI: &GrayPyramid, pointsI: &[Coordinates<f32>], pyrJ: &GrayPyramid)
        -> Result<Vec<Option<Coordinates<f32>>>, Self::Err>
    {
        let (wJ, hJ) = pyrJ[0].dimensions();
        
        let mut corresponding_points = Vec::with_capacity(pointsI.len());
            
        for pointI in pointsI {
            let &Coordinates { x: xI, y: yI } = pointI;
            
            if xI < 0.0 || yI < 0.0 || xI >= (wJ as f32) || yI >= (hJ as f32) {
                return Err(
                    Error::new(
                        MotionCategory::Generic,
                        format!("Point {} is outside image bounds {}x{}", pointI, wJ, hJ)
                    )
                );
            }
            
            // The optical flow vector at level `L` or the flow at the bottom of the pyramid.
            let mut flow_bop = Vec2 {x: 0., y: 0.};
            
            // Initialization of the pyramidal guess
            let mut pyr_guess = Vec2 {x: 0., y: 0.};
            
            // for L=L_m down to 0 with a step of -1
            for (level, (pyrI, pyrJ)) in pyrI.iter().zip(pyrJ.iter()).enumerate().rev() {
                // Spatial gradient matrix
                let mut HpyrI = Mat2::new(0., 0., 0., 0.);
                
                // Location of point `scaled_u` on the image `currI`: `uᶫ = u/2ᶫ`
                // Observe that in particular, `u⁰ = u/2⁰ = u`
                let scaled_xI = xI / (2.0f32).powi(level as i32);
                let scaled_yI = yI / (2.0f32).powi(level as i32);
                let Dimensions { width: window_width, height: window_height } = self.win;
                
                // farthest left x
                let min_xI = scaled_xI - window_width as f32;
                
                // farthest bottom y
                let min_yI = scaled_yI - window_height as f32;
                
                // farthest right x
                let max_xI = scaled_xI + window_width as f32;
                
                // farthest top y
                let max_yI = scaled_yI + window_height as f32;
                
                // Derivatives
                let capacity = {max_xI - min_xI}.ceil() * {max_yI - min_yI}.ceil();
                let mut derivativesI = Vec::with_capacity(capacity as usize);
                
                // Area of the neighborhood (integration window)
                for x in RangeInc(min_xI, max_xI) { for y in RangeInc(min_yI, max_yI) {
                    
                    // Derivative of `pyrI` wrt `x`
                    let fx = (util::subpx(pyrI, x + 1., y) - util::subpx(pyrI, x - 1., y)) / 2.0;
                    
                    // Derivative of `pyrI` wrt `y`
                    let fy = (util::subpx(pyrI, x, y + 1.) - util::subpx(pyrI, x, y - 1.)) / 2.0;
                    
                    derivativesI.push((fx, fy));
                    
                    HpyrI.m11 += fx * fx;
                    HpyrI.m12 += fx * fy;
                    HpyrI.m21 += fx * fy;
                    HpyrI.m22 += fy * fy;
                }}
                
                // The inverse spatial gradient matrix
                let HpyrI_inv = HpyrI.inv() // TODO return `None` instead?
                    .ok_or(Error::new(MotionCategory::Generic, "An error occurred while inverting a matrix"))?;
                
                // Initialization of iterative L-K
                let mut flowlk = Vec2::new(0.0, 0.0);
                
                for _ in 0..self.k {
                    // Image mismatch vector
                    let mut mismatch = Vec2::new(0.0, 0.0);
                    
                    let mut it = derivativesI.iter();
                    
                    for xIwin in RangeInc(min_xI, max_xI) { for yIwin in RangeInc(min_yI, max_yI) {
                        
                        let xJwin = xIwin + pyr_guess[0] + flowlk[0];
                        let yJwin = yIwin + pyr_guess[1] + flowlk[1];
                        
                        // Image difference
                        let delta = util::subpx(pyrI, xIwin, yIwin) - util::subpx(pyrJ, xJwin, yJwin);
                        
                        let &(fx, fy) = it.next().unwrap();
                        mismatch[0] += delta * fx;
                        mismatch[1] += delta * fy;
                    }}
                    
                    // Optical flow (Lucas-Kanade)
                    let ηk = HpyrI_inv * mismatch;
                    
                    // Guess for next iteration
                    flowlk = flowlk + ηk;
                }
                
                if level == 0 {
                    flow_bop = flowlk;
                } else {
                    // Guess for next iteration
                    pyr_guess = (pyr_guess + flowlk) * 2.0
                }
            }
            
            // The final optical flow vector
            let flowf = pyr_guess + flow_bop;
            
            // Location of point on image `J` (translation).
            let xJ = xI + flowf.x;
            let yJ = yI + flowf.y;
            
            if parcmp::min(xJ, yJ) < 0.0 || xJ >= (wJ as f32) || yJ >= (hJ as f32) {
                
                corresponding_points.push(None);
                continue;
            }
            
            // Location of point on `J` (v = u + final optical flow vector)
            let corresponding_point = Some([xJ, yJ].into());
            
            corresponding_points.push(corresponding_point);
        }
            
        Ok(corresponding_points)
    }
}