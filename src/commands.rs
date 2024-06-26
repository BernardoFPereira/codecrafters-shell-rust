use std::env::*;
use std::fs::*;
use std::path::*;
use std::process::{exit, Command, Stdio};

#[derive(Clone, Debug)]
pub enum CommandType {
    Echo,
    Exit,
    Type,
    Help,
    Pwd,
    Cd,
    Execute(String),
}
impl CommandType {
    pub fn run(&self, cmd_args: String) {
        match self {
            CommandType::Echo => command_echo(cmd_args),
            CommandType::Exit => command_exit(cmd_args),
            CommandType::Type => command_type(cmd_args),
            CommandType::Help => command_help(),
            CommandType::Pwd => command_print_working_directory(),
            CommandType::Cd => command_change_working_directory(cmd_args),
            CommandType::Execute(cmd) => command_execute(cmd.to_owned(), cmd_args),
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
        "help" => true,
        "cd" => true,
        "pwd" => true,
        _ => false,
    };

    if is_builtin {
        println!("{} is a shell builtin", cmd_args);
        return;
    } else {
        match find_executable_in_path(cmd_args.to_string()) {
            Ok(dir) => println!(
                "{} is {}{}{}",
                cmd_args,
                dir.display(),
                MAIN_SEPARATOR,
                cmd_args
            ),
            Err(e) => println!("{e}"),
        }
    }
}

pub fn command_execute(cmd: String, cmd_args: String) {
    match find_executable_in_path(cmd.clone().trim().to_string()) {
        Ok(dir) => {
            let executable_path = format!("{}{}{}", dir.display(), MAIN_SEPARATOR, cmd);
            let command = Command::new(executable_path)
                .arg(cmd_args.trim().to_string())
                .stdin(Stdio::piped())
                .spawn()
                .expect("Something went wrong");

            command.wait_with_output().expect("failed to wait");
        }
        Err(error) => {
            println!("{}", error);
        }
    }
}

pub fn command_print_working_directory() {
    if let Ok(path) = current_dir() {
        println!("{}", path.display())
    }
}

pub fn command_change_working_directory(path: String) {
    let path_to_go = if path.trim() == "~" {
        // Set path to HOME environment variable
        let home_path = env!("HOME");
        // let target_path = format!("{}{}", home_path)
        Path::new(home_path.trim())
    } else {
        Path::new(path.trim())
    };
    if let Ok(data) = metadata(path_to_go) {
        if data.is_dir() {
            if set_current_dir(path_to_go).is_ok() {
                return;
            }
        }
        if data.is_file() {
            println!("Error! Can't cd into a file!");
            return;
        }
    }
    println!("{}: No such file or directory", path.trim());
}

pub fn command_help() {
    println!("welcome to russh! Rust Simple Shell");
}

fn find_executable_in_path(executable: String) -> Result<PathBuf, String> {
    if let Some(var) = var_os("PATH") {
        let target = format!("{}{}", MAIN_SEPARATOR, executable);
        let mut path_dirs = split_paths(&var);
        if let Some(dir) =
            path_dirs.find(|path| metadata(format!("{}{}", path.display(), target)).is_ok())
        {
            return Ok(dir);
        }
    }
    return Err(format!("{executable}: not found"));
}
