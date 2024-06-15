use crate::Command;
use std::env::*;
use std::fs::*;
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
        let env_var = var_os("PATH");
        if let Some(var) = env_var {
            let path_dirs = split_paths(&var);

            for dir in path_dirs {
                // println!("{}", dir.display());

                for entry in dir.read_dir().unwrap() {
                    let file_path = entry.unwrap().path();
                    if file_path.display().to_string().contains(cmd_args) {
                        println!("{} is {}", cmd_args, file_path.display())
                    }
                }
            }
        }
    }
}
