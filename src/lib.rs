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
mod stream;
mod token;
mod state;

use std::fs::{File, OpenOptions};
use std::io::{self, Read, Write};
use std::path::Path;
use std::sync::Arc;
use std::thread;

use error::Result;
use parser::{parse_program, parse_stmt};
use state::State;

pub use stream::{Event, Stream};

use rl_sys::readline;
use rl_sys::history::{histfile, listmgmt};

pub fn run_file(file_name: &str) -> Result<()> {
    let mut file = File::open(file_name).expect("Unable to open file");
    let mut program_str = String::new();

    file.read_to_string(&mut program_str).expect("Unable to read file");
    run_program(&program_str)
}

pub fn run_program(program_str: &str) -> Result<()> {
    let program = parse_program(&program_str).unwrap();
    let mut state = State::new();

    for stmt in program {
        try!(stmt.eval(&mut state, None));
    }

    Ok(())
}

pub fn run_program_with_stream(program_str: &str) -> Arc<Stream> {
    let program = parse_program(&program_str).unwrap();
    let stream = Arc::new(Stream::new());
    let cloned_stream = stream.clone();
    let mut state = State::new();

    thread::spawn(move || {
        for stmt in program {
            if let Err(e) = stmt.eval(&mut state, Some(cloned_stream.clone())) {
                cloned_stream.write_output(&format!("{}", e));
                break;
            }
        }

        cloned_stream.finished();
    });

    stream
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
            Ok(stmt) => match stmt.eval(&mut state, None) {
                Ok(_) => (),
                Err(e) => writeln!(stderr, "{}", e).unwrap(),
            },
            Err(_) => println!("Sorry! That's an invalid statement"),
        };

    }

    println!("");
}
