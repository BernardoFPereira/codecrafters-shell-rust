use std::env::*;
use std::fs::*;
use std::path::*;
use std::process::exit;

#[derive(Debug)]
pub enum Command {
    Echo,
    Exit,
    Type,
    // None,
}
impl Command {
    pub fn run(&self, cmd_args: String) {
        match self {
            Command::Echo => command_echo(cmd_args),
            Command::Exit => command_exit(cmd_args),
            Command::Type => command_type(cmd_args),
        }
    }
}

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

pub fn command_type(cmd_args: String) {
    let cmd_args = cmd_args.trim();
    let is_builtin = match cmd_args {
        "exit" => true,
        "echo" => true,
        "type" => true,
        _ => false,
    };

    if is_builtin {
        println!("{} is a shell builtin", cmd_args);
        return;
    } else {
        if let Some(var) = var_os("PATH") {
            let target = format!("{}{}", MAIN_SEPARATOR, cmd_args);
            let mut path_dirs = split_paths(&var);
            match path_dirs.find(|path| metadata(format!("{}{}", path.display(), target)).is_ok()) {
                Some(dir) => println!("{} is {}{}", cmd_args, dir.display(), target),
                None => println!("{}: not found", cmd_args),
            }
        }
    }
}
