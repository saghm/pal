extern crate pal;

use std::io::{self, Read, Write};
use std::net::TcpListener;

fn main() {
    let mut stderr = io::stderr();
    let mut string = String::new();

    loop {
        let listener = TcpListener::bind("localhost:7777").expect("Unable to open socket");
        let (mut stream, _) = listener.accept().expect("Unable to accept connection");

        if let Err(e) = stream.read_to_string(&mut string) {
            writeln!(stderr, "Error reading string from source: {}", e).unwrap();
            continue;
        }

        if string.is_empty() {
            continue;
        }

        if let Err(e) = pal::run_program(&string) {
            writeln!(stderr, "{}", e).unwrap();
        }

        string.clear();
    }
}
