use capture::Capture;
use current::{Current, CurrentGuard};
use lychee::image::ImageBuffer;
use piston_window::{Event, EventLoop, PistonWindow, Texture, TextureSettings, Window, WindowSettings};
use gfx_device_gl::Resources;
use std::{mem, thread};
use std::time::Duration;

pub fn currentize<F>(func: F) where F: Fn() {

	let mut window = {

		let settings = {

			let title = "Mirage - Interactive";
			let size = [1280, 720];

			WindowSettings::new(title, size)
						   .exit_on_esc(true)
						   //.samples(0)
	           			   //.vsync(true)
	           			   .resizable(false)
	           			   .decorated(false)
	    };

	    let closure = |error| panic!("{}", error);

		settings.build::<PistonWindow>()
				.unwrap_or_else(closure)
				.ups(60)
				.max_fps(60)
	};

	let mut texture: Texture<Resources> = {

		let buf = ImageBuffer::new(1280, 720);
		let settings = TextureSettings::new();
		let ref mut factory = window.factory;

		Texture::from_image(factory, &buf, &settings).expect("failed to create texture")
	};

	let mut capture = Capture::init();
	let mut event = None;

	let capture_guard: CurrentGuard<Capture> = CurrentGuard::new(&mut capture);
	let window_guard: CurrentGuard<PistonWindow> = CurrentGuard::new(&mut window);
	let texture_guard = CurrentGuard::new(&mut texture);
	let event_guard: CurrentGuard<Option<Event>> = CurrentGuard::new(&mut event);

	loop {

		func();

		if unsafe { &*Current::<PistonWindow>::new() }.should_close() {

			info!("closing window");

			mem::drop(event_guard);
			mem::drop(texture_guard);
			mem::drop(window_guard);
			mem::drop(capture_guard);

			break
		}

		let (secs, nanos) = (2, 0);

		thread::sleep(
			Duration::new(secs, nanos)
		);

		info!("reloading main.rs");
	}
}