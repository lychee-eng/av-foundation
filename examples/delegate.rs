use av::AvCaptureVideoDataOutputSampleBufferDelegate;
use av::{AvCaptureDevice, AvCaptureDeviceInput, AvCaptureVideoDataOutput, AvCaptureSession, AvMediaType};
use dispatch::ffi::dispatch_queue_create;
use objc::runtime::Class;
use objc_foundation::NSObject;
use std::ffi::CString;
use std::ptr;

pub struct Buf {

	// input: AvCaptureDeviceInput,
	// output: AvCaptureVideoDataOutput,
	pub session: AvCaptureSession,
	//session_queue: ..,

	pub super_: AvCaptureVideoDataOutputSampleBufferDelegate,
}

impl Buf {

	pub fn init() -> Buf {
		let super_ = AvCaptureVideoDataOutputSampleBufferDelegate::init();

		// A session is used to control the flow of the data from the input to the 
		// output device.
		let mut session = AvCaptureSession::init();

		// Start the session configuration
		session.beginConfiguration();

		// =========== Input
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
		// =========== Input



		// =========== Output
		let mut data_output = AvCaptureVideoDataOutput::init();

		let queue = unsafe {

			let label = "example.basic.videoQueue";

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

			unsafe {
				let key = ::av::ffi::kCVPixelBufferPixelFormatTypeKey as *mut _;
				let value: *mut NSObject = 
					msg_send![Class::get("NSNumber").unwrap(), numberWithInt:1111970369];

				let dictionary: *mut NSObject = 
					msg_send![
						Class::get("NSDictionary").unwrap(), 
						dictionaryWithObject:value forKey:key
					];

				data_output.set__videoSettings(dictionary);

				// let _: () = msg_send![value, release];
				// let _: () = msg_send![dictionary, release];
			}

			session.addOutput(data_output);
		}

		// =========== Output

		session.commitConfiguration();

		Buf {
			session: session,
			super_: super_,
		}
	}
}