use axum::{
    extract::{Request, State},
    http::StatusCode,
    middleware::Next,
    response::Response,
};

use crate::utils::jwt;

const AUTH_HEADER: &str = "authorization";
const BEARER_PREFIX: &str = "Bearer ";

// Public paths that don't require authentication
const PUBLIC_PATHS: &[&str] = &["/api/register", "/api/login", "/health"];

pub async fn auth_middleware(
    State(jwt_secret): State<String>,
    mut request: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    // Skip auth for public paths
    let path = request.uri().path();
    if PUBLIC_PATHS.iter().any(|public| path.starts_with(public)) {
        return Ok(next.run(request).await);
    }

    let auth_header = request
        .headers()
        .get(AUTH_HEADER)
        .and_then(|h| h.to_str().ok())
        .ok_or_else(|| StatusCode::UNAUTHORIZED)?;

    if !auth_header.starts_with(BEARER_PREFIX) {
        return Err(StatusCode::UNAUTHORIZED);
    }

    let token = &auth_header[BEARER_PREFIX.len()..];

    let claims = jwt::verify_token(token, &jwt_secret)
        .map_err(|_| StatusCode::UNAUTHORIZED)?;

    // Store user_id in request extensions
    request.extensions_mut().insert(claims.sub);

    Ok(next.run(request).await)
}
