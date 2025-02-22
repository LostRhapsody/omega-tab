use axum::{http::Request, middleware::Next, response::Response};
use jsonwebtoken::{decode, decode_header, Algorithm, DecodingKey, Validation};
use serde::{Deserialize, Serialize};

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

    println!("JWKS: {:?}", jwks);

    let header = decode_header(&token).map_err(|e| {
        eprintln!("Error: {}", e);
        axum::http::StatusCode::UNAUTHORIZED
    })?;

    println!("Header: {:?}", header);

    let kid = header.kid.ok_or(axum::http::StatusCode::UNAUTHORIZED)?;

    println!("Kid: {:?}", kid);

    let jwk = jwks["keys"]
        .as_array()
        .ok_or(axum::http::StatusCode::UNAUTHORIZED)?
        .iter()
        .find(|&jwk| jwk["kid"] == kid)
        .ok_or(axum::http::StatusCode::UNAUTHORIZED)?;

    println!("JWK: {:?}", jwk);

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

    println!("Have decoding key");

    let mut validation = Validation::new(Algorithm::RS256);
    validation.set_audience(&[
        std::env::var("DOMAIN").unwrap_or_else(|_| "http://localhost:3000".to_string())
    ]);

    let token_data = decode::<Claims>(&token, &decoding_key, &validation).map_err(|e| {
        eprintln!("Error: {}", e);
        axum::http::StatusCode::UNAUTHORIZED
    })?;

    println!("Token data: {:?}", token_data);

    let claims = token_data.claims;

    println!("Claims: {:?}", claims);

    if claims.exp < chrono::Utc::now().timestamp() as usize
        || claims.nbf > chrono::Utc::now().timestamp() as usize
    {
        println!("Token expired");
        return Err(axum::http::StatusCode::UNAUTHORIZED);
    }

    println!("User authenticated");

    Ok(next.run(req).await)
}
