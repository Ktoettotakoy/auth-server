use bcrypt::{hash, verify, DEFAULT_COST};
use crate::error::Error;
use crate::models::{Result};

/// Hash a password using bcrypt
pub fn hash_password(password: &str) -> Result<String> {
    hash(password, DEFAULT_COST)
        .map_err(|_| Error::PasswordHashError)
}

/// Verify a password against its hash
pub fn verify_password(password: &str, hash: &str) -> Result<bool> {
    verify(password, hash)
        .map_err(|_| Error::PasswordVerificationError)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_password_hashing() {
        let password = "testpassword123";

        // Hash the password
        let hashed = hash_password(password).unwrap();

        // Verify correct password
        assert!(verify_password(password, &hashed).unwrap());

        // Verify incorrect password fails
        assert!(!verify_password("wrongpassword", &hashed).unwrap());
    }

    #[test]
    fn test_hash_uniqueness() {
        let password = "samepassword";

        let hash1 = hash_password(password).unwrap();
        let hash2 = hash_password(password).unwrap();

        // Same password should produce different hashes (due to salt)
        assert_ne!(hash1, hash2);

        // But both should verify correctly
        assert!(verify_password(password, &hash1).unwrap());
        assert!(verify_password(password, &hash2).unwrap());
    }
}
