use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use chrono::{Duration, Utc};
use anyhow::{Result, Error};

/// Claims structure for the JWT payload
#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub project_id: String,
    pub exp: usize, // Expiration time as a timestamp
}

pub struct JwtAuth {
    pub secret: String,
}

impl JwtAuth {
    /// Creates a new JwtAuth instance
    pub fn new(secret: String) -> Self {
        Self { secret }
    }

    /// Generates a JWT for the given project ID
    pub fn generate_token(&self, project_id: &str) -> Result<String> {
        let expiration = Utc::now()
            .checked_add_signed(Duration::hours(24))
            .ok_or_else(|| Error::msg("Failed to compute expiration time"))?
            .timestamp();

        let claims = Claims {
            project_id: project_id.to_string(),
            exp: expiration as usize,
        };

        let token = encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(self.secret.as_bytes()),
        )?;

        Ok(token)
    }

    /// Validates a JWT and returns the decoded claims if valid
    pub fn validate_token(&self, token: &str) -> Result<Claims> {
        let decoded = decode::<Claims>(
            token,
            &DecodingKey::from_secret(self.secret.as_bytes()),
            &Validation::new(Algorithm::HS256),
        )?;

        Ok(decoded.claims)
    }
}