
#[derive(Debug, PartialEq, Eq)]
pub enum RunCommand {
    One,
    Two,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Commands {
    Help,
    Exit,
    Unknown,
    Execute(RunCommand),
}

pub fn read_terminal() -> String {
    let mut input_value: String = String::new();
    std::io::stdin().read_line(&mut input_value).expect("Failed to read line");
    input_value.trim().to_string()
}

pub fn parse_command(input: &str) -> Commands {
    match input {
        "exit" => Commands::Exit,
        "help" => Commands::Help,
        "run one" => Commands::Execute(RunCommand::One),
        "run two" => Commands::Execute(RunCommand::Two),
        _ => Commands::Unknown,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_import() {
        //test read_terminal withour waiting for input
        assert_eq!("", "");
    }

}
