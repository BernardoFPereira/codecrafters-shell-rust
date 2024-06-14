#[allow(unused_imports)]
mod commands;

use commands::*;
use std::io::{self, Write};

#[derive(Debug)]
enum Command {
    Echo,
    Exit,
    Type,
    // None,
}

fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    // println!("Logs from your program will appear here!");

    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        let stdin = io::stdin();
        let mut input = String::new();
        stdin.read_line(&mut input).unwrap();

        if input.trim() == "" {
            continue;
        }

        let (command, args) = parse_input(&input);

        if let Some(command) = command {
            match command {
                Command::Exit => command_exit(args),
                Command::Echo => command_echo(args),
                Command::Type => command_type(args),
            }
        } else {
            println!("Command not found: {}", &input.trim());
        }
    }
}

fn parse_input(input: &String) -> (Option<Command>, String) {
    let (command, args) = input
        .split_once(' ')
        .unwrap_or_else(|| (input.as_str(), ""));

    let command_type = match command.trim() {
        "exit" => Some(Command::Exit),
        "echo" => Some(Command::Echo),
        "type" => Some(Command::Type),
        _ => None,
    };

    (command_type, args.to_string())
}
