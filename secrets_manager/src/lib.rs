use std::env;

#[derive(Copy, Clone)]
pub enum SecretsStorage {
    Env
}

#[derive(Debug)]
pub enum SecretsError {
    NotFound,
    InvalidSecret
}

#[derive(Copy, Clone)]
pub struct SecretsManager {
    secrets_storage: SecretsStorage
}

impl SecretsManager {

    fn get_env_secret(self, secret_key: &str) -> Result<String, SecretsError> {
        match env::var(secret_key) {
            Ok(s) => Ok(s),
            Err(env::VarError::NotPresent) => Err(SecretsError::NotFound),
            Err(env::VarError::NotUnicode(_)) => Err(SecretsError::InvalidSecret)
        }
    }

    fn set_env_secret(self, secret_key: &str, secret_value: &str) {
        env::set_var(secret_key, secret_value)
    }

    pub fn new(storage_method: SecretsStorage) -> SecretsManager {
        SecretsManager {
            secrets_storage: storage_method
        }
    }

    pub fn get_secret(&self, secret_name: &str) -> Result<String, SecretsError> {
        match self.secrets_storage {
            SecretsStorage::Env => self.get_env_secret(secret_name),
        }
    }

    pub fn set_secret(&self, secret_key: &str, secret_value: &str) {
        self.set_env_secret(secret_key, secret_value)
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    #[test]
    fn test_get_secret() {
        env::set_var("test_secret", "testing");

        let secrets_manager = SecretsManager::new(SecretsStorage::Env);
        let secret = secrets_manager.get_secret("test_secret").unwrap();
        assert_eq!(secret, "testing");
    }

    #[test]
    fn test_set_secret() {
        let secrets_manager = SecretsManager::new(SecretsStorage::Env);
        secrets_manager.set_secret("test", "test2");

        let secret = secrets_manager.get_secret("test").unwrap();
        assert_eq!(secret, "test2");
    }
}