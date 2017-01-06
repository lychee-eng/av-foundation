use objc_foundation::NSObject;
use AvMediaType;

/// An `AVCaptureDevice` object represents a physical capture device and the properties 
/// associated with that device. You use a capture device to configure the properties of the 
/// underlying hardware. A capture device also provides input data (such as audio or video) to 
/// an `AVCaptureSession` object.
pub struct AvCaptureDevice { pub(super) obj: *mut NSObject }

impl AvCaptureDevice {

	/// Returns the default device used to capture data of a given media type.
	pub fn default__withMediaType(media_type: AvMediaType) -> AvCaptureDevice {
		use ffi::AVCaptureDevice;

		let obj = unsafe {

			msg_send![&AVCaptureDevice, defaultDeviceWithMediaType:media_type.as_ptr()]
		};

		AvCaptureDevice {
			obj,
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