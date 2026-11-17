pub struct CommandLine {
    pub cmd: String,
    pub args: Vec<String>,
}

pub fn parse(input: &str) -> Option<CommandLine> {
    let mut words = input.split_whitespace().map(String::from);
    let cmd = words.next()?;
    let args = words.collect();
    Some(CommandLine { cmd, args })
}
