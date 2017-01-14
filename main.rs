use high::{capture, piston};

capture::conn();

'window: while piston::open() {
	if piston::render() {
		
		let image = capture::read();

		piston::draw_image(&image)?;
	}

	if let Some(key) = piston::pressed_key() {

		if key <= 255 {

			match key as u8 as char {
				'r' => {
					
					info!("reloading window");
					break 'window; // will reload if the window is still open
				},

				_ => {

				}
			}
		}
	}
}

capture::disconn();