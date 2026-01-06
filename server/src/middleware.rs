use axum::{http::Request, middleware::Next, response::Response};
use crate::user_jwt;

#[derive(Clone, Debug)]
pub struct UserContext {
    pub user_id: String,
    pub email: String,
}

pub async fn extract_user(
    mut req: Request<axum::body::Body>,
    next: Next,
) -> Result<Response, axum::http::StatusCode> {
    tracing::debug!("Extracting user context");

    // Skip authentication for public paths
    let public_paths = ["/login", "/register", "/staging_login", "/health"];
    if public_paths.contains(&req.uri().path()) {
        tracing::debug!("Skipping user extraction for public path: {}", req.uri().path());
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
    tracing::debug!("Authenticating user");

    // Skip authentication for public paths
    let public_paths = ["/login", "/register", "/staging_login", "/health"];
    if public_paths.contains(&req.uri().path()) {
        tracing::debug!("Skipping authentication for public path: {}", req.uri().path());
        return Ok(next.run(req).await);
    }

    let token = req
        .headers()
        .get("Authorization")
        .and_then(|v| v.to_str().ok())
        .map(|v| v.trim_start_matches("Bearer ").to_string())
        .ok_or(axum::http::StatusCode::UNAUTHORIZED)?;

    // Validate JWT token
    let claims = user_jwt::validate_jwt(&token).map_err(|e| {
        tracing::warn!("JWT validation failed: {:?}", e);
        axum::http::StatusCode::UNAUTHORIZED
    })?;

    tracing::debug!("User authenticated: {}", claims.user_id);

    // Check if the token needs to be refreshed (within 15 minutes of expiration)
    match user_jwt::needs_refresh(&token) {
        Ok(true) => {
            tracing::debug!("JWT token needs refresh for user: {}", claims.user_id);
            
            // Generate new token
            if let Ok(new_token) = user_jwt::generate_jwt(&claims.user_id, &claims.plan) {
                tracing::debug!("Generated new JWT token for user: {}", claims.user_id);
                
                // Run the next middleware and get the response
                let mut response = next.run(req).await;
                
                // Add the new token to the response headers
                response.headers_mut().insert(
                    "X-New-Auth-Token",
                    axum::http::HeaderValue::from_str(&new_token)
                        .unwrap_or_else(|_| axum::http::HeaderValue::from_static("")),
                );
                
                return Ok(response);
            }
        }
        Ok(false) => {
            // Token doesn't need refresh, continue
        }
        Err(e) => {
            tracing::debug!("Error checking if token needs refresh: {:?}", e);
            // Continue with request even if refresh check fails
        }
    }

    Ok(next.run(req).await)
}
