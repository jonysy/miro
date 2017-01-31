use std::iter::{Iterator, Map};
use std::mem;
use std::ops::Range;

pub type StepBy<T> = Map<::std::iter::StepBy<u32, Range<u32>>, fn(u32) -> T>;

pub struct Window {
	empty: bool,

	factor: f64,

	image_width: u32,
	image_height: u32,

	width: f64,
	height: f64,

	x: f64,
	y: f64,

	xstep: u32,
	ystep: u32,

	horiz: StepBy<f64>,
	vert: StepBy<f64>,
}


/// ```rust,ignore
/// use high::{capture, piston};
/// use miro::detection;
/// 
/// let color = [0.8125, 0.8125, 0.8125, 0.75];
/// let image_size = (640, 360);
/// let min_window_size = (100.0, 100.0);
/// let step = (20, 20);
/// let mut win = detection::window(image_size, min_window_size, step, 1.5);
/// 
/// capture::conn();
/// 
/// 'window: while piston::open() {
/// 
/// 	if piston::render() {
/// 
/// 		piston::clear([1.0; 4])?;
/// 
/// 		let rgba_image = capture::read();
/// 
/// 		piston::draw_image(&rgba_image)?;
/// 
/// 		if let Some(r) = win.next() {
/// 
/// 			piston::draw_border(color, r)?;
/// 		}
/// 	}
/// }
/// 
/// capture::disconn();
/// ``
pub fn window(im: (u32, u32), win: (f64, f64), step: (u32, u32), f: f64) -> Window {

	let (image_width, image_height) = im;
	let (width, height) = win;
	let (xstep, ystep) = step;

	if width as u32 >= image_width || height as u32 >= image_height {

		return unsafe { Window {
			empty: true,
			factor: mem::uninitialized(),
			image_width: mem::uninitialized(),
			image_height: mem::uninitialized(),
			width: mem::uninitialized(),
			height: mem::uninitialized(),
			x: mem::uninitialized(),
			y: mem::uninitialized(),
			xstep: mem::uninitialized(),
			ystep: mem::uninitialized(),
			horiz: mem::uninitialized(),
			vert: mem::uninitialized(),
		}};
	}

	let horiz: StepBy<f64> = (0..image_width).step_by(xstep).map(Into::<f64>::into);
	let mut vert: StepBy<f64> = (0..image_height).step_by(ystep).map(Into::<f64>::into);

	let _ = vert.next().unwrap();

	Window {
		empty: false,
		factor: f,
		image_width: image_width,
		image_height: image_height,

		width: width,
		height: height,

		x: 0.0,
		y: 0.0,

		xstep: xstep,
		ystep: ystep,

		horiz: horiz,
		vert: vert
	}
}

impl Iterator for Window {

	type Item = [f64; 4];

	fn next(&mut self) -> Option<Self::Item> {

		if self.empty {

			return None;
		}

		if let Some(x) = self.horiz.next() {
			let top_r = (x + self.width) as u32;

			if !(top_r >= self.image_width) {
				self.x = x;
				return Some([self.x, self.y, self.width, self.height]);
			}
		}

		if let Some(y) = self.vert.next() {
			self.horiz = (0..self.image_width).step_by(self.xstep).map(Into::<f64>::into);
			let x = self.horiz.next().unwrap();
			self.x = x;

			let bottom = (y + self.height) as u32;

			if !(bottom >=  self.image_height) {
				self.y = y;
				return Some([self.x, self.y, self.width, self.height]);
			}
		}

		self.width *= self.factor;
		self.height *= self.factor;

		if !(self.width as u32 >= self.image_width || self.height as u32 >= self.image_height) {

			self.vert = (0..self.image_height).step_by(self.ystep).map(Into::<f64>::into);
			let y = self.vert.next().unwrap();
			self.y = y;
			
			return Some([self.x, self.y, self.width, self.height]);
		}

		self.empty = true;

		None
	}
}

#[cfg(test)]
mod tests {

	#[test]
	fn test1() {
		let image_width = 640;
		let image_height = 360;

		let factor: f64 = 1.5;

		let (xstep, ystep) = (10, 10);

		let (minW, minH) = (100.0, 100.0);

		let mut slider = super::window((image_width, image_height), (minW, minH), (xstep, ystep), factor);

		let scaled = |n: u32| {
			let f = factor.powi(n as i32);

			(minW * f, minH * f)
		};

		// iterate over a range of scaled regions (smallest scale -> largest)
		for (w, h) in (0..).map(scaled) {

			if w as u32 >= image_width || h as u32 >= image_height {

				// scale is bigger than image, so break.
				break
			}


			// for each row, scan each column
			for y in (0..image_height).step_by(ystep).map(|y| y as f64) {

				let bottom = (y + h) as u32;

				if bottom >=  image_height {

					break
				}

				for x in (0..image_width).step_by(xstep).map(|x| x as f64) {

					let top_r = (x + w) as u32;

					if top_r >= image_width {

						break
					}

					let a = slider.next().unwrap();
					let b = [x, y, w, h];

					assert_eq!(a, b);
				}
			}
		}

		assert!(slider.next().is_none());
		assert!(slider.next().is_none());
		assert!(slider.next().is_none());
		assert!(slider.next().is_none());
		assert!(slider.next().is_none());
		assert!(slider.next().is_none());
	}

	#[test]
	fn test2() {
		let dim = (640, 360);
		let step = (10, 10);
		let min = (100.0, 360.0);
		let factor = 1.5;

		let mut slider = super::window(dim, min, step, factor);

		assert!(slider.next().is_none());
		assert!(slider.next().is_none());
	}

	#[test]
	fn test3() {
		let dim = (640, 360);
		let step = (10, 10);
		let min = (640.0, 100.0);
		let factor = 1.5;

		let mut slider = super::window(dim, min, step, factor);

		assert!(slider.next().is_none());
		assert!(slider.next().is_none());
	}

	#[test]
	fn test4() {
		let dim = (640, 360);
		let step = (540, 260);
		let min = (100.0, 100.0);
		let factor = 1.5;

		let mut slider = super::window(dim, min, step, factor);

		assert!(slider.next().is_some()); // (0, 0) 100 x 100
		assert!(slider.next().is_some()); // (0, 0) (100 x 100) * 1.5 = (0, 0) (150 x 150)
		assert!(slider.next().is_some()); // (0, 0) (150 x 150) * 1.5 = (0, 0) (225 x 225)
		assert!(slider.next().is_some()); // (0, 0) (225 x 225) * 1.5 = (0, 0) (337.5 x 337.5)
		assert!(slider.next().is_none()); // (0, 0) (337.5 x 337.5) * 1.5 = (0, 0) (506.25 x 506.25) > (w x 360)
	}
}