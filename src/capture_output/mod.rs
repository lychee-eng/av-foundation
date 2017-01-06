use objc_foundation::NSObject;

/// `AVCaptureOutput` is an abstract base class describing an output destination of 
/// an `AVCaptureSession` object.
pub struct AvCaptureOutput {
	pub(super) obj: *mut NSObject,
}

impl Drop for AvCaptureOutput {

	fn drop(&mut self) {
		let _: () = unsafe {
			msg_send![self.obj, release]
		};
	}
}