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
use std::io::Write;

#[link(name = "AVFoundation", kind = "framework")]
extern "C" {
    pub(super) static AVVideoCodecKey: *mut Object;
    pub(super) static AVVideoCodecJPEG: *mut Object;
    pub(super) static AVMediaTypeVideo: *mut Object;

    pub(super) static AVCaptureDeviceTypeExternalUnknown: *mut Object;
    pub(super) static AVCaptureDeviceTypeBuiltInWideAngleCamera: *mut Object;
}

fn write_jpeg<W: Write>(writer: &mut W, bytes: &[u8]) -> Result<(), String> {
    writer.write_all(bytes).map_err(|e| e.to_string())
}

fn capture_result(has_error: bool, has_image_data: bool, wrote_file: bool) -> Result<(), String> {
    match (has_error, has_image_data, wrote_file) {
        (true, _, _) => Err("Camera reported a capture error".into()),
        (_, false, _) => Err("Camera returned no JPEG data".into()),
        (_, _, false) => Err("Could not write captured image".into()),
        _ => Ok(()),
    }
}

// Keep callback delivery and capture-session cleanup separate from AVFoundation.
fn receive_capture<F: FnOnce()>(
    receiver: std::sync::mpsc::Receiver<Result<(), String>>,
    cleanup: F,
) -> Result<(), String> {
    let outcome = receiver
        .recv()
        .map_err(|_| "Capture callback was dropped".to_string())
        .and_then(|result| result);
    cleanup();
    outcome
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
            let wrote_file = match (error.is_null(), image_data.is_null(), filename.as_str()) {
                (true, false, "-") => {
                    let len: usize = unsafe { msg_send![image_data, length] };
                    let ptr: *const u8 = unsafe { msg_send![image_data, bytes] };
                    match (ptr.is_null(), len) {
                        (true, _) | (_, 0) => false,
                        (false, _) => {
                            let jpeg = unsafe { std::slice::from_raw_parts(ptr, len) };
                            write_jpeg(&mut std::io::stdout().lock(), jpeg).is_ok()
                        }
                    }
                }
                (true, false, path) => {
                    let path = NSString::from_str(path);
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
        receive_capture(rx, || {
            unsafe { msg_send![session, stopRunning] }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::{capture_result, receive_capture, write_jpeg};
    use std::sync::mpsc;
    use std::cell::Cell;

    #[test]
    fn writes_jpeg_bytes_without_text_prefix() {
        let bytes = [0xff, 0xd8, 0x00, 0xff, 0xd9];
        let mut output = Vec::new();
        write_jpeg(&mut output, &bytes).unwrap();
        assert_eq!(output, bytes);
    }

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
    #[test]
    fn callback_success_and_failure_both_clean_up() {
        for delivered in [Ok(()), Err("capture failed".to_string())] {
            let (sender, receiver) = mpsc::channel();
            sender.send(delivered.clone()).unwrap();
            let called = Cell::new(false);
            let outcome = receive_capture(receiver, || called.set(true));
            assert_eq!(outcome, delivered);
            assert!(called.get());
        }
    }

    #[test]
    fn dropped_callback_still_cleans_up() {
        let (sender, receiver) = mpsc::channel::<Result<(), String>>();
        drop(sender);
        let called = Cell::new(false);
        let outcome = receive_capture(receiver, || called.set(true));
        assert_eq!(outcome.unwrap_err(), "Capture callback was dropped");
        assert!(called.get());
    }

}
