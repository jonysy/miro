use core::motion::Flow;
use error::Result;
use euclidean::Size2D;
use float::FloatGuard;
use image::GrayPyramid;
use nalgebra::{Inv, Mat2, Vec2};
use num;
use piston_image::{GrayImage, Luma};
use piston_imageproc::filter;
use std::cmp;
use utility::plane_euclidean::{OptPoint, Point};

/// Pyramidal Implementation of the Lucas Kanade Feature Tracker
///
/// Given a point in one image, the Lucas-Kanade tracking algorithm will attempt to locate the same 
/// point in the following image.
///
/// ## Computing the optical flow
///
/// Processes an image `imI` containing points `u ∈ U` and returns the corresponding
/// points `v ∈ V` on image `imJ`, where `imI(u)` and `imJ(v)` are similar grayscaled values.
///
/// `v = u + d` where `d` is the optical flow at `u`.
///
/// ## "Lost" features/points
///
/// There are two cases that should give rise to  a “lost” feature.
///
/// 1) The point falls outside of the image. 
/// 2) The image patch around the tracked point varies too much between image `I` and 
///    image `J` (the point disappears due to occlusion).
///
/// ## References/Resources
///
/// * [Jean-Yves Bouguet. Pyramidal Impl. of the Lucas-Kanade Feature Tracker. Intel Corp., 2001][1]
/// * [Optical Flow](http://docs.opencv.org/trunk/d7/d8b/tutorial_py_lucas_kanade.html)
/// * [Optical Flow Measurement using Lucas kanade Method][2]
/// 
/// [1]: (http://robots.stanford.edu/cs223b04/algo_tracking.pdf)
/// [2]: https://pdfs.semanticscholar.org/6841/d3368d4dfb52548cd0ed5fef29199d14c014.pdf
#[derive(Debug)]
pub struct PyramLk {
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
    height: usize,
    
    /// Integration window size.
    ///
    /// Because of the *aperture problem*, it is essential to define the notion of similarity in 
    /// a 2D neighborhood sense. We define the image velocity as being the vector that minimizes 
    /// the residual function ε (see equation 1). The similarity function is measured on a image 
    /// neighborhood of size (2ωₓ₁ + 1) × (2ωₓ₂ + 1). This neighborhood will be also called 
    /// integration window. Typical values for ωₓ₁ and ωₓ₂ are 2,3,4,5,6,7 pixels.
    win: Size2D<u16>,
    
    k: usize,
}

impl PyramLk {

    pub fn new(height: usize, win: u16, k: usize) -> PyramLk {

        PyramLk {
            height,
            win: [win, win].into(),
            k,
        }
    }
}

impl Default for PyramLk {
    
    /// Constructs a new `PyramLk` using the default `height` and `win` parameters.
    fn default() -> Self {
        PyramLk {
            height: 4,
            win: [7, 7].into(),
            k: 4,
        }
    }
}

impl Flow<GrayImage> for PyramLk {

    /// Computes the optical flow.
    fn flow(&self, image_i: &GrayImage, points_i: &[Point<f32>], image_j: &GrayImage) 
        -> Result<Vec<OptPoint<f32>>> {

        let pyramid_i = build_pyramid(image_i, self.height);
            
        let pyramid_j = build_pyramid(image_j, self.height);
        
        self.flow(&pyramid_i, points_i, &pyramid_j)
    }
}

impl Flow<GrayPyramid> for PyramLk {

