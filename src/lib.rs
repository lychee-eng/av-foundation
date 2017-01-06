#![allow(warnings)]
#![feature(pub_restricted, type_ascription)]

#[macro_use] extern crate log;
#[macro_use] extern crate objc;

extern crate dispatch;
extern crate objc_foundation;
extern crate objc_id;

pub use capture_device::AvCaptureDevice;
pub use capture_device_input::AvCaptureDeviceInput;
pub use capture_input::{AvCaptureInput, AvCaptureInputPort};
pub use capture_output::AvCaptureOutput;
pub use capture_session::AvCaptureSession;
pub use capture_video_data_output::AvCaptureVideoDataOutput;
pub use error::AvError;
pub use media_type::AvMediaType;

mod capture_device;
mod capture_device_input;
mod capture_input;
mod capture_output;
mod capture_session;
mod capture_video_data_output;
mod error;
mod media_type;
mod ffi;