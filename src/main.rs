#[macro_use]
extern crate lazy_static;
extern crate anyhow;
extern crate getopts;

use anyhow::{anyhow, Result};
use futures::executor::block_on;
use getopts::Options;
use imagesnap::{Camera, Device};
use std::env;
use std::sync::Mutex;

static DEFAULT_FILE: &str = "snapshot.jpg";

lazy_static! {
    static ref ARGS: Vec<String> = env::args().collect();
    static ref OPTS: Mutex<Options> = Mutex::new(Options::new());
}

fn main() -> Result<()> {
    let mut opts = OPTS.lock().unwrap();
    opts.optflag("q", "quiet", "Do not output any text");
    opts.optopt("w", "warmup", "Warm up camera for x seconds [0-10]", "x.x");
    opts.optflag("l", "list", "List available capture devices");
    opts.optopt("d", "device", "Use device with QUERY in its name", "QUERY");
    opts.optflag("h", "help", "This help message");

    let matches = OPTS.lock().unwrap().parse(&ARGS[1..])?;
    match (
        matches.free.get(0).map(|s| s.as_str()),
        matches.free.get(1),
        matches.opt_present("l"),
        matches.opt_present("h"),
        !matches.opt_present("q"),
        matches.opt_str("w").map(|s| s.parse()).transpose(),
        matches.opt_str("d").map(|d| Device::find(d)).transpose(),
    ) {
        (_, None, false, false, _, Ok(Some(w)), Ok(_)) if w < 0.0 || w > 10.0 => {
            Err(anyhow!("Warmup must be between 0 and 10 seconds"))
        }
        (maybe_file, None, false, false, verbose, Ok(warmup), Ok(device)) => {
            snap(maybe_file.unwrap_or(DEFAULT_FILE), verbose, warmup, device)
        }
        (None, None, true, false, _, Ok(None), Ok(None)) => list_devices(),
        (None, None, false, true, _, Ok(None), Ok(None)) => Ok(print_usage()),
        (_, None, false, false, _, Err(_), _) => Err(anyhow!("Failed to parse warmup!")),
        (_, None, false, false, _, Ok(_), Err(e)) => Err(anyhow!(e.to_string())),
        (_, _, _, _, _, _, _) => Err(anyhow!("Invalid combination of arguments.")),
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

fn snap<S: Into<String>>(
    filename: S,
    verbose: bool,
    warmup: Option<f32>,
    device: Option<Device>,
) -> Result<()> {
    let camera = Camera::new(device, warmup)?;
    let filename = filename.into();
    let result = camera.snap(&filename);
    if verbose {
        println!(
            "Capturing image from device \"{}\"..................{}",
            camera.device, &filename
        )
    }
    Ok(block_on(result)?)
}
