use miro_error::{Error, TrackingCategory};
use miro_euclidean::{Coordinates, Region};
use miro_extn::{Flow, Tracker, Track};
use miro_image::GrayImage;
use miro_misc::statistics;

/// [Median Flow][1] tracker: A tracker based on [optical flow][2]
///
/// [1]: http://personal.ee.surrey.ac.uk/Personal/Z.Kalal/Publications/2010_icpr.pdf
/// [2]: https://en.wikipedia.org/wiki/Optical_flow
pub struct MedianFlow<F> { algorithm: F, λ: usize }

impl<F> MedianFlow<F> {
    
    pub fn new(algorithm: F, density: usize) -> Self {
        MedianFlow { algorithm: algorithm, λ: density }
    }
}

impl<F> Default for MedianFlow<F> where F: Default {
    
    fn default() -> Self {
        let algorithm = Default::default();
        
        MedianFlow { algorithm: algorithm, λ: 10 }
    }
}

impl<F> Tracker for MedianFlow<F> {
    type Err = Error;
}

impl<F> Track<GrayImage> for MedianFlow<F>
    where F: Flow<GrayImage>, F::Err: 'static + Send + Sync
{
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
    fn track(&self, imI: &GrayImage, region: Region, imJ: &GrayImage) -> Result<Region, Self::Err> {
        // TODO forward-backwards tracking

        let ρ = self.λ * self.λ;

        let mut pointsI = Vec::with_capacity(ρ);

        let (xregion, yregion, wregion, hregion) = region.into();

        // Generate a set of points on a `λ x λ` rectangular grid within the `region`.
        //
        // These points are then tracked by Lucas-Kanade tracker which generates a sparse motion 
        // flow between `imI` and `imJ`.
        
        let λ_x = wregion / self.λ as f64;
        let λ_y = hregion / self.λ as f64;
        
        for x in (0..self.λ).map(|x| (xregion + (x + 1) as f64 * λ_x) as f32) {
            for y in (0..self.λ).map(|y| (yregion + (y + 1) as f64 * λ_y) as f32) {
            
                pointsI.push(Coordinates { x: x, y: y });
            }
        }
        
        let pointsJ = {
            self.algorithm
                .flow(imI, &pointsI, imJ)
                .map_err(|err| Error::new(TrackingCategory::Generic, err))?
        };
        
        // calculate medians
        let (mxδ, myδ, mxC, myC) = {

            let mut δxs = Vec::with_capacity(ρ);
            let mut δys = Vec::with_capacity(ρ);

            let capacity = (0..ρ).fold(0, |sum, n| sum + n);
            let mut xscales = Vec::with_capacity(capacity);
            let mut yscales = Vec::with_capacity(capacity);
            
            for m in 0..ρ {
                if let Some(pointJ_m) = pointsJ[m] {

                    let Coordinates { x: pointI_m_x, y: pointI_m_y } = pointsI[m];
                    let Coordinates { x: pointJ_m_x, y: pointJ_m_y } = pointJ_m;

                    let δx = pointJ_m_x - pointI_m_x;
                    let δy = pointJ_m_y - pointI_m_y;

                    δxs.push(δx);
                    δys.push(δy);

                    for n in (m + 1)..ρ {
                        if let Some(pointJ_n) = pointsJ[n] {
                            // scale
                            let δxI = pointsI[n].x - pointI_m_x;
                            let δyI = pointsI[n].y - pointI_m_y;
                                
                            let δxJ = pointJ_n.x - pointJ_m_x;
                            let δyJ = pointJ_n.y - pointJ_m_y;

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
                statistics::median(&mut δxs) as f64,
                statistics::median(&mut δys) as f64,

                statistics::median(&mut xscales) as f64,
                statistics::median(&mut yscales) as f64
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