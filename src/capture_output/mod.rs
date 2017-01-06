use objc_foundation::NSObject;
use super::{AvCaptureConnection, AvMediaType};

/// `AVCaptureOutput` is an abstract base class describing an output destination of 
/// an `AVCaptureSession` object.
pub struct AvCaptureOutput {
	pub(super) obj: *mut NSObject,
}

impl AvCaptureOutput {

	pub fn new(obj: *mut NSObject) -> AvCaptureOutput {
		
		AvCaptureOutput {
			obj: obj,
		}
	}

	pub fn connectionWithMediaType(&self, media_type: AvMediaType) -> AvCaptureConnection {

		let obj = unsafe {
			msg_send![self.obj, connectionWithMediaType:media_type.ns_string()]
		};

		AvCaptureConnection {
			obj: obj,
		}
	}
}

impl Drop for AvCaptureOutput {

	fn drop(&mut self) {
		let _: () = unsafe {
			msg_send![self.obj, release]
		};
	}
}