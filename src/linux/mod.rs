use std::fs;
use std::io::Write;

fn write_jpeg<W: Write>(writer: &mut W, bytes: &[u8]) -> Result<(), String> {
    writer.write_all(bytes).map_err(|e| e.to_string())
}

fn write_frame(filename: &str, frame: &[u8]) -> Result<(), String> {
    match filename {
        "-" => write_jpeg(&mut std::io::stdout().lock(), frame),
        path => {
            let mut file = fs::File::create(path).map_err(|e| e.to_string())?;
            write_jpeg(&mut file, frame)
        }
    }
}

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
        write_frame(&filename, &frame)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::write_jpeg;

    #[test]
    fn writes_jpeg_bytes_without_text_prefix() {
        let bytes = [0xff, 0xd8, 0x00, 0xff, 0xd9];
        let mut output = Vec::new();
        write_jpeg(&mut output, &bytes).unwrap();
        assert_eq!(output, bytes);
    }

    #[test]
    fn reports_write_failures() {
        struct Failure;
        impl std::io::Write for Failure {
            fn write(&mut self, _: &[u8]) -> std::io::Result<usize> {
                Err(std::io::Error::new(std::io::ErrorKind::BrokenPipe, "closed"))
            }
            fn flush(&mut self) -> std::io::Result<()> { Ok(()) }
        }
        assert!(write_jpeg(&mut Failure, b"jpeg").is_err());
    }
}
