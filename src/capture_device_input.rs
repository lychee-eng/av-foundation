use objc_foundation::NSObject;
use super::{AvCaptureDevice, AvCaptureInput};

/// Capture Device Input
///
/// `AVCaptureDeviceInput` is a concrete sub-class of `AVCaptureInput` you use to capture data from 
/// an `AVCaptureDevice` object.
pub struct AvCaptureDeviceInput {
	/// The device with which the input is associated.
	device: AvCaptureDevice,
	pub(super) sup: AvCaptureInput,
}

impl AvCaptureDeviceInput {

	/// Initializes an input to use a specified device.
	pub fn init__device(device: AvCaptureDevice) -> AvCaptureDeviceInput {

		unimplemented!()
	}
}