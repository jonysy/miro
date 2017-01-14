use av_foundation::{AvCaptureVideoDataOutputSampleBufferDelegate, AvCaptureSession};
use av_foundation::{AvCaptureDevice, AvCaptureDeviceInput, AvCaptureVideoDataOutput, AvMediaType};
use dispatch::ffi::dispatch_queue_create;
use lychee::image::{ImageBuffer, RgbaImage};
use std::ffi::CString;
use std::{mem, ptr};

pub struct Capture {
	super_: AvCaptureVideoDataOutputSampleBufferDelegate,
	session: AvCaptureSession,
	conn: bool,

	// input: AvCaptureDeviceInput,
	// output: AvCaptureVideoDataOutput,
	// session_queue: ..,
}

impl Capture {
	#[allow(dead_code)]
	pub(super::super) fn init() -> Capture {
		info!("initializing capture delegate");

		let super_ = AvCaptureVideoDataOutputSampleBufferDelegate::init();

		// A session is used to control the flow of the data from the input to the output device.
		let mut session = AvCaptureSession::init();

		// Start the session configuration
		session.beginConfiguration();

		let device_input = 
			// Define the input of the capture device.
			AvCaptureDeviceInput::init__device(
				// Define the capture device you want to use (a camera or a microphone).
				AvCaptureDevice::default__withMediaType(
					// FaceTime HD Camera (Built-in)
					AvMediaType::Video
				)
			);

		if session.canAddInput(&device_input) {

			// Add the capture device to the session.
			session.addInput(device_input);
		}

		let mut data_output = AvCaptureVideoDataOutput::init();

		let queue = unsafe {

			let label = "lychee.videoQueue";

			// warning: It is your responsibility to make sure that the underlying memory is not 
			// freed too early. This happens because the pointer returned by `as_ptr` does not 
			// carry any lifetime information and the string is deallocated immediately after 
			// the `CString::new("Hello").unwrap().as_ptr()` expression is evaluated. To fix the 
			// problem, bind the string to a local variable:
			let cstring = CString::new(label).unwrap();
			let ptr = cstring.as_ptr();

			dispatch_queue_create(ptr, ptr::null())
		};

		data_output.set__sampleBufferDelegate__queue(&super_, queue);

		if session.canAddOutput(&data_output) {

			data_output.set__videoSettings_default();

			session.addOutput(data_output);
		}

		session.commitConfiguration();

		Capture {
			super_: super_,
			session: session,
			conn: false,
		}
	}

	pub(super) fn conn(&mut self) {
		info!("starting the flow of data from the inputs to the outputs");
		if self.conn {
			return ();
		}

		self.session.startRunning();
		self.conn = true;
	}

	pub(super) fn stop(&mut self) {
		info!("stopping the flow of data from the inputs to the outputs");

		if !self.conn {
			return ();
		}

		self.session.stopRunning();
		self.conn = false;
	}

	pub(super) fn frame(&self) -> RgbaImage {

		let mut array: [u8; 4 * 1280 * 720] = unsafe { mem::uninitialized() };

		self.super_.frame(&mut array);

		const WIDTH: usize = 1280;
		const HEIGHT: usize = 720;
		const NCHANNELS: usize = 4;

		// BGRA

		for x in 0..(WIDTH - 1) {
			for y in 0..(HEIGHT - 1) {

				let i = NCHANNELS * (y * WIDTH + x);
				let range = i..(i + NCHANNELS);

				let slice: &mut [_] = &mut array[range];
				slice.swap(0, 2);
			}
		}

		let vec = array.to_vec();

		ImageBuffer::from_raw(WIDTH as u32, HEIGHT as u32, vec).unwrap()
	}
}