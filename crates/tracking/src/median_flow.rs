use core::error::{Error, TrackingErr};
use euclidean::{Coordinates, Region};
use extn::{FlowFn, Tracker, TrackFn};
use extensions::statistics;
use image::GrayImage;

/// [Median Flow][1] tracker: A tracker based on [optical flow][2]
///
/// [1]: http://personal.ee.surrey.ac.uk/Personal/Z.Kalal/Publications/2010_icpr.pdf
/// [2]: https://en.wikipedia.org/wiki/Optical_flow
pub struct MedianFlow<F> { algorithm: F, λ: usize }

impl<F> MedianFlow<F> {
    
    pub fn new(algorithm: F, density: usize) -> Self {
        MedianFlow { algorithm, λ: density }
    }
}

impl<F> Default for MedianFlow<F> where F: Default {
    
    fn default() -> Self {
        let algorithm = Default::default();
        
        MedianFlow { algorithm, λ: 10 }
    }
}

impl<F> Tracker for MedianFlow<F> {
    type Err = Error;
}

impl<F> TrackFn<GrayImage> for MedianFlow<F>
    where F: FlowFn<GrayImage>, F::Err: 'static
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
        let (originx, originy, width, height) = region.into();
        
        // A set of points is initialized on a rectangular grid within the `region`.
        //
        // These points are then tracked by Lucas-Kanade tracker which generates a sparse motion 
        // flow between imI and imJ.
        
        let res_x = width / self.λ as f64;
        let res_y = height / self.λ as f64;
        
        let ρ = self.λ * self.λ;
        
        let mut pointsI = Vec::with_capacity(ρ);
        
        for x in 1...self.λ { for y in 1...self.λ {
            
            let (x, y) = (x as f64, y as f64);
            
            let pointI = Coordinates {
                x: (originx + x * res_x) as f32,
                y: (originy + y * res_y) as f32
            };
            
            pointsI.push(pointI);
        }}
        
        let pointsJ = {
            self.algorithm
                .flow(imI, &pointsI, imJ)
                .map_err(|err| Error::new(TrackingErr, err))?
        };
        
        let mut δxs = Vec::with_capacity(self.λ);
        let mut δys = Vec::with_capacity(self.λ);
        
        for (pointI, optional_pointJ) in pointsI.iter().zip(pointsJ.iter()) {
            
            if let &Some(pointJ) = optional_pointJ {
                let &Coordinates { x: xI, y: yI } = pointI;
                let Coordinates { x: xJ, y: yJ } = pointJ;

                let δx = xJ - xI;
                let δy = yJ - yI;

                δxs.push(δx);
                δys.push(δy);
            }
        }
        
        let medianX = statistics::median(&mut δxs) as f64;
        let medianY = statistics::median(&mut δys) as f64;
        
        let mut scales = Vec::with_capacity((1..ρ).fold(0, |sum, n| sum + n));
        
        for m in 0..ρ { for n in (m + 1)..ρ {
            if let (Some(pointJ_m), Some(pointJ_n)) = (pointsJ[m], pointsJ[n]) {
                let δxI = pointsI[n].x - pointsI[m].x;
                let δyI = pointsI[n].y - pointsI[m].y;
                    
                let δxJ = pointJ_n.x - pointJ_m.y;
                let δyJ = pointJ_n.y - pointJ_m.y;
                    
                let δI = (δxI * δxI + δyI * δyI).sqrt();
                let δJ = (δxJ * δxJ + δyJ * δyJ).sqrt();
                
                if δI != 0.0 && δJ != 0.0 {
                    scales.push(δJ / δI);
                }
            }
        }}
        
        let medianS = statistics::median(&mut scales) as f64;
        
        let offsetX = width * (medianS - 1.0);
        let offsetY = height * (medianS - 1.0);
        
        let trajIJ = [
            offsetX + originx, offsetY + originy,
            width + (-offsetX * 0.5 + medianX), height + (-offsetY * 0.5 + medianY)
        ].into();
        
        Ok(trajIJ)
    }
}