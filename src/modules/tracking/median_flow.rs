use core::motion::Flow;
use core::tracking::Track;
use error::{Error, ErrorKind, Result};
use euclidean::{Point2D as Point, Region2D as Region};
use float::FloatGuard;
use piston_image::GrayImage;
use std::cmp;
use utility::statistics;

//use super::super::{Error, ErrorKind, OpticFlow, Result, Track};

// use miro_error::{Error, TrackingCategory};
// use miro_euclidean::{Coordinates, Region};
// use miro_extn::{OpticFlow, Track};
// use miro_image::GrayImage;
// use miro_misc::statistics;


/// [Median Flow][1] tracker: A tracker based on [optical flow][2]
///
/// [1]: http://personal.ee.surrey.ac.uk/Personal/Z.Kalal/Publications/2010_icpr.pdf
/// [2]: https://en.wikipedia.org/wiki/Optical_flow
pub struct MedianFlow<F> { algorithm: F, λ: usize }

impl<F> MedianFlow<F> {
    
    /// Construct a `MedianFlow` from an optical flow algorithm and a density `λ`.
    ///
    /// If `density` is 0, 1 is chosen as the density. 
    pub fn new(algorithm: F, density: usize) -> Self {

        MedianFlow { algorithm, λ: cmp::max(1, density) }
    }
}

impl<F> Default for MedianFlow<F> where F: Default {
    
    fn default() -> Self {
        
        MedianFlow { algorithm: Default::default(), λ: 10 }
    }
}

impl<F> Track<GrayImage> for MedianFlow<F> where F: Flow<GrayImage> {

    /// Median Flow tracker
    ///
    /// # Arguments
    ///
    /// * `imI` - Image at `t`.
    /// * `region` - A bounding box.
    /// * `imJ` - Image at `t + 1`.
    ///
    /// # Returns
    ///
    /// Returns the bounding box at `t + 1`.
    #[allow(non_snake_case)]
    fn track(&self, image_i: &GrayImage, region: Region, image_j: &GrayImage) -> Result<Region> {
        // TODO forward-backwards tracking

        let ρ = self.λ * self.λ;

        let mut points_i = Vec::with_capacity(ρ);

        let (xregion, yregion, wregion, hregion) = region.bounds();

        // Generate a set of points on a `λ x λ` rectangular grid within the `region`.
        //
        // These points are then tracked by Lucas-Kanade tracker which generates a sparse motion 
        // flow between `imI` and `imJ`.
        
        let λ_x = wregion / self.λ as f64;
        let λ_y = hregion / self.λ as f64;
        
        for x in (0..self.λ).map(|x| unsafe {
            FloatGuard::from_unchecked((xregion + (x + 1) as f64 * λ_x) as f32)
        }) {
            for y in (0..self.λ).map(|y| unsafe { 
                FloatGuard::from_unchecked((yregion + (y + 1) as f64 * λ_y) as f32) 
            }) {
            
                points_i.push(Point::from([x, y]));
            }
        }
        
        let points_j = {
            self.algorithm
                .flow(image_i, &points_i, image_j)
                .map_err(|err| Error::new(ErrorKind::Tracking, err))?
        };
        
        // calculate medians
        let (mxδ, myδ, mxC, myC) = {

            let mut δxs = Vec::with_capacity(ρ);
            let mut δys = Vec::with_capacity(ρ);

            let capacity = (0..ρ).fold(0, |sum, n| sum + n);
            let mut xscales = Vec::with_capacity(capacity);
            let mut yscales = Vec::with_capacity(capacity);
            
            for m in 0..ρ {
                if let Some(point_j_m) = points_j[m] {

                    let [point_i_m_x, point_i_m_y]: [FloatGuard<f32>; 2] = points_i[m].into();
                    let [point_j_m_x, point_j_m_y]: [FloatGuard<f32>; 2] = point_j_m.into();

                    let δx = point_j_m_x - point_i_m_x;
                    let δy = point_j_m_y - point_i_m_y;

                    δxs.push(δx);
                    δys.push(δy);

                    for n in (m + 1)..ρ {
                        if let Some(point_j_n) = points_j[n] {
                            // scale
                            let δxI = points_i[n].x() - point_i_m_x;
                            let δyI = points_i[n].y() - point_i_m_y;
                                
                            let δxJ = point_j_n.x() - point_j_m_x;
                            let δyJ = point_j_n.y() - point_j_m_y;

                            if δxI != 0. && δyI != 0. && δxJ != 0. && δyJ != 0. {
                                // ^^ignore zero values

                                xscales.push(δxJ / δxI);
                                yscales.push(δyJ / δyI);
                            }
                        }
                    }
                }
            }

            (
                statistics::median(&mut δxs).into(): f32 as f64,
                statistics::median(&mut δys).into(): f32 as f64,

                statistics::median(&mut xscales).into(): f32 as f64,
                statistics::median(&mut yscales).into(): f32 as f64
            )
        };

        let xoffset = wregion * mxC - wregion;
        let yoffset = hregion * myC - hregion;

        let x = xregion - xoffset / 2.0 + mxδ;
        let y = yregion - yoffset / 2.0 + myδ;
        let width = wregion + xoffset;
        let height = hregion + yoffset;

        Ok([x, y, width, height].into())
    }
}