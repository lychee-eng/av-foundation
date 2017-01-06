use objc_foundation::NSObject;
use super::{AvCaptureInput, AvCaptureInputPort};
use super::super::AvCaptureDevice;

/// Capture Device Input
///
/// `AVCaptureDeviceInput` is a concrete sub-class of `AVCaptureInput` you use to capture data from 
/// an `AVCaptureDevice` object.
pub struct AvCaptureDeviceInput {
	/// The device with which the input is associated.
	pub device: AvCaptureDevice,

	/// The capture input’s ports.
	///
	/// # Discussion
	///
	/// The array contains one or more instances of `AVCaptureInputPort`.
	///
	/// Each individual `AVCaptureInputPort` instance posts 
	/// an `AVCaptureInputPortFormatDescriptionDidChange` when the `formatDescription` of that 
	/// port changes.
	pub ports: Vec<Box<AvCaptureInputPort>>,

	/// `AVCaptureDeviceInput`
	pub(super::super) obj: *mut NSObject,
}

impl AvCaptureDeviceInput {

	/// Initializes an input to use a specified device.
	pub fn init__device(device: AvCaptureDevice) -> AvCaptureDeviceInput {
		use ffi::AVCaptureDeviceInput;

		unsafe {

			let error = 0 as *mut NSObject; // TODO

			// Create an instance of `AVCaptureDeviceInput` by calling `alloc`, then `init`.
			let obj: *mut NSObject = msg_send![&AVCaptureDeviceInput, alloc];
			let obj: *mut NSObject = msg_send![obj, initWithDevice:device.obj error:&error];

			AvCaptureDeviceInput { obj, device, ports: vec![] }
		}
	}
}

impl AvCaptureInput for AvCaptureDeviceInput {

	fn ports(&self) -> &Vec<Box<AvCaptureInputPort>> {
		&self.ports
	}
}

impl Drop for AvCaptureDeviceInput {

	fn drop(&mut self) {
		let _: () = unsafe {
			msg_send![self.obj, release]
		};
	}
}