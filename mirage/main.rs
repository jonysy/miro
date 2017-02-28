use high::{capture, piston};
use image::ConvertBuffer;

use miro::core::tracking::Track;
use miro::modules::motion::PyramLk;
use miro::modules::tracking::MedianFlow;

mod util {

	include!(concat!("../main-", "util.rs"));
}

let mf = MedianFlow::<PyramLk>::default();

let color = [0.8125, 0.8125, 0.8125, 0.75];
let mut pressed = false;
let mut rectangle = None::<[f64; 4]>;

let mut images = [None, None];

capture::conn();

'window: while piston::open() {

	if piston::render() {

		piston::clear([1.0; 4])?;

		let rgba_image = capture::read();

		util::update_images(&mut images, rgba_image.convert());

		piston::draw_image(&rgba_image)?;

		match &mut rectangle {
			&mut Some(ref mut r) => {

				piston::draw_border(color, *r)?;

				if !pressed {

					if let [Some(ref imI), Some(ref imJ)] = images {

						if let Ok(re) = mf.track(imI, (*r).into(), imJ) {

							*r = re.into();
						}
					}
				}
			},

			_ => {

			}
		}
	}

	util::update_drag(&mut pressed, &mut rectangle);

	if Some('r' as u64) == piston::pressed_key() {

		info!("reloading window");
		break 'window;
	}
}

capture::disconn();