use codecrafters_shell::parser::{CommandLine, parse};

#[test]
fn blank_returns_none() {
    assert!(parse(" ").is_none())
}

#[test]
fn parser_splits_words() {
    let input = "echo a   b";
    let CommandLine { cmd, args } = parse(input).unwrap();
    assert_eq!(cmd, "echo");
    assert_eq!(args, ["a", "b"]);
}
