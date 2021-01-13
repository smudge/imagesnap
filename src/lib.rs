#[cfg_attr(target_os = "macos", path = "macos/mod.rs")]
mod os;

pub struct Camera {
    device: String,
    verbose: bool,
    warmup: f32,
}

impl Camera {
    pub fn new(
        device: Option<String>,
        verbose: bool,
        warmup: Option<f32>,
    ) -> Result<Camera, String> {
        let device = device.unwrap_or(Camera::default_device());
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

    pub fn list_devices() -> Result<(), String> {
        os::Client::list_devices()
    }

    pub fn default_device() -> String {
        os::Client::default_device()
    }
}
