use std::ops::Deref;

/// A pyramid representation of image of type `I`.
pub struct Pyramid<I> { images: Vec<I> }

impl<I> Pyramid<I> {

	/// Constructs a pyramid representation of the provided image.
	pub fn new<F>(image: I, height: usize, function: F) -> Pyramid<I> where F: Fn(&I) -> I {

		let mut images = Vec::with_capacity(height);

		images.push(image);

		for n in 1..height {

			let image = {
				let ref prev = images[n - 1];

				function(prev)
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

#[test]
fn simple1() {
	use super::GrayImage;

	let gray_image = GrayImage::new(2, 2);
	let pyramid = Pyramid::new(gray_image, 2, |_| GrayImage::new(1, 1));
	
	let (w, h) = pyramid[1].dimensions();

	assert!(w == 1, h == 1);
}

#[test]
fn simple2() {
	use super::GrayImage;

	let gray_image = GrayImage::new(1, 1);
	let pyramid = Pyramid::new(gray_image, 2, |prev| prev.clone());
	
	assert_eq!(pyramid.len(), 2);
}