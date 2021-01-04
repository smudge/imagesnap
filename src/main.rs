extern crate getopts;

use getopts::Options;
use imagesnap::Camera;
use std::env::args;

fn main() -> Result<(), String> {
    let args: Vec<String> = args().collect();

    let mut opts = Options::new();
    opts.optflag("q", "quiet", "Do not output any text");
    opts.optopt("w", "warmup", "Warm up camera for x.xx seconds", "x.xx");
    opts.optflag("l", "list", "List available capture devices");
    opts.optopt("d", "device", "Use specific capture device", "NAME");
    opts.optflag("h", "help", "This help message");

    opts.parse(&args[1..])
        .map_or_else(
            |_| err("Failed to parse args!"),
            |m| run(&args[0], &opts, m),
        )
        .map_err(|e| {
            print_usage(&args[0], &opts);
            e
        })
}

fn print_usage(program: &String, opts: &Options) {
    println!("{} v{}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
    println!("  {}\n", env!("CARGO_PKG_DESCRIPTION"));
    print!("Usage:\n  {} [<options>] [OUTPUT.(jpg|png|tif)]", program);
    println!("{}", opts.usage(""));
}

fn run(program: &String, opts: &Options, matches: getopts::Matches) -> Result<(), String> {
    match (
        matches.free.get(0).map(|s| s.to_owned()),
        matches.free.get(1),
        matches.opt_present("l"),
        matches.opt_present("h"),
        !matches.opt_present("q"),
        matches.opt_str("w").map(|s| s.parse()).transpose(),
        matches.opt_str("d"),
    ) {
        (maybe_file, None, false, false, verbose, Ok(warmup), device) => {
            Camera::new(device, verbose, warmup)
                .snap(maybe_file.unwrap_or("snapshot.jpg".to_string()))
        }
        (None, None, true, false, _, Ok(_), _) => Camera::list_devices(),
        (None, None, false, true, _, Ok(_), _) => Ok(print_usage(program, opts)),
        (_, None, false, false, _, Err(_), _) => err("Failed to parse warmup!"),
        (_, _, _, _, _, _, _) => err("Invalid combination of arguments."),
    }
}

fn err(value: &str) -> Result<(), String> {
    Err(value.to_string())
}
