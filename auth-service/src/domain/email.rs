#[derive(Debug, Clone, PartialEq, Hash, Eq)]
pub struct Email(String);

impl Email {
    pub fn parse(email: String) -> Result<Self, &'static str> {
        if email.contains('@') && !email.is_empty() {
            Ok(Self(email))
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
