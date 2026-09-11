#[allow(unused_imports)]
use std::env;
use std::io::{self, Write};
use std::path::PathBuf;
use std::process::Command;

use is_executable::IsExecutable;

const BUILTINS: [&str; 5] = ["echo", "exit", "type", "pwd", "cd"];

fn find_in_path(cmd: &str) -> Option<PathBuf> {
    let path = env::var("PATH").unwrap();
    for dir in env::split_paths(&path) {
        let full_path = dir.join(cmd);
        if full_path.exists() && full_path.is_executable() {
            return Some(full_path);
        }
    }
    None
}

fn check_type(args: &str) -> String {
    if BUILTINS.contains(&args) {
        return format!("{args} is a shell builtin");
    }

    match find_in_path(args) {
        Some(full_path) => format!("{args} is {}", full_path.display()),
        None => format!("{args}: not found"),
    }
}

fn is_runnable(cmd: &str) -> bool {
    find_in_path(cmd).is_some()
}

fn run_exec_if_possible(cmd: &str, args: &str) -> bool {
    if !is_runnable(cmd) {
        return false;
    }

    Command::new(cmd)
        .args(args.split_whitespace())
        .status()
        .unwrap();

    true
}

fn get_absolute_path(path: &str) -> String {
    let expanded = match (path.strip_prefix('~'), env::home_dir()) {
        (Some(r), Some(h)) => format!("{}{r}", h.display()),
        _ => path.to_string(),
    };
    return expanded;
}

fn main() {
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        let n = io::stdin().read_line(&mut input).unwrap();
        if n == 0 {
            break;
        }

        let input = input.trim();

        let (cmd, args) = match input.split_once(' ') {
            Some((c, a)) => (c, a),
            None => (input, ""),
        };

        if cmd.is_empty() {
            continue;
        } else if cmd == "exit" {
            break;
        } else if cmd == "echo" {
            println!("{}", args);
        } else if cmd == "type" {
            println!("{}", check_type(args));
        } else if cmd == "pwd" {    
            println!("{}", env::current_dir().unwrap().display());
        } else if cmd == "cd" {
            let absolute_path = get_absolute_path(args);
            if let Err(_) = env::set_current_dir(absolute_path) {
                println!("{cmd}: {args}: No such file or directory")
            }
        } else if run_exec_if_possible(cmd, args) {
            continue;
        } else {
            println!("{}: command not found", cmd);
        }
    }
}
