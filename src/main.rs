#[macro_use]
extern crate lazy_static;
extern crate anyhow;
extern crate getopts;

use anyhow::{anyhow, Result};
use futures_lite::*;
use getopts::Options;
use imagesnap::{Camera, Device};
use std::io::Write;
use std::sync::{mpsc, Mutex};
use std::{env, io, thread, time};

macro_rules! log{
    ($($arg:tt)*) => ({
        if *VERBOSE.lock().unwrap() {
            print!($($arg)*);
            io::stdout().flush().unwrap();
        }
    })
}

static DEFAULT_FILE: &str = "snapshot.jpg";

lazy_static! {
    static ref ARGS: Vec<String> = env::args().collect();
    static ref OPTS: Mutex<Options> = Mutex::new(Options::new());
    static ref VERBOSE: Mutex<bool> = Mutex::new(true);
}

fn main() -> Result<()> {
    let mut opts = OPTS.lock().unwrap();
    opts.optflag("q", "quiet", "Do not output any text");
    opts.optopt("w", "warmup", "Warm up camera for x seconds [0-10]", "x.x");
    opts.optflag("l", "list", "List available capture devices");
    opts.optopt("d", "device", "Use device with QUERY in its name", "QUERY");
    opts.optflag("h", "help", "This help message");

    let matches = opts.parse(&ARGS[1..])?;
    if matches.opt_present("h") {
        drop(opts);
        print_usage();
        return Ok(());
    }
    drop(opts);

    let mut verbose = VERBOSE.lock().unwrap();
    *verbose = !matches.opt_present("q");
    drop(verbose);

    match (
        matches.free.as_slice(),
        matches.opt_present("l"),
        matches.opt_str("w").map(|s| s.parse::<f32>()).transpose(),
        matches.opt_str("d"),
    ) {
        (_, _, Err(_), _) => Err(anyhow!("Failed to parse warmup!")),
        (_, _, Ok(Some(w)), _) if !w.is_finite() || !(0.0..=10.0).contains(&w) => {
            Err(anyhow!("Warmup must be between 0 and 10 seconds"))
        }
        ([], true, Ok(None), None) => list_devices(),
        ([filename], false, Ok(warmup), device) => {
            snap(filename.as_str(), warmup, device.map(Device::find).transpose()?)
        }
        ([], false, Ok(warmup), device) => {
            snap(DEFAULT_FILE, warmup, device.map(Device::find).transpose()?)
        }
        _ => Err(anyhow!("Invalid combination of arguments.")),
    }
}

fn print_usage() {
    println!("{} v{}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
    println!("  {}\n", env!("CARGO_PKG_DESCRIPTION"));
    println!("Usage:\n  {} [<OPTIONS>] [FILENAME]\n", &ARGS[0]);
    println!("FILENAME:");
    print!("  Defaults to '{}' (only JPG supported)", DEFAULT_FILE);
    println!("{}", OPTS.lock().unwrap().usage(""));
}

fn list_devices() -> Result<()> {
    Ok(for device in Device::all()? {
        println!("{}", device);
    })
}

fn snap<S: Into<String>>(filename: S, warmup: Option<f32>, device: Option<Device>) -> Result<()> {
    let camera = Camera::new(device, warmup)?;
    let filename = filename.into();
    let f = filename.clone();

    log!("Capturing image from device \"{}\"...", camera.device);
    loading_dots(move || Ok(future::block_on(camera.snap(&f))?))?;

    Ok(log!("{}\n", &filename))
}

fn loading_dots<F>(f: F) -> Result<()>
where
    F: Fn() -> Result<()>,
{
    let (tx, rx) = mpsc::channel::<bool>();
    let handler = thread::spawn(move || loop {
        match rx.try_recv() {
            Ok(_) => break,
            Err(_) => log!("."),
        }
        thread::sleep(time::Duration::from_millis(100));
    });
    let result = f();
    let _ = tx.send(true);
    handler
        .join()
        .map_err(|_| anyhow!("Progress thread panicked"))?;
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn loading_dots_propagates_capture_error() {
        let result = loading_dots(|| Err(anyhow!("capture failed")));
        assert_eq!(result.unwrap_err().to_string(), "capture failed");
    }

    #[test]
    fn loading_dots_propagates_success() {
        assert!(loading_dots(|| Ok(())).is_ok());
    }
}
