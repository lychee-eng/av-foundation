use objc_foundation::NSObject;

pub struct AvCaptureVideoDataOutputSampleBufferDelegate {
	pub(super) obj: *mut NSObject,
}

impl AvCaptureVideoDataOutputSampleBufferDelegate {

	pub fn init() -> AvCaptureVideoDataOutputSampleBufferDelegate {
		use ffi::CaptureVideoDataOutputSampleBufferDelegate;

		let obj = unsafe {

			let obj: *mut NSObject =  msg_send![&CaptureVideoDataOutputSampleBufferDelegate, alloc];
			msg_send![obj, init]
		};

		AvCaptureVideoDataOutputSampleBufferDelegate {
			obj,
		}
	}
}

impl Drop for AvCaptureVideoDataOutputSampleBufferDelegate {

	fn drop(&mut self) {
		let _: () = unsafe {
			msg_send![self.obj, release]
		};
	}
}