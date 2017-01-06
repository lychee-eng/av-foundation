#[macro_use] extern crate objc;
extern crate av_foundation as av;
extern crate dispatch;

pub use delegate::Buf;
mod delegate;

fn main() {
	let buf = Buf::init();

	buf.session.startRunning();

	loop { }
}