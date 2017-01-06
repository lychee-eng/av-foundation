use objc_foundation::NSObject;
use super::{ffi, AvMediaType};

/// An `AVCaptureDevice` object represents a physical capture device and the properties 
/// associated with that device. You use a capture device to configure the properties of the 
/// underlying hardware. A capture device also provides input data (such as audio or video) to 
/// an `AVCaptureSession` object.
pub struct AvCaptureDevice { obj: *mut NSObject }

impl AvCaptureDevice {

	/// Returns the default device used to capture data of a given media type.
	pub fn default__withMediaType(media_type: AvMediaType) -> AvCaptureDevice {

		let obj = unsafe {

			msg_send![&ffi::AVCaptureDevice, defaultDeviceWithMediaType:media_type.ns_string()]
		};

		AvCaptureDevice {
			obj: obj,
		}
	}
}

impl Drop for AvCaptureDevice {

	fn drop(&mut self) {
		let _: () = unsafe {
			msg_send![self.obj, release]
		};
	}
}