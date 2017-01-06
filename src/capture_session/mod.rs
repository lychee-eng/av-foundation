use objc_foundation::NSObject;
use super::{ffi, AvCaptureDeviceInput, AvCaptureInput, AvCaptureVideoDataOutput};

/// You use an `AVCaptureSession` object to coordinate the flow of data from AV input 
/// devices to outputs.
pub struct AvCaptureSession {
	obj: *mut NSObject,
	//inputs: Vec<AvCaptureInput>,
}

impl AvCaptureSession {

	pub fn new() -> AvCaptureSession {

		let obj: *mut NSObject = unsafe {
			msg_send![msg_send![&ffi::AVCaptureSession, alloc]: *mut NSObject, init]
		};

		AvCaptureSession {
			obj: obj,
		}
	}

	// TODO: pub fn canAddInput<I: AvCaptureInput>(input: I) -> bool
	/// Returns a Boolean value that indicates whether a given input can be added to the session.
	pub fn canAddInput(&self, input: &AvCaptureDeviceInput) -> bool {
		unsafe {
			msg_send![self.obj, canAddInput:input.sup.obj]
		}
	}

	/// Adds a given input to the session.
	pub fn addInput(&mut self, input: AvCaptureDeviceInput) {

		unimplemented!()
	}

	// TODO: pub fn canAddOutput<O: AvCaptureOutput>(output: O) -> bool
	/// Returns a Boolean value that indicates whether a given output can be added to the session.
	pub fn canAddOutput(&self, output: &AvCaptureVideoDataOutput) -> bool {
		unsafe {
			msg_send![self.obj, canAddOutput:output.sup.obj]
		}
	}

	/// Adds a given output to the session.
	///
	/// # Arguments
	///
	/// * `output` - An output to add to the session.
	pub fn addOutput(&mut self, output: AvCaptureVideoDataOutput) {

		unimplemented!()
	}

	/// Indicates the start of a set of configuration changes to be made atomically.
	pub fn beginConfiguration(&mut self) {

		unimplemented!()
	}

	/// Commits a set of configuration changes.
	pub fn commitConfiguration(&mut self) {

		unimplemented!()
	}

}

impl Drop for AvCaptureSession {

	fn drop(&mut self) {
		let _: () = unsafe {
			msg_send![self.obj, release]
		};
	}
}