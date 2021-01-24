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
}

#[cfg_attr(target_os = "macos", path = "macos/mod.rs")]
mod os;

pub struct Camera {
    device: Device,
    warmup: f32,
    on_snap: Box<dyn FnMut(Device, String)>,
}

#[derive(Clone)]
pub struct Device {
    name: String,
}

impl std::fmt::Display for Device {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        f.pad(self.name.as_str())
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

    pub fn find(name: String) -> Result<Device, ImagesnapError> {
        match Device::all()?
            .iter()
            .filter(|e| e.name.contains(name.as_str()))
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
        Ok(Camera {
            device,
            warmup,
            on_snap: Box::new(|_, _| ()),
        })
    }

    pub fn snap<S: Into<String>>(&mut self, filename: S) -> Result<(), ImagesnapError> {
        let filename = filename.into();
        (self.on_snap)(self.device.clone(), filename.clone());
        os::Client::capture(filename.clone(), self.warmup);
        Ok(())
    }

    pub fn on_snap(&mut self, c: impl FnMut(Device, String) + 'static) -> &mut Camera {
        self.on_snap = Box::new(c);
        self
    }
}
