use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};
use std::env;
use anyhow::Result;

// Define the JWT claims structure
#[derive(Debug, Serialize, Deserialize)]
pub struct UserClaims {
    pub user_id: String,
    pub email: String,
    pub exp: usize, // Expiration time (as UTC timestamp)
    pub iat: usize, // Issued at (as UTC timestamp)
}

// Helper function to get current timestamp
fn get_current_timestamp() -> usize {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs() as usize
}

pub fn generate_jwt(user_id: &str, email: &str) -> Result<String> {
    // Get JWT secret from environment variable
    let secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set");

    // Set expiration time to 100 years from now (effectively forever for a local app)
    let exp = get_current_timestamp() + (100 * 365 * 24 * 60 * 60); // ~100 years in seconds
    let iat = get_current_timestamp();

    let claims = UserClaims {
        user_id: user_id.to_string(),
        email: email.to_string(),
        exp,
        iat,
    };
    
    // Create header
    let header = Header::new(Algorithm::HS256);
    
    // Encode the JWT
    let token = encode(
        &header,
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )?;
    
    Ok(token)
}

pub fn validate_jwt(token: &str) -> Result<UserClaims> {
    // Get JWT secret from environment variable
    let secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    
    // Decode and validate the JWT
    let validation = Validation::new(Algorithm::HS256);
    
    let token_data = decode::<UserClaims>(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &validation,
    )?;
    
    Ok(token_data.claims)
}

// Function to check if token needs refresh (if it's close to expiring)
pub fn needs_refresh(token: &str) -> Result<bool> {
    let claims = validate_jwt(token)?;
    
    // Get current time
    let now = get_current_timestamp();
    
    // If token is more than 45 minutes old (75% of its lifetime), it needs refresh
    Ok(claims.exp - now < 900) // 900 = 15 minutes left before expiration
}