#![allow(warnings)]
#![feature(field_init_shorthand, pub_restricted)]

#[macro_use] extern crate log;
#[macro_use] extern crate objc;

extern crate dispatch;
extern crate libc;
extern crate objc_foundation;
extern crate objc_id;

pub use capture::{
	AvCaptureConnection, 
	AvCaptureDevice, AvCaptureDeviceInput, AvCaptureInput, AvCaptureInputPort,
	AvCaptureOutput, 
	AvCaptureVideoDataOutput, AvCaptureVideoDataOutputSampleBufferDelegate,
	AvCaptureSession,
};

pub use media_type::AvMediaType;

pub mod ffi;

mod capture;
mod media_type;