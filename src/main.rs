use std::env::args;

fn print_usage(program: &String) {
    println!("{} v{}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
    println!("  {}\n", env!("CARGO_PKG_DESCRIPTION"));
    println!("usage:\n  {} [-h | --help] [<args>] [<output>]\n", program);
    println!("args:");
    println!("  --help | -h      This help message");
    println!("  --list | -l      List available capture devices");
    println!("  --device | -d    Use specific capture device");
}

fn main() {
    let args: Vec<String> = args().collect();

    print_usage(&args[0]);
}
