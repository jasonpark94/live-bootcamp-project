#[derive(Debug, Clone, PartialEq)]
pub struct Email(String);

impl Email {
    pub fn parse(email: &str) -> Result<Self, &'static str> {
        if email.contains('@') && !email.is_empty() {
            Ok(Self(email.to_string()))
        } else {
            Err("Invalid email address")
        }
    }
}

impl AsRef<str> for Email {
    fn as_ref(&self) -> &str {
        &self.0
    }
}