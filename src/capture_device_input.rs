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