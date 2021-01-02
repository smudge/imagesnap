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
    let verbose = !matches.opt_present("q");
    let warmup = match matches.opt_str("w") {
        Some(val) => val.parse().unwrap(),
        None => 0.0,
    };

    match (
        &matches.free[..],
        matches.opt_present("l"),
        matches.opt_present("h"),
    ) {
        ([], false, false) => Snap::new(device, "snapshot.jpg".to_string(), verbose, warmup)
            .create()
            .unwrap(),
        ([filename], false, false) => Snap::new(device, filename.clone(), verbose, warmup)
            .create()
            .unwrap(),
        ([], true, false) => Snap::list_devices(),
        ([], false, true) => print_usage(&program, opts, 0),
        (_, _, _) => print_usage(&program, opts, 1),
    }
}
