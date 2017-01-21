use high::{capture, piston};
use miro::misc::parcmp;

// use miro::extn::Flow;
// use miro::motion::PyrLk;

let mut pressed = false;
let mut rectangle = [0.0, 0.0, 50.0, 50.0];
let (width, height) = piston::window_size();

capture::conn();

'window: while piston::open() {

	if piston::render() {

		piston::clear([1.0; 4])?;
		let image = capture::read();
		piston::draw_image(&image)?;
		piston::draw_rectangle([1.0, 0.0, 0.0, 1.0], rectangle)?;
	}

	// =========

	if Some('r' as u64) == piston::pressed_key() {

		info!("reloading window");
		break 'window; // will reload if the window is still open
	}

	if Some(1) == piston::pressed_mouse_button() || Some(1) == piston::released_mouse_button(){

		info!("Left mouse button pressed/released");
		pressed = !pressed;
	}

	if pressed {

		if let Some(position) = piston::mouse_cursor_position() {
			rectangle[0] = position[0];
			rectangle[1] = position[1];

			//println!("{:?}", position);
		}
	}
}

capture::disconn();