use piston_image::GrayImage;
use std::ops::Deref;

pub type GrayPyramid = Pyramid<GrayImage>;

/// A pyramid representation of an image of type `I`.
#[derive(Debug)]
pub struct Pyramid<I> {
    images: Vec<I>
}

impl<I> Pyramid<I> {

    /// Builds a pyramid representation of the provided image.
    ///
    /// The pyramid representation is built in a recursive fashion: compute I¹ from I⁰, then 
    /// compute I² from I¹, and so on.. The bottom of the pyramid is the initial, and the largest,
    /// image.
    ///
    /// # Arguments
    ///
    /// * `im` - The highest resolution image (the raw image or the "zeroᵗʰ" level image).
    /// * `nlayers` - The height of the pyramid.
    ///
    /// # Returns
    ///
    /// Returns the pyramid representation of image `im`
    pub fn new<F>(image: I, height: usize, apply: F) -> Pyramid<I> where F: Fn(&I) -> I {

        let mut images = Vec::with_capacity(height);

        images.push(image);

        for n in 1..height {

            let image = {
                let ref prev = images[n - 1];

                apply(prev)
            };

            images.push(image);
        }

        Pyramid { images: images }
    }
}

impl<I> Deref for Pyramid<I> {

    type Target = Vec<I>;

    fn deref(&self) -> &Vec<I> {
        &self.images
    }
}