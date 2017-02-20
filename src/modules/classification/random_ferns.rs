use miro_error::Never;
use miro_euclidean::Region;
use miro_extn::SupervisedMut;
use miro_image::{GrayImage, GrayPyramid, IntegralImage};
use rand::Rng;

/// A binary test
pub struct Binary;

/// Represents a single fern.
pub struct Fern<T> {
    /// The tests used as classifier features are picked completely as random.
    de: Vec<T>,
}

// pub type Posterior<F> where F: Float = F;
// pub type Distributions<F> where F: Float = Vec<Posterior<F>>;

/// Random Ferns
///
/// Random Ferns rely on randomization to achieve good performance, which is particularly effective
/// for applications such as real-time 3D object detection 
/// and _Simultaneous Localization and Mapping_ (SLAM) that require scale and perspective invariance,
/// involve a very large number of classes, but can tolerate significant error rates [1][paper-1].
///
/// * [Fast Keypoint Recognition using Random Ferns][paper-1]
/// * [Random Forests and Ferns][slide-1]
/// * [Ferns: Planar Object Detection Demo][ferns-demo]
///
///
/// [paper-1]: http://cvlabwww.epfl.ch/~lepetit/papers/ozuysal_pami10.pdf
/// [slide-1]: http://vision.cse.psu.edu/seminars/talks/2009/random_tff/ForestsAndFernsTalk.pdf
/// [ferns-demo]: http://cvlab.epfl.ch/software/ferns/index.php
pub struct RandomFerns<T = Binary> { /*TODO where P: PatchEvaluator */
    /// A vector of ferns used to classify patches.
    ///
    /// Each fern consists of a small set of binary tests and returns the probability that a patches
    /// belongs to any one of the classes that have been learned during training.
    ferns: Vec<Fern<T>>,
}

impl<T> RandomFerns<T> {

    pub fn new<R>(rng: &mut R, /* regression: bool */) -> RandomFerns where R: Rng {

        unimplemented!()
    }
}

impl SupervisedMut<IntegralImage> for RandomFerns {
    /// `RandomFerns` should never return an error.
    type Err = Never;

    fn train_mut<R, I>(&mut self, integral_image: &IntegralImage, region: R, label: I) -> Result<(), Self::Err>
        where R: Into<Option<Region>>,
              I: Into<usize>
    {

        unimplemented!()
    }
}

impl<T> SupervisedMut<GrayImage> for RandomFerns<T> {
    /// `RandomFerns` should never return an error.
    type Err = Never;

    fn train_mut<R, I>(&mut self, _: &GrayImage, _: R, _: I) -> Result<(), Self::Err>
        where R: Into<Option<Region>>,
              I: Into<usize>
    {


        // from the paper:
        //
        // > Training starts by selecting a subset of the keypoints detected on the 
        // > image. This is done by deforming the images many times, applying the keypoint 
        // > detector, and keeping track of the number of times the same keypoint is detected. The 
        // > keypoints that are found most often are assumed to be the most stable and 
        // > retained. These stable keypoints are assigned a unique class number.

        unimplemented!()
    }
}

impl<T> SupervisedMut<GrayPyramid> for RandomFerns<T> {
    /// `RandomFerns` should never return an error.
    type Err = Never;

    fn train_mut<R, I>(&mut self, _: &GrayPyramid, _: R, _: I) -> Result<(), Self::Err>
        where R: Into<Option<Region>>,
              I: Into<usize>
    {

        unimplemented!()
    }
}