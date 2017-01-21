use high::{capture, piston};

const COLOR: [f32; 4] = [1.0, 0.0, 0.0, 0.20];
let mut pressed = false;
let mut rectangle = None::<[f64; 4]>;

capture::conn();

'window: while piston::open() {

	if piston::render() {

		piston::clear([1.0; 4])?;
		let image = capture::read();
		piston::draw_image(&image)?;

		if let Some(r) = rectangle {

			piston::draw_rectangle(COLOR, r)?
		}
	}

	// =========

	if Some('r' as u64) == piston::pressed_key() {

		info!("reloading window");
		break 'window; // will reload if the window is still open
	}

	if Some(1) == piston::pressed_mouse_button() || Some(1) == piston::released_mouse_button(){

		info!("Left mouse button pressed/released");
		if !pressed { rectangle = None; }
		pressed = !pressed;
	}

	update_drag_state(pressed, &mut rectangle);
}

capture::disconn();

fn update_drag(pressed: bool, rectangle: &mut Option<[f64; 4]>) {
	if pressed {

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