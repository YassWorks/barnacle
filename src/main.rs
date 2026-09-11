#[allow(unused_imports)]
use std::env;
use std::io::{self, Write};
use std::path::PathBuf;
use std::process::Command;

use is_executable::IsExecutable;

const BUILTINS: [&str; 4] = ["echo", "exit", "type", "pwd"];

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
    return find_in_path(cmd) != None;
}

fn run_exec_if_possible(cmd: &str, args: &str) -> bool {
    if !is_runnable(cmd) {
        return false;
    }

    Command::new(cmd)
        .args(args.split_whitespace())
        .status()
        .unwrap();

    return true;
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

        if cmd == "" {
            continue;
        } else if cmd == "exit" {
            break;
        } else if cmd == "echo" {
            println!("{}", args);
        } else if cmd == "type" {
            println!("{}", check_type(args));
        } else if cmd == "pwd" {    
            println!("{}", env::current_dir().unwrap().display());
        } else if run_exec_if_possible(cmd, args) {
            continue;
        } else {
            println!("{}: command not found", cmd);
        }
    }
}
