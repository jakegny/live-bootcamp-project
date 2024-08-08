use std::collections::{hash_map::Entry, HashMap};

use crate::domain::{
    data_stores::{UserStore, UserStoreError},
    email::Email,
    password::Password,
    user::User,
};

#[derive(Default)]
pub struct HashmapUserStore {
    users: HashMap<Email, User>,
}

#[async_trait::async_trait]
impl UserStore for HashmapUserStore {
    async fn add_user(&mut self, user: User) -> Result<(), UserStoreError> {
        match self.users.entry(user.email.clone()) {
            Entry::Occupied(_) => Err(UserStoreError::UserAlreadyExists),
            Entry::Vacant(entry) => {
                entry.insert(user);
                Ok(())
            }
        }
    }

    async fn get_user(&self, email: &Email) -> Result<User, UserStoreError> {
        match self.users.get(&email) {
            Some(user) => Ok(user.clone()),
            None => Err(UserStoreError::UserNotFound),
        }
    }

    async fn validate_user(
        &self,
        email: &Email,
        password: &Password,
    ) -> Result<(), UserStoreError> {
        match self.users.get(email) {
            Some(user) => {
                if user.password == *password {
                    Ok(())
                } else {
                    Err(UserStoreError::InvalidCredentials)
                }
            }
            None => Err(UserStoreError::UserNotFound),
        }
    }
}

// TODO: Add unit tests for your `HashmapUserStore` implementation
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_add_user() {
        let email = Email::parse("test@email.com").unwrap();
        let password = Password::parse("passwordtest").unwrap();
        let user = User::new(email, password, false);

        let mut store = HashmapUserStore::default();

        let result = store.add_user(user.clone()).await;

        assert_eq!(result, Ok(()));
    }

    #[tokio::test]
    async fn test_get_user() {
        let email = Email::parse("test@email.com").unwrap();
        let password = Password::parse("passwordtest").unwrap();
        let user = User::new(email, password, false);

        let mut store = HashmapUserStore::default();

        let _ = store.add_user(user.clone()).await;
        let result = store.get_user(&user.email).await;

        assert_eq!(result, Ok(user));
    }

    #[tokio::test]
    async fn test_validate_user() {
        let email = Email::parse("test@email.com").unwrap();
        let password = Password::parse("passwordtest").unwrap();
        let user = User::new(email, password, false);

        let mut store = HashmapUserStore::default();

        let _ = store.add_user(user.clone()).await;
        let result = store.validate_user(&user.email, &user.password).await;

        assert_eq!(result, Ok(()));
    }
}
