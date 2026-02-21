use axum::{
    routing::{get, post, put, patch, delete},
    Router,
    Json,
};
use dotenv::dotenv;
use std::sync::Arc;
use tower_http::cors::{Any, CorsLayer};
use tower_http::trace::TraceLayer;

mod db;
mod handlers;
mod middleware;
mod models;
mod utils;

use handlers::auth::AppState;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();

    // Initialize tracing
    tracing_subscriber::fmt::init();

    // Load environment variables
    let database_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "sqlite:///data/tasks.db".to_string());
    let jwt_secret = std::env::var("JWT_SECRET")
        .unwrap_or_else(|_| "dev-secret-key-change-in-production".to_string());
    let host = std::env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
    let port = std::env::var("PORT")
        .unwrap_or_else(|_| "3000".to_string())
        .parse::<u16>()
        .unwrap_or(3000);

    println!("🚀 Starting Task Manager API...");
    println!("📦 Database: {}", database_url);
    println!("🔐 JWT Secret: {}", if jwt_secret.contains("change") { "[DEFAULT]" } else { "[SET]" });

    // Create database pool
    let pool = db::create_pool(&database_url).await?;
    println!("✅ Database connected");

    let state = AppState {
        pool,
        jwt_secret: jwt_secret.clone(),
    };

    // Create CORS layer
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    // Build all routes with proper state
    let app = Router::new()
        // Health check
        .route("/health", get(|| async { Json(serde_json::json!({ "status": "ok" })) }))
        // Auth routes (no auth middleware)
        .route("/api/register", post(handlers::auth::register))
        .route("/api/login", post(handlers::auth::login))
        // Task routes
        .route("/api/tasks", get(handlers::tasks::get_tasks).post(handlers::tasks::create_task))
        .route("/api/tasks/:id", get(handlers::tasks::get_task).put(handlers::tasks::update_task).delete(handlers::tasks::delete_task))
        .route("/api/tasks/:id/status", patch(handlers::tasks::update_task_status))
        // Apply CORS
        .layer(cors)
        // Apply tracing
        .layer(TraceLayer::new_for_http())
        // Apply auth middleware to task routes
        .route_layer(axum::middleware::from_fn_with_state(
            jwt_secret,
            middleware::auth_middleware,
        ))
        // Set state
        .with_state(state);

    // Run the server
    let addr = format!("{}:{}", host, port);
    let listener = tokio::net::TcpListener::bind(&addr).await?;
    println!("✅ Server listening on http://{}", addr);
    println!("📝 API endpoints:");
    println!("   GET  /health");
    println!("   POST /api/register");
    println!("   POST /api/login");
    println!("   GET  /api/tasks");
    println!("   POST /api/tasks");
    println!("   PUT  /api/tasks/:id");
    println!("   DELETE /api/tasks/:id");
    println!("   PATCH /api/tasks/:id/status");
    println!();

    axum::serve(listener, app).await?;

    Ok(())
}
