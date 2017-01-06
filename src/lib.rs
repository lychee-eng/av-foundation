#![allow(warnings)]
#![feature(pub_restricted)]

#[macro_use] extern crate log;
#[macro_use] extern crate objc;

extern crate dispatch;
extern crate objc_foundation;
extern crate objc_id;

pub use error::AvError;

mod error;