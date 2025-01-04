#[allow(unused_imports)]
use std::io::{self, Write};

fn main() {
    loop {
        // Print shell prompt
        print!("$ ");
        io::stdout().flush().unwrap();

        // Wait for user input
        let stdin = io::stdin();
        let mut input = String::new();
        stdin.read_line(&mut input).unwrap();

        // switch case for input 
        match input.trim() {
            "exit 0" => break,
            &_ => {
                println!("{}: command not found", input.trim());
            }
        }
    }

}
