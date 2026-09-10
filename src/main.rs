#[allow(unused_imports)]
use std::io::{self, Write};

const BUILTINS: [&str; 3] = ["echo", "exit", "type"];

fn check_type(args: &str) -> String {
    if BUILTINS.contains(&args) {
        format!("{args} is a shell builtin")
    } else {
        format!("{args}: not found")
    }
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
