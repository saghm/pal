extern crate rl_sys;
extern crate lalrpop_util;
#[macro_use] extern crate stepper;
extern crate unicode_xid;

#[macro_use]
mod macros;

mod ast;
mod error;
mod eval;
mod grammar;
mod parser;
mod token;
mod state;

use std::fs::{File, OpenOptions};
use std::io::{self, Read, Write};
use std::path::Path;

use parser::{parse_program, parse_stmt};
use state::State;

use rl_sys::readline;
use rl_sys::history::{histfile, listmgmt};

pub fn run_file(file_name: &str) {
    let mut file = File::open(file_name).expect("Unable to open file");
    let mut program_str = String::new();

    file.read_to_string(&mut program_str).expect("Unable to read file");
    let program = parse_program(&program_str).unwrap();
    let mut state = State::new();

    for stmt in program {
        stmt.eval(&mut state).unwrap();
    }
}

pub fn repl() {
    let mut state = State::new();
    let mut stderr = io::stderr();

    // Create history file if it doesn't already exist.
    OpenOptions::new().write(true).create(true).truncate(false).open(".history").expect("Unable to create history file");

    // Read in history from the file.
    histfile::read(Some(Path::new(".history"))).expect("Unable to read history file");

    while let Some(input) = readline::readline(">> ").unwrap() {
        if input.is_empty() {
            continue;
        }

        // Add input to both temporary and permanent history.
        listmgmt::add(&input).unwrap();
        let _ = histfile::write(Some(Path::new(".history")));

        match parse_stmt(&input) {
            Ok(stmt) => match stmt.eval(&mut state) {
                Ok(_) => (),
                Err(e) => writeln!(stderr, "{}", e).unwrap(),
            },
            Err(_) => println!("Sorry! That's an invalid statement"),
        };

    }

    println!("");
}
