use objc_foundation::NSObject;
use super::{ffi, AvCaptureOutput};

pub struct AvCaptureVideoDataOutput {
	pub(super) sup: AvCaptureOutput,
}

impl AvCaptureVideoDataOutput {

	pub fn new() -> AvCaptureVideoDataOutput {

		let obj = unsafe {

			msg_send![msg_send![&ffi::AVCaptureVideoDataOutput, alloc]: *mut NSObject, init]
		};

		AvCaptureVideoDataOutput {
			sup: AvCaptureOutput::new(obj),
		}
	}
}