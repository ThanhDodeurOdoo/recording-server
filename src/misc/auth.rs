use jsonwebtoken::{decode, DecodingKey, Validation, Algorithm };
use serde::de::DeserializeOwned;
use crate::config::{AUTH_KEY };
pub fn verify<T>(token: &str) -> actix_web::Result<T, jsonwebtoken::errors::Error>
where
    T: DeserializeOwned,
{
    let validation = Validation::new(Algorithm::HS256);
    let token_data = decode(
        token,
        &DecodingKey::from_secret(AUTH_KEY.as_ref()),
        &validation,
    )?;
    Ok(token_data.claims)
}
