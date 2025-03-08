use axum::{http::Request, middleware::Next, response::Response};
use jsonwebtoken::{decode, decode_header, Algorithm, DecodingKey, Validation};
use serde::{Deserialize, Serialize};
use crate::user_jwt;

#[derive(Clone, Debug)]
pub struct UserContext {
    pub user_id: String,
    pub email: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    // email: String,
    exp: usize,
    nbf: usize,
    azp: String,
}

pub async fn extract_user(
    mut req: Request<axum::body::Body>,
    next: Next,
) -> Result<Response, axum::http::StatusCode> {
    println!("Extracting user context");

    // Skip authentication for the staging login path
    if req.uri().path() == "/staging_login" {
        println!("Skipping user extraction for staging login path");
        return Ok(next.run(req).await);
    }

    let headers = req.headers();
    let user_id = headers
        .get("X-User-Id")
        .and_then(|v| v.to_str().ok())
        .map(|v| v.to_string())
        .ok_or(axum::http::StatusCode::UNAUTHORIZED)?;

    let email = headers
        .get("X-User-Email")
        .and_then(|v| v.to_str().ok())
        .map(|v| v.to_string())
        .ok_or(axum::http::StatusCode::UNAUTHORIZED)?;

    let user_context = UserContext { user_id, email };
    req.extensions_mut().insert(user_context);

    Ok(next.run(req).await)
}

pub async fn authenticate_user(
    req: Request<axum::body::Body>,
    next: Next,
) -> Result<Response, axum::http::StatusCode> {
    println!("Authenticating user");

    // Skip authentication for the staging login path
    if req.uri().path() == "/staging_login" {
        println!("Skipping authentication for staging login path");
        return Ok(next.run(req).await);
    }

    let token = req
        .headers()
        .get("Authorization")
        .and_then(|v| v.to_str().ok())
        .map(|v| v.trim_start_matches("Bearer ").to_string())
        .ok_or(axum::http::StatusCode::UNAUTHORIZED)?;

    let jwks_url = std::env::var("CLERK_JWKS_URL")
        .unwrap_or_else(|_| "https://api.clerk.com/v1/jwks".to_string());
    let bearer_token = std::env::var("CLERK_API_KEY").map_err(|e| {
        eprintln!("Error: {}", e);
        axum::http::StatusCode::INTERNAL_SERVER_ERROR
    })?;

    let client = reqwest::Client::new();
    let jwks: serde_json::Value = client
        .get(&jwks_url)
        .bearer_auth(bearer_token)
        .send()
        .await
        .map_err(|e| {
            eprintln!("Error: {}", e);
            axum::http::StatusCode::INTERNAL_SERVER_ERROR
        })?
        .json()
        .await
        .map_err(|e| {
            eprintln!("Error: {}", e);
            axum::http::StatusCode::INTERNAL_SERVER_ERROR
        })?;

    let header = decode_header(&token).map_err(|e| {
        eprintln!("Error: {}", e);
        axum::http::StatusCode::UNAUTHORIZED
    })?;

    let kid = header.kid.ok_or(axum::http::StatusCode::UNAUTHORIZED)?;

    let jwk = jwks["keys"]
        .as_array()
        .ok_or(axum::http::StatusCode::UNAUTHORIZED)?
        .iter()
        .find(|&jwk| jwk["kid"] == kid)
        .ok_or(axum::http::StatusCode::UNAUTHORIZED)?;

    let decoding_key = DecodingKey::from_rsa_components(
        jwk["n"]
            .as_str()
            .ok_or(axum::http::StatusCode::UNAUTHORIZED)?,
        jwk["e"]
            .as_str()
            .ok_or(axum::http::StatusCode::UNAUTHORIZED)?,
    )
    .map_err(|e| {
        eprintln!("Error: {}", e);
        axum::http::StatusCode::UNAUTHORIZED
    })?;

    let mut validation = Validation::new(Algorithm::RS256);
    validation.set_audience(&[
        std::env::var("DOMAIN").unwrap_or_else(|_| "http://localhost:3000".to_string())
    ]);

    let token_data = decode::<Claims>(&token, &decoding_key, &validation).map_err(|e| {
        eprintln!("Error: {}", e);
        axum::http::StatusCode::UNAUTHORIZED
    })?;

    let claims = token_data.claims;

    if claims.exp < chrono::Utc::now().timestamp() as usize
        || claims.nbf > chrono::Utc::now().timestamp() as usize
    {
        println!("Token expired");
        return Err(axum::http::StatusCode::UNAUTHORIZED);
    }

    println!("User authenticated");

    // Check for user auth token and refresh if needed
    if let Some(auth_token) = req.headers().get("X-User-Authorization") {
        if let Ok(token_str) = auth_token.to_str() {
            // Check if the token needs to be refreshed
            match user_jwt::needs_refresh(token_str) {
                Ok(needs_refresh) => {
                    if needs_refresh {
                        println!("JWT token needs refresh");
                        
                        // Get user ID and plan from token
                        if let Ok(claims) = user_jwt::validate_jwt(token_str) {
                            // Generate new token
                            if let Ok(new_token) = user_jwt::generate_jwt(&claims.user_id, &claims.plan) {
                                println!("Generated new JWT token for user {}", claims.user_id);
                                
                                // Run the next middleware and get the response
                                let mut response = next.run(req).await;
                                
                                // Add the new token to the response headers
                                response.headers_mut().insert(
                                    "X-New-Auth-Token",
                                    axum::http::HeaderValue::from_str(&new_token).unwrap_or_else(|_| axum::http::HeaderValue::from_static("")),
                                );
                                
                                return Ok(response);
                            }
                        }
                    }
                }
                Err(e) => {
                    println!("Error checking if token needs refresh: {:?}", e);
                    // Continue with request even if refresh check fails
                }
            }
        }
    }

    Ok(next.run(req).await)
}
