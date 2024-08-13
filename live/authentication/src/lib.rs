
pub fn greet_user(name: &str) -> String {
    format!("Hello, {name}!")
}

pub fn reverse_user_name(name: &str) -> String {
    name.chars().rev().collect()
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


}


