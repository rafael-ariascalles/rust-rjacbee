use std::sync::RwLock;
use once_cell::sync::Lazy;
use std::collections::HashMap;

static USERS: Lazy<RwLock<HashMap<String,i8>>> = Lazy::new(|| RwLock::new(create_users()));

fn create_users() ->HashMap<String,i8> {
    let mut users: HashMap<String, i8> = HashMap::new();
    users.insert("Alice".to_string(), 0);
    users.insert("Bob".to_string(), 0);
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
            std::thread::sleep(std::time::Duration::from_secs(5));
            println!("Current users: {:?}", USERS.read().unwrap());
            
        }
    });

    loop {
        println!("Enter a new user or 'q' to quit:");
        let input: String = read_line();

        if input == "quit" {
            break;
        }else {
            let mut lock = USERS.write().unwrap();
            
            if lock.contains_key(&input) {
                let count: &mut i8 = lock.get_mut(&input).unwrap();
                *count += 1;
                println!("User count updated!");
                continue;
            }

            lock.insert(input, 1);
            println!("User added!");
        }
    }
}
