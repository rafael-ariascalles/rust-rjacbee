use terminal::read_terminal;
use terminal::parse_command;
fn main() {
    loop {
        let input: String = read_terminal();
        match parse_command(&input) {
            terminal::Commands::Exit => {
                println!("Exit");
                break;
            },
            terminal::Commands::Help => {
                println!("Help");
            },
            terminal::Commands::Unknown => {
                println!("Unknown");
            },
            terminal::Commands::Execute(command) => {
                match command {
                    terminal::RunCommand::One => {
                        let v: &str = "1";
                        println!("{}",v.repeat(100));
                    },
                    terminal::RunCommand::Two => {
                        let v: &str = "2";
                        println!("{}",v.repeat(100));
                    }
                }
            },
        }
    }
}
