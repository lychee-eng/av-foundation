/// An `AVCaptureInputPort` represents a stream of data from a capture input.
pub trait AvCaptureInputPort {
	
	/// Indicates whether the port is enabled.
	fn is_enabled(&self) -> bool;
}