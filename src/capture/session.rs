use objc_foundation::NSObject;
use super::{AvCaptureDeviceInput, AvCaptureVideoDataOutput};

/// You use an `AVCaptureSession` object to coordinate the flow of data from AV input 
/// devices to outputs.
pub struct AvCaptureSession {
	obj: *mut NSObject,
	pub inputs: Vec<AvCaptureDeviceInput>,
	pub outputs: Vec<AvCaptureVideoDataOutput>,
}

impl AvCaptureSession {

	pub fn init() -> AvCaptureSession {
		use ffi::AVCaptureSession;

		let obj = unsafe {
			
			let obj: *mut NSObject = msg_send![&AVCaptureSession, alloc];
			msg_send![obj, init]
		};

		AvCaptureSession {
			obj,
			inputs: vec![],
			outputs: vec![],
		}
	}

	// TODO: pub fn canAddInput<I: AvCaptureInput>(input: I) -> bool
	/// Returns a Boolean value that indicates whether a given input can be added to the session.
	pub fn canAddInput(&self, input: &AvCaptureDeviceInput) -> bool {

		unsafe {

			msg_send![self.obj, canAddInput:input.obj]
		}
	}

	/// Adds a given input to the session.
	pub fn addInput(&mut self, input: AvCaptureDeviceInput) {

		let _: () = unsafe { msg_send![self.obj, addInput:input.obj] };

		self.inputs.push(input);
	}

	// TODO: pub fn canAddOutput<O: AvCaptureOutput>(output: O) -> bool
	/// Returns a Boolean value that indicates whether a given output can be added to the session.
	pub fn canAddOutput(&self, output: &AvCaptureVideoDataOutput) -> bool {
		unsafe {
			msg_send![self.obj, canAddOutput:output.obj]
		}
	}

	/// Adds a given output to the session.
	///
	/// # Arguments
	///
	/// * `output` - An output to add to the session.
	pub fn addOutput(&mut self, output: AvCaptureVideoDataOutput) {

		let _: () = unsafe { msg_send![self.obj, addOutput:output.obj] };

		self.outputs.push(output);
	}

	/// Indicates the start of a set of configuration changes to be made atomically.
	pub fn beginConfiguration(&self) {

		unsafe {

			msg_send![self.obj, beginConfiguration]
		}
	}

	/// Commits a set of configuration changes.
	pub fn commitConfiguration(&self) {

		unsafe {

			msg_send![self.obj, commitConfiguration]
		}
	}

	/// Tells the receiver to start running.
	///
	/// This method is used to start the flow of data from the inputs to the outputs connected to 
	/// the `AVCaptureSession` instance that is the receiver. This method is synchronous and blocks 
	/// until the receiver has either completely started running or failed to start running. If 
	/// an error occurs during this process and the receiver fails to start running, you receive 
	/// an `AVCaptureSessionRuntimeError`.
	pub fn startRunning(&self) {

		unsafe {

			msg_send![self.obj, startRunning]
		}
	}

	/// Tells the receiver to stop running.
	///
	/// This method is used to stop the flow of data from the inputs to the outputs connected to 
	/// the `AVCaptureSession` instance that is the receiver. This method is synchronous and 
	/// blocks until the receiver has completely stopped running.
	pub fn stopRunning(&self) {

		unsafe {

			msg_send![self.obj, stopRunning]
		}
	}
}

impl Drop for AvCaptureSession {

	fn drop(&mut self) {
		let _: () = unsafe {
			msg_send![self.obj, release]
		};
	}
}