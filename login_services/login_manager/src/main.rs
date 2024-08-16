use clap::{Parser, Subcommand};
use authentication::get_users;

#[derive(Parser, Debug)]
#[command()]
struct Args {
    #[command(subcommand)]
    command: Option<Command>
}

#[derive(Subcommand, Debug)]
enum Command {
    /// List the information of the users
    List,
    ListAll,
}

fn list_users() {
    println!("{:<20}{:<20}\n", "Name", "Role");
    println!("{:-<40}","");

    let users = get_users();
    users
        .iter()
        .for_each(|(_, user)|{
            println!("{:<20}{:<20?}", user.name, user.role);
        });
}

fn main() {
    let cli: Args = Args::parse();
    match cli.command {
        Some(Command::List) => {
            list_users();
        },
        Some(Command::ListAll) => {
            let lines: String = "-".repeat(100).to_string();
            println!("{}",lines);
            println!("the list of all users is 1");
            println!("{}",lines);
        },
        None => {
            print!("run --help for more information");
        }

    }
}
