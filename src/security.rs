use crate::result::{OpenCliError, Result};
use argon2::password_hash::SaltString;
use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use rand_core::OsRng;
use sha2::{Digest, Sha256};
use std::path::Path;
use tokio::fs;

#[derive(Default)]
pub struct SecurityManager {
    argon2: Argon2<'static>,
}

impl SecurityManager {
    pub fn new() -> Self {
        Self::default()
    }

    pub async fn hash_file(&self, file_path: &Path) -> Result<String> {
        let content = fs::read(file_path).await?;
        let mut hasher = Sha256::new();
        hasher.update(&content);
        let file_hash = hasher.finalize();

        let salt = SaltString::generate(&mut OsRng);
        let argon2_hash = self
            .argon2
            .hash_password(&file_hash, &salt)
            .map_err(|e| OpenCliError::Process(format!("Failed to hash file: {}", e).into()))?;

        Ok(argon2_hash.to_string())
    }

    pub async fn verify_file(&self, file_path: &Path, stored_hash: &str) -> Result<bool> {
        let content = fs::read(file_path).await?;
        let mut hasher = Sha256::new();
        hasher.update(&content);
        let file_hash = hasher.finalize();

        let parsed_hash = PasswordHash::new(stored_hash)
            .map_err(|e| OpenCliError::Process(format!("Invalid hash format: {}", e).into()))?;

        match self.argon2.verify_password(&file_hash, &parsed_hash) {
            Ok(_) => Ok(true),
            Err(_) => Ok(false),
        }
    }

    pub async fn hash_file_content(&self, content_hash: &[u8]) -> Result<String> {
        let salt = SaltString::generate(&mut OsRng);
        let argon2_hash = self
            .argon2
            .hash_password(content_hash, &salt)
            .map_err(|e| OpenCliError::Process(format!("Failed to hash content: {}", e).into()))?;

        Ok(argon2_hash.to_string())
    }
}
