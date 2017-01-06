extern crate av_foundation as av;
extern crate dispatch;

use av::{AvCaptureDevice, AvCaptureDeviceInput, AvCaptureInput, AvCaptureSession, AvMediaType};
use av::AvCaptureVideoDataOutput;
use av::{
	AvCaptureVideoDataOutputSampleBufferDelegate,
	WrappedAvCaptureVideoDataOutputSampleBufferDelegate};

use dispatch::ffi::dispatch_queue_create;
use std::ffi::CString;
use std::ptr;

fn main() {
	let cm = CameraM::new();

	pub struct CameraM {
		delegate: WrappedAvCaptureVideoDataOutputSampleBufferDelegate,
	}

	impl AvCaptureVideoDataOutputSampleBufferDelegate for CameraM {

		fn init(delegate: WrappedAvCaptureVideoDataOutputSampleBufferDelegate) -> CameraM {
			// A session is used to control the flow of the data from the input to the 
			// output device.
			let mut session = AvCaptureSession::new();

			let device_input = 
				// Define the input of the capture device.
				AvCaptureDeviceInput::init__device(
					// Define the capture device you want to use (a camera or a microphone).
					AvCaptureDevice::default__withMediaType(
						// FaceTime HD Camera (Built-in)
						AvMediaType::Video
					)
				);

			// Start the session configuration
			session.beginConfiguration();

			if session.canAddInput(&device_input) {

				// Add the capture device to the session.
				session.addInput(device_input);
			}

			let mut data_output = AvCaptureVideoDataOutput::new();
			//data_output.video_settings = ..
			//dataOutput.always_discards_late_video_frames = true

			if session.canAddOutput(&data_output) {

				session.addOutput(data_output);
			}

			session.commitConfiguration();

			let queue = unsafe {

				let label = "example.av.videoQueue";

				// warning: It is your responsibility to make sure that the underlying memory is not 
				// freed too early. This happens because the pointer returned by `as_ptr` does not 
				// carry any lifetime information and the string is deallocated immediately after 
				// the `CString::new("Hello").unwrap().as_ptr()` expression is evaluated. To fix the 
				// problem, bind the string to a local variable:
				let cstring = CString::new(label).unwrap();
				let ptr = cstring.as_ptr();

				dispatch_queue_create(ptr, ptr::null())
			};

			CameraM {
				delegate: delegate,
			}
		}
	}
}