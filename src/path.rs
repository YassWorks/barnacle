use std::env;
use std::path::PathBuf;

use is_executable::IsExecutable;

pub fn find_in_path(cmd: &str) -> Option<PathBuf> {
    let path = env::var("PATH").ok()?;
    env::split_paths(&path)
        .map(|dir| dir.join(cmd))
        .find(|full_path| full_path.is_executable())
}

pub fn expand_tilde(path: &str) -> Option<PathBuf> {
    let Some(rest) = path.strip_prefix('~') else {
        return Some(PathBuf::from(path));
    };
    let home = env::home_dir()?;
    Some(home.join(rest.trim_start_matches('/')))
}
