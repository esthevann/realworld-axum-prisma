use argon2::{PasswordHash, Argon2};
use argon2::password_hash::SaltString;

use crate::AppResult;
use crate::error::AppError;


pub async fn hash_password(password: String) -> AppResult<String> {
    // Argon2 hashing is designed to be computationally intensive,
    // so we need to do this on a blocking thread.
    tokio::task::spawn_blocking(move || -> AppResult<String> {
        let salt = SaltString::generate(rand::thread_rng());
        Ok(
            PasswordHash::generate(Argon2::default(), password, salt.as_str())
                .map_err(|_e| AppError::HashingError)?
                .to_string(),
        )
    })
    .await
    .unwrap()
}

pub async fn verify_password(password: String, password_hash: String) -> AppResult<()> {
    tokio::task::spawn_blocking(move || -> AppResult<()> {
        let hash = PasswordHash::new(&password_hash)
            .map_err(|_e| AppError::HashingError)?;

        hash.verify_password(&[&Argon2::default()], password)
            .map_err(|e| match e {
                argon2::password_hash::Error::Password => AppError::Unathorized,
                _ => AppError::HashingError,
            })
    })
    .await
    .unwrap()
}