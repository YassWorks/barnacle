use codecrafters_shell::builtin::Builtin;

#[test]
fn name_and_parse_agree() {
    let cmd = "echo";
    assert_eq!(cmd.parse::<Builtin>().unwrap().name(), cmd);
}
