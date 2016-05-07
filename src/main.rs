extern crate rl_sys;

mod ast;
mod error;
mod eval;
mod grammar;
mod state;

use grammar::parse_Stmt;
use state::State;

use rl_sys::readline;
use rl_sys::history::listmgmt;

fn main() {
    let mut state = State::new();

    loop {
        let input = match readline::readline(">> ").unwrap() {
            Some(s) => s,
            None => break,
        };

        if input.is_empty() {
            continue;
        }

        match parse_Stmt(&input) {
            Ok(stmt) => match stmt.eval(&mut state) {
                Ok(_) => (),
                Err(e) => println!("{}", e),
            },
            Err(_) => println!("Sorry! That's an invalid statement"),
        };

        listmgmt::add(&input).unwrap();
    }

    println!("");
}
