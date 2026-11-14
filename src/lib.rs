extern crate thiserror;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum ImagesnapError {
    #[error("Multiple matching devices found!")]
    MultipleMatchingDevices,
    #[error("No matching devices found!")]
    NoMatchingDevices,
    #[error("Error discovering devices!")]
    DeviceLookupError,
    #[error("Error capturing image: {0}")]
    CaptureError(String),
}

#[cfg_attr(target_os = "macos", path = "macos/mod.rs")]
#[cfg_attr(target_os = "linux", path = "linux/mod.rs")]
mod os;

pub struct Camera {
    pub device: Device,
    warmup: f32,
}

#[derive(Clone)]
pub struct Device {
    name: String,
}

impl std::fmt::Display for Device {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        f.pad(&self.name)
    }
}

impl Device {
    pub fn all() -> Result<Vec<Device>, ImagesnapError> {
        Ok(os::Client::device_names()
            .map_err(|_| ImagesnapError::DeviceLookupError)?
            .iter()
            .map(|name| Device { name: name.clone() })
            .collect())
    }

    pub fn find<S: Into<String>>(name: S) -> Result<Device, ImagesnapError> {
        let name = name.into();
        match Device::all()?
            .iter()
            .filter(|e| e.name.contains(&name))
            .collect::<Vec<_>>()
            .split_first()
        {
            Some((a, [])) => Ok((*a).clone()),
            Some((_, _)) => Err(ImagesnapError::MultipleMatchingDevices),
            None => Err(ImagesnapError::NoMatchingDevices),
        }
    }

    pub fn default() -> Device {
        Device {
            name: os::Client::default_device(),
        }
    }
}

fn select_device<F: FnOnce() -> Device>(device: Option<Device>, default: F) -> Device {
    device.unwrap_or_else(default)
}

impl Camera {
    pub fn new(device: Option<Device>, warmup: Option<f32>) -> Result<Camera, ImagesnapError> {
        let device = select_device(device, Device::default);
        let warmup = warmup.unwrap_or(0.5);
        Ok(Camera { device, warmup })
    }

    pub fn default() -> Result<Camera, ImagesnapError> {
        Camera::new(None, None)
    }

    pub async fn snap<S: Into<String>>(&self, filename: S) -> Result<(), ImagesnapError> {
        let filename = filename.into();
        os::Client::capture(&self.device.name, &filename, self.warmup)
            .await
            .map_err(ImagesnapError::CaptureError)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn capture_error_preserves_backend_detail() {
        let error = ImagesnapError::CaptureError("Capture timed out after 30 seconds".into());
        assert!(error.to_string().contains("Capture timed out after 30 seconds"));
    }

    #[test]
    fn explicit_device_does_not_resolve_default() {
        let selected = Device {
            name: "chosen camera".to_string(),
        };
        let device = select_device(Some(selected), || panic!("Default device should not be queried"));
        assert_eq!(device.name, "chosen camera");
    }

    #[test]
    fn missing_device_uses_default_provider() {
        let device = select_device(None, || Device {
            name: "default camera".to_string(),
        });
        assert_eq!(device.name, "default camera");
    }
}
