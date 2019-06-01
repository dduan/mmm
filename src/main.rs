mod mmm;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: {} PATH", args[0]);
        return;
    }

    if args[1] == "-v" || args[1] == "--version" {
        mmm::print_version();
        return;
    }

    mmm::run_mmm(&args[1])
}
