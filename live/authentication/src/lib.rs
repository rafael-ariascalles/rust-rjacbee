
pub fn greet_user(name: &str) -> String {
    format!("Hello, {name}!")
}

pub fn reverse_user_name(name: &str) -> String {
    name.chars().rev().collect()
}

pub fn login(user : &str, password : &str) -> bool {
    user.to_lowercase() == "alice" && password == "1234"
}

pub fn read_line() -> String {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Failed to read line");
    input.trim().to_string()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_greet_user() {
        assert_eq!(greet_user("Alice"), "Hello, Alice!");
    }

    #[test]
    fn test_reverse_user_name() {
        assert_eq!(reverse_user_name("Jose"), "esoJ");
    }

    #[test]
    fn test_login() {
        assert!(login("alice", "1234"));
        assert!(login("Alice", "1234"));
        assert!(!login("Alice-not", "12345"));
        assert!(!login("Alice", "12345-not"));
    }


}


