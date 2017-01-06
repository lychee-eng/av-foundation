pub use self::port::AvCaptureInputPort;
mod port;

use objc_foundation::NSObject;

/// `AVCaptureInput` is an abstract base-class describing an input data source to 
/// an `AVCaptureSession` object.
pub struct AvCaptureInput {
	pub(super) obj: *mut NSObject,
	/// The capture input’s ports.
	ports: Vec<Box<AvCaptureInputPort>>,
}

impl Drop for AvCaptureInput {

	fn drop(&mut self) {
		let _: () = unsafe {
			msg_send![self.obj, release]
		};
	}
}