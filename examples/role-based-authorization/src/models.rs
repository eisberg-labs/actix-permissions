#[derive(Clone, PartialOrd, PartialEq, Copy)]
pub enum Role {
    Administrator, Moderator, User
}

pub struct User {
    pub username: String,
    pub role: Role,
    pub password: String
}

impl User {
    pub fn new(username: &str, role: Role, password: &str) -> Self {
        Self {
            username: username.to_string(),
            role,
            password: password.to_string()
        }
    }

    pub fn list()->Vec<User>{
        vec![
            User::new("admin", Role::Administrator, "1"),
            User::new("moderator", Role::Moderator, "2"),
            User::new("user", Role::User, "3"),
        ]
    }
}
