use dispatch::ffi::dispatch_queue_t;
use objc_foundation::NSObject;
use std::ops::Deref;
use super::{ffi, AvCaptureOutput, WrappedAvCaptureVideoDataOutputSampleBufferDelegate};

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

	pub fn set__sampleBufferDelegate__queue(
		&mut self, 
		del: &WrappedAvCaptureVideoDataOutputSampleBufferDelegate, 
		queue: dispatch_queue_t) {

		unsafe {

			msg_send![self.sup.obj, setSampleBufferDelegate:&del.0 queue:queue];
		}
	}
}

impl Deref for AvCaptureVideoDataOutput {

	type Target = AvCaptureOutput;

	fn deref(&self) -> &AvCaptureOutput {

		&self.sup
	}
}