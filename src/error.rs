use std::io;

#[derive(Debug, thiserror::Error)]
pub enum ShellError {
    #[error("{0}: command not found")]
    NotFound(String), // carries a command name

    #[error("cd: {0}: No such file or directory")]
    NoSuchDirectory(String), // carries a dir name

    #[error(transparent)]
    Io(#[from] io::Error), // carries an OS error

    #[error("cd: HOME not set")]
    HomeNotSet,
}
