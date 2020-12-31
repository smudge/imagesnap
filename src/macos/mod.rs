extern crate objc;
use objc::runtime::*;
use objc::{class, msg_send, sel, sel_impl};
use objc_foundation::{INSArray, INSObject, INSString, NSArray, NSObject, NSString};
use std::os::raw::c_int;

#[link(name = "AVFoundation", kind = "framework")]
extern "C" {
    pub(super) static AVMediaTypeVideo: *mut NSString;

    pub(super) static AVCaptureDeviceTypeExternalUnknown: *mut NSString;
    pub(super) static AVCaptureDeviceTypeBuiltInWideAngleCamera: *mut NSString;
}

pub struct Client {}

impl Client {
    pub fn default_device() {
        let av_capture_device = class!(AVCaptureDevice);
        let device: *mut Object = unsafe {
            msg_send![
                av_capture_device,
                defaultDeviceWithMediaType: AVMediaTypeVideo
            ]
        };
        let name: *mut NSString = unsafe { msg_send![device, localizedName] };
        println!("DEVICE: {:#?}", unsafe { name.as_ref().unwrap().as_str() });
    }
}
