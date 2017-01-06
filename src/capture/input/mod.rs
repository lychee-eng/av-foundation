pub use self::device::AvCaptureDeviceInput;
pub use self::port::AvCaptureInputPort;

mod device;
mod port;

use objc_foundation::NSObject;

/// `AVCaptureInput` is an abstract base-class describing an input data source to 
/// an `AVCaptureSession` object.
pub trait AvCaptureInput {

	fn ports(&self) -> &Vec<Box<AvCaptureInputPort>>;
}