    /// Computes the optical flow using pre-computed image pyramids.
    fn flow(&self, pyramid_i: &GrayPyramid, points_i: &[Point<f32>], pyramid_j: &GrayPyramid)
        -> Result<Vec<OptPoint<f32>>> 
    {

        let [window_width, window_height] = self.win.dimensions();
        let (w_j, h_j) = pyramid_j[0].dimensions();
        
        let mut corresponding_points = Vec::with_capacity(points_i.len());
            
        'points: for point_i in points_i {
            let [x_i, y_i] = point_i.coordinates();
            
            if x_i < 0.0 || y_i < 0.0 || x_i >= (w_j as f32) || y_i >= (h_j as f32) {
                // TODO return error instead?
                debug!("Point {} is outside image bounds {}x{}", point_i, w_j, h_j);

                corresponding_points.push(None);

                continue;
            }

            let zero: FloatGuard<f32> = num::zero();
            
            // The optical flow vector at level `L` or the flow at the bottom of the pyramid.
            let mut ground_flow = Vec2 {x: zero, y: zero};
            
            // Initialization of the pyramidal guess
            let mut pyramidal_guess = Vec2 {x: zero, y: zero};
            
            // for L=L_m down to 0 with a step of -1
            for (level, (i, j)) in pyramid_i.iter().zip(pyramid_j.iter()).enumerate().rev() {
                // Spatial gradient matrix
                let mut h_i = Mat2::new(zero, zero, zero, zero);
                
                // Location of point `scaled_u` on the image `currI`: `uᶫ = u/2ᶫ`
                // Observe that in particular, `u⁰ = u/2⁰ = u`
                let scaled_x_i = x_i / (2.0f32).powi(level as i32);
                let scaled_y_i = y_i / (2.0f32).powi(level as i32);
                
                // farthest left x
                let min_x_i = scaled_x_i - window_width as f32;
                
                // farthest bottom y
                let min_y_i = scaled_y_i - window_height as f32;
                
                // farthest right x
                let max_x_i = scaled_x_i + window_width as f32;
                
                // farthest top y
                let max_y_i = scaled_y_i + window_height as f32;
                
                // Derivatives
                let capacity = {max_x_i - min_x_i}.ceil() * {max_y_i - min_y_i}.ceil();

                let mut derivatives_i = Vec::with_capacity(num::cast(capacity).unwrap());
                
                // Area of the neighborhood (integration window)
                for x in min_x_i..max_x_i {
                    for y in min_y_i..max_y_i {
                        
                        let x: f32 = x.into();
                        let y: f32 = y.into();

                        // Derivative of `i` wrt `x`
                        let fx = (
                            subpx(i, x + 1., y) - subpx(i, x - 1., y)
                        ) / 2.0;
                        
                        // Derivative of `i` wrt `y`
                        let fy = (
                            subpx(i, x, y + 1.) - subpx(i, x, y - 1.)
                        ) / 2.0;
                        
                        derivatives_i.push((fx, fy));
                        
                        h_i.m11 += fx * fx;
                        h_i.m12 += fx * fy;
                        h_i.m21 += fx * fy;
                        h_i.m22 += fy * fy;
                    }
                }
                
                // The inverse spatial gradient matrix
                let h_i_inv = {
                    match h_i.inv() {
                        Some(inv) => inv,
                        None => {
                            debug!("An error occurred while inverting a matrix");
                            // TODO return an error for the current point instead?
                            corresponding_points.push(None);
                            continue 'points;
                        }
                    }
                };
                
                // Initialization of iterative L-K
                let mut flowlk = Vec2::new(zero, zero);
                
                for _ in 0..self.k {
                    // Image mismatch vector
                    let mut mismatch = Vec2::new(zero, zero);
                    
                    let mut it = derivatives_i.iter();
                    
                    for x_i_win in min_x_i..max_x_i {
                        for y_i_win in min_y_i..max_y_i {
                        
                            let x_j_win: f32 = (x_i_win + pyramidal_guess[0] + flowlk[0]).into();
                            let y_j_win: f32 = (y_i_win + pyramidal_guess[1] + flowlk[1]).into();
                            
                            let x_i_win: f32 = x_i_win.into();
                            let y_i_win: f32 = y_i_win.into();

                            // Image difference
                            let delta = subpx(i, x_i_win, y_i_win) - subpx(j, x_j_win, y_j_win);
                        
                            let &(fx, fy) = it.next().unwrap();
                            mismatch[0] += delta * fx;
                            mismatch[1] += delta * fy;
                        }
                    }
                    
                    // Optical flow (Lucas-Kanade)
                    let ηk = h_i_inv * mismatch;
                    
                    // Guess for next iteration
                    flowlk = flowlk + ηk;
                }
                
                if level == 0 {
                    ground_flow = flowlk;
                } else {
                    let two = unsafe { FloatGuard::from_unchecked(2.0) };
                    // Guess for next iteration
                    pyramidal_guess[0] = (pyramidal_guess[0] + flowlk[0]) * two;
                    pyramidal_guess[1] = (pyramidal_guess[1] + flowlk[1]) * two;
                }
            }
            
            // The final optical flow vector
            let resultant_flow = pyramidal_guess + ground_flow;
            
            // Location of point on image `J` (translation).
            let x_j = x_i + resultant_flow.x;
            let y_j = y_i + resultant_flow.y;
            
            if cmp::min(x_j, y_j) < 0.0 || x_j >= (w_j as f32) || y_j >= (h_j as f32) {
                
                corresponding_points.push(None);
                continue;
            }
            
            // Location of point on `J` (v = u + final optical flow vector)
            let corresponding_point = Some([x_j, y_j].into());
            
            corresponding_points.push(corresponding_point);
        }
            
