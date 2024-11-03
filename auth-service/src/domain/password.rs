#[derive(Debug, Clone, PartialEq)]
pub struct Password(String);

impl Password {
    pub fn parse(password: &str) -> Result<Self, &'static str> {
        if password.len() >= 8 {
            Ok(Self(password.to_string()))
        } else {
            Err("Password must be at least 8 characters long")
        }
    }
}

impl AsRef<str> for Password {
    fn as_ref(&self) -> &str {
        &self.0
    }
}