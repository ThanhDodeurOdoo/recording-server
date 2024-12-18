use crate::config::AUTH_KEY;
use jsonwebtoken::{decode, Algorithm, DecodingKey, Validation};
use serde::de::DeserializeOwned;
use std::ops::Deref;

// TODO may not need JWT if the recording server API is network isolated.
pub fn verify<T>(token: &str) -> Result<T, jsonwebtoken::errors::Error>
where
    T: DeserializeOwned,
{
    let validation = Validation::new(Algorithm::HS256);
    let token_data =
        decode(token, &DecodingKey::from_secret(AUTH_KEY.deref().as_bytes()), &validation)?;
    Ok(token_data.claims)
}
