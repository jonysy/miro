use high::piston;
use miro::image::GrayImage;

pub fn update_drag(pressed: &mut bool, rectangle: &mut Option<[f64; 4]>) {

	if Some(1) == piston::pressed_mouse_button() || Some(1) == piston::released_mouse_button() {

		info!("Left mouse button pressed/released");
		if !*pressed { *rectangle = None; }
		*pressed = !*pressed;
	}

	if *pressed {

		if let Some(position) = piston::mouse_cursor_position() {

			match rectangle {
				&mut Some([x, y, ref mut dim..]) => {

					dim[0] = position[0] - x;
					dim[1] = position[1] - y;
				},

				r @ _ => {
					*r = Some([position[0], position[1], 0.0, 0.0]);
				}
			}
		}
	}
}

// fn conv(region: [f64; 4]) -> Vec<[f32; 2]> {

// 	unimplemented!()
// }

pub fn update_images(
	images: &mut [Option<GrayImage>; 2], 
	 image: GrayImage) {

	images.swap(0, 1);
	images[1] = Some(image);
}