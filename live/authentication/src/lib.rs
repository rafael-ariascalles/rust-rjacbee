
pub fn greet_user(name: &str) -> String {
    format!("Hello, {name}!")
}

pub fn reverse_user_name(name: &str) -> String {
    name.chars().rev().collect()
}

pub fn login(user : &str, password : &str) -> bool {
    user.to_lowercase() == "alice" && password == "1234"
}

pub fn login_users(user : &str, password : &str) -> Option<LoginAction> {
    let user: String = user.to_lowercase();
    let mut result: Option<LoginAction> = Some(LoginAction::Denied);
    
    if user != "admin" && user != "alice" {
        return None;
    } 

    if user == "admin" && password == "1234" {
        result = Some(LoginAction::Role(LoginRole::Admin));
    }else if user == "alice" && password == "1234" {
        result = Some(LoginAction::Role(LoginRole::User));
    }

    result
}

#[derive(Debug, PartialEq, Eq)]
pub enum LoginRole {
    Admin,
    User,
}

pub enum LoginAction {
    Role(LoginRole),
    Denied,
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

    #[test]
    fn test_login_users() {
        //assert_eq!(login_users("admin", "1234"), LoginAction::Admin);
        //assert_eq!(login_users("alice", "1234"), LoginAction::User);
        //assert_eq!(login_users("Alice", "1234"), LoginAction::User);
        //assert_eq!(login_users("Alice-not", "12345"), LoginAction::Denied);
    }


}


