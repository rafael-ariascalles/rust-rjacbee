use authentication::greet_user;
use authentication::reverse_user_name;

fn main() {
    let name: String = "Alice".to_string();
    
    println!("{}",greet_user(&name));
    println!("{}",reverse_user_name(&name));
}
