#[cfg_attr(target_os = "macos", path = "macos/mod.rs")]
mod os;

pub struct Camera {
    device: Device,
    verbose: bool,
    warmup: f32,
}

pub struct Device {
    name: String,
}

impl std::fmt::Display for Device {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        f.pad(self.name.as_str())
    }
}

impl Device {
    pub fn all() -> Vec<Device> {
        os::Client::device_names()
            .unwrap()
            .iter()
            .map(|name| Device { name: name.clone() })
            .collect()
    }

    pub fn find(_name: String) -> Result<Device, String> {
        Ok(Device::default())
    }

    pub fn default() -> Device {
        Device {
            name: os::Client::default_device(),
        }
    }
}

impl Camera {
    pub fn new(
        device: Option<Device>,
        verbose: bool,
        warmup: Option<f32>,
    ) -> Result<Camera, String> {
        let device = device.unwrap_or(Device::default());
        let warmup = warmup.unwrap_or(0.5);
        Ok(Camera {
            device,
            verbose,
            warmup,
        })
    }

    pub fn snap<S: Into<String>>(&self, filename: S) -> Result<(), String> {
        let filename = filename.into();
        if self.verbose {
            println!(
                "Capturing image from device \"{}\"..................{}",
                self.device, filename,
            );
        }
        os::Client::capture(filename.clone(), self.warmup);
        Ok(())
    }
}
