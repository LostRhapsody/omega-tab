use axum::{
    response::Response,
    middleware::Next,
    extract::Request,
};

#[derive(Clone, Debug)]
pub struct UserContext {
    pub user_id: String,
    pub email: String,
}

pub async fn extract_user(
    mut req: Request,
    next: Next,
) -> Result<Response, axum::http::StatusCode> {
    println!("Extracting user context");
    let headers = req.headers();
    let user_id = headers
        .get("X-User-Id")
        .and_then(|v| v.to_str().ok())
        .map(|v| v.to_string())
        .ok_or(axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;

    let email = headers
        .get("X-User-Email")
        .and_then(|v| v.to_str().ok())
        .map(|v| v.to_string())
        .ok_or(axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;

    let user_context = UserContext { user_id, email };
    req.extensions_mut().insert(user_context);

    Ok(next.run(req).await)
}
