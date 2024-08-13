use authentication::login;
use authentication::read_line;

fn main() {
    let mut  tries: i32 = 0;
    loop {
        println!("Enter your user:");
        let user: String = read_line();
        println!("Enter your password:");
        let password: String = read_line();

        if login(&user, &password) {
            println!("{}", "@".repeat(100));
            println!("Welcome to the system {user}");
            println!("{}", "@".repeat(100));
            break;
        } else {
            println!("Invalid user or password");
            println!("{}", "-".repeat(100));
            tries += 1;
            if tries > 3 {
                println!("Too many tries");
                break;
            }
        }
    }
}
