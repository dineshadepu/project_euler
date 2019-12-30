use std::env;
use std::io;
use std::io::prelude::*;
use std::process::exit;
use std::fs::read_to_string;

#[macro_use]
extern crate serde_derive; // all
extern crate serde; // all

mod p1;
mod p2;

const USAGE: &str = "
Usage: project_euler <test_number> [ options ]
       project_euler --help
Project euler problems solved in rust lang.
Alternatively, you can run individual cases by running `project_euler foo`,
where `foo` is the problem number such as `p1` of a benchmark. Each benchmark
has its own options and modes, so try `project_euler foo --help`.
Benchmarks:
  - p1 : problem 1
  - p2 : problem 2
";

fn usage() -> ! {
    let _ = writeln!(&mut io::stderr(), "{}", USAGE);
    exit(1);
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        usage();
    }

    let bench_name = &args[1];
    match &bench_name[..] {
        "p1" => p1::main(&args[1..]),
        "p2" => p2::main(&args[1..]),
        _ => usage(),
    }
}
