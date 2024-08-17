use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use std::path::Path;
use std::fs;


#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub enum LoginRole {
    Admin,
    User,
}

pub enum LoginAction {
    Role(LoginRole),
    Denied,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub username: String,
    pub password: String,
    pub role: LoginRole,
}

impl User {
    pub fn new(username: &str, password: &str, user_role: LoginRole) -> User {
        User {
            username: username.to_string(),
            password: hash_password(password),
            role: user_role,
        }
    }
}

pub fn hash_password(password: &str) -> String {
    use sha2::{Sha256, Digest};
    let mut hasher = Sha256::new();
    hasher.update(password);
    // hexadecimal representation of a Numer :X 
    format!("{:X}", hasher.finalize())
}

pub fn get_default_users() -> HashMap<String,User> {
    let mut users: HashMap<String, User> = HashMap::new();
    users.insert("admin".to_string(), User::new("admin", "admin", LoginRole::Admin));
    users.insert( "moni".to_string(), User::new("moni", "moni", LoginRole::User));
    users
}


pub fn get_users() -> HashMap<String,User> {
    let users_path: &Path = Path::new("users.json");
    if users_path.exists() {
        let users_json: String = fs::read_to_string(users_path).expect("Unable to read file");
        let users: HashMap<String, User> = serde_json::from_str(&users_json).unwrap();
        users
    }else {
        let users: HashMap<String, User> = get_default_users();
        let users_json: String = serde_json::to_string(&users).unwrap();
        fs::write(users_path, users_json).expect("Unable to write file");
        users
    }
}

pub fn save_users(users: HashMap<String, User>) {
    let users_path: &Path = Path::new("users.json");
    let users_json: String = serde_json::to_string(&users).unwrap();
    fs::write(users_path, users_json).expect("Unable to write file");

    
}

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
    let password: String = hash_password(password);

    let mut result: Option<LoginAction> = Some(LoginAction::Denied);
    
    let users:HashMap<String, User>  = get_users();
    if let Some(userdb) = users.get(&user) {
        if userdb.password == password {
            result = Some(LoginAction::Role(userdb.role.clone()));
        } else {
            result = Some(LoginAction::Denied);
        }
    }
    result
}

pub fn login_users_fixed(user : &str, password : &str) -> Option<LoginAction> {
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


