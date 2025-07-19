use std::fs;
use std::io::Write;

pub struct Client;

impl Client {
    pub fn default_device() -> String {
        "/dev/video0".to_string()
    }

    pub fn device_names() -> Result<Vec<String>, String> {
        let mut names = Vec::new();
        if let Ok(entries) = fs::read_dir("/dev") {
            for entry in entries {
                let entry = entry.map_err(|e| e.to_string())?;
                let filename = entry.file_name();
                if let Some(name) = filename.to_str() {
                    if name.starts_with("video") {
                        names.push(format!("/dev/{}", name));
                    }
                }
            }
        }
        Ok(names)
    }

    pub async fn capture<S: Into<String>>(
        device_name: S,
        filename: S,
        warmup: f32,
    ) -> Result<(), String> {
        let device_name = device_name.into();
        let filename = filename.into();
        let mut camera = rscam::new(&device_name).map_err(|e| e.to_string())?;
        camera
            .start(&rscam::Config {
                interval: (1, 30),
                resolution: (640, 480),
                format: b"MJPG",
                ..Default::default()
            })
            .map_err(|e| e.to_string())?;
        std::thread::sleep(std::time::Duration::from_secs_f32(warmup));
        let frame = camera.capture().map_err(|e| e.to_string())?;
        let mut file = fs::File::create(&filename).map_err(|e| e.to_string())?;
        file.write_all(&frame[..]).map_err(|e| e.to_string())?;
        Ok(())
    }
}
