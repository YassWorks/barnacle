pub struct CommandLine {
    pub cmd: String,
    pub args: Vec<String>,
}

pub fn parse(input: &str) -> Option<CommandLine> {
    let mut current = String::new();
    let mut words = Vec::new();
    let mut enclosed = false;
    for c in input.chars() {
        if c == '\'' {
            enclosed = !enclosed;
            continue;
        }
        if enclosed || !c.is_whitespace() {
            current.push(c);
        } else if !current.is_empty() {
            words.push(std::mem::take(&mut current));
        }
    }
    if !current.is_empty() {
        words.push(current);
    }
    if words.is_empty() {
        return None;
    }
    Some(CommandLine {
        cmd: words.remove(0),
        args: words,
    })
}
