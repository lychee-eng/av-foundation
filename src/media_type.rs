use objc_foundation::{NSString, };
use std::ops::Deref;
use super::ffi;

pub enum AvMediaType {
	Video,

	__NonExhaustive,
}

impl AvMediaType {

	pub(super) fn ns_string(&self) -> *mut NSString {

		match *self {
			AvMediaType::Video => {
				ffi::AVMediaTypeVideo
			},

			_ => unimplemented!(),
		}
	}
}