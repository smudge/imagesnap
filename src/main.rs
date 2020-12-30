extern crate getopts;

use getopts::Options;
use std::env::args;

fn print_usage(program: &String, opts: Options) {
    println!("{} v{}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
    println!("  {}\n", env!("CARGO_PKG_DESCRIPTION"));
    let brief = format!("Usage:\n  {} [<options>] [OUTPUT.(jpg|png|tif)]", program);
    print!("{}", opts.usage(&brief));
}

fn main() {
    let args: Vec<String> = args().collect();

    let mut opts = Options::new();
    opts.optflag("l", "list", "List available capture devices");
    opts.optopt("d", "device", "Use specific capture device", "NAME");
    opts.optflag("h", "help", "This help message");

    print_usage(&args[0], opts);
}
