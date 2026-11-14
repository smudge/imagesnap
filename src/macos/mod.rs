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
    pub(super) static AVVideoCodecKey: *mut Object;
    pub(super) static AVVideoCodecJPEG: *mut Object;
    pub(super) static AVMediaTypeVideo: *mut Object;

    pub(super) static AVCaptureDeviceTypeExternalUnknown: *mut Object;
    pub(super) static AVCaptureDeviceTypeBuiltInWideAngleCamera: *mut Object;
}

fn capture_result(has_error: bool, has_image_data: bool, wrote_file: bool) -> Result<(), String> {
    match (has_error, has_image_data, wrote_file) {
        (true, _, _) => Err("Camera reported a capture error".into()),
        (_, false, _) => Err("Camera returned no JPEG data".into()),
        (_, _, false) => Err("Could not write captured image".into()),
        _ => Ok(()),
    }
}

pub struct Client {}

impl Client {
    pub fn default_device() -> String {
        let av_capture_device = class!(AVCaptureDevice);
        let default_input: *mut Object = unsafe {
            msg_send![
                av_capture_device,
                defaultDeviceWithMediaType: AVMediaTypeVideo
            ]
        };
        let name: *mut NSString = unsafe { msg_send![default_input, localizedName] };
        unsafe { name.as_ref() }.unwrap().as_str().to_string()
    }

    pub fn devices() -> Result<*mut NSArray<NSObject>, String> {
        let discovery_session = class!(AVCaptureDeviceDiscoverySession);
        let device_types = unsafe {
            vec![
                Id::from_ptr(AVCaptureDeviceTypeBuiltInWideAngleCamera as *mut NSString),
                Id::from_ptr(AVCaptureDeviceTypeExternalUnknown as *mut NSString),
            ]
        };
        let device_types: Id<NSArray<NSString, Owned>> = NSArray::from_vec(device_types);
        let position = 2 as c_int;
        let discovery_session: *mut Object = unsafe {
            msg_send![discovery_session, discoverySessionWithDeviceTypes:device_types mediaType:AVMediaTypeVideo position:position]
        };
        let devices: *mut NSArray<NSObject> = unsafe { msg_send![discovery_session, devices] };
        Ok(devices)
    }

    pub fn device_names() -> Result<Vec<String>, String> {
        let mut device_names = vec![];
        let devices = unsafe { Client::devices()?.as_ref() }.unwrap().enumerator();

        for device in devices {
            let name: *mut NSString = unsafe { msg_send![device, localizedName] };
            device_names.push(unsafe { name.as_ref() }.unwrap().as_str().to_string())
        }
        Ok(device_names)
    }

    pub fn device_with_name<S: Into<String>>(name: S) -> Result<*mut NSObject, String> {
        let name = name.into();
        let devices = unsafe { Client::devices()?.as_ref() }.unwrap().enumerator();

        for device in devices {
            let n: *mut NSString = unsafe { msg_send![device, localizedName] };
            let n = unsafe { n.as_ref() }.unwrap().as_str().to_string();
            if name == n {
                let unique_id: Box<*mut NSObject> =
                    Box::new(unsafe { msg_send![device, uniqueID] });
                let av_capture_device = class!(AVCaptureDevice);
                let device: *mut NSObject =
                    unsafe { msg_send![av_capture_device, deviceWithUniqueID: *unique_id] };
                return Ok(device);
            }
        }
        Err("No device found".to_string())
    }

    pub async fn capture<S: Into<String>>(
        device_name: S,
        filename: S,
        warmup: f32,
    ) -> Result<(), String> {
        let filename = filename.into();

        let session = class!(AVCaptureSession);
        let session: *mut Object = unsafe { msg_send![session, alloc] };
        let session: *mut Object = unsafe { msg_send![session, init] };

        let device = Box::new(Client::device_with_name(device_name)?);
        let input = class!(AVCaptureDeviceInput);
        let null: *const i32 = std::ptr::null();
        let input: *mut Object =
            unsafe { msg_send![input, deviceInputWithDevice: *device error: null] };

        unsafe { msg_send![session, addInput: input] }

        let av_capture_class = class!(AVCaptureStillImageOutput);
        let avcsio: *mut Object = unsafe { msg_send![av_capture_class, new] };

        let settings = unsafe {
            NSDictionary::from_keys_and_objects(
                &[&*(AVVideoCodecKey as *mut NSString)],
                vec![Id::from_ptr(AVVideoCodecJPEG as *mut NSString)],
            )
        };

        unsafe { msg_send![avcsio, setOutputSettings: settings] }
        unsafe { msg_send![session, addOutput: avcsio] }

        let (tx, rx) = std::sync::mpsc::channel();
        let handler = ConcreteBlock::new(move |photo: *const Object, error: *const Object| {
            let image_data: *mut NSData = match (error.is_null(), photo.is_null()) {
                (true, false) => unsafe {
                    msg_send![av_capture_class, jpegStillImageNSDataRepresentation: photo]
                },
                _ => std::ptr::null_mut(),
            };
            let wrote_file = match (error.is_null(), image_data.is_null()) {
                (true, false) => {
                    let path = NSString::from_str(&filename);
                    let written: BOOL =
                        unsafe { msg_send![image_data, writeToFile:path atomically:YES] };
                    written == YES
                }
                _ => false,
            };
            let result = capture_result(!error.is_null(), !image_data.is_null(), wrote_file);
            let _ = tx.send(result);
        });

        let connections: *mut Object = unsafe { msg_send![avcsio, connections] };
        let connection: *mut Object = unsafe { msg_send![connections, firstObject] };

        unsafe { msg_send![session, startRunning] }
        std::thread::sleep(std::time::Duration::from_secs_f32(warmup));
        unsafe {
            msg_send![avcsio, captureStillImageAsynchronouslyFromConnection:connection completionHandler:handler.copy()]
        }
        let result = rx.recv().map_err(|_| "Capture callback was dropped".to_string());

        unsafe { msg_send![session, stopRunning] }
        result?
    }
}

#[cfg(test)]
mod tests {
    use super::capture_result;

    #[test]
    fn capture_callback_outcomes() {
        assert!(capture_result(false, true, true).is_ok());
        assert_eq!(
            capture_result(true, false, false).unwrap_err(),
            "Camera reported a capture error"
        );
        assert_eq!(
            capture_result(false, false, false).unwrap_err(),
            "Camera returned no JPEG data"
        );
        assert_eq!(
            capture_result(false, true, false).unwrap_err(),
            "Could not write captured image"
        );
    }
}
