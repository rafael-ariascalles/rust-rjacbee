use authentication::login_users;
use authentication::read_line;
use authentication::LoginAction;


fn main() {
    let mut  tries: i32 = 0;
    loop {
        println!("Enter your user:");
        let user: String = read_line();
        println!("Enter your password:");
        let password: String = read_line();

        match login_users(&user, &password) {
            Some(LoginAction::Role(role)) => {
                match role {
                    authentication::LoginRole::Admin => {
                        println!("Welcome Admin");
                    },
                    authentication::LoginRole::User => {
                        println!("Welcome User");
                    }
                }
                break;
            },
            Some(LoginAction::Denied) => {
                println!("Invalid user or password");
                println!("{}", "-".repeat(100));
                tries += 1;
                if tries > 3 {
                    println!("Too many tries");
                    break;
                }
            },
            None => {
                println!("{}","$".repeat(100));
                println!("New User, proceed to register");
                println!("{}","$".repeat(100));
                break;
            }
        }
    }
}
