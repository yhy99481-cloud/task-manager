use anyhow::{anyhow, Result};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

const JWT_EXPIRATION_MINUTES: u64 = 60;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub sub: String, // user_id
    pub username: String,
    pub exp: usize,
}

pub fn generate_token(user_id: &str, username: &str, secret: &str) -> Result<String> {
    let expiration = SystemTime::now()
        .duration_since(UNIX_EPOCH)?
        .as_secs()
        + (JWT_EXPIRATION_MINUTES * 60);

    let claims = Claims {
        sub: user_id.to_string(),
        username: username.to_string(),
        exp: expiration as usize,
    };

    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )?;

    Ok(token)
}

pub fn verify_token(token: &str, secret: &str) -> Result<Claims> {
    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::default(),
    )?;

    Ok(token_data.claims)
}

pub fn extract_user_id(token: &str, secret: &str) -> Result<String> {
    let claims = verify_token(token, secret)?;
    Ok(claims.sub)
}
