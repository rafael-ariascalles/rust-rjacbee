use clap::{Parser, Subcommand};
use authentication::get_users;
use authentication::save_users;
use authentication::LoginRole;
use authentication::User;

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
    /// Add users to the system
    Add {
        /// The username of the user to add
        username: String,
        /// The password assigned to the user
        password: String,
        /// if the user is an admin
        #[arg(long)]
        admin: bool,
    },
    /// Remove users from the system
    Delete {
        /// The username of the user to remove
        username: String,
    }
}

fn add_user(username: String, password: String, admin: bool) {
    let mut users: std::collections::HashMap<String, User> = get_users();

    let role: LoginRole = if admin {
        LoginRole::Admin
    } else {
        LoginRole::User
    };

    let new_user = User::new(&username, &password, role);
    users.insert(username, new_user);
    save_users(users);
}

fn delete_user(username: String) {
    let mut users: std::collections::HashMap<String, User> = get_users();
    if users.contains_key(&username) {
        users.remove(&username);
        save_users(users);
    } else {
        println!("User {} not found", username);
    }
    
}

fn list_users() {
    println!("{:<20}{:<20}\n", "Name", "Role");
    println!("{:-<40}","");

    let users = get_users();
    users
        .iter()
        .for_each(|(_, user)|{
            println!("{:<20}{:<20?}", user.username, user.role);
        });
}

fn main() {
    let cli: Args = Args::parse();


    match cli.command {
        Some(Command::List) => {
            list_users();
        },
        Some(Command::Add {username, password, admin}) => {
            add_user(username, password, admin);
        },
        Some(Command::Delete {username}) => {
            delete_user(username);
        },
        None => {
            print!("run --help for more information");
        }

    }
}
