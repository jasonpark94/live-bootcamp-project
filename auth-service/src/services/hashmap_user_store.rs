use std::collections::HashMap;

use crate::domain::{Email, Password, User, UserStore, UserStoreError};


#[derive(Default)]
pub struct HashmapUserStore {
    users: HashMap<String, User>,
}

#[async_trait::async_trait]
impl UserStore for HashmapUserStore {
    async fn add_user(&mut self, user: User) -> Result<(), UserStoreError> {
        if self.users.contains_key(&user.email.as_ref().to_string()) {
            return Err(UserStoreError::UserAlreadyExists);
        }

        self.users.insert(user.email.as_ref().to_string(), user);
        Ok(())
    }

    async fn get_user(&self, email: Email) -> Result<User, UserStoreError> {
        match self.users.get(&email.as_ref().to_string()) {
            Some(user) => Ok(user.clone()),
            None => Err(UserStoreError::UserNotFound),
        }
    }

    async fn validate_user(&self, email: Email, password: Password) -> Result<(), UserStoreError> {
        match self.users.get(&email.as_ref().to_string()) {
            Some(user) => {
                if user.password == password {
                    Ok(())
                } else {
                    Err(UserStoreError::InvalidCredentials)
                }
            }
            None => Err(UserStoreError::UserNotFound),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::domain::{Email, Password};

    use super::*;

    #[tokio::test]
    async fn test_add_user() {
        let mut store = HashmapUserStore::default();
        let user = User {
            email: Email::parse("test@test.com").unwrap(), 
            password: Password::parse("password").unwrap(),
            requires_2fa: false,
        };
        assert_eq!(store.add_user(user).await, Ok(()));
    }

    #[tokio::test]
    async fn test_get_user() {

        let user1 = User {
            email: Email::parse("test1@test.com").unwrap(),
            password: Password::parse("password").unwrap(),
            requires_2fa: false,
        };
        
        let user2 = User {
            email: Email::parse("test2@test.com").unwrap(),
            password: Password::parse("password").unwrap(),
            requires_2fa: false,
        };


        let mut store = HashmapUserStore::default();
        store.add_user(user1.clone()).await.unwrap();
        store.add_user(user2.clone()).await.unwrap();

        assert_eq!(store.get_user(Email::parse("test1@test.com").unwrap()).await.unwrap(), user1);
    }

    #[tokio::test]
    async fn test_validate_user() {

        let store = HashmapUserStore::default();

        let user1 = User {
            email: Email::parse("test1@test.com").unwrap(),
            password: Password::parse("password").unwrap(),
            requires_2fa: false,
        };
        
        let mut store = HashmapUserStore::default();
        store.add_user(user1.clone()).await.unwrap();

        assert_eq!(store.validate_user(user1.email, user1.password).await, Ok(()));
    }
}
