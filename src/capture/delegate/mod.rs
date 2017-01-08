use objc::runtime::{Class, Object};

pub struct AvCaptureVideoDataOutputSampleBufferDelegate {
	pub(super) obj: *mut Object,
}

impl AvCaptureVideoDataOutputSampleBufferDelegate {

	pub fn init() -> AvCaptureVideoDataOutputSampleBufferDelegate {
		use ffi::CaptureVideoDataOutputSampleBufferDelegate;

		let obj = unsafe {

			let obj: *mut Object =  msg_send![&CaptureVideoDataOutputSampleBufferDelegate, alloc];
			msg_send![obj, init]
		};

		AvCaptureVideoDataOutputSampleBufferDelegate {
			obj,
		}
	}

	// TODO
	pub fn memcopy<A>(&self, dst: *mut A) {
		use libc::{c_void, memcpy};

		pub struct NSLock;

		unsafe impl<'a> ::objc::Encode for &'a mut NSLock {

			fn encode() -> ::objc::Encoding {
				unsafe { ::objc::Encoding::from_str("@\"NSLock\"") }
			}
		}

		let _: () = unsafe {

			let mut nsLock = (*self.obj).get_mut_ivar::<&mut NSLock>("nsLock");
			let mut nsLock = *nsLock as *mut NSLock as *mut Object;

			let _: () = msg_send![nsLock, lock];

			let dst = dst as *mut c_void;
			let src = *(*self.obj).get_ivar::<*mut i8>("frame") as *const c_void;
			let size = 1280 * 720 * 4;

			let _ = memcpy(dst, src, size);

			let _: () = msg_send![nsLock, unlock];
		};
	}
}

impl Drop for AvCaptureVideoDataOutputSampleBufferDelegate {

	fn drop(&mut self) {
		let _: () = unsafe {
			msg_send![self.obj, release]
		};
	}
}