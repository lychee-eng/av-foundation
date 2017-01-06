use dispatch::ffi::dispatch_queue_t;
use objc_foundation::NSObject;

use AvCaptureVideoDataOutputSampleBufferDelegate as Delegate;
use super::AvCaptureOutput;
use super::super::AvCaptureConnection;
use super::super::super::AvMediaType;

pub struct AvCaptureVideoDataOutput {
	pub(super::super) obj: *mut NSObject,
}

impl AvCaptureVideoDataOutput {
	pub fn init() -> AvCaptureVideoDataOutput {
		use ffi::AVCaptureVideoDataOutput;

		let obj = unsafe {

			let obj: *mut NSObject = msg_send![&AVCaptureVideoDataOutput, alloc];
			msg_send![obj, init]
		};

		AvCaptureVideoDataOutput {
			obj,
		}
	}

	/// Sets the sample buffer delegate and the queue on which callbacks should be invoked.
	pub fn set__sampleBufferDelegate__queue(&mut self, del: &Delegate, queue: dispatch_queue_t) {

		unsafe {

			msg_send![self.obj, setSampleBufferDelegate:del.obj queue:queue]
		}
	}
}

impl AvCaptureOutput for AvCaptureVideoDataOutput {

	fn connections(&self) -> &Vec<AvCaptureConnection> {

		unimplemented!()
	}

	fn connection__withMediaType(&self, media_type: AvMediaType) -> &AvCaptureConnection {

		unimplemented!()
	}
}

impl Drop for AvCaptureVideoDataOutput {

	fn drop(&mut self) {
		let _: () = unsafe {
			msg_send![self.obj, release]
		};
	}
}