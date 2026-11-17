use crate::error::ShellError;
use crate::path::{expand_tilde, find_in_path};
use std::env;
use std::str::FromStr;

pub enum Flow {
    Continue,
    Exit(i32),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Builtin {
    Echo,
    Exit,
    Type,
    Pwd,
    Cd,
}

impl FromStr for Builtin {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "echo" => Self::Echo,
            "exit" => Self::Exit,
            "type" => Self::Type,
            "pwd" => Self::Pwd,
            "cd" => Self::Cd,
            _ => return Err(()),
        })
    }
}

impl Builtin {
    pub fn name(self) -> &'static str {
        match self {
            Self::Echo => "echo",
            Self::Exit => "exit",
            Self::Type => "type",
            Self::Pwd => "pwd",
            Self::Cd => "cd",
        }
    }

    pub fn run(self, args: &[String]) -> Result<Flow, ShellError> {
        match self {
            Self::Echo => {
                println!("{}", args.join(" "));
                Ok(Flow::Continue)
            }
            Self::Exit => Ok(Flow::Exit(
                args.first().and_then(|s| s.parse().ok()).unwrap_or(0),
            )),
            Self::Type => {
                if let Some(arg) = args.first() {
                    println!("{}", check_type(arg));
                }
                Ok(Flow::Continue)
            }
            Self::Pwd => {
                println!("{}", env::current_dir()?.display());
                Ok(Flow::Continue)
            }
            Self::Cd => {
                let path = match args.first() {
                    Some(p) => p,
                    None => "~",
                };
                let expanded_path = expand_tilde(path).ok_or(ShellError::HomeNotSet)?;
                env::set_current_dir(expanded_path)
                    .map_err(|_| ShellError::NoSuchDirectory(path.to_string()))?;
                Ok(Flow::Continue)
            }
        }
    }
}

fn check_type(cmd: &str) -> String {
    if cmd.parse::<Builtin>().is_ok() {
        return format!("{cmd} is a shell builtin");
    }
    match find_in_path(cmd) {
        Some(full_path) => format!("{cmd} is {}", full_path.display()),
        None => format!("{cmd}: not found"),
    }
}
