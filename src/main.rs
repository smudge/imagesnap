extern crate getopts;

use getopts::Options;
use imagesnap::Snap;
use std::env::args;

fn main() {
    let args: Vec<String> = args().collect();

    let mut opts = Options::new();
    opts.optflag("q", "quiet", "Do not output any text");
    opts.optopt("w", "warmup", "Warm up camera for x.xx seconds", "x.xx");
    opts.optflag("l", "list", "List available capture devices");
    opts.optopt("d", "device", "Use specific capture device", "NAME");
    opts.optflag("h", "help", "This help message");

    match opts.parse(&args[1..]) {
        Ok(m) => handle_args(&args[0], opts, m),
        Err(_) => print_usage(&args[0], opts, 1),
    };
}

fn print_usage(program: &String, opts: Options, code: i32) {
    println!("{} v{}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
    println!("  {}\n", env!("CARGO_PKG_DESCRIPTION"));
    opts.usage(&format!(
        "Usage:\n  {} [<options>] [OUTPUT.(jpg|png|tif)]",
        program
    ));
    std::process::exit(code);
}

fn handle_args(program: &String, opts: Options, matches: getopts::Matches) {
    let device = match matches.opt_str("d") {
        Some(val) => val,
        None => Snap::default_device(),
    };
    let filename: Result<Option<String>, ()> = match &matches.free[..] {
        [] => Ok(None),
        [filename] => Ok(Some(filename.clone())),
        _ => Err(()),
    };

    match (
        filename,
        matches.opt_present("l"),
        matches.opt_present("h"),
        !matches.opt_present("q"),
        matches.opt_str("w").map(|s| s.parse().unwrap()),
    ) {
        (Ok(filename), false, false, verbose, warmup) => {
            Snap::new(device, filename, verbose, warmup)
                .create()
                .unwrap()
        }
        (Ok(_), true, false, _, _) => Snap::list_devices(),
        (Ok(_), false, true, _, _) => print_usage(&program, opts, 0),
        (_, _, _, _, _) => print_usage(&program, opts, 1),
    }
}
