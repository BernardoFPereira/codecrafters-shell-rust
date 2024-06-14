use crate::Command;
use std::process::exit;

pub fn command_echo(args: String) {
    println!("{}", args.as_str().trim());
}

pub fn command_exit(args: String) {
    let exit_code = args.trim().parse();
    if let Ok(code) = exit_code {
        exit(code);
    }
    exit(0);
}

pub fn command_type(args: String) {
    let args = args.trim();
    let is_builtin = match args {
        "exit" => true,
        "echo" => true,
        "type" => true,
        _ => false,
    };

    if is_builtin {
        println!("{} is a shell builtin", args);
    } else {
        println!("{}: not found", args);
    }
}
