use objc::runtime::Class;
use objc_foundation::NSString;

#[link(name = "AVFoundation", kind = "framework")]
extern {

    #[link_name = "OBJC_CLASS_$_AVCaptureDevice"]
    pub(super) static AVCaptureDevice: Class;
    
    #[link_name = "OBJC_CLASS_$_AVCaptureDeviceInput"]
    pub(super) static AVCaptureDeviceInput: Class;
    
    #[link_name = "OBJC_CLASS_$_AVCaptureVideoDataOutput"]
    pub(super) static AVCaptureVideoDataOutput: Class;
    
    #[link_name = "OBJC_CLASS_$_AVCaptureSession"]
    pub(super) static AVCaptureSession: Class;

	pub(super) static AVMediaTypeVideo: *mut NSString;
}

#[link(name = "CoreFoundation", kind = "framework")]
extern {

    pub static kCVPixelBufferPixelFormatTypeKey: *mut Class;
}

#[link(name = "capture")]
extern {
    
    #[link_name = "OBJC_CLASS_$_CaptureVideoDataOutputSampleBufferDelegate"]
    pub(super) static CaptureVideoDataOutputSampleBufferDelegate: Class;
}