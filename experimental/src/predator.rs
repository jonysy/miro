use miro::{classification, detection, euclidean, image, motion, tracking};
use miro::error::Result;

/// Tracking-Learning-Detection (TLD)
///
/// TLD, also known as "Predator" algorithm, was developed by Zdenek Kalal.
///
/// For more information, visit Zdenek's [homepage] and/or read Zdenek's [paper].
///
/// [homepage]: http://info.ee.surrey.ac.uk/Personal/Z.Kalal/tld.html
/// [paper]: http://vision.stanford.edu/teaching/cs231b_spring1415/papers/Kalal-PAMI.pdf
pub struct Predator {
    /// Pyramidal Implementation of the Lucas-Kanade Feature Tracker
    pub motion: motion::PyramLucasKanade,
    /// A tracker based on
    pub tracker: tracking::MedianFlow<motion::PyramLucasKanade>,

    /// Confidence of the previous frame's trajectory patch
    pub confidence: f64,
}

impl Predator {

    pub fn process(&mut self, frameI: image::GrayImage, frameJ: image::GrayImage) 
        -> Result<euclidean::Region>
    {

        unimplemented!()
    }
}