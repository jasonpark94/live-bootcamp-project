use crate::domain::{Email, Password};

#[derive(Debug, Clone, PartialEq)]
pub struct User {
    pub email: Email,
    pub password: Password,
    pub requires_2fa: bool,
}

impl User {
    pub fn new(email: &str, password: &str, requires_2fa: bool) -> Self {
        Self {
            email: Email::parse(email).unwrap(),
            password: Password::parse(password).unwrap(),
            requires_2fa: requires_2fa,
        }
    }
}