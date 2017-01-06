#![allow(warnings)]
#![feature(field_init_shorthand, pub_restricted)]

#[macro_use] extern crate log;
#[macro_use] extern crate objc;

extern crate dispatch;
extern crate objc_foundation;
extern crate objc_id;

pub use capture::{
	AvCaptureConnection, 
	AvCaptureDevice, AvCaptureDeviceInput, AvCaptureInput, AvCaptureInputPort,
	AvCaptureOutput, 
	AvCaptureVideoDataOutput, AvCaptureVideoDataOutputSampleBufferDelegate,
	AvCaptureSession,

	update_settings_todo
};

pub use media_type::AvMediaType;

mod capture;
mod media_type;

mod ffi;