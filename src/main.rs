use codecrafters_shell::builtin::{Builtin, Flow};
use codecrafters_shell::error::ShellError;
use codecrafters_shell::parser::{CommandLine, parse};
use codecrafters_shell::path::find_in_path;

use std::io::{self, Write};
use std::process::Command;

fn run_external(cmd: &str, args: &[String]) -> Result<(), ShellError> {
    find_in_path(cmd).ok_or_else(|| ShellError::NotFound(cmd.to_string()))?;
    Command::new(cmd).args(args).status()?;
    Ok(())
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

        let Some(line) = parse(&input) else { continue };
        let CommandLine { cmd, args } = line;

        match cmd.parse::<Builtin>() {
            Ok(builtin_cmd) => match builtin_cmd.run(&args) {
                Ok(Flow::Continue) => {}
                Ok(Flow::Exit(code)) => std::process::exit(code),
                Err(e) => eprintln!("{e}"),
            },
            Err(_) => {
                if let Err(e) = run_external(&cmd, &args) {
                    eprintln!("{e}");
                }
            }
        }
    }
}
