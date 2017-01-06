extern crate gcc;

use gcc::Config;

fn main() {
	let _ = {
		Config::new()
			   .file("src-build/capture.m")
			   .flag("-fmodules")
			   .compile("libcapture.a")
	};


}