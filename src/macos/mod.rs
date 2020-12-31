extern crate objc;
use objc::declare::ClassDecl;
use objc::runtime::*;
use objc::{class, msg_send, sel, sel_impl};
use objc_foundation::{INSArray, INSString, NSArray, NSObject, NSString};
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

    pub fn capture() {
        let avcpo = class!(AVCapturePhotoOutput);
        let avcpo: *mut Object = unsafe { msg_send![avcpo, alloc] };
        let avcpo: *mut Object = unsafe { msg_send![avcpo, init] };

        let avcps = class!(AVCapturePhotoSettings);
        let avcps: *mut Object = unsafe { msg_send![avcps, photoSettings] };

        let session = class!(AVCaptureSession);
        let session: *mut Object = unsafe { msg_send![session, alloc] };
        let session: *mut Object = unsafe { msg_send![session, init] };

        let device: *mut Object = Client::default_input();
        let input = class!(AVCaptureDeviceInput);
        let null: *const i32 = std::ptr::null();
        let input: *mut Object =
            unsafe { msg_send![input, deviceInputWithDevice: device error: null] };

        unsafe { msg_send![session, addInput: input] }
        unsafe { msg_send![session, addOutput: avcpo] }
        unsafe { msg_send![session, startRunning] }

        let superclass = class!(NSObject);
        let mut delegate = ClassDecl::new("CaptureDelegate", superclass).unwrap();

        extern "C" fn capture_output_fn(
            _this: &mut Object,
            _cmd: Sel,
            capture_output: *mut Object,
            did_finish_processing_photo: *mut Object,
            error: *mut Object,
        ) {
            println!("capture_output: {:?}", capture_output);
            println!("did_finish: {:?}", did_finish_processing_photo);
            println!("error: {:?}", error);
        }
        let capture_output: extern "C" fn(&mut Object, Sel, *mut Object, *mut Object, *mut Object) =
            capture_output_fn;
        unsafe {
            delegate.add_method(
                sel!(captureOutput:didFinishProcessingPhoto:error:),
                capture_output,
            );
        }
        let delegate = delegate.register();
        let delegate: *mut Object = unsafe { msg_send![delegate, alloc] };
        let delegate: *mut Object = unsafe { msg_send![delegate, init] };

        unsafe { msg_send![avcpo, capturePhotoWithSettings: avcps delegate: delegate] }
    }
}
