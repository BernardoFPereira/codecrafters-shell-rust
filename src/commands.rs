use crate::Command;
use std::env::*;
use std::fs::*;
use std::path::*;
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
        let target = format!("{}{}", MAIN_SEPARATOR, cmd_args);

        if let Some(var) = env_var {
            let mut path_dirs = split_paths(&var);
            if let Some(dir) =
                path_dirs.find(|path| metadata(format!("{}{}", path.display(), target)).is_ok())
            {
                println!("{} is {}", cmd_args, dir.display());
            } else {
                println!("{}: not found", cmd_args);
            }
        }
    }
}
