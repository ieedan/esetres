use jsonwebtoken::{
    decode, encode, Algorithm, DecodingKey, EncodingKey, Header, TokenData, Validation,
};
use serde::{Deserialize, Serialize};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use crate::config;

const ISSUER: &str = "esetres-cli";

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    /// Name of the token
    pub nm: String,
    /// Bucket scope
    pub scp: String,
    /// Created at
    pub crt: Duration,
    /// Issuer
    pub iss: String,
}

/// Create a new token
pub fn create(name: String, scope: String) -> Result<String, Box<dyn std::error::Error>> {
    let config = config::get();

    let timestamp = SystemTime::now().duration_since(UNIX_EPOCH)?;

    let claims = Claims {
        nm: name,
        scp: scope,
        crt: timestamp,
        iss: ISSUER.to_string(),
    };

    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(config.token_secret.as_ref()),
    )?;

    Ok(token)
}

/// Attempts to validate the jwt upon success returns the token with its claims
pub fn validate(token: String) -> Result<TokenData<Claims>, jsonwebtoken::errors::Error> {
    let config = config::get();

    let validation = create_validator();

    decode::<Claims>(
        &token.as_str(),
        &DecodingKey::from_secret(config.token_secret.as_ref()),
        &validation,
    )
}

/// Creates a new instance of a validator
fn create_validator() -> Validation {
    let mut validation = Validation::new(Algorithm::HS256);
    validation.set_issuer(&[ISSUER]);
    validation.set_required_spec_claims(&["iss", "nm", "crt", "scp"]);

    validation
}
