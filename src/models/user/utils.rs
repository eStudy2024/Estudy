use bcrypt::{hash, verify, BcryptResult, DEFAULT_COST};

use crate::error::AppError;


pub fn hash_password(naive_pw: &str) -> BcryptResult<String> {
    hash(naive_pw, DEFAULT_COST)
}

pub fn verify_password(naive_pw: &str, hashed_pw: &str) -> Result<bool,AppError> {
    Ok(verify(naive_pw, hashed_pw)?)
}