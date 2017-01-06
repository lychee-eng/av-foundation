#![allow(warnings)]
#![feature(pub_restricted)]

#[macro_use] extern crate log;
#[macro_use] extern crate objc;

extern crate dispatch;
extern crate objc_foundation;
extern crate objc_id;

pub use capture_device::AvCaptureDevice;
pub use error::AvError;
pub use media_type::AvMediaType;

mod capture_device;
mod error;
mod media_type;
mod ffi;