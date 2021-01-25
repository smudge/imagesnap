extern crate block;
extern crate objc;
extern crate objc_foundation;
extern crate objc_id;

use block::ConcreteBlock;
use objc::runtime::*;
use objc::{class, msg_send, sel, sel_impl};
use objc_foundation::*;
use objc_id::{Id, Owned};
use std::os::raw::c_int;

#[link(name = "AVFoundation", kind = "framework")]
extern "C" {
    pub(super) static AVVideoCodecKey: *mut NSString;
    pub(super) static AVVideoCodecJPEG: *mut NSString;
    pub(super) static AVMediaTypeVideo: *mut NSString;

    pub(super) static AVCaptureDeviceTypeExternalUnknown: *mut NSString;
    pub(super) static AVCaptureDeviceTypeBuiltInWideAngleCamera: *mut NSString;
}

pub struct Client {}

impl Client {
    pub fn default_input() -> *mut Object {
        let av_capture_device = class!(AVCaptureDevice);
        unsafe {
            msg_send![
                av_capture_device,
                defaultDeviceWithMediaType: AVMediaTypeVideo
            ]
        }
    }

    pub fn default_device() -> String {
        let name: *mut NSString = unsafe { msg_send![Client::default_input(), localizedName] };
        unsafe { name.as_ref() }.unwrap().as_str().to_string()
    }

    pub fn device_names() -> Result<Vec<String>, String> {
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
        let mut device_names = vec![];
        for device in &devices {
            let name: *mut NSString = unsafe { msg_send![*device, localizedName] };
            device_names.push(unsafe { name.as_ref() }.unwrap().as_str().to_string())
        }
        Ok(device_names)
    }

    pub fn capture(filename: String, warmup: f32) {
        let session = class!(AVCaptureSession);
        let session: *mut Object = unsafe { msg_send![session, alloc] };
        let session: *mut Object = unsafe { msg_send![session, init] };

        let device: *mut Object = Client::default_input();
        let input = class!(AVCaptureDeviceInput);
        let null: *const i32 = std::ptr::null();
        let input: *mut Object =
            unsafe { msg_send![input, deviceInputWithDevice: device error: null] };

        unsafe { msg_send![session, addInput: input] }

        let av_capture_class = class!(AVCaptureStillImageOutput);
        let avcsio: *mut Object = unsafe { msg_send![av_capture_class, new] };

        let settings = unsafe {
            NSDictionary::from_keys_and_objects(
                &[&*AVVideoCodecKey],
                vec![Id::from_ptr(AVVideoCodecJPEG)],
            )
        };

        unsafe { msg_send![avcsio, setOutputSettings: settings] }
        unsafe { msg_send![session, addOutput: avcsio] }

        let (tx, rx) = std::sync::mpsc::channel();
        let handler = ConcreteBlock::new(move |photo: *const Object, _error: *const Object| {
            let image_data: *mut NSData =
                unsafe { msg_send![av_capture_class, jpegStillImageNSDataRepresentation: photo] };
            let filename = NSString::from_str(&filename);
            unsafe { msg_send![image_data, writeToFile:filename atomically:YES] }
            tx.send("success").unwrap();
        });

        let connections: *mut Object = unsafe { msg_send![avcsio, connections] };
        let connection: *mut Object = unsafe { msg_send![connections, firstObject] };

        unsafe { msg_send![session, startRunning] }
        std::thread::sleep(std::time::Duration::from_secs_f32(warmup));
        unsafe {
            msg_send![avcsio, captureStillImageAsynchronouslyFromConnection:connection completionHandler:handler.copy()]
        }
        rx.recv().unwrap();

        unsafe { msg_send![session, stopRunning] }
    }
}
