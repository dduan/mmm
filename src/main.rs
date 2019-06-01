mod mmm;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: {} PATH", args[0]);
        return;
    }

    mmm::run_mmm(&args[1])
}
