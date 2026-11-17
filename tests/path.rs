use codecrafters_shell::path::{expand_tilde, find_in_path};
use std::path::PathBuf;

#[test]
fn absolute_path_is_untouched() {
    assert_eq!(expand_tilde("/tmp/x"), Some(PathBuf::from("/tmp/x")));
}

#[test]
fn bare_tilde_is_home() {
    assert_eq!(expand_tilde("~"), std::env::home_dir());
}

#[test]
fn missing_command_is_not_found_in_path() {
    assert_eq!(find_in_path("fake-cmd"), None);
}
