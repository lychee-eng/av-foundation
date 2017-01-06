use objc::runtime::Class;
use objc_foundation::NSObject;
use super::ffi;

pub struct WrappedAvCaptureVideoDataOutputSampleBufferDelegate(pub(super) Class);

pub trait AvCaptureVideoDataOutputSampleBufferDelegate: Sized {

	fn new() -> Self {
		let avcvdosb_delegate = unsafe {

			msg_send![
				msg_send![
					&ffi::CaptureVideoDataOutputSampleBufferDelegate, 
					alloc
				]: *mut NSObject, 

				init
			]
		};

		Self::init(WrappedAvCaptureVideoDataOutputSampleBufferDelegate(avcvdosb_delegate))
	}

	fn init(WrappedAvCaptureVideoDataOutputSampleBufferDelegate) -> Self;
}