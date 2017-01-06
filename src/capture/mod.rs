pub use self::connection::AvCaptureConnection;
pub use self::delegate::AvCaptureVideoDataOutputSampleBufferDelegate;
pub use self::device::AvCaptureDevice;
pub use self::input::{AvCaptureDeviceInput, AvCaptureInput, AvCaptureInputPort};
pub use self::output::{AvCaptureOutput, AvCaptureVideoDataOutput};
pub use self::session::AvCaptureSession;

mod connection;
mod delegate;
mod device;
mod input;
mod output;
mod session;

pub fn update_settings_todo(
	del: &AvCaptureVideoDataOutputSampleBufferDelegate, 
	data_output: &AvCaptureVideoDataOutput
) {

	unsafe {
		
		msg_send![del.obj, settings:data_output.obj]
	}
}