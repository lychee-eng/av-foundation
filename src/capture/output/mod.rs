pub use self::video_data::AvCaptureVideoDataOutput;
mod video_data;

use super::AvCaptureConnection;
use super::super::AvMediaType;

/// `AVCaptureOutput` is an abstract base class describing an output destination of 
/// an `AVCaptureSession` object.
pub trait AvCaptureOutput {

	/// The capture output object’s connections.
	fn connections(&self) -> &Vec<AvCaptureConnection>;

	/// Returns the first connection in the connections array with an input port of a 
	/// specified media type.
	fn connection__withMediaType(&self, media_type: AvMediaType) -> &AvCaptureConnection;
}