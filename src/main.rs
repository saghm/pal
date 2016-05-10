extern crate rl_sys;
extern crate lalrpop_util;
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

use std::fs::OpenOptions;
use std::io::{self, Write};
use std::path::Path;

use parser::parse_stmt;
use state::State;

use rl_sys::readline;
use rl_sys::history::{histfile, listmgmt};

fn main() {
    let mut state = State::new();
    let mut stderr = io::stderr();

    OpenOptions::new().write(true).create(true).truncate(false).open(".history").expect("Unable to create history file");

    match histfile::read(Some(Path::new(".history"))) {
        Ok(_) => (),
        Err(_) => panic!("Unable to read history file"),
    };

    while let Some(input) = readline::readline(">> ").unwrap() {
        if input.is_empty() {
            continue;
        }

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
