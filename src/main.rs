extern crate getopts;

use getopts::Options;
use imagesnap::Camera;
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
    print!("Usage:\n  {} [<options>] [OUTPUT.(jpg|png|tif)]", program);
    print!("{}", opts.usage(""));
    std::process::exit(code);
}

fn handle_args(program: &String, opts: Options, matches: getopts::Matches) {
    match (
        matches.free.get(0).map(|s| s.to_owned()),
        matches.free.get(1),
        matches.opt_present("l"),
        matches.opt_present("h"),
        !matches.opt_present("q"),
        matches.opt_str("w").map(|s| s.parse()).transpose(),
        matches.opt_str("d"),
    ) {
        (filename, None, false, false, verbose, Ok(warmup), device) => {
            Camera::new(device, verbose, warmup)
                .snap(filename.unwrap_or("snapshot.jpg".to_string()))
        }
        (None, None, true, false, _, Ok(_), _) => Camera::list_devices(),
        (None, None, false, true, _, Ok(_), _) => Ok(print_usage(&program, opts, 0)),
        (_, _, _, _, _, _, _) => Ok(print_usage(&program, opts, 1)),
    }
    .unwrap();
}
