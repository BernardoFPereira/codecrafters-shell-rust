use std::env::*;
use std::fs::*;
use std::path::*;
use std::process::{exit, Command};

#[derive(Debug)]
pub enum CommandType {
    Echo,
    Exit,
    Type,
    Execute(String),
}
impl CommandType {
    pub fn run(&self, cmd_args: String) {
        match self {
            CommandType::Echo => command_echo(cmd_args),
            CommandType::Exit => command_exit(cmd_args),
            CommandType::Type => command_type(cmd_args),
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
    // println!("{cmd} {cmd_args}");
    match find_executable_in_path(cmd.clone().trim().to_string()) {
        Ok(dir) => {
            //run executable
            let executable_path = format!("{}{}{}", dir.display(), MAIN_SEPARATOR, cmd);
            if let Ok(_) = File::open(&executable_path) {
                Command::new(executable_path)
                    .env("PATH", "/bin")
                    .arg(cmd_args)
                    .spawn()
                    .expect("Something went wrong");
            }
        }
        Err(error) => {
            // didn't find executable in PATH
            println!("{}!", error);
        }
    }
}

fn find_executable_in_path(executable: String) -> Result<PathBuf, String> {
    if let Some(var) = var_os("PATH") {
        let target = format!("{}{}", MAIN_SEPARATOR, executable);
        let mut path_dirs = split_paths(&var);
        if let Some(dir) =
            path_dirs.find(|path| metadata(format!("{}{}", path.display(), target)).is_ok())
        {
            return Ok(dir);
            // println!("{} is {}{}", executable, dir.display(), target);
        }
    }
    return Err(format!("{executable}: not found"));
}
