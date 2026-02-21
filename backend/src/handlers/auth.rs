use axum::{
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Json},
    Json as ResponseJson,
};
use serde::Serialize;

use bcrypt::{hash, verify, DEFAULT_COST};

use crate::models::{AuthResponse, LoginRequest, RegisterRequest, UserResponse};
use crate::utils::jwt;

// AppState is defined in main.rs
#[derive(Clone)]
pub struct AppState {
    pub pool: crate::db::DbPool,
    pub jwt_secret: String,
}

#[derive(Serialize)]
pub struct ErrorMessage {
    pub error: String,
}

pub async fn register(
    State(state): State<AppState>,
    Json(payload): Json<RegisterRequest>,
) -> Result<impl IntoResponse, (StatusCode, Json<ErrorMessage>)> {
    println!("📝 Registration attempt for user: {}", payload.username);

    // Validate input
    if payload.username.trim().is_empty() {
        println!("❌ Registration failed: username is empty");
        return Err((
            StatusCode::BAD_REQUEST,
            Json(ErrorMessage {
                error: "Username cannot be empty".to_string(),
            }),
        ));
    }

    if payload.password.len() < 6 {
        println!("❌ Registration failed: password too short");
        return Err((
            StatusCode::BAD_REQUEST,
            Json(ErrorMessage {
                error: "Password must be at least 6 characters".to_string(),
            }),
        ));
    }

    // Hash password
    let password_hash = match hash(&payload.password, DEFAULT_COST) {
        Ok(hash) => hash,
        Err(e) => {
            println!("❌ Registration failed: password hash error - {}", e);
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorMessage {
                    error: "Failed to process password".to_string(),
                }),
            ));
        }
    };

    // Create user
    let user = crate::models::User::new(payload.username.clone(), password_hash);

    // Insert into database
    let result = sqlx::query(
        "INSERT INTO users (id, username, password_hash, created_at) VALUES (?, ?, ?, ?)"
    )
    .bind(&user.id)
    .bind(&user.username)
    .bind(&user.password_hash)
    .bind(user.created_at)
    .execute(&state.pool)
    .await;

    match result {
        Ok(_) => {
            println!("✅ User registered successfully: {}", user.username);

            // Generate token
            let token = match jwt::generate_token(&user.id, &user.username, &state.jwt_secret) {
                Ok(token) => token,
                Err(e) => {
                    println!("❌ Failed to generate token: {}", e);
                    return Err((
                        StatusCode::INTERNAL_SERVER_ERROR,
                        Json(ErrorMessage {
                            error: "Failed to generate authentication token".to_string(),
                        }),
                    ));
                }
            };

            let response = AuthResponse {
                token,
                user: UserResponse::from(user.clone()),
            };

            Ok((StatusCode::CREATED, ResponseJson(response)))
        }
        Err(sqlx::Error::Database(db_err)) => {
            if db_err.to_string().contains("UNIQUE") {
                println!("❌ Registration failed: username already exists");
                Err((
                    StatusCode::CONFLICT,
                    Json(ErrorMessage {
                        error: "Username already exists".to_string(),
                    }),
                ))
            } else {
                println!("❌ Database error: {}", db_err);
                Err((
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(ErrorMessage {
                        error: format!("Database error: {}", db_err),
                    }),
                ))
            }
        }
        Err(e) => {
            println!("❌ Unexpected error: {}", e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorMessage {
                    error: format!("Internal server error: {}", e),
                }),
            ))
        }
    }
}

pub async fn login(
    State(state): State<AppState>,
    Json(payload): Json<LoginRequest>,
) -> Result<impl IntoResponse, (StatusCode, Json<ErrorMessage>)> {
    println!("🔐 Login attempt for user: {}", payload.username);

    // Find user
    let user = match sqlx::query_as::<_, crate::models::User>(
        "SELECT * FROM users WHERE username = ?"
    )
    .bind(&payload.username)
    .fetch_optional(&state.pool)
    .await
    {
        Ok(Some(user)) => user,
        Ok(None) => {
            println!("❌ Login failed: user not found");
            return Err((
                StatusCode::UNAUTHORIZED,
                Json(ErrorMessage {
                    error: "Invalid username or password".to_string(),
                }),
            ));
        }
        Err(e) => {
            println!("❌ Database error: {}", e);
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorMessage {
                    error: "Database error".to_string(),
                }),
            ));
        }
    };

    // Verify password
    match verify(&payload.password, &user.password_hash) {
        Ok(true) => {}
        Ok(false) => {
            println!("❌ Login failed: invalid password");
            return Err((
                StatusCode::UNAUTHORIZED,
                Json(ErrorMessage {
                    error: "Invalid username or password".to_string(),
                }),
            ));
        }
        Err(e) => {
            println!("❌ Password verification error: {}", e);
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorMessage {
                    error: "Authentication error".to_string(),
                }),
            ));
        }
    }

    println!("✅ User logged in successfully: {}", user.username);

    // Generate token
    let token = match jwt::generate_token(&user.id, &user.username, &state.jwt_secret) {
        Ok(token) => token,
        Err(e) => {
            println!("❌ Failed to generate token: {}", e);
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorMessage {
                    error: "Failed to generate authentication token".to_string(),
                }),
            ));
        }
    };

    let response = AuthResponse {
        token,
        user: UserResponse::from(user),
    };

    Ok(ResponseJson(response))
}
