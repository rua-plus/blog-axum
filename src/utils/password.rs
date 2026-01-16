use argon2::{Algorithm, Argon2, Params, PasswordHash, PasswordHasher, PasswordVerifier, Version};
use password_hash::{SaltString, rand_core::OsRng};
use std::fmt;

use crate::error::AppError;

#[derive(Debug)]
pub enum PasswordError {
    HashError(String),
    InvalidHash(String),
}

impl fmt::Display for PasswordError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PasswordError::HashError(msg) => write!(f, "Failed to hash password: {}", msg),
            PasswordError::InvalidHash(msg) => write!(f, "Invalid password hash: {}", msg),
        }
    }
}

impl std::error::Error for PasswordError {}

impl From<argon2::Error> for PasswordError {
    fn from(err: argon2::Error) -> Self {
        PasswordError::HashError(err.to_string())
    }
}

impl From<password_hash::Error> for PasswordError {
    fn from(err: password_hash::Error) -> Self {
        PasswordError::InvalidHash(err.to_string())
    }
}

impl From<PasswordError> for AppError {
    fn from(err: PasswordError) -> Self {
        AppError::new(err)
    }
}

/// Hashes a password using Argon2id algorithm with recommended minimum parameters:
/// - Memory: 19 MiB
/// - Iterations: 2
/// - Parallelism: 1
pub fn hash_password(password: &str) -> Result<String, PasswordError> {
    let salt = SaltString::generate(&mut OsRng);

    // Argon2id with minimum recommended parameters
    let params = Params::new(
        19 * 1024, // 19 MiB memory
        2,         // 2 iterations
        1,         // 1 parallelism
        None,
    )?;

    let argon2 = Argon2::new(Algorithm::Argon2id, Version::V0x13, params);

    let hash = argon2.hash_password(password.as_bytes(), &salt)?;

    Ok(hash.to_string())
}

/// Verifies a password against a stored hash
pub fn verify_password(password: &str, hash: &str) -> Result<(), PasswordError> {
    let parsed_hash = PasswordHash::new(hash)?;

    let argon2 = Argon2::default();

    argon2.verify_password(password.as_bytes(), &parsed_hash)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash_and_verify_password() {
        let password = "test_password_123";

        let hash = hash_password(password).expect("Failed to hash password");
        assert!(!hash.is_empty());
        assert!(hash.starts_with("$argon2id$v=19$m=19456,t=2,p=1$"));

        let result = verify_password(password, &hash);
        assert!(result.is_ok());

        let wrong_password_result = verify_password("wrong_password", &hash);
        assert!(wrong_password_result.is_err());
    }

    #[test]
    fn test_hash_different_salts() {
        let password = "test_password_123";

        let hash1 = hash_password(password).expect("Failed to hash password 1");
        let hash2 = hash_password(password).expect("Failed to hash password 2");

        assert_ne!(hash1, hash2);
    }
}
