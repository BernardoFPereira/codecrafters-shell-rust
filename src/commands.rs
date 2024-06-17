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
        if let Some(var) = env_var {
            let path_dirs = split_paths(&var);

            let mut entries = path_dirs
                .flat_map(|dir| dir.read_dir().unwrap())
                .filter(|entry| {
                    let target = format!("{}{}", MAIN_SEPARATOR_STR, cmd_args);
                    if let Ok(entry) = entry.as_ref() {
                        entry.path().display().to_string().ends_with(&target)
                    } else {
                        false
                    }
                });

            if let Some(entry) = entries.next() {
                println!("{} is {}", cmd_args, entry.display());
            } else {
                println!("{}: not found", cmd_args);
            }

            println!("{:?}", entries.next());

            // for dir in path_dirs {
            //     for entry in dir.read_dir().unwrap() {
            //         let mut target = String::from("/");
            //         target.push_str(cmd_args);
            //         let file_path = entry.unwrap().path();
            //         // Finds anything that contains the chars in cmd_args
            //         if file_path.display().to_string().ends_with(target.as_str()) {
            //             println!("{} is {}", cmd_args, file_path.display());
            //             return;
            //         }
            //     }
            // }

            // println!("{}: not found", cmd_args);
        }
    }
}