        Ok(corresponding_points)
    }
}

fn build_pyramid(im: &GrayImage, height: usize) -> GrayPyramid {
    
    /// Anti-aliasing filter kernel is used for pyramid construction (used for image anti-aliasing 
    /// before image sub-sampling).
    const KERNEL: [f64; 9] = [
        // [1/16 1/4 3/8 1/4 1/16] × [1/16 1/4 3/8 1/4 1/16]T
        1.0/16.0,   1.0/8.0,    1.0/16.0,
        1.0/8.0,    1.0/4.0,    1.0/8.0,
        1.0/16.0,   1.0/8.0,    1.0/16.0
    ];

    GrayPyramid::new(im.clone(), height, |prev| {

        //  TODO : `filter3x3` creates a new image
        let filtered_prev = filter::filter3x3(prev, &KERNEL);
        
        let (prev_w, prev_h) = prev.dimensions();
        
        let width = (prev_w + 1) / 2;
        let height = (prev_h + 1) / 2;

        let mut buffer = GrayImage::new(width, height);
        
        for x in 0..width {
            for y in 0..height {
                let prev_x = 2.0 * x as f32;
                let prev_y = 2.0 * y as f32;

                let sum1 = safepx(&filtered_prev, prev_x, prev_y) / 4.0;

                let sum2 = {
                    let mut sum = 0.0;
                    sum += safepx(&filtered_prev, prev_x - 1.0, prev_y);
                    sum += safepx(&filtered_prev, prev_x + 1.0, prev_y);
                    sum += safepx(&filtered_prev, prev_x, prev_y - 1.0);
                    sum += safepx(&filtered_prev, prev_x, prev_y + 1.0);
                    sum /= 8.0;
                    sum
                };

                let sum3 = {
                    let mut sum = 0.0;
                    sum += safepx(&filtered_prev, prev_x - 1.0, prev_y - 1.0);
                    sum += safepx(&filtered_prev, prev_x + 1.0, prev_y - 1.0);
                    sum += safepx(&filtered_prev, prev_x - 1.0, prev_y + 1.0);
                    sum += safepx(&filtered_prev, prev_x + 1.0, prev_y + 1.0);
                    sum /= 16.0;
                    sum
                };

                let sum = sum1 + sum2 + sum3;

                let data = [sum as u8];
                let luma = Luma { data: data };

                buffer.put_pixel(x, y, luma);
            }
        }

        buffer
    })
}

fn safepx(im: &GrayImage, x: f32, y: f32) -> f32 {
    let (w, h) = im.dimensions();
    let (w, h) = (w as f32, h as f32);

    let x = x.min(w - 1.0).max(0.0) as u32;
    let y = y.min(h - 1.0).max(0.0) as u32;

    im[(x, y)].data[0] as f32
}

/// Subpixel computation
pub fn subpx(im: &GrayImage, x: f32, y: f32) -> f32 {
    let x_0 = x.floor();
    let y_0 = y.floor();
        
    let α_x = x - x_0;
    let α_y = y - y_0;
        
    (1.0 - α_x) 
        * (1.0 - α_y) 
        * safepx(im, x_0, y_0)
        + α_x * (1.0 - α_y) 
        * safepx(im, x_0 + 1.0, y_0)
        + (1.0 - α_x) 
        * α_y 
        * safepx(im, x_0, y_0 + 1.0)
        + α_x 
        * α_y 
        * safepx(im, x_0 + 1.0, y_0 + 1.0)
}

#[cfg(test)]
mod tests {

    #[test]
    fn safepx_computation() {

        unimplemented!()
    }

    #[test]
    fn subpx_computation() {

        unimplemented!()
    }
}