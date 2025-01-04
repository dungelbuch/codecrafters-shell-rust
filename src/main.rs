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

        // Check for echo command 
        if input.starts_with("echo ") {
            let echo_cmd = input.trim_start_matches("echo ");
            println!("{}", echo_cmd.trim());
        } else {
            // Cases for whole command
            match input.trim() {
                "exit" | "exit 0" => break,

                &_ => {
                    println!("{}: command not found", input.trim());
                }
            }
        }
    }

}
