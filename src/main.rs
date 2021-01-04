extern crate getopts;

use getopts::Options;
use imagesnap::Camera;
use std::env::args;

type Exit = Result<Output, Output>;

#[derive(Debug)]
struct Output {
    help: bool,
    msg: Option<String>,
}

impl From<String> for Output {
    fn from(err: String) -> Output {
        Output {
            msg: Some(err),
            help: false,
        }
    }
}

fn main() -> Result<(), String> {
    let args: Vec<String> = args().collect();

    let mut opts = Options::new();
    opts.optflag("q", "quiet", "Do not output any text");
    opts.optopt("w", "warmup", "Warm up camera for x seconds [0-10]", "x.x");
    opts.optflag("l", "list", "List available capture devices");
    opts.optopt("d", "device", "Use specific capture device", "NAME");
    opts.optflag("h", "help", "This help message");

    let print_usage = |out: &Output| {
        if out.help {
            print_usage(&args[0], &opts);
        }
    };

    opts.parse(&args[1..])
        .map_or_else(|m| usage_err(&m.to_string()), |m| run(m))
        .or_else(|out| {
            print_usage(&out);
            Err(out.msg.unwrap())
        })
        .and_then(|out| {
            print_usage(&out);
            Ok(())
        })
}

fn print_usage(program: &String, opts: &Options) {
    println!("{} v{}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
    println!("  {}\n", env!("CARGO_PKG_DESCRIPTION"));
    print!("Usage:\n  {} [<options>] [OUTPUT.(jpg|png|tif)]", program);
    println!("{}", opts.usage(""));
}

fn run(matches: getopts::Matches) -> Exit {
    match (
        matches.free.get(0).map(|s| s.to_owned()),
        matches.free.get(1),
        matches.opt_present("l"),
        matches.opt_present("h"),
        !matches.opt_present("q"),
        matches.opt_str("w").map(|s| s.parse()).transpose(),
        matches.opt_str("d"),
    ) {
        (_, None, false, false, _, Ok(Some(w)), _) if w < 0.0 || w > 10.0 => {
            usage_err("Warmup must be between 0 and 10 seconds")
        }
        (maybe_file, None, false, false, verbose, Ok(warmup), device) => handle(
            Camera::new(device, verbose, warmup)
                .snap(maybe_file.unwrap_or("snapshot.jpg".to_string())),
        ),
        (None, None, true, false, _, Ok(None), _) => handle(Camera::list_devices()),
        (None, None, false, true, _, Ok(None), _) => help(),
        (_, None, false, false, _, Err(_), _) => usage_err("Failed to parse warmup!"),
        (_, _, _, _, _, _, _) => usage_err("Invalid combination of arguments."),
    }
}

fn help() -> Exit {
    Ok(Output {
        help: true,
        msg: None,
    })
}

fn handle(result: Result<(), String>) -> Exit {
    if result.is_err() {
        Err(Output {
            help: false,
            msg: Some(result.err().unwrap()),
        })
    } else {
        Ok(Output {
            help: false,
            msg: None,
        })
    }
}

fn usage_err(value: &str) -> Exit {
    Err(Output {
        help: true,
        msg: Some(value.to_string()),
    })
}
