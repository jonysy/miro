use current::Current;
use image::RgbaImage;
use piston_window::{self, Event, PistonWindow, Texture};
use std::borrow::Cow;

const NO_EVENT: &'static str = "No event!";

/// # Example
///
/// ```rust
/// use high::piston;
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

pub fn window_size() -> (f64, f64) {
	use piston_window::Window;
	
	let size = unsafe { &mut *Current::<PistonWindow>::new() }.size();

	(size.width as f64, size.height as f64)
}

/// Returns the keyboard key for press event.
///
/// # Example
///
/// ```rust
/// use high::piston;
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

pub fn pressed_mouse_button() -> Option<u64> {
	use piston_window::{Button, PressEvent};

	let opt_event = unsafe { &*Current::<Option<Event>>::new() };

	if let &Some(ref event) = opt_event {
		if let Some(Button::Mouse(mouse_button)) = event.press_args() {
			return Some(mouse_button as u64);
		}
	}

	None
}

pub fn released_mouse_button() -> Option<u64> {
	use piston_window::{Button, ReleaseEvent};

	let opt_event = unsafe { &*Current::<Option<Event>>::new() };

	if let &Some(ref event) = opt_event {
		if let Some(Button::Mouse(mouse_button)) = event.release_args() {
			return Some(mouse_button as u64);
		}
	}

	None
}

pub fn mouse_cursor_position() -> Option<[f64; 2]> {
	use piston_window::MouseCursorEvent;

	let opt_event = unsafe { &*Current::<Option<Event>>::new() };

	match opt_event {
		&Some(ref event) => {

			event.mouse_cursor_args().map(|position| position.into())
		},

		_ => None
	}
}

/// Returns `true` when time to render.
pub fn render() -> bool {

	unsafe {
		
		Current::<Option<Event>>::new().is_some()
	}
}

fn helper2<F>(f: F) -> Result<(), Cow<'static, str>> where F: FnOnce(&mut PistonWindow, &Event) {

	let window = unsafe { &mut *Current::<PistonWindow>::new() };
	let opt_event = unsafe { &*Current::<Option<Event>>::new() };

	match opt_event {
		&Some(ref event) => {
			f(window, event);

			Ok(())
		},

		_ => Err(Cow::from(NO_EVENT)),
	}
}

/// Clears the screen.
pub fn clear(color: [f32; 4]) -> Result<(), Cow<'static, str>> {

	helper2(|window, event| {

		let _ = window.draw_2d(event, |_, g| piston_window::clear(color, g));
	})
}

pub fn draw_image(image: &RgbaImage) -> Result<(), Cow<'static, str>> {

	helper2(|window, event| {

		let texture = unsafe { &mut *Current::<Texture<_>>::new() };

		texture.update(&mut window.encoder, image).unwrap();

		window.draw_2d(event, |c, g| {

			piston_window::image(texture, c.transform, g);
		});
	})
}

/// Draws rectangle.
pub fn draw_rectangle<R>(color: [f32; 4], region: R) -> Result<(), Cow<'static, str>> 
	where R: Into<[f64; 4]> {

	helper2(|window, event| {
		window.draw_2d(event, |c, g| {

			piston_window::rectangle(color, region, c.transform, g)
		});
	})
}