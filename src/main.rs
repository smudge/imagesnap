extern crate getopts;

use getopts::Options;
use imagesnap::Camera;
use std::env::args;

#[derive(Debug)]
enum Output {
    Help(Option<String>),
    Msg(String),
    Success,
}

impl From<String> for Output {
    fn from(err: String) -> Output {
        Output::Msg(err)
    }
}

fn main() -> Result<(), Output> {
    let args: Vec<String> = args().collect();

    let mut opts = Options::new();
    opts.optflag("q", "quiet", "Do not output any text");
    opts.optopt("w", "warmup", "Warm up camera for x.xx seconds", "x.xx");
    opts.optflag("l", "list", "List available capture devices");
    opts.optopt("d", "device", "Use specific capture device", "NAME");
    opts.optflag("h", "help", "This help message");

    match opts
        .parse(&args[1..])
        .map_or_else(|m| usage_err(&m.to_string()), |m| run(m))
    {
        Err(Output::Help(val)) => {
            print_usage(&args[0], &opts);
            match val {
                Some(m) => Err(Output::Help(Some(m))),
                None => Ok(()),
            }
        }
        Err(other) => Err(other),
        Ok(_) => Ok(()),
    }
}

fn print_usage(program: &String, opts: &Options) {
    println!("{} v{}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
    println!("  {}\n", env!("CARGO_PKG_DESCRIPTION"));
    print!("Usage:\n  {} [<options>] [OUTPUT.(jpg|png|tif)]", program);
    println!("{}", opts.usage(""));
}

fn run(matches: getopts::Matches) -> Result<Output, Output> {
    match (
        matches.free.get(0).map(|s| s.to_owned()),
        matches.free.get(1),
        matches.opt_present("l"),
        matches.opt_present("h"),
        !matches.opt_present("q"),
        matches.opt_str("w").map(|s| s.parse()).transpose(),
        matches.opt_str("d"),
    ) {
        (maybe_file, None, false, false, verbose, Ok(warmup), device) => success(
            Camera::new(device, verbose, warmup)
                .snap(maybe_file.unwrap_or("snapshot.jpg".to_string()))?,
        ),
        (None, None, true, false, _, Ok(_), _) => success(Camera::list_devices()?),
        (None, None, false, true, _, Ok(_), _) => usage_help(),
        (_, None, false, false, _, Err(_), _) => usage_err("Failed to parse warmup!"),
        (_, _, _, _, _, _, _) => usage_err("Invalid combination of arguments."),
    }
}

fn success(_: ()) -> Result<Output, Output> {
    Ok(Output::Success)
}

fn usage_help() -> Result<Output, Output> {
    Err(Output::Help(None))
}

fn usage_err(value: &str) -> Result<Output, Output> {
    Err(Output::Help(Some(value.to_string())))
}
