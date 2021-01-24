extern crate anyhow;
extern crate getopts;

use anyhow::{anyhow, Result};
use getopts::Options;
use imagesnap::{Camera, Device};
use std::env;

static DEFAULT_FILE: &str = "snapshot.jpg";

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();

    let mut opts = Options::new();
    opts.optflag("q", "quiet", "Do not output any text");
    opts.optopt("w", "warmup", "Warm up camera for x seconds [0-10]", "x.x");
    opts.optflag("l", "list", "List available capture devices");
    opts.optopt("d", "device", "Use specific capture device", "NAME");
    opts.optflag("h", "help", "This help message");

    let matches = opts.parse(&args[1..])?;
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
            Ok(Camera::new(device, verbose, warmup)?.snap(maybe_file.unwrap_or(DEFAULT_FILE))?)
        }
        (None, None, true, false, _, Ok(None), Ok(None)) => list_devices(),
        (None, None, false, true, _, Ok(None), Ok(None)) => Ok(print_usage(&args[0], &opts)),
        (_, None, false, false, _, Err(_), _) => Err(anyhow!("Failed to parse warmup!")),
        (_, None, false, false, _, Ok(_), Err(e)) => Err(anyhow!(e.to_string())),
        (_, _, _, _, _, _, _) => Err(anyhow!("Invalid combination of arguments.")),
    }
}

fn print_usage(program: &String, opts: &Options) {
    println!("{} v{}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
    println!("  {}\n", env!("CARGO_PKG_DESCRIPTION"));
    println!("Usage:\n  {} [<OPTIONS>] [FILENAME]\n", program);
    println!("FILENAME:");
    print!("  Defaults to '{}' (only JPG supported)", DEFAULT_FILE);
    println!("{}", opts.usage(""));
}

fn list_devices() -> Result<()> {
    Ok(for device in Device::all()? {
        println!("{}", device);
    })
}
