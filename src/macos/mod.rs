extern crate objc;
use objc::runtime::*;
use objc::{class, msg_send, sel, sel_impl};
use objc_foundation::{INSArray, INSObject, INSString, NSArray, NSObject, NSString};
use objc_id::{Id, Owned};
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

    pub fn list_devices() {
        let discovery_session = class!(AVCaptureDeviceDiscoverySession);
        let device_types = unsafe {
            vec![
                Id::from_ptr(AVCaptureDeviceTypeBuiltInWideAngleCamera),
                Id::from_ptr(AVCaptureDeviceTypeExternalUnknown),
            ]
        };
        let device_types: Id<NSArray<NSString, Owned>> = NSArray::from_vec(device_types);
        let position = 2 as c_int;
        let discovery_session: *mut Object = unsafe {
            msg_send![discovery_session, discoverySessionWithDeviceTypes:device_types mediaType:AVMediaTypeVideo position:position]
        };
        let devices: *mut NSArray<NSObject> = unsafe { msg_send![discovery_session, devices] };
        let devices = unsafe { devices.as_ref().unwrap().to_vec() };
        for device in &devices {
            let name: *mut NSString = unsafe { msg_send![*device, localizedName] };
            println!("{}", unsafe { name.as_ref().unwrap().as_str() });
        }
    }
}
