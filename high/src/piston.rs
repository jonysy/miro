use current::Current;
use lychee::image::RgbaImage;
use piston_window::{self, Event, PistonWindow, Texture};

/// # Example
///
/// ```rust
/// use mirage::piston;
///
/// while piston::open() {
///
/// 	// ..
///	}
/// ```
pub fn open() -> bool {

	let window = unsafe { &mut *Current::<PistonWindow>::new() };
	let event = unsafe { &mut *Current::<Option<Event>>::new() };

	let next = window.next();
	let open = next.is_some();
	*event = next;
	
	return open;
}

/// Returns the keyboard key for press event.
///
/// # Example
///
/// ```rust
/// use mirage::piston;
///
/// while piston::open() {
///
///		if piston::pressed_key() == Some(114) {
///			println!("`r` pressed!");
///			break
///		}
///	}
/// ```
pub fn pressed_key() -> Option<u64> {
	use piston_window::{Button, PressEvent};

	let opt_event = unsafe { &*Current::<Option<Event>>::new() };

	if let &Some(ref event) = opt_event {
		if let Some(Button::Keyboard(key)) = event.press_args() {
			return Some(key as u64);
		}
	}

	None
}

/// Returns `true` when time to render.
pub fn render() -> bool {

	unsafe {
		
		Current::<Option<Event>>::new().is_some()
	}
}

pub fn draw_image(image: &RgbaImage) -> Result<(), String> {
	let window = unsafe { &mut *Current::<PistonWindow>::new() };
	let opt_event = unsafe { &*Current::<Option<Event>>::new() };
	let texture = unsafe { &mut *Current::<Texture<_>>::new() };

	match opt_event {
		&Some(ref event) => {
			texture.update(&mut window.encoder, image).unwrap();

			window.draw_2d(event, |context, graphics_2d| {
				//clear([0.5, 1.0, 0.5, 1.0], graphics_2d);
				piston_window::image(texture, context.transform, graphics_2d);
			});

			Ok(())
		},

		_ => Err(String::from("No event!")),
	}
}