use thiserror::Error;
use argon2::password_hash::Error as Argon2Error;

#[derive(Error, Debug)]
pub enum DatabaseError {
    #[error("failed to hash password {0}")]
    Hashing(Argon2Error),

    #[error("Database error: {0}")]
    Diesel(#[from] diesel::result::Error)
}

impl From<Argon2Error> for DatabaseError {
    fn from(err: Argon2Error) -> Self {
        Self::Hashing(err)
    }
}