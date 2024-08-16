use std::collections::HashMap;

#[derive(Debug)]
pub struct User {
    pub name: String,
    pub password: String,
    pub role: UserRole,
}

impl User {
    pub fn new(name: &str, password: &str, user_role: UserRole) -> User {
        User {
            name: name.to_string(),
            password: password.to_string(),
            role: user_role,
        }
    }
}
#[derive(Debug)]
pub enum UserRole {
    Admin,
    User,
}

pub fn get_users() -> HashMap<String,User> {
    let mut users: HashMap<String, User> = HashMap::new();
    users.insert("admin".to_string(), User::new("admin", "admin", UserRole::Admin));
    users.insert( "moni".to_string(), User::new("moni", "moni", UserRole::User));
    users
}

pub fn save_user(user: HashMap<String, User>) {

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let users = get_users();
        assert_eq!(users.len(), 2);
    }
}
