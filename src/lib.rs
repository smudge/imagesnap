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
    #[error("Error capturing image!")]
    CaptureError,
}

#[cfg_attr(target_os = "macos", path = "macos/mod.rs")]
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

impl Camera {
    pub fn new(device: Option<Device>, warmup: Option<f32>) -> Result<Camera, ImagesnapError> {
        let device = device.unwrap_or(Device::default());
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
            .map_err(|_| ImagesnapError::CaptureError)
    }
}
