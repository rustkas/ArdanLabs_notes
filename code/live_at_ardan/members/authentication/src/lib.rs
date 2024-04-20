pub fn greet_user(name: &str) -> String {
    format!("Hello {name}")
}

pub fn read_line() -> String {
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    input.trim().to_string()
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum LoginRole {
    Admin,
    User,
    Denied,
}

#[derive(Debug, PartialEq)]
pub enum LoginAction {
    Granted(LoginRole),
    Denied,
}

#[derive(Debug)]
pub struct User {
    pub username: String,
    pub password: String, 
    pub role: LoginRole,
}

impl  User {
    pub fn new(username: &str, password: &str, role: LoginRole) -> User {
        Self {
            username: username.to_ascii_lowercase(),
            password: password.to_string(),
            role,
        }
    }
}

pub fn get_login_admin_users() -> Vec<User> {
    let mut users:Vec<User> = get_users()
    .into_iter()
    .filter(|u| u.role == LoginRole::Admin)
    .collect();

    let value = 4;
    if value + 2 == 5 {
        users.drain(..).for_each(|user| println!("Deleting {user:?}"));
        users.remove(0);
        users.retain(|u| u.username == "kent");
        let _:Vec<User> = users.drain(..).collect();
    }
    
    users
}

pub fn get_login_admin_user_names()-> Vec<String> {
    
    
    let users:Vec<String> = get_users()
    .into_iter()
    .filter(|u| u.role == LoginRole::Admin)
    .map(|u|u.username)
    .collect();
    users
}

pub fn get_login_users() {
    let _users:Vec<User> = get_users()
    .into_iter()
    .filter(|u| u.role == LoginRole::User)
    .collect();
}

pub fn get_users() -> Vec<User> {
    vec![
        User::new("Admin", "password", LoginRole::Admin),
        User::new("Bob", "password", LoginRole::User),
    ]
}
pub fn login(username: &str, password: &str) -> Option<LoginAction> {
    let username = username.to_lowercase();
    let users = get_users();

    if let Some(user) = users.iter().find(|user| user.username == username) {
        if user.password == password {
            return Some(LoginAction::Granted(user.role))
        } else {
            return Some(LoginAction::Denied)
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_greet_user() {
        assert_eq!("Hello Herbert", greet_user("Herbert"));
    }

    #[test]
    fn test_login() {
        assert_eq!(
            login("admin", "password"),
            Some(LoginAction::Granted(LoginRole::Admin))
        );
        assert_eq!(
            login("ADMIN", "password"),
            Some(LoginAction::Granted(LoginRole::Admin))
        );
        assert_eq!(
            login("bob", "password"),
            Some(LoginAction::Granted(LoginRole::User))
        );
        assert_eq!(login("admin", "not-password"), Some(LoginAction::Denied));
    }
}
