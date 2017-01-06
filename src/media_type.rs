use objc_foundation::{NSString, };

use ffi::AVMediaTypeVideo;

pub enum AvMediaType { Video }

impl AvMediaType {

	pub(super) fn as_ptr(&self) -> *mut NSString {
		use self::AvMediaType::*;

		match self {
			&Video => AVMediaTypeVideo,
		}
	}
}