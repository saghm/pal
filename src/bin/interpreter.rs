extern crate pal;

use std::env;
use std::io::{self, Read};

use pal::{repl, run_file, run_program};

fn main() {
    let mut args = env::args();
    let _ = args.next();

    match args.next() {
        Some(ref arg) if arg == "-e" => {
            let mut buf = String::new();
            let mut stdin = io::stdin();
            stdin.read_to_string(&mut buf).unwrap();

            run_program(&buf).unwrap();
        },
        Some(arg) => run_file(&arg).unwrap(),
        None => repl(),
    }
}
