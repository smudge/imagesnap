extern crate getopts;

use getopts::Options;
use imagesnap::Snap;
use std::env::args;

fn main() {
    let args: Vec<String> = args().collect();

    let mut opts = Options::new();
    opts.optflag("l", "list", "List available capture devices");
    opts.optopt("d", "device", "Use specific capture device", "NAME");
    opts.optflag("h", "help", "This help message");

    match opts.parse(&args[1..]) {
        Ok(m) => handle_args(&args[0], opts, m),
        Err(_) => print_usage(&args[0], opts),
    };
}

fn print_usage(program: &String, opts: Options) {
    println!("{} v{}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
    println!("  {}\n", env!("CARGO_PKG_DESCRIPTION"));
    let brief = format!("Usage:\n  {} [<options>] [OUTPUT.(jpg|png|tif)]", program);
    print!("{}", opts.usage(&brief));
}

fn handle_args(program: &String, opts: Options, matches: getopts::Matches) {
    if matches.opt_present("h") {
        print_usage(&program, opts);
        return;
    }
    if matches.opt_present("l") {
        Snap::list_devices();
        return;
    }
    let device = if matches.opt_present("d") {
        matches.opt_str("d").unwrap()
    } else {
        "DEFAULT DEVICE".to_string()
    };
    let filename = if matches.free.is_empty() {
        "snapshot.jpg".to_string()
    } else {
        matches.free[0].clone()
    };

    Snap::new(device, filename).create().unwrap()
}
