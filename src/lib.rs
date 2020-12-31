#[cfg_attr(target_os = "macos", path = "macos/mod.rs")]
mod os;

pub struct Snap {
    device: String,
    filename: String,
}

impl Snap {
    pub fn new(device: String, filename: String) -> Snap {
        Snap { device, filename }
    }

    pub fn create(&self) -> Result<(), String> {
        println!(
            "Capturing image from device \"{}\"..................{}",
            self.device, self.filename
        );
        Ok(())
    }

    pub fn list_devices() {
        os::Client::list_devices()
    }

    pub fn default_device() -> String {
        os::Client::default_device()
    }
}
