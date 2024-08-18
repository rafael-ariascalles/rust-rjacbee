use std::sync::RwLock;
use once_cell::sync::Lazy;
use std::collections::HashMap;

static USERS: Lazy<RwLock<HashMap<String,String>>> = Lazy::new(|| RwLock::new(create_users()));

fn create_users() ->HashMap<String,String> {
    let mut users = HashMap::new();
    users.insert("Alice".to_string(), "Active".to_string());
    users.insert("Bob".to_string(), "Inactive".to_string());
    users
}

fn read_line() -> String {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

fn main() {
    std::thread::spawn(|| {
        loop {
            println!("Current users: {:?}", USERS.read().unwrap());
            std::thread::sleep(std::time::Duration::from_secs(5));
        }
    });

    loop {
        println!("Enter a new user or 'q' to quit:");
        let input: String = read_line();

        if input == "quit" {
            break;
        }else {
            let mut lock = USERS.write().unwrap();
            lock.insert(input, "Active".to_string());
            println!("User added!");
        }
    }
}
