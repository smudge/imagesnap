extern crate getopts;

use getopts::{Matches, Options};
use imagesnap::{Camera, Device};
use std::{env, fmt};

type Exit = Result<(), Error>;

enum Error {
    UsageError(Option<String>),
    InternalError(String),
}

impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            Error::UsageError(Some(val)) => f.pad(val),
            Error::InternalError(val) => f.pad(val),
            Error::UsageError(None) => unreachable!(),
        }
    }
}

impl From<String> for Error {
    fn from(msg: String) -> Error {
        Error::InternalError(msg)
    }
}

impl Error {
    fn err(msg: &str) -> Result<(), Error> {
        Err(Error::UsageError(Some(msg.to_string())))
    }

    fn print_usage() -> Result<(), Error> {
        Err(Error::UsageError(None))
    }
}

static DEFAULT_FILE: &str = "snapshot.jpg";

fn main() -> Result<(), Error> {
    let args: Vec<String> = env::args().collect();

    let mut opts = Options::new();
    opts.optflag("q", "quiet", "Do not output any text");
    opts.optopt("w", "warmup", "Warm up camera for x seconds [0-10]", "x.x");
    opts.optflag("l", "list", "List available capture devices");
    opts.optopt("d", "device", "Use specific capture device", "NAME");
    opts.optflag("h", "help", "This help message");

    Ok(opts
        .parse(&args[1..])
        .map_or_else(|m| Error::err(&m.to_string()), |m| run(m))
        .or_else(|e| {
            if let Error::UsageError(_) = e {
                print_usage(&args[0], &opts);
            }
            match e {
                Error::UsageError(None) => Ok(()),
                other => Err(other),
            }
        })?)
}

fn print_usage(program: &String, opts: &Options) {
    println!("{} v{}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
    println!("  {}\n", env!("CARGO_PKG_DESCRIPTION"));
    println!("Usage:\n  {} [<OPTIONS>] [FILENAME]\n", program);
    println!("FILENAME:");
    print!("  Defaults to '{}' (only JPG supported)", DEFAULT_FILE);
    println!("{}", opts.usage(""));
}

fn run(matches: Matches) -> Exit {
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
            Error::err("Warmup must be between 0 and 10 seconds")
        }
        (maybe_file, None, false, false, verbose, Ok(warmup), Ok(device)) => {
            Ok(Camera::new(device, verbose, warmup)?.snap(maybe_file.unwrap_or(DEFAULT_FILE))?)
        }
        (None, None, true, false, _, Ok(None), Ok(None)) => list_devices(),
        (None, None, false, true, _, Ok(None), Ok(None)) => Error::print_usage(),
        (_, None, false, false, _, Err(_), _) => Error::err("Failed to parse warmup!"),
        (_, _, _, _, _, _, _) => Error::err("Invalid combination of arguments."),
    }
}

fn list_devices() -> Exit {
    for device in Device::all() {
        println!("{}", device);
    }
    Ok(())
}
