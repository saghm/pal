extern crate pal;

use std::env;

use pal::{repl, run_file};

fn main() {
    let mut args = env::args();
    let _ = args.next();

    match args.next() {
        Some(arg) => run_file(&arg).unwrap(),
        None => repl(),
    }
}
