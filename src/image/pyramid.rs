/// A pyramid representation of an image of type `I`.
#[derive(Debug)]
pub struct Pyramid<I> {
    images: Vec<I>
}

impl<I> Pyramid<I> {

    /// Constructs a pyramid representation of the provided image.
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