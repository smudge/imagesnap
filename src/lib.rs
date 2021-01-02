#[cfg_attr(target_os = "macos", path = "macos/mod.rs")]
mod os;

pub struct Snap {
    device: String,
    filename: String,
    verbose: bool,
    warmup: f32,
}

impl Snap {
    pub fn new(
        device: Option<String>,
        filename: Option<String>,
        verbose: bool,
        warmup: Option<f32>,
    ) -> Snap {
        let device = device.unwrap_or(Snap::default_device());
        let filename = filename.unwrap_or("snapshot.jpg".to_string());
        let warmup = warmup.unwrap_or(0.0);
        Snap {
            device,
            filename,
            verbose,
            warmup,
        }
    }

    pub fn create(&self) -> Result<(), String> {
        if self.verbose {
            println!(
                "Capturing image from device \"{}\"..................{}",
                self.device, self.filename
            );
        }
        os::Client::capture(self.filename.clone(), self.warmup);
        Ok(())
    }

    pub fn list_devices() {
        os::Client::list_devices()
    }

    pub fn default_device() -> String {
        os::Client::default_device()
    }
}
