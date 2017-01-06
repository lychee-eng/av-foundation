#[macro_use] extern crate objc;
extern crate av_foundation as av;
extern crate dispatch;
extern crate objc_foundation;

pub use delegate::Buf;
mod delegate;

fn main() {
	let buf = Buf::init();

	buf.session.startRunning();

	loop { }
}

// pub struct Frame<'a> {
// 	delegate: &'a Buf,
// }

// impl<'a> Iterator for Frame<'a> {

// 	type Item = [[[u8; 4]; 1280]; 720];

// 	fn next(&mut self) -> Option<Self::Item> {
// 		use std::mem;

// 		unsafe {
// 			let mut item: [[[u8; 4]; 1280]; 720] = unsafe { mem::uninitialized() };

// 			let _: () = msg_send![self.delegate.super_, readFrame:&mut item];

// 			Some(item)
// 		}
// 	}
// }