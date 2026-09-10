use is_executable::IsExecutable;

#[allow(unused_imports)]
use std::env;
use std::io::{self, Write};

const BUILTINS: [&str; 3] = ["echo", "exit", "type"];

fn check_type(args: &str) -> String {
    let path = env::var("PATH").unwrap();

    if BUILTINS.contains(&args) {
        return format!("{args} is a shell builtin");
    }

    for dir in env::split_paths(&path) {
        let full_path = dir.join(args);
        if full_path.exists() && full_path.is_executable() {
            return format!("{args} is {}", full_path.display());
        }
    }

    format!("{args}: not found")
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

        match cmd {
            "exit" => break,
            "" => continue,
            "echo" => println!("{}", args),
            "type" => println!("{}", check_type(args)),
            _ => println!("{}: command not found", input),
        };
    }
}
