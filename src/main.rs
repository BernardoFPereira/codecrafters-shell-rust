#[allow(unused_imports)]
use std::io::{self, Write};
use std::process::exit;

fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    // println!("Logs from your program will appear here!");

    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        let stdin = io::stdin();
        let mut input = String::new();
        stdin.read_line(&mut input).unwrap();

        let (command, args) = parse_input(input);

        match command.as_str().trim() {
            "exit" => {
                match args.as_str().trim() {
                    "0" => exit(0),
                    _ => {}
                };
            }
            _ => {
                println!("{}: command not found", command.trim());
            }
        }
    }
}

fn parse_input(input: String) -> (String, String) {
    let (command, args) = input
        .split_once(' ')
        .unwrap_or_else(|| (input.as_str(), ""));

    (command.to_string(), args.to_string())
}
