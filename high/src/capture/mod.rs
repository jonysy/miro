pub(super) use self::av::Capture;
mod av;

use current::Current;
use image::RgbaImage;

pub fn conn() {
	let capture = unsafe { &mut *Current::<Capture>::new() };
	capture.conn()
}

pub fn disconn() {
	let capture = unsafe { &mut *Current::<Capture>::new() };
	capture.stop()
}

pub fn read() -> RgbaImage {
	let capture = unsafe { &*Current::<Capture>::new() };
	capture.frame()
}