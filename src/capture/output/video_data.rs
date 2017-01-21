use dispatch::ffi::dispatch_queue_t;
use objc::runtime::{Class, Object};
use objc_foundation::NSObject;

use AvCaptureVideoDataOutputSampleBufferDelegate as Delegate;
use super::AvCaptureOutput;
use super::super::AvCaptureConnection;
use super::super::super::AvMediaType;

pub struct AvCaptureVideoDataOutput {
	pub(super::super) obj: *mut Object,
}

impl AvCaptureVideoDataOutput {
	pub fn init() -> AvCaptureVideoDataOutput {
		use ffi::AVCaptureVideoDataOutput;

		let obj = unsafe {

			let obj: *mut Object = msg_send![&AVCaptureVideoDataOutput, alloc];
			msg_send![obj, init]
		};

		AvCaptureVideoDataOutput {
			obj,
		}
	}

	// TODO The only key currently supported is the kCVPixelBufferPixelFormatTypeKey key.
	// TODO return a list of supported formats
	
	pub fn set__videoSettings(&self, videoSettings: *mut NSObject) {

		unsafe {
			
			let _: () = msg_send![self.obj, setVideoSettings:videoSettings];
		}
	}

	pub fn set__videoSettings_default(&self, width: u32, height: u32) {
		use ffi::{kCVPixelBufferWidthKey, kCVPixelBufferHeightKey, kCVPixelBufferPixelFormatTypeKey};

		let NSMutableDictionary = Class::get("NSMutableDictionary").unwrap();
		let NSNumber = Class::get("NSNumber").unwrap();

		unsafe {

			let dictionary: *mut NSObject = {

				// kCVPixelFormatType_32BGRA
				let px: *mut NSObject = msg_send![NSNumber, numberWithInt:0x42475241];

				msg_send![
					NSMutableDictionary,
					dictionaryWithObject:px forKey:kCVPixelBufferPixelFormatTypeKey
				]
			};

			{
				let height: *mut NSObject = msg_send![NSNumber, numberWithUnsignedLong:height];

				let _: () = msg_send![
					dictionary, 
					setObject:height forKey:kCVPixelBufferHeightKey
				];
			}

			{
				let width: *mut NSObject = msg_send![NSNumber, numberWithUnsignedLong:width];

				let _: () = msg_send![dictionary, setObject:width forKey:kCVPixelBufferWidthKey];
			}
			
			let _: () = msg_send![self.obj, setVideoSettings:dictionary];

			// let _: () = msg_send![value, release];
			// let _: () = msg_send![dictionary, release];
